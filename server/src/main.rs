mod handlers;
mod models;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use models::user_queries::{get_users, register_user};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

//use handlers::handlers::{add_user_handler, get_users_handler};

use std::env;

pub struct AppState {
    db: Pool<Postgres>,
}
use crate::handlers::handlers::{get_users_handler, login_handler, register_user_handler};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new().connect(&db).await {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::COOKIE,
                header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .route("/health_check", web::get().to(health_check))
            //.route("/api/get_users", web::get().to(get_users_handler))
            .service(get_users_handler)
            .service(login_handler)
            .service(register_user_handler)
    }
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
