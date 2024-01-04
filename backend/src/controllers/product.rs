use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

use crate::{models::product::Product, utils::connection::internarnal_error};

pub async fn get_products(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<Product>>, (StatusCode, String)> {
    sqlx::query_as!(Product, "SELECT * FROM products")
        .fetch_all(&pool)
        .await
        .map(|product| axum::Json(product))
        .map_err(internarnal_error)
}

#[derive(Deserialize)]
pub struct CreateProduct {
    name: String,
    color: String,
    category: String,
    price: i32,
}

#[derive(Deserialize)]
pub struct DeleteProduct {
    id: i32,
}

#[derive(Deserialize)]
pub struct UpdateProduct {
    id: i32,
    name: Option<String>,
    color: Option<String>,
    category: Option<String>,
    price: Option<i32>,
}

pub async fn create_product(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CreateProduct>,
) -> Result<(), (StatusCode, String)> {
    let _ = sqlx::query!(
        "INSERT INTO products (name, color, category, price) VALUES ($1, $2, $3, $4)",
        payload.name,
        payload.color,
        payload.category,
        payload.price,
    )
    .execute(&pool)
    .await;

    Ok(())
}

pub async fn delete_product(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<DeleteProduct>,
) -> Result<(), (StatusCode, String)> {
    let _ = sqlx::query!("DELETE FROM products WHERE id = $1", payload.id)
        .execute(&pool)
        .await;

    Ok(())
}

pub async fn update_product(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<UpdateProduct>,
) -> Result<(), (StatusCode, String)> {
    let _ = sqlx::query!(
        "UPDATE products
        SET name=coalesce($2, name),
        color=coalesce($3, color),
        category=coalesce($4, category),
        price=coalesce($5, price)
        WHERE id = $1",
        payload.id,
        payload.name,
        payload.color,
        payload.category,
        payload.price
    )
    .execute(&pool)
    .await;

    Ok(())
}
