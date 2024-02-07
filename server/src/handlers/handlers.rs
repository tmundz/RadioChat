use crate::models::user_service::{create_user, get_users};
use axum::http::StatusCode;

use axum::{
    extract::Form,
    response::{IntoResponse, Json},
    Extension,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;
#[derive(Deserialize)]
pub struct CreateUserForm {
    user_name: String,
    password: String,
}

/*pub fn routes() -> Router {
    Router::new().route("/create_user", post(create_user_handler))
}*/
pub async fn add_user_handler(
    Extension(db_pool): Extension<Arc<PgPool>>,
    Form(body): Form<CreateUserForm>,
) -> impl IntoResponse {
    match create_user(&db_pool, &body.user_name, &body.password).await {
        Ok(uid) => (StatusCode::OK, Json(uid)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to add user: {}", e),
        )
            .into_response(),
    }
}

pub async fn get_users_handler(Extension(db_pool): Extension<Arc<PgPool>>) -> impl IntoResponse {
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
