use axum::{extract::State, Json};
use crate::models::{User, Task};
use crate::app_config::AppConfig;
use crate::dbs::{Database, TaskRepository, UserRepository};

pub async fn get_users_async(State(state): State<AppConfig>) -> Json<Vec<User>> {
    let db = Database::new(state.db.clone());
    let users = db.get_users().await.unwrap();

    Json(users)
}

pub async fn get_tasks_async(State(state): State<AppConfig>) -> Json<Vec<Task>> {
    let db = Database::new(state.db.clone());
    let tasks = db.get_tasks().await.expect("Unable to get tasks from database");

    Json(tasks)
}
