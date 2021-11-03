use axum::extract::{Extension, FromRequest, RequestParts};
use axum::http::StatusCode;
use bb8::{Pool, PooledConnection};
#[cfg(not(test))]
use bb8_postgres::PostgresConnectionManager;
#[cfg(not(test))]
use bb8_postgres::tokio_postgres::NoTls;
#[cfg(test)]
use bb8_rusqlite::RusqliteConnectionManager;

#[cfg(not(test))]
pub type ConnectionManager = PostgresConnectionManager<NoTls>;
#[cfg(test)]
pub type ConnectionManager = RusqliteConnectionManager;
pub type ConnectionPool = Pool<ConnectionManager>;
pub type Connection = PooledConnection<'static, ConnectionManager>;

#[cfg(not(test))]
pub async fn create_pool(database_url: String) -> ConnectionPool {
    let manager = PostgresConnectionManager::new_from_stringlike(database_url, NoTls).unwrap();

    Pool::builder().build(manager).await.unwrap()
}

#[cfg(test)]
pub async fn create_pool(database_url: String) -> ConnectionPool {
    let manager =  RusqliteConnectionManager::new(database_url);

    Pool::builder().build(manager).await.unwrap()
}

pub struct DatabaseConnection(pub Connection);

#[async_trait]
impl<B> FromRequest<B> for DatabaseConnection
    where
        B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(pool) = Extension::<ConnectionPool>::from_request(req)
            .await
            .map_err(internal_error)?;

        let conn = pool.get_owned().await.map_err(internal_error)?;

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