use crate::models::{
    model::{CreateUserSchema, LoginUserSchema},
    user_queries,
};
use actix_web::{get, post, web, HttpResponse, Responder};
use futures::task::ArcWake;
use serde_json;
use sqlx::PgPool;

#[get("/api/get_users")]
pub async fn get_users_handler(db_pool: web::Data<PgPool>) -> impl Responder {
    match user_queries::get_users(&db_pool).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/api/auth/register")]
pub async fn register_user_handler(
    db_pool: web::Data<PgPool>,
    body: web::Json<CreateUserSchema>,
) -> impl Responder {
    println!("Received body: {:?}", body);
    match user_queries::register_user(&db_pool, &body).await {
        Ok(()) => {
            println!("in ok");
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "userName": body.user_name,
                "email": body.email,
                "message": "User Registered Successfully"
            }))
        } // need to add option for if user already exists with that user name
        Err(_) => {
            HttpResponse::InternalServerError().json(serde_json::json!({ //this is a db error
                "status": "failure",
                "userName": body.user_name,
                "email": body.email,
                "message": "Unable to register new user",
            }))
        }
    }
}

#[post("/api/auth/login")]
pub async fn login_handler(
    db_pool: web::Data<PgPool>,
    body: web::Json<LoginUserSchema>,
) -> impl Responder {
    match user_queries::verify_user(&db_pool, &body).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({ //update last login time
            "status": "success",
            "user": body.user_name,
            "message": "User logged in."
        })),
        Ok(false) => HttpResponse::Unauthorized().json(serde_json::json!({
            "status": "failure",
            "userName": body.user_name,
            "message": "username or password is incorrect",
        })),
        Err(_) => {
            // Handle unexpected errors, such as database connection issues
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": "An error occurred while processing your request."
            }))
        }
    }
}
