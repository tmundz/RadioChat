use crate::models::user_queries;
use crate::models::model::CreateUserSchema;
use actix_web::{get, web, HttpResponse, Responder};
use serde_json;
use futures::task::ArcWake;
use sqlx::PgPool;

#[get("/api/get_users")]
pub async fn get_users_handler(db_pool: web::Data<PgPool>) -> impl Responder {
    match user_queries::get_users(&db_pool).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/api/register")]
pub async fn register_user_handler(db_pool: web::Data<PgPool>, body: web::Json<CreateUserSchema>) -> impl Responder {
    match user_queries::register_user(&db_pool, &body) {
        Ok(()) =>  HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "userName": body.user_name,
            "email": body.email,
            "message": "User Registered Successfully"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "failure",
            "userName": body.user_name,
            "email": body.email,
            "message": "Unable to register new user",
        })),
    }
}


#[post("/api/login")]

