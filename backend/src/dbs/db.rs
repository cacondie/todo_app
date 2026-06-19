use async_trait::async_trait;
use sqlx::SqlitePool;
use crate::dbs::UserRepository;
use crate::models::{User, Task};
use crate::errors::SqlResult;
use crate::dbs::TaskRepository;

pub struct Database {
    pool: SqlitePool
}

impl Database{
    pub fn new(pool: SqlitePool) -> Self {
        Database{pool}
    }
}


#[async_trait]
impl UserRepository for Database {
    async fn get_user(&self, user_id: i32) -> SqlResult<Option<User>>{
        let query = "SELECT id, name FROM users WHERE id = ?";
        sqlx::query_as::<_, User>(query)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn get_users(&self) -> SqlResult<Vec<User>> {
        let query = "SELECT id, name FROM users";
        let users = sqlx::query_as::<_, User>(query)
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    async fn insert_user(&self, user_name: &str) -> SqlResult<()> {
        let query = "INSERT INTO users(name) VALUES(?)";
    
        sqlx::query(query)
            .bind(user_name)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

impl TaskRepository for Database
{
    async fn get_tasks(&self) -> SqlResult<Vec<Task>> {
        let query = "SELECT * FROM tasks";

        let tasks = sqlx::query(query)
            .execute(&self.pool)
            .await?;

        Ok(tasks);
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use sqlx::SqlitePool;

    async fn setup_test_db() -> Database {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("Failed to create in-memory database");

        sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL);")
            .execute(&pool)
            .await
            .expect("Failed to create user table");

        Database::new(pool)
    }

    #[tokio::test]
    async fn test_insert_and_get_user_live_in_memory() -> SqlResult<()> {
        let sut = setup_test_db().await;
        let test_subject = "Mickey Mouse";
        sut.insert_user(test_subject).await.expect("Unable to insert user");
        let actual = sut.get_user(1).await.expect("Unable to retrieve user_id 1");

        assert!(actual.is_some(), "User should have been found");
        let user = actual.unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Mickey Mouse");

        Ok(())
    }

    #[tokio::test]
    async fn test_non_existing_user_live_in_memory() -> SqlResult<()> {
        let sut = setup_test_db().await;
        let fetched_user = sut.get_user(987).await.expect("Unable to run function");
        assert!(fetched_user.is_none());

        Ok(())
    }
}