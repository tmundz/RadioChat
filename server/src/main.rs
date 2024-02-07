mod handlers;
mod models;

use axum::{
    /*extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Form,
        State,
    },*/
    response::Html,
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

use handlers::handlers::{add_user_handler, get_users_handler};

use std::env;
use std::sync::Arc;

async fn create_user_form() -> Html<&'static str> {
    Html(include_str!("../static/create_user_form.html"))
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
