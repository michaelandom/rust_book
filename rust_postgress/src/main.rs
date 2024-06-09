use sqlx::prelude::FromRow;
use sqlx::PgPool;
use sqlx::Row;
use std::sync::Arc;
use std::error::Error;
use futures::future::join_all;
use std::time::Instant;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::task;
#[derive(Debug,FromRow,Serialize,Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug)]
pub struct NotificationUser {
    pub id: i32,
    pub user_id: i32,
    pub email: String,
    pub delivery_type: i32
}

#[derive(Serialize,Deserialize)]
struct UserData {
    id: i32,
    name: String,
    email: String,
}


#[derive(Debug)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Debug)]
pub struct GroupUser {
    pub group_id: i32,
    pub user_id: i32,
}

pub async fn create_schema(db_pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations").run(db_pool).await?;
    Ok(())
}

pub async fn create_user(db_pool: &PgPool, user: User) -> Result<User , sqlx::Error> {
    sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
    .bind(&user.name)
    .bind(&user.email)
    .execute(db_pool)
    .await?;

    Ok(user)
}

pub async fn create_group(db_pool: &PgPool, group: Group) -> Result<Group, sqlx::Error> {
    sqlx::query("INSERT INTO groups (name, description)
    VALUES ($1, $2)")
    .bind(&group.name)
    .bind(&group.description)
    .execute(db_pool)
    .await?;
    Ok(group)
}
pub async fn create_notification(db_pool: &PgPool, notification_user: NotificationUser) -> Result<NotificationUser, sqlx::Error> {
    let file=sqlx::query("INSERT INTO notification_user (user_id, email,delivery_type)
    VALUES ($1, $2,$3)")
    .bind(&notification_user.user_id)
    .bind(&notification_user.email)
    .bind(&notification_user.delivery_type)
    .execute(db_pool)
    .await?;

    // println!("asada {:?}",file);
    Ok(notification_user)
}

pub async fn add_user_to_group(
    db_pool: &PgPool,
    group_user: GroupUser,
) -> Result<GroupUser, sqlx::Error> {

    sqlx::query("INSERT INTO group_users (group_id, user_id)
    VALUES ($1, $2)")
    .bind(&group_user.group_id)
    .bind(&group_user.user_id)
    .execute(db_pool)
    .await?;

    Ok(group_user)
}
async fn send_users_to_endpoint(users_stream: Vec<User>, db_pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let endpoint_url = "https://rustag.requestcatcher.com/notification";

    // let user_data: Vec<UserData> = users_stream
    //     .into_iter()
    //     .map(|user| UserData {
    //         id: user.id,
    //         name: user.name,
    //         email: user.email,
    //     })
    //     .collect();

    let request_body = json!({ "users": users_stream });

    let response = client
        .post(endpoint_url)
        .header("Content-Type", "application/json")
        .body(request_body.to_string())
        .send()
        .await?;

    if response.status().is_success() {
     let user_futures: Vec<_> = users_stream.into_iter()
     .map(|user| {
         let user = NotificationUser {
            id: user.id,
            user_id:user.id,
            email:user.email,
            delivery_type: 4,
        };
        create_notification(db_pool, user)
     })
     .collect();
    join_all(user_futures).await;
        println!("Users data sent successfully.");
    } else {
        println!("Failed to send users data: {:?}", response.status());
    }

    Ok(())
}
async fn read_users_in_batches(db_pool: &PgPool, start_offset: i32, end_offset: i32,db_pool2: &PgPool,) -> Result<(), sqlx::Error> {
    let mut offset = start_offset;
    let mut tasks: Vec<tokio::task::JoinHandle<Result<(), sqlx::Error>>> = Vec::new();
    while offset < end_offset {
        let users: Vec<User> = sqlx::query_as::<_,User>(
            "SELECT id, name, email FROM users LIMIT $1 OFFSET $2",
        )
        .bind(1000i32)
        .bind(offset)
        .fetch_all(db_pool)
        .await?;
        
    // if let Err(e) = send_users_to_endpoint(users).await {
    //     println!("Error sending users to endpoint: {}", e);
    // }
    let db_pool_arc = Arc::new(db_pool.clone());

    tasks.push(task::spawn(async move {
        if let Err(e) = send_users_to_endpoint(users,db_pool_arc.as_ref()).await {
            println!("Error sending users to endpoint: {}", e);
        }
        Ok(())
    }));

        // println!("Fetched {} users from offset {}", users.len(), offset);
        offset += 1000;
    }
    join_all(tasks).await;


    Ok(())
}

async fn seed_data(db_pool: &PgPool) -> Result<(), sqlx::Error> {
    let start = Instant::now();
     // Insert users
     let user_futures: Vec<_> = (1..=100_000)
     .map(|i| {
         let user = User {
             id: i,
             name: format!("User {}", i),
             email: format!("user{}@example.com", i),
         };
         create_user(db_pool, user)
     })
     .collect();

 let _users: Vec<Result<User, sqlx::Error>> = join_all(user_futures).await;

 let duration = start.elapsed();
    println!("Seeding user data took {:.2?}", duration);

    let start = Instant::now();

    // // Insert groups
    let group_futures: Vec<_> = (1..=100)
        .map(|i| {
            let group = Group {
                id: i,
                name: format!("Group {}", i),
                description: format!("Group {} description", i),
            };
            create_group(db_pool, group)
        })
        .collect();

    let _groups: Vec<Result<Group, sqlx::Error>> = join_all(group_futures).await;

    let duration = start.elapsed();
    println!("Seeding group data took {:.2?}", duration);

    let start = Instant::now();

    // Add users to groups
    let group_user_futures: Vec<_> = (1..=100_000)
        .map(|user_id| {
            let group_user = GroupUser {
                group_id: (user_id % 100) + 1,
                user_id,
            };
            add_user_to_group(db_pool, group_user)
        })
        .collect();

    let _group_users: Vec<Result<GroupUser, sqlx::Error>> = join_all(group_user_futures).await;

    let duration = start.elapsed();
    println!("Seeding group user data took {:.2?}", duration);


    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:postgres@localhost:5440/postgres";

    let pool = sqlx::postgres::PgPool::connect(url).await?;

    create_schema(&pool).await?;

    //seed_data(&pool).await?;


    // Read user data concurrently
    let read_users_start = Instant::now();
    let _var_name = tokio::join!(
        read_users_in_batches(&pool, 0, 25_000,&pool),
        read_users_in_batches(&pool, 25_000, 50_000,&pool),
        read_users_in_batches(&pool, 50_000, 75_000,&pool),
        read_users_in_batches(&pool, 75_000, 100_000,&pool),
    );
    let read_users_duration = read_users_start.elapsed();
    println!("Reading user data in batches took {:.2?}", read_users_duration);



    Ok(())
}
