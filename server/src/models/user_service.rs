use serde::Serialize;
use sqlx::postgres::PgPool;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct User {
    uid: Uuid,
    user_name: String,
    password: String, // Note: In real applications, you shouldn't expose passwords. Consider excluding this field in production code.
}
pub async fn create_user(
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

pub async fn get_users(db_pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT uid, user_name, password FROM users")
        .fetch_all(db_pool)
        .await
}
// need an option to create admin
