use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use jiff::Timestamp;


// #[derive(Serialize, Deserialize, FromRow)]
pub enum Status {
    //Enum: TODO, IN_PROGRESS, DONE. Default: TODO)
    todo = 0,
    in_progress = 1,
    done = 2
}


// #[derive(Serialize, Deserialize, FromRow)]
pub enum Priority {
    low = 3,
    medium = 2,
    high = 1
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub priority: Priority,
    pub due_date: Timestamp,
    pub created_at: Timestamp,
    pub updated_at: Timestamp
}
