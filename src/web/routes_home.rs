use axum::{
    http::{HeaderMap},
    response::{IntoResponse,Response},
    routing::{post, Router,get},
    Json,
};

use sqlx::{Connection, SqlitePool};
use axum::extract::Extension;
use serde_json::json;
use crate::errors::Error;
use hyper::StatusCode;
use crate::strs::get_jwt_identity;
use axum::handler::Handler;

pub fn home_routes(pool:SqlitePool) -> Router {
    Router::new().route("/api/home", get(home_handler.layer(Extension(pool.clone()))))
}



async fn home_handler(
    headers: HeaderMap,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let identity = get_jwt_identity(headers)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    match identity.get("role") {
        Some(_) => {
            let rows = sqlx::query!(
                "SELECT ProductID, Name,Description,Price, CategoryID, StockQuantity FROM products"
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                println!("Failed to execute query: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            let products: Vec<_> = rows
                .into_iter()
                .map(|row| {
                    json!({
                        "product_id": row.ProductID,
                        "product_name": row.Name,
                        "product_description": row.Description,
                        "product_price": row.Price,
                        "product_category_id": row.CategoryID,
                        "product_quantity": row.StockQuantity,
                    })
                })
                .collect();

            Ok(Json(json!({ "products": products })))
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}


