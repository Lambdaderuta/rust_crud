use axum::routing::{delete, get, post, put};
use axum::Router;
use sqlx::postgres::PgPoolOptions;

use controllers::product::{create_product, delete_product, get_products, update_product};

pub mod controllers;
pub mod models;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:1234@localhost/backend")
        .await?;

    let app = Router::new()
        .route("/get_all", get(get_products))
        .route("/create", post(create_product))
        .route("/update", put(update_product))
        .route("/delete", delete(delete_product))
        .with_state(pool);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
