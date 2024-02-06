mod handlers;
mod models;

use axum::http::StatusCode;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Form, State,
    },
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use futures::{sink::SinkExt, stream::StreamExt};
//use handlers::handlers::create_user_handler;
//use models::{model, user_service::create_user};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use sqlx::FromRow;
use sqlx::{
    database,
    postgres::{PgPool, PgPoolOptions},
};
use std::env;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

async fn create_user_form() -> Html<&'static str> {
    Html(include_str!("../static/create_user_form.html"))
}

#[derive(Deserialize)]
struct CreateUserForm {
    user_name: String,
    password: String,
}

async fn add_user_handler(
    Extension(db_pool): Extension<Arc<PgPool>>,
    Form(body): Form<CreateUserForm>,
) -> impl IntoResponse {
    // Here you can use form_data.user_name and form_data.password
    // For example, insert the user into the database
    match create_user(&db_pool, &body.user_name, &body.password).await {
        Ok(uid) => (StatusCode::OK, Json(uid)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to add user: {}", e),
        )
            .into_response(),
    }
}
async fn create_user(
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

#[derive(Serialize, FromRow)]
struct User {
    uid: Uuid,
    user_name: String,
    password: String, // Note: In real applications, you shouldn't expose passwords. Consider excluding this field in production code.
}

async fn get_users_handler(Extension(db_pool): Extension<Arc<PgPool>>) -> impl IntoResponse {
    let result = get_users(&db_pool).await;

    match result {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to fetch users" })),
        )
            .into_response(),
    }
}
async fn get_users(db_pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT uid, user_name, password FROM users")
        .fetch_all(db_pool)
        .await
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect to the database");

    let app = Router::new()
        .route("/register", get(create_user_form))
        .route("/add_user", post(add_user_handler))
        .route("/users", get(get_users_handler))
        .layer(Extension(Arc::new(db_pool)));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
