use crate::models::{
    model::{CreateUserSchema, LoginUserSchema, LogoutUserSchema},
    user_queries,
    user_queries::{register_user, UserCheckResults},
};
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
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
    session: Session,
    body: web::Json<CreateUserSchema>,
) -> impl Responder {
    println!("Received body: {:?}", body);
    match user_queries::check_uname_email_availability(&db_pool, &body).await {
        UserCheckResults::UserNameExists => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "failure",
            "userName": body.user_name,
            "email": body.email,
            "message": "User Name Already In Use",
        })),
        UserCheckResults::EmailExists => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "failure",
            "userName": body.user_name,
            "email": body.email,
            "message": "Email Already In Use",
        })),
        UserCheckResults::DbError(_) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "failure",
                "userName": body.user_name,
                "email": body.email,
                "message": "Database Server Error",
            }))
        }
        UserCheckResults::Available => match register_user(&db_pool, &body).await {
            Ok(user) => {
                if let Err(e) = session.insert("uid", &user.uid) {
                    println!("Session insert error: {}", e);
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "status": "failure",
                        "userName": body.user_name,
                        "email": body.email,
                        "message": "Creating session error",
                    }));
                };
                HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "userName": body.user_name,
                    "email": body.email,
                    "message": "User Registered successfully",
                }))
            }
            Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "failure",
                "userName": body.user_name,
                "email": body.email,
                "message": "Database Server Error",
            })),
        },
    }
}

#[post("/api/auth/login")]
pub async fn login_handler(
    db_pool: web::Data<PgPool>,
    session: Session,
    body: web::Json<LoginUserSchema>,
) -> impl Responder {
    match user_queries::verify_user(&db_pool, &body).await {
        Ok(Some(user)) => {
            if let Err(e) = session.insert("uid", &user.uid) {
                println!("Session insert error: {}", e);
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "failure",
                    "userName": body.user_name,
                    "message": "Creating session error",
                }));
            };
            HttpResponse::Ok().json(serde_json::json!({ //update last login time
                "status": "success",
                "user": body.user_name,
                "message": "User logged in."
            }))
        }
        Ok(None) => HttpResponse::Unauthorized().json(serde_json::json!({
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

#[post("/api/auth/logout")]
pub async fn logout_handler(session: Session) -> impl Responder {
    if let Err(e) = session.purge() {
        println!("failed to purge session: {:?}", e);
        return HttpResponse::InternalServerError().json("Failed to log out");
    }
    HttpResponse::Ok().json("Logged out successfully")
}
