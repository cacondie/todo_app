use axum::{extract::State, Json};
use crate::models::User;
use crate::app_config::AppConfig;
use crate::dbs::{Database, UserRepository};

pub async fn get_users_async(State(state): State<AppConfig>) -> Json<Vec<User>> {
    let db = Database::new(state.db.clone());
    let users = db.get_users().await.unwrap();

    Json(users)
}
