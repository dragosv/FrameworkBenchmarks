use axum::extract::{Extension, FromRequest, RequestParts};
use axum::http::StatusCode;

use sqlx::Pool;
use sqlx::pool::PoolConnection;

#[cfg(not(test))]
pub type DbType = sqlx::Postgres;
#[cfg(test)]
pub type DbType = sqlx::Sqlite;

pub type DbPool = Pool<DbType>;
pub type DbPoolConnection = PoolConnection<DbType>;

#[cfg(not(test))]
pub async fn create_pool(database_url: String) -> DbPool {
    sqlx::postgres::PgPoolOptions::new().max_connections(100).connect(&*database_url).await.unwrap()
}

#[cfg(test)]
pub async fn create_pool(database_url: String) -> DbPool {
    sqlx::sqlite::SqlitePoolOptions::new().connect(&*database_url).await.unwrap()
}

pub struct DatabaseConnection(pub DbPoolConnection);

#[async_trait]
impl<B> FromRequest<B> for DatabaseConnection
    where
        B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(pool) = Extension::<DbPool>::from_request(req)
            .await
            .map_err(internal_error)?;

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}