use async_trait::async_trait;
use crate::models::Task;
use crate::errors::SqlResult;

#[async_trait]
pub trait TaskRepository {
    async fn get_tasks(&self) -> SqlResult<Vec<Task>>;
    async fn get_task(&self, task_id: i32) -> SqlResult<Option<Task>>;
    async fn get_tasks_for_user(&self, user_id: i32) -> SqlResult<Vec<Task>>;
    async fn insert_task(&self, task: Task) -> SqlResult<()>;
}
