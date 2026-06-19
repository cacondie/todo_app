use sqlx;

pub type SqlResult<T> =std::result::Result<T, sqlx::Error>;
