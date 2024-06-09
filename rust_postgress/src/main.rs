use anyhow::Result;
use futures::future::join_all;
use futures::stream::FuturesOrdered;
use rayon::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::to_string;
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use sqlx::Row;
use std::error::Error;
use std::sync::Arc;
use std::time::Instant;
use tokio::task;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
enum CustomError {
    SqlxError(sqlx::Error),
    SendEmailError(String),
    SendSmsError(String),
    SendVoiceError(String),
    SendPushError(String),
}

impl std::error::Error for CustomError {}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::SqlxError(e) => write!(f, "SQL error: {}", e),
            CustomError::SendEmailError(e) => write!(f, "Error sending email: {}", e),
            CustomError::SendSmsError(e) => write!(f, "Error sending SMS: {}", e),
            CustomError::SendVoiceError(e) => write!(f, "Error sending voice: {}", e),
            CustomError::SendPushError(e) => write!(f, "Error sending push notification: {}", e),
        }
    }
}

impl From<sqlx::Error> for CustomError {
    fn from(err: sqlx::Error) -> Self {
        CustomError::SqlxError(err)
    }
}
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
    let file = sqlx::query(
        "INSERT INTO notification_user (user_id, email,delivery_type)
    VALUES ($1, $2,$3)",
    )
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
async fn send_users_email_to_endpoint(
    users_stream: &Vec<User>,
    db_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let endpoint_url = "https://rust_.requestcatcher.com/notification/email";

    // let user_data: Vec<UserData> = users_stream
    //     .into_iter()
    //     .map(|user| UserData {
    //         id: user.id,
    //         name: user.name,
    //         email: user.email,
    //     })
    //     .collect();

    let request_body = json!({ "users": users_stream });

    fun_name(users_stream, db_pool,&4).await?;


    // let response = client
    //     .post(endpoint_url)
    //     .header("Content-Type", "application/json")
    //     .body(request_body.to_string())
    //     .send()
    //     .await?;
    sleep(Duration::from_secs(30)).await;
    

    if true {
        fun_name_update(users_stream, db_pool,&4).await?;

        println!("Users data sent successfully.");
    } else {
        println!("Failed to send users data: {:?}", "response.status()");
    }

    Ok(())
}

async fn fun_name(users_stream: &Vec<User>, db_pool: &sqlx::Pool<sqlx::Postgres>,delivery_type: &i32) -> Result<(), Box<dyn Error>> {
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
            // create_notification(db_pool, user)
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
    Ok(match sqlx::query(newa.as_str()).execute(&mut transaction).await {
        Ok(_) => {
            transaction.commit().await?;
            // Ok(())
        }
        Err(e) => {
            transaction.rollback().await?;
            // Err(e)
        }
    })
}

async fn fun_name_update(users_stream: &Vec<User>, db_pool: &sqlx::Pool<sqlx::Postgres>, delivery_type: &i32) -> Result<(), Box<dyn Error>> {
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
           // ss.push(',');
           // ss.push('\'');
           // ss.push_str(&user.email.to_string());
            //ss.push('\'');
           // ss.push(',');
           // ss.push_str(&user.delivery_type.to_string());
            ss.push(',');
            user
            // create_notification(db_pool, user)
        })
        .collect();
    if ss.ends_with(',') {
        ss.pop();
    } else {
        return Ok(());
    }

        
        ss.push(')');
         
   
    
    
    let mut newa =
        String::from(format!("UPDATE notification_user SET status = 'sent' WHERE delivery_type={} and user_id IN ",delivery_type));
        newa.push_str(&ss);

        let mut transaction = db_pool.begin().await?;
    Ok(match sqlx::query(newa.as_str()).execute(&mut transaction).await {
        Ok(_) => {
            transaction.commit().await?;
            // Ok(())
        }
        Err(e) => {
            transaction.rollback().await?;
            // Err(e)
        }
    })
}

async fn send_users_sms_to_endpoint(
    users_stream: &Vec<User>,
    db_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let endpoint_url = "https://rust_sms.requestcatcher.com/notification/sms";

    // let user_data: Vec<UserData> = users_stream
    //     .into_iter()
    //     .map(|user| UserData {
    //         id: user.id,
    //         name: user.name,
    //         email: user.email,
    //     })
    //     .collect();

    let request_body = json!({ "users": users_stream });
    fun_name(users_stream, db_pool,&3).await?;

    // let response = client
    //     .post(endpoint_url)
    //     .header("Content-Type", "application/json")
    //     .body(request_body.to_string())
    //     .send()
    //     .await?;
    sleep(Duration::from_secs(15)).await;
    

    if true  {
         fun_name_update(users_stream, db_pool,&3).await?;
        
        println!("Users data sent successfully.");
    } else {
        println!("Failed to send users data: {:?}", "response.status()");
    }

    Ok(())
}

async fn send_users_voice_to_endpoint(
    users_stream: &Vec<User>,
    db_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let endpoint_url = "https://rust_voice.requestcatcher.com/notification/voice";

    // let user_data: Vec<UserData> = users_stream
    //     .into_iter()
    //     .map(|user| UserData {
    //         id: user.id,
    //         name: user.name,
    //         email: user.email,
    //     })
    //     .collect();

    let request_body = json!({ "users": users_stream });
    fun_name(users_stream, db_pool,&2).await?;

    // let response = client
    //     .post(endpoint_url)
    //     .header("Content-Type", "application/json")
    //     .body(request_body.to_string())
    //     .send()
    //     .await?;
    sleep(Duration::from_secs(15)).await;
    

    if true {
        fun_name_update(users_stream, db_pool,&2).await?;
      
        println!("Users data sent successfully.");
    } else {
        println!("Failed to send users data: {:?}", "response.status()");
    }

    Ok(())
}

async fn send_users_push_to_endpoint(
    users_stream: &Vec<User>,
    db_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let endpoint_url = "https://rust_push.requestcatcher.com/notification/push";

    // let user_data: Vec<UserData> = users_stream
    //     .into_iter()
    //     .map(|user| UserData {
    //         id: user.id,
    //         name: user.name,
    //         email: user.email,
    //     })
    //     .collect();

    let request_body = json!({ "users": users_stream });
    fun_name(users_stream, db_pool,&1).await?;
 
 
 // let response = client
    //     .post(endpoint_url)
    //     .header("Content-Type", "application/json")
    //     .body(request_body.to_string())
    //     .send()
    //     .await?;
    
    sleep(Duration::from_secs(15)).await;
    

    if true  {
    fun_name_update(users_stream, db_pool,&1).await?;

        println!("Users data sent successfully.");
    } else {
        println!("Failed to send users data: {:?}", "response.status()");
    }

    Ok(())
}

async fn read_users_in_batches(
    db_pool: &PgPool,
    start_offset: i32,
    end_offset: i32,
) -> Result<(), sqlx::Error> {
    let mut offset = start_offset;
    let mut tasks: Vec<tokio::task::JoinHandle<Result<(), sqlx::Error>>> = Vec::new();
    while offset < end_offset {
        let users: Vec<User> =
            sqlx::query_as::<_, User>("SELECT id, name, email FROM users LIMIT $1 OFFSET $2")
                .bind(1000i32)
                .bind(offset)
                .fetch_all(db_pool)
                .await?;
        let users2 = Arc::new(users.to_vec());
        let users3 = Arc::new(users.to_vec());
        let users4 = Arc::new(users.to_vec());

        // if let Err(e) = send_users_to_endpoint(users).await {
        //     println!("Error sending users to endpoint: {}", e);
        // }
        let db_pool_arc = Arc::new(db_pool.clone());

        tasks.push(task::spawn(async move {
            // sleep(Duration::from_millis(200)).await;
            if let Err(e) = send_users_email_to_endpoint(&users, db_pool_arc.as_ref()).await {
                println!("Error sending users to endpoint: {}", e);
            }
            // if let Err(e) = send_users_sms_to_endpoint(&users, db_pool_arc.as_ref()).await {
            //     println!("Error sending users to endpoint: {}", e);
            // }
            // if let Err(e) = send_users_voice_to_endpoint(&users, db_pool_arc.as_ref()).await {
            //     println!("Error sending users to endpoint: {}", e);
            // }
            // if let Err(e) = send_users_push_to_endpoint(&users, db_pool_arc.as_ref()).await {
            //     println!("Error sending users to endpoint: {}", e);
            // }
            Ok(())
        }));

        let db_pool_arc = Arc::new(db_pool.clone());

        tasks.push(task::spawn(async move {
            // if let Err(e) = send_users_email_to_endpoint(&users, db_pool_arc.as_ref()).await {
            //     println!("Error sending users to endpoint: {}", e);
            // }
            // sleep(Duration::from_millis(200)).await;
            if let Err(e) = send_users_sms_to_endpoint(&users2, db_pool_arc.as_ref()).await {
                println!("Error sending users to endpoint: {}", e);
            }
            // if let Err(e) = send_users_voice_to_endpoint(&users, db_pool_arc.as_ref()).await {
            //     println!("Error sending users to endpoint: {}", e);
            // }
            // if let Err(e) = send_users_push_to_endpoint(&users, db_pool_arc.as_ref()).await {
            //     println!("Error sending users to endpoint: {}", e);
            // }
            Ok(())
        }));

        let db_pool_arc = Arc::new(db_pool.clone());

        tasks.push(task::spawn(async move {
            // if let Err(e) = send_users_email_to_endpoint(&users, db_pool_arc.as_ref()).await {
            //     println!("Error sending users to endpoint: {}", e);
            // }
            // if let Err(e) = send_users_sms_to_endpoint(&users3, db_pool_arc.as_ref()).await {
            //     println!("Error sending users to endpoint: {}", e);
            // }
            // if let Err(e) = send_users_voice_to_endpoint(&users, db_pool_arc.as_ref()).await {
            //     println!("Error sending users to endpoint: {}", e);
            // }
            sleep(Duration::from_millis(200)).await;
            if let Err(e) = send_users_push_to_endpoint(&users3, db_pool_arc.as_ref()).await {
                println!("Error sending users to endpoint: {}", e);
            }
            Ok(())
        }));

        let db_pool_arc = Arc::new(db_pool.clone());

        tasks.push(task::spawn(async move {
            // if let Err(e) = send_users_email_to_endpoint(&users, db_pool_arc.as_ref()).await {
            //     println!("Error sending users to endpoint: {}", e);
            // }
            // if let Err(e) = send_users_sms_to_endpoint(&users4, db_pool_arc.as_ref()).await {
            //     println!("Error sending users to endpoint: {}", e);
            // }
            // sleep(Duration::from_millis(200)).await;
            if let Err(e) = send_users_voice_to_endpoint(&users4, db_pool_arc.as_ref()).await {
                println!("Error sending users to endpoint: {}", e);
            }
            // if let Err(e) = send_users_push_to_endpoint(&users, db_pool_arc.as_ref()).await {
            //     println!("Error sending users to endpoint: {}", e);
            // }
            Ok(())
        }));

        // let _var_name = tokio::join!(
        //     send_users_email_to_endpoint(&users,db_pool_arc.as_ref()),
        // );

        // println!("Fetched {} users from offset {}", users.len(), offset);
        offset += 1000;
    }
    join_all(tasks).await;

    Ok(())
}

async fn process_users_in_batches(
    db_pool: &PgPool,
    start_offset: i32,
    end_offset: i32,
    num_threads: usize,
) -> Result<(), sqlx::Error> {
    let db_pool = Arc::new(db_pool.clone());
    let mut tasks: Vec<tokio::task::JoinHandle<Result<(), CustomError>>> = Vec::new();
    for _ in 0..num_threads {
        let db_pool = db_pool.clone();
        let offset = start_offset;
        let end = end_offset;

        tasks.push(task::spawn(async move {
            let mut current_offset = offset;
            while current_offset < end {
                let users: Vec<User> = sqlx::query_as::<_, User>(
                    "SELECT id, name, email FROM users LIMIT $1 OFFSET $2",
                )
                .bind(2000i32)
                .bind(current_offset)
                .fetch_all(db_pool.as_ref())
                .await?;

                let email_result = send_users_email_to_endpoint(&users, db_pool.as_ref())
                    .await
                    .map_err(|e| CustomError::SendEmailError(e.to_string()));
                let sms_result = send_users_sms_to_endpoint(&users, db_pool.as_ref())
                    .await
                    .map_err(|e| CustomError::SendSmsError(e.to_string()));
                let voice_result = send_users_voice_to_endpoint(&users, db_pool.as_ref())
                    .await
                    .map_err(|e| CustomError::SendVoiceError(e.to_string()));
                let push_result = send_users_push_to_endpoint(&users, db_pool.as_ref())
                    .await
                    .map_err(|e| CustomError::SendPushError(e.to_string()));

                if let Err(e) = email_result {
                    println!("{}", e);
                }
                if let Err(e) = sms_result {
                    println!("{}", e);
                }
                if let Err(e) = voice_result {
                    println!("{}", e);
                }
                if let Err(e) = push_result {
                    println!("{}", e);
                }

                current_offset += 2000;
            }
            Ok(())
        }));
    }

    futures::future::join_all(tasks).await;
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

    // seed_data(&pool).await?;

    // Read user data concurrently
    let read_users_start = Instant::now();
    // let _var_name = tokio::join!(
    //     read_users_in_batches(&pool, 0, 25_000,&pool),
    //     read_users_in_batches(&pool, 25_000, 50_000,&pool),
    //     read_users_in_batches(&pool, 50_000, 75_000,&pool),
    //     read_users_in_batches(&pool, 75_000, 100_000,&pool),
    // );

    let results: Vec<_> = (0..4)
        .into_par_iter()
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

    // let read_users_start = Instant::now();
    // process_users_in_batches(&pool, 0, 100_000, 20).await?;
    // let read_users_duration = read_users_start.elapsed();
    //  println!("Reading user data in batches took {:.2?}", read_users_duration);

    Ok(())
}
