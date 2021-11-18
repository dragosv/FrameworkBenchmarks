extern crate serde_derive;
extern crate dotenv;
extern crate async_trait;
extern crate tokio_pg_mapper_derive;
extern crate tokio_pg_mapper;

mod common_handlers;
mod models_common;
mod server;

use dotenv::dotenv;
use axum::{Router, routing::get};
use axum::http::{header, HeaderValue};
use tower_http::set_header::SetResponseHeaderLayer;
use hyper::Body;

use common_handlers::{json, plaintext};

fn main() {
    dotenv().ok();

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    for _ in 1..num_cpus::get() {
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(serve());
        });
    }

    rt.block_on(serve());
}

async fn serve() {
    println!("Started http server: 127.0.0.1:8000");

    let router =  Router::new()
        .route("/plaintext", get(plaintext))
        .route("/json", get(json))
        .layer(SetResponseHeaderLayer::<_, Body>::if_not_present(header::SERVER, HeaderValue::from_static("Axum")));

    server::builder()
        .http1_pipeline_flush(true)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
