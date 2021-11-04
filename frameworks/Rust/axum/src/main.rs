#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
#[macro_use]
extern crate async_trait;

mod models;
mod random;
mod database;
mod request;

use dotenv::dotenv;
use std::net::{Ipv4Addr, SocketAddr};
use std::env;
use std::sync::Arc;
use yarte::Template;
use crate::database::{DatabaseConnection, DbPool};
use sqlx::Acquire;
use axum::{
    extract::{Query},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    AddExtensionLayer, Json, Router,
};
use axum::extract::Extension;
use axum::http::{header, HeaderValue};
use axum::response::Html;
use serde::{Deserialize};
use tower_http::set_header::SetResponseHeaderLayer;
use hyper::Body;

use models::{World, Fortune, Message};
use random::DynRandomizerFactory;
use request::RequestId;
use database::create_pool;
use crate::random::StandardRandomizerFactory;

#[derive(Debug, Deserialize)]
struct Params {
    q: Option<u16>,
}

async fn plaintext() -> &'static str {
    "Hello, World!"
}

async fn json() -> impl IntoResponse {
    let message = Message {
        message: "Hello, World!",
    };

    (StatusCode::OK, Json(message))
}

async fn db(DatabaseConnection(mut conn): DatabaseConnection, Extension(randomizer_factory): Extension<DynRandomizerFactory>) -> impl IntoResponse {
    let mut randomizer = randomizer_factory.create();
    let number = randomizer.next();

    let world : World = sqlx::query_as("SELECT id, randomnumber FROM World WHERE id = $1").bind(number)
        .fetch_one(&mut conn).await.ok().expect("error loading world");

    (StatusCode::OK, Json(world))
}

async fn queries(DatabaseConnection(mut conn): DatabaseConnection, Extension(randomizer_factory): Extension<DynRandomizerFactory>, Query(params): Query<Params>) -> impl IntoResponse {
    let mut q = 0;

    if params.q.is_some() {
        q = params.q.ok_or("could not get value").unwrap();
    }

    let q = if q == 0 {
        1
    } else if q > 500 {
        500
    } else {
        q
    };

    let mut randomizer = randomizer_factory.create();

    let mut results = Vec::with_capacity(q as usize);

    for _ in 0..q {
        let query_id = randomizer.next();

        let result :World = sqlx::query_as("SELECT * FROM World WHERE id = $1").bind(query_id)
            .fetch_one(&mut conn).await.ok().expect("error loading world");

        results.push(result);
    }

    (StatusCode::OK, Json(results))
}

#[derive(Template)]
#[template(path = "fortunes.html.hbs")]
pub struct FortunesTemplate<'a> {
    pub fortunes: &'a Vec<Fortune>,
}

async fn fortunes(DatabaseConnection(mut conn): DatabaseConnection) -> impl IntoResponse {
    let mut fortunes: Vec<Fortune> = sqlx::query_as("SELECT * FROM Fortune").fetch_all(&mut conn).await
        .ok().expect("Could not load Fortunes");

    fortunes.push(Fortune {
        id: 0,
        message: "Additional fortune added at request time.".to_string(),
    });

    fortunes.sort_by(|a, b| a.message.cmp(&b.message));

    Html(
        FortunesTemplate {
            fortunes: &fortunes,
        }
        .call()
        .expect("error rendering template"),
    )
}

async fn updates(DatabaseConnection(mut conn): DatabaseConnection, Extension(randomizer_factory): Extension<DynRandomizerFactory>, Query(params): Query<Params>) -> impl IntoResponse {
    let mut q = 0;

    if params.q.is_some() {
        q = params.q.ok_or("could not get value").unwrap();
    }

    let q = if q == 0 {
        1
    } else if q > 500 {
        500
    } else {
        q
    };

    let mut randomizer = randomizer_factory.create();

    let mut results = Vec::with_capacity(q as usize);

    for _ in 0..q {
        let query_id = randomizer.next();
        let mut result :World = sqlx::query_as("SELECT * FROM World WHERE id = $1").bind(query_id)
            .fetch_one(&mut conn).await.ok().expect("World was not found");

        result.random_number = randomizer.next();
        results.push(result);
    }

    let mut tx = conn.begin().await.ok().expect("could not start transaction");

    for w in &results {
        sqlx::query("UPDATE World SET randomnumber = $1 WHERE id = $2")
            .bind(w.random_number).bind(w.id)
            .execute(&mut tx)
            .await.ok().expect("Could not update World");
    }

    tx.commit().await.ok().expect("could not update worlds");

    (StatusCode::OK, Json(results))
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let database_url = env::var("AXUM_TECHEMPOWER_DATABASE_URL").ok()
        .expect("AXUM_TECHEMPOWER_DATABASE_URL environment variable was not set");

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8000));
    tracing::debug!("axum listening on {}", addr);

    // setup connection pool
    let pool = create_pool(database_url).await;

    let app = router(pool).await;

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn router(pool: DbPool) -> Router {
    let randomizerFactory = Arc::new(StandardRandomizerFactory) as DynRandomizerFactory;

    Router::new()
        .route("/plaintext", get(plaintext))
        .route("/json", get(json))
        .route("/fortunes", get(fortunes))
        .route("/db", get(db))
        .route("/queries", get(queries))
        .route("/updates", get(updates))
        .layer(AddExtensionLayer::new(pool))
        .layer(AddExtensionLayer::new(randomizerFactory))
        .layer(SetResponseHeaderLayer::<_, Body>::if_not_present(header::SERVER, HeaderValue::from_static("Axum")))
}

#[cfg(test)]
mod tests
{
    use super::*;
    use axum::{body::Body, http::Request};
    use axum::body::BoxBody;
    use axum::http::Response;
    use tower::ServiceExt;
    use crate::database::{DbPool, DbPoolConnection};

    #[tokio::test]
    async fn server_header() {
        let response = get_request_response("/plaintext").await;

        assert_eq!(true, response.headers().contains_key("Server"));
        assert_eq!("Axum", response.headers().get("Server").expect("could not retrieve Server header").to_str().unwrap());
    }

    #[tokio::test]
    async fn plaintext() {
        let response = get_request("/plaintext").await;

        assert_eq!(response.as_str(), "Hello, World!");
    }

    #[tokio::test]
    async fn json() {
        let response = get_request("/json").await;
    
        assert_eq!(response.as_str(), "{\"message\":\"Hello, World!\"}");
    }
    
    #[tokio::test]
    async fn db() {
        let response = get_request("/db").await;
    
        assert_eq!(response.as_str(), "{\"id\":1,\"randomNumber\":101}");
    }
    
    #[tokio::test]
    async fn queries_empty() {
        let response = get_request("/queries").await;
    
        assert_eq!(response.as_str(), "[{\"id\":1,\"randomNumber\":101}]");
    }
    
    #[tokio::test]
    async fn queries_non_empty() {
        let response = get_request("/queries?q=3").await;
    
        assert_eq!(response.as_str(), "[{\"id\":1,\"randomNumber\":101},{\"id\":2,\"randomNumber\":102},{\"id\":3,\"randomNumber\":103}]");
    }
    
    #[tokio::test]
    async fn fortunes() {
        let response = get_request("/fortunes").await;
    
        assert_eq!(response.as_str(), "<!DOCTYPE html><html><head><title>Fortunes</title></head><body><table><tr><th>id</th><th>message</th></tr><tr><td>11</td><td>&lt;script&gt;alert(&quot;This should not be displayed in a browser alert box.&quot;);&lt;&#x2f;script&gt;</td></tr><tr><td>4</td><td>A bad random number generator: 1, 1, 1, 1, 1, 4.33e+67, 1, 1, 1</td></tr><tr><td>5</td><td>A computer program does what you tell it to do, not what you want it to do.</td></tr><tr><td>2</td><td>A computer scientist is someone who fixes things that aren&#x27;t broken.</td></tr><tr><td>8</td><td>A list is only as strong as its weakest link. — Donald Knuth</td></tr><tr><td>0</td><td>Additional fortune added at request time.</td></tr><tr><td>3</td><td>After enough decimal places, nobody gives a damn.</td></tr><tr><td>7</td><td>Any program that runs right is obsolete.</td></tr><tr><td>10</td><td>Computers make very fast, very accurate mistakes.</td></tr><tr><td>6</td><td>Emacs is a nice operating system, but I prefer UNIX. — Tom Christaensen</td></tr><tr><td>9</td><td>Feature: A bug with seniority.</td></tr><tr><td>1</td><td>fortune: No such file or directory</td></tr><tr><td>12</td><td>フレームワークのベンチマーク</td></tr></table></body></html>");
    }
    
    #[tokio::test]
    async fn updates_empty() {
        let response = get_request("/updates").await;
    
        assert_eq!(response.as_str(), "[{\"id\":1,\"randomNumber\":2}]");
    }
    
    #[tokio::test]
    async fn updates_non_empty() {
        let response = get_request("/updates?q=3").await;
    
        assert_eq!(response.as_str(), "[{\"id\":1,\"randomNumber\":2},{\"id\":3,\"randomNumber\":4},{\"id\":5,\"randomNumber\":6}]");
    }

    async fn get_request(url: &str) -> String {
        // setup connection pool
        let response = get_request_response(url).await;

        let body = response
            .into_body();

        let bytes = hyper::body::to_bytes(body).await.unwrap();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    async fn get_request_response(url: &str) -> Response<BoxBody> {
        let pool: DbPool = create_pool(":memory:".to_string()).await;

        let mut conn: DbPoolConnection = pool.acquire().await.unwrap();
        sqlx::migrate!("db/migrations").run(&mut conn).await.ok().expect("error executing migrations");

        let response = super::router(pool).await
            .oneshot(
                Request::builder()
                    .uri(url)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        response
    }
}


