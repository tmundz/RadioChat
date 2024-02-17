use serde::Serialize;
use sqlx::postgres::PgPool;
use sqlx::FromRow;
use tower::util::Optional;
use uuid::Uuid;

use crate::handlers::server::User;

use super::model::CreateUserSchema;
use super::model::LoginUserSchema;
use super::model::PubUserModel;
use super::model::UserModel;

#[derive(Debug)]
pub enum UserCheckResults {
    UserNameExists,
    EmailExists,
    DbError(sqlx::Error),
    Available,
}

/*
 * Get/Check User Queries
 */
pub async fn get_users(db_pool: &PgPool) -> Result<Vec<UserModel>, sqlx::Error> {
    sqlx::query_as::<_, UserModel>("SELECT * FROM users")
        .fetch_all(db_pool)
        .await
}

pub async fn check_uname_email_availability(
    db_pool: &PgPool,
    data: &CreateUserSchema,
) -> UserCheckResults {
    let user_name_check = sqlx::query!("SELECT * FROM users WHERE user_name = $1", data.user_name,)
        .fetch_optional(db_pool)
        .await;
    if let Err(e) = user_name_check {
        return UserCheckResults::DbError(e);
    }

    if user_name_check.unwrap().is_some() {
        return UserCheckResults::UserNameExists;
    }
    let email_check = sqlx::query!("SELECT * FROM users WHERE email = $1", data.email,)
        .fetch_optional(db_pool)
        .await;

    if let Err(e) = email_check {
        return UserCheckResults::DbError(e);
    }

    if email_check.unwrap().is_some() {
        return UserCheckResults::UserNameExists;
    }
    UserCheckResults::Available
}

/*
 * User Queries that adjust the DB
 */
pub async fn register_user(
    db_pool: &PgPool,
    body: &CreateUserSchema,
) -> Result<PubUserModel, sqlx::Error> {
    let uid = Uuid::new_v4();
    let user = sqlx::query_as::<_, PubUserModel>(
        "INSERT 
        INTO users (uid, user_name, email, password, created_at, updated_at, last_login) 
        VALUES ($1, $2, $3, $4, NOW(), NOW(), NOW())",
    )
    .bind(uid)
    .bind(&body.user_name)
    .bind(&body.email)
    .bind(&body.password)
    .fetch_one(db_pool)
    .await?;
    Ok(user)
}

pub async fn verify_user(
    db_pool: &sqlx::PgPool,
    body: &LoginUserSchema,
) -> Result<Option<PubUserModel>, sqlx::Error> {
    let user = sqlx::query_as::<_, PubUserModel>(
        "SELECT * FROM users WHERE user_name = $1 AND password = $2",
    )
    .bind(&body.user_name)
    .bind(&body.password)
    .fetch_optional(db_pool)
    .await?;
    Ok(user)
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
