use axum::{extract::State, http::StatusCode};

use crate::{types::ConnectionPool, utils::connection::internarnal_error};

pub async fn get_products(
    State(pool): State<ConnectionPool>,
) -> Result<String, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internarnal_error)?;

    let row = conn
        .query_one("select 1 + 1", &[])
        .await
        .map_err(internarnal_error)?;

    let two: i32 = row.try_get(0).map_err(internarnal_error)?;

    Ok(two.to_string())
}
