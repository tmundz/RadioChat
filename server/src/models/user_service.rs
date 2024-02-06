use sqlx::PgPool;
use uuid::Uuid;

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

// need an option to create admin
