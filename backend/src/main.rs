use axum::http::StatusCode;
use axum::Router;
use axum::{extract::State, routing::get};
use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

use controllers::product::get_products;

pub mod controllers;
pub mod models;
pub mod types;
pub mod utils;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let manager =
        PostgresConnectionManager::new_from_stringlike("host=localhost user=postgres password=1234", NoTls)
            .unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();

    let app = Router::new()
        .route("/products", get(get_products))
        .with_state(pool);

    axum::serve(listener, app).await.unwrap()
}