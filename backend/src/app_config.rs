use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppConfig{
    pub db: SqlitePool
}
