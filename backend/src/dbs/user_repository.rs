use async_trait::async_trait;
use crate::errors::SqlResult;
use crate::models::User;


#[async_trait]
pub trait UserRepository {
    async fn get_user(&self, user_id: i32) -> SqlResult<Option<User>>;
    async fn get_users(&self) -> SqlResult<Vec<User>>;
    async fn insert_user(&self, user_name: &str) -> SqlResult<()>;
}
