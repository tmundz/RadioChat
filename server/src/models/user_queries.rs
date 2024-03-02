use serde::Serialize;
use sqlx::postgres::PgPool;
use sqlx::FromRow;
use tower::util::Optional;
use uuid::Uuid;

use super::model::CreateUserSchema;
use super::model::LoginUserSchema;
use super::model::LogoutUserSchema;
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
        return UserCheckResults::EmailExists;
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
        "INSERT INTO users (uid, user_name, email, password, created_at, updated_at, last_login) 
        VALUES ($1, $2, $3, $4, NOW(), NOW(), NOW())
        RETURNING uid, user_name, email, created_at, updated_at, is_active, last_login",
    )
    .bind(uid)
    .bind(&body.user_name)
    .bind(&body.email)
    .bind(&body.password)
    .fetch_one(db_pool)
    .await?;
    Ok(user)
}

// TODO need to set up set active
pub async fn verify_user(
    db_pool: &sqlx::PgPool,
    body: &LoginUserSchema,
) -> Result<Option<PubUserModel>, sqlx::Error> {
    println!("verify here");
    let user = sqlx::query_as::<_, PubUserModel>(
        "SELECT * FROM users WHERE user_name = $1 and password = $2",
    )
    .bind(&body.user_name)
    .bind(&body.password)
    .fetch_optional(db_pool)
    .await?;
    Ok(user)
}

// TODO need to disable active
pub async fn logout_user(
    db_pool: &sqlx::PgPool,
    body: &LogoutUserSchema,
) -> Result<Option<PubUserModel>, sqlx::Error> {
    println!("verify here");
    let user = sqlx::query_as::<_, PubUserModel>(
        "
        UPDATE users 
        SET is_active = $2 
        WHERE uid = $1
        RETURNING uid, user_name, email, created_at, updated_at, is_active, last_login,
        ",
    )
    .bind(&body.uid)
    .bind(false)
    .fetch_optional(db_pool)
    .await?;
    Ok(user)
}

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
