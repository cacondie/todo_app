use async_trait::async_trait;
use sqlx::SqlitePool;
use crate::dbs::UserRepository;
use crate::models::{User, Task};
use crate::errors::SqlResult;
use crate::dbs::TaskRepository;
use crate::models::{Status, Priority};
use jiff::Timestamp as JiffTimeStamp;

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


#[async_trait]
impl TaskRepository for Database
{
    async fn get_tasks(&self) -> SqlResult<Vec<Task>> {
        let query = "SELECT * FROM tasks";

        let tasks = sqlx::query_as::<_, Task>(query)
            .fetch_all(&self.pool)
            .await?;

        Ok(tasks)
    }

    async fn get_tasks_for_user(&self, user_id: i32) -> SqlResult<Vec<Task>> {
        let query = "SELECT * FROM tasks WHERE user_id = ?";
        let tasks = sqlx::query_as::<_, Task>(query)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(tasks)
    }

    async fn get_task(&self, task_id: i32) -> SqlResult<Option<Task>> {
        let query = "SELECT * FROM tasks WHERE id = ?";

        let task = sqlx::query_as::<_, Task>(query)
            .bind(task_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(task)
    }

    async fn insert_task(&self, task: Task) -> SqlResult<()> {
        let query = "INSERT INTO tasks (user_id, title, description, status, priority, due_date, created_at, updated_at) VALUES(?, ?, ?, ?, ?, ?, ?, ?);";
        sqlx::query(query)
            .bind(task.user_id)
            .bind(task.title)
            .bind(task.description)
            .bind(task.status)
            .bind(task.priority)
            .bind(task.due_date.to_string())
            .bind(task.created_at.to_string())
            .bind(task.updated_at.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
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

        sqlx::query("CREATE TABLE tasks (id INTEGER PRIMARY KEY AUTOINCREMENT, user_id INTEGER NOT NULL, title TEXT NOT NULL, description TEXT NULL, status INTEGER NOT NULL, priority INTEGER NOT NULL, due_date TEXT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("Failed to create task table");

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

    #[tokio::test]
    async fn get_tasks_test() -> SqlResult<()> {
        let now = JiffTimeStamp::now();
        let task1 = Task::new(1, "some Title 1", "Description 1", Status::Todo, Priority::Medium, now);
        let task2 = Task::new(1, "some Title 2", "Description 2", Status::Todo, Priority::Medium, now);
        let task3 = Task::new(1, "some Title 3", "Description 3", Status::Todo, Priority::Medium, now);
        let sut = setup_test_db().await;
        sut.insert_task(task1).await.expect("unable to write task 1 to database");
        sut.insert_task(task2).await.expect("unable to write task 1 to database");
        sut.insert_task(task3).await.expect("unable to write task 1 to database");

        let user_tasks = sut.get_tasks_for_user(1).await.expect("unable to get records from database");

        assert_eq!(user_tasks.len(), 3);
        Ok(())
    }
}
