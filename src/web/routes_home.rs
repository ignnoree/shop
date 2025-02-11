use axum::{
    http::{HeaderMap},
    response::{IntoResponse,Response},
    routing::{post, Router,get},
    Json,
};

use serde_json::json;
use crate::errors::Error;
use hyper::StatusCode;
use crate::strs::get_jwt_identity;
pub fn home_routes() -> Router {
    Router::new().route("/api/home", get(home_handler))
}
use rusqlite::{params, Connection, Result};

async fn home_handler(headers:HeaderMap) -> Result<Json<serde_json::Value>, StatusCode>{
    let identity = get_jwt_identity(headers)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    match identity.get("role") {
        Some(role) => {
            let conn=Connection::open("database.db").map_err(|e| {
                println!("Failed to open database connection: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            println!("Connected to database");
            let mut stmt = conn.prepare("SELECT * FROM products").map_err(|e| {
                println!("Failed to prepare query: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            let products = stmt.query_map(params![], |row| {
                Ok(json!({
                    "product_id": row.get::<_, i32>(0)?,
                    "product_name": row.get::<_, String>(1)?,
                    "product_description": row.get::<_, String>(2)?,
                    "product_price": row.get::<_, f64>(3)?,
                    "product_category_id": row.get::<_, i32>(4)?,
                    "product_quantity": row.get::<_, i32>(5)?,
                    
                }))
            }).map_err(|e| {
                println!("Failed to execute query: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            let mut products_json = Vec::new();
            for product in products {
                products_json.push(product.map_err(|e| {
                    println!("Failed to get product: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?);
            }
            Ok(Json(json!({"products": products_json})))

                
        }
        None => Err(StatusCode::UNAUTHORIZED)
}}


