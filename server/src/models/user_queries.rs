use serde::Serialize;
use sqlx::postgres::PgPool;
use sqlx::FromRow;
use uuid::Uuid;

use super::model::CreateUserSchema;
use super::model::LoginUserSchema;
use super::model::UserModel;

pub async fn register_user(db_pool: &PgPool, body: &CreateUserSchema) -> Result<(), sqlx::Error> {
    let uid = Uuid::new_v4();
    match sqlx::query!(
        "INSERT 
        INTO users (uid, user_name, email, password, created_at, updated_at, last_login) 
        VALUES ($1, $2, $3, $4, NOW(), NOW(), NOW())",
        uid,
        body.user_name,
        body.email,
        body.password,
    )
    .execute(db_pool)
    .await
    {
        Ok(result) => println!(
            "Query executed successfully. Rows affected: {}",
            result.rows_affected()
        ),

        Err(e) => {
            println!("Query execution error: {:?}", e);
            return Err(e);
        }
    }
    Ok(())
}

pub async fn verify_user(
    db_pool: &sqlx::PgPool,
    body: &LoginUserSchema,
) -> Result<bool, sqlx::Error> {
    let user = sqlx::query!(
        "SELECT * FROM users WHERE user_name = $1 AND password = $2",
        body.user_name,
        body.password,
    )
    .fetch_optional(db_pool)
    .await?;
    Ok(user.is_some())
}

/*pub async fn delete_user(
    db_pool: &PgPool,
    user_name: &str,
    password: &str,
) -> Result<Uuid, sqlx::Error> {
    let uid = Uuid::new_v4();
    match sqlx::query!(
        "INSERT INTO users (uid, user_name, password, created_at) VALUES ($1, $2, $3, NOW())",
        uid,
        user_name,
        password,
    )
    .execute(db_pool)
    .await
    {
        Ok(result) => println!(
            "Query executed successfully. Rows affected: {}",
            result.rows_affected()
        ),

        Err(e) => {
            println!("Query execution error: {:?}", e);
            return Err(e);
        }
    }
}*/

pub async fn update_user(
    db_pool: &PgPool,
    user_name: &str,
    password: &str,
) -> Result<Uuid, sqlx::Error> {
    let uid = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO users (uid, user_name, password, created_at) VALUES ($1, $2, $3, NOW())",
        uid,
        user_name,
        password,
    )
    .execute(db_pool)
    .await?;
    Ok(uid)
}

pub async fn get_users(db_pool: &PgPool) -> Result<Vec<UserModel>, sqlx::Error> {
    sqlx::query_as::<_, UserModel>("SELECT * FROM users")
        .fetch_all(db_pool)
        .await
}
// need an option to create admin
