use crate::models::user_service::create_user;
use axum::http::StatusCode;
use axum::routing::post;
use axum::Router;
use axum::{
    extract::{Form, State},
    response::IntoResponse,
    Extension, Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
#[derive(Deserialize)]
pub struct CreateUserForm {
    user_name: String,
    password: String,
}

/*pub fn routes() -> Router {
    Router::new().route("/create_user", post(create_user_handler))
}*/
pub async fn create_user_handler(
    Extension(db_pool): Extension<Arc<PgPool>>,
    Form(form): Form<CreateUserForm>,
) -> impl IntoResponse {
    match create_user(&db_pool, &form.user_name, &form.password).await {
        Ok(uid) => Ok(Json(uid)),
        // create more errors to better define the error
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create user: {}", e),
        )),
    }
}
