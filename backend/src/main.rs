use axum::{routing::get, Router};
// use serde::Serialize;
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;
use dotenvy;
use std;

mod routes;
mod app_config;
mod models;
mod dbs;
pub mod errors;

use dbs::Database;
use app_config::AppConfig;
use routes::health;
use routes::{get_users_async, get_tasks_async};


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL missing");
    let pool = SqlitePool::connect(&database_url).await.expect("failed to connect to the database");
    let database= Database::new(pool.clone());
    let app_config = AppConfig{db: pool};

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/users", get(get_users_async))
        .route("/api/tasks", get(get_tasks_async))
        .with_state(app_config)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");


    axum::serve(listener, app).await.unwrap();
}
