use serde::Serialize;
use sqlx::postgres::PgPool;
use sqlx::FromRow;
use uuid::Uuid;

use super::model::CreateUserSchema;
use super::model::UserModel;

pub async fn register_user(db_pool: &PgPool, body: &CreateUserSchema) -> Result<(), sqlx::Error> {
    let uid = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO users (uid, user_name, password, email, created_at, last_login) VALUES ($1, $2, $3, $4, NOW(), NOW())",
        uid,
        body.user_name,
        body.email,
        body.password,
    )
    .execute(db_pool)
    .await?;

    Ok(())
}

pub async fn delete_user(
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
