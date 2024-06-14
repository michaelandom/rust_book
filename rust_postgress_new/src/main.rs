use anyhow::Result;
use futures::future::join_all;
use rayon::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use std::error::Error;
use std::sync::Arc;
use std::time::Instant;
use tokio::time::{sleep, Duration};
use tokio::task;
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
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
    pub delivery_type: i32,
}

#[derive(Serialize, Deserialize)]
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

pub async fn create_user(db_pool: &PgPool, user: User) -> Result<User, sqlx::Error> {
    sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind(&user.name)
        .bind(&user.email)
        .execute(db_pool)
        .await?;

    Ok(user)
}

pub async fn create_group(db_pool: &PgPool, group: Group) -> Result<Group, sqlx::Error> {
    sqlx::query(
        "INSERT INTO groups (name, description)
    VALUES ($1, $2)",
    )
    .bind(&group.name)
    .bind(&group.description)
    .execute(db_pool)
    .await?;
    Ok(group)
}
pub async fn create_notification(
    db_pool: &PgPool,
    notification_user: NotificationUser,
) -> Result<NotificationUser, sqlx::Error> {
     sqlx::query(
        "INSERT INTO notification_user (user_id, email,delivery_type)
    VALUES ($1, $2,$3)",
    )
    .bind(&notification_user.user_id)
    .bind(&notification_user.email)
    .bind(&notification_user.delivery_type)
    .execute(db_pool)
    .await?;

    Ok(notification_user)
}

pub async fn add_user_to_group(
    db_pool: &PgPool,
    group_user: GroupUser,
) -> Result<GroupUser, sqlx::Error> {
    sqlx::query(
        "INSERT INTO group_users (group_id, user_id)
    VALUES ($1, $2)",
    )
    .bind(&group_user.group_id)
    .bind(&group_user.user_id)
    .execute(db_pool)
    .await?;

    Ok(group_user)
}

async fn create_user_confirmation(
    users_stream: &Vec<User>,
    db_pool: &sqlx::Pool<sqlx::Postgres>,
    delivery_type: &i32,
) -> Result<(), Box<dyn Error>> {
    let mut ss = String::from("");
    let _user_futures: Vec<NotificationUser> = users_stream
        .into_iter()
        .map(|user| {
            let user: NotificationUser = NotificationUser {
                id: user.id,
                user_id: user.id,
                email: String::from(&user.email),
                delivery_type: 4,
            };
            ss.push('(');
            ss.push_str(&user.user_id.to_string());
            ss.push(',');
            ss.push('\'');
            ss.push_str(&user.email.to_string());
            ss.push('\'');
            ss.push(',');
            ss.push_str(&delivery_type.to_string());
            ss.push(')');
            ss.push(',');
            user
        })
        .collect();
    if ss.ends_with(',') {
        ss.pop();
    } else {
        return Ok(());
    }
    let mut newa =
        String::from("INSERT INTO notification_user (user_id, email, delivery_type) VALUES ");
    newa.push_str(&ss);
    let mut transaction = db_pool.begin().await?;
    Ok(
        match sqlx::query(newa.as_str()).execute(&mut transaction).await {
            Ok(_) => {
                transaction.commit().await?;
            }
            Err(_) => {
                transaction.rollback().await?;
            }
        },
    )
}

async fn update_user_confirmation(
    users_stream: &Vec<User>,
    db_pool: &sqlx::Pool<sqlx::Postgres>,
    delivery_type: &i32,
) -> Result<(), Box<dyn Error>> {
    let mut ss = String::from("");
    ss.push('(');

    let _user_futures: Vec<NotificationUser> = users_stream
        .into_iter()
        .map(|user| {
            let user: NotificationUser = NotificationUser {
                id: user.id,
                user_id: user.id,
                email: String::from(&user.email),
                delivery_type: 4,
            };
            ss.push_str(&user.user_id.to_string());
            ss.push(',');
            user
        })
        .collect();
    if ss.ends_with(',') {
        ss.pop();
    } else {
        return Ok(());
    }
    ss.push(')');
    let mut newa = String::from(format!(
        "UPDATE notification_user SET status = 'sent' WHERE delivery_type={} and user_id IN ",
        delivery_type
    ));
    newa.push_str(&ss);

    let mut transaction = db_pool.begin().await?;
    Ok(
        match sqlx::query(newa.as_str()).execute(&mut transaction).await {
            Ok(_) => {
                transaction.commit().await?;
                // Ok(())
            }
            Err(_) => {
                transaction.rollback().await?;
                // Err(_)
            }
        },
    )
}

async fn read_users_in_batches(
    db_pool: &PgPool,
    start_offset: i32,
    end_offset: i32,
) -> Result<(), sqlx::Error> {
    let mut offset = start_offset;
    let mut tasks: Vec<tokio::task::JoinHandle<Result<(), sqlx::Error>>> = Vec::new();
    let mut count=0;
    let start_time = Instant::now();
  
    while offset < end_offset {
        let users: Vec<User> =
            sqlx::query_as::<_, User>("SELECT id, name, email FROM users LIMIT $1 OFFSET $2")
                .bind(1000i32)
                .bind(offset)
                .fetch_all(db_pool)
                .await?;

        count=count+users.len();
        let users2 = Arc::new(users.to_vec());
        let users3 = Arc::new(users.to_vec());
        let users4 = Arc::new(users.to_vec());

        let db_pool_arc = Arc::new(db_pool.clone());

        tasks.push(task::spawn(async move {
            if let Err(e) = send_users_email_to_endpoint(&users, db_pool_arc.as_ref()).await {
                println!("Error sending users to endpoint: {}", e);
            }
            Ok(())
        }));

        let db_pool_arc = Arc::new(db_pool.clone());

        tasks.push(task::spawn(async move {
            if let Err(e) = send_users_sms_to_endpoint(&users2, db_pool_arc.as_ref()).await {
                println!("Error sending users to endpoint: {}", e);
            }
            Ok(())
        }));

        let db_pool_arc = Arc::new(db_pool.clone());

        tasks.push(task::spawn(async move {
            if let Err(e) = send_users_push_to_endpoint(&users3, db_pool_arc.as_ref()).await {
                println!("Error sending users to endpoint: {}", e);
            }
            Ok(())
        }));

        let db_pool_arc = Arc::new(db_pool.clone());

        tasks.push(task::spawn(async move {
            if let Err(e) = send_users_voice_to_endpoint(&users4, db_pool_arc.as_ref()).await {
                println!("Error sending users to endpoint: {}", e);
            }
            Ok(())
        }));

        offset += 1000;
    }

    let elapsed_time = start_time.elapsed().as_millis();

    println!("count of user: {} number", count);
    println!("count of jobs: {} number", tasks.len());
    println!("Time taken for the DB: {} ms", elapsed_time);

    join_all(tasks).await;

    Ok(())
}

async fn send_users_email_to_endpoint(
    users_stream: &Vec<User>,
    db_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let endpoint_url = "http://192.168.1.8:3000/a";
    let request_body = json!({ "users": users_stream });
    // create_user_confirmation(users_stream, db_pool, &4).await?;
    let start_time = Instant::now();
    let response = client
        .post(endpoint_url)
        .header("Content-Type", "application/json")
        .body(request_body.to_string())
        .send()
        .await?;
    let elapsed_time = start_time.elapsed().as_secs();

    let sent_count = users_stream.len();

    let received_count = if response.status().is_success() {
        // update_user_confirmation(users_stream, db_pool, &4).await?;
        // println!("Users data sent successfully.");
        sent_count
    } else {
        // println!("Failed to send users data: {:?}", "response.status()");
        0
    };

    println!("Sent emails: {}", sent_count);
    println!("Received emails: {}", received_count);
    println!("Time taken: {} s", elapsed_time);

    Ok(())
}


async fn send_users_sms_to_endpoint(
    users_stream: &Vec<User>,
    db_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let endpoint_url = "http://192.168.1.8:3000/a";
    let request_body = json!({ "users": users_stream });
    // create_user_confirmation(users_stream, db_pool, &4).await?;
    let start_time = Instant::now();
    let response = client
        .post(endpoint_url)
        .header("Content-Type", "application/json")
        .body(request_body.to_string())
        .send()
        .await?;
    let elapsed_time = start_time.elapsed().as_secs();

    let sent_count = users_stream.len();

    let received_count = if response.status().is_success() {
        // update_user_confirmation(users_stream, db_pool, &4).await?;
        // println!("Users data sent successfully.");
        sent_count
    } else {
        // println!("Failed to send users data: {:?}", "response.status()");
        0
    };

    println!("Sent sms: {}", sent_count);
    println!("Received sms: {}", received_count);
    println!("Time taken: {} s", elapsed_time);

    Ok(())
}

async fn send_users_voice_to_endpoint(
    users_stream: &Vec<User>,
    db_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let endpoint_url = "http://192.168.1.8:3000/a";
    let request_body = json!({ "users": users_stream });
    // create_user_confirmation(users_stream, db_pool, &4).await?;
    let start_time = Instant::now();
    let response = client
        .post(endpoint_url)
        .header("Content-Type", "application/json")
        .body(request_body.to_string())
        .send()
        .await?;
    let elapsed_time = start_time.elapsed().as_secs();

    let sent_count = users_stream.len();

    let received_count = if response.status().is_success() {
        // update_user_confirmation(users_stream, db_pool, &4).await?;
        // println!("Users data sent successfully.");
        sent_count
    } else {
        // println!("Failed to send users data: {:?}", "response.status()");
        0
    };

    println!("Sent voice: {}", sent_count);
    println!("Received voice: {}", received_count);
    println!("Time taken: {} s", elapsed_time);
    Ok(())
}

async fn send_users_push_to_endpoint(
    users_stream: &Vec<User>,
    db_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let endpoint_url = "http://192.168.1.8:3000/a";
    let request_body = json!({ "users": users_stream });
    // create_user_confirmation(users_stream, db_pool, &4).await?;
    let start_time = Instant::now();
    let response = client
        .post(endpoint_url)
        .header("Content-Type", "application/json")
        .body(request_body.to_string())
        .send()
        .await?;
    let elapsed_time = start_time.elapsed().as_secs();

    let sent_count = users_stream.len();

    let received_count = if response.status().is_success() {
        // update_user_confirmation(users_stream, db_pool, &4).await?;
        // println!("Users data sent successfully.");
        sent_count
    } else {
        // println!("Failed to send users data: {:?}", "response.status()");
        0
    };

    println!("Sent push: {}", sent_count);
    println!("Received push: {}", received_count);
    println!("Time taken: {} s", elapsed_time);
    Ok(())
}

async fn test() {
    sleep(Duration::from_secs(30)).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:postgres@localhost:5440/postgres";

    let pool = sqlx::postgres::PgPool::connect(url).await?;

    create_schema(&pool).await?;

    // seed_data(&pool).await?;

    let read_users_start = Instant::now();

    let results: Vec<_> = (0..4)
        .into_iter()
        .map(|i| {
            let start = i * 25_000;
            let end = (i + 1) * 25_000;
            read_users_in_batches(&pool, start, end)
        })
        .collect();

    let _all_users = join_all(results).await;
    let read_users_duration = read_users_start.elapsed();

    println!(
        "Reading user data in batches took {:?}",
        read_users_duration
    );

    Ok(())
}
