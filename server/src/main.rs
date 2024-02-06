mod handlers;
mod models;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use handlers::server::{Room, Server};
use models::model;
use std::sync::{Arc, Mutex};

async fn create_user_form() -> Html<&'static str> {
    Html(include_str!("../static/create_user_form.html"))
}

#[tokio::main]
async fn main() {
    let server = Arc::new(Server::new());
    let app = Router::new().route("/create_user", get(create_user_form));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
