use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use jiff_sqlx::Timestamp;
use jiff::Timestamp as JiffTimeStamp;


#[derive(Serialize, Deserialize, Debug, Type)]
#[sqlx(type_name = "integer")]
#[repr(i32)]
pub enum Status {
    Todo = 0,
    InProgress = 1,
    Done = 2
}

impl Default for Status {
    fn default() -> Self {
        Status::Todo
    }
}

#[derive(Serialize, Deserialize, Debug, Type)]
#[sqlx(type_name = "integer")]
#[repr(i32)]
pub enum Priority {
    Low = 3,
    Medium = 2,
    High = 1
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub priority: Priority,
    #[sqlx(try_from = "jiff_sqlx::Timestamp")]
    pub due_date: JiffTimeStamp,
    #[sqlx(try_from = "jiff_sqlx::Timestamp")]
    pub created_at: JiffTimeStamp,
    #[sqlx(try_from = "jiff_sqlx::Timestamp")]
    pub updated_at: JiffTimeStamp
}
