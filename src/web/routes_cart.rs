

use std::result;

use axum::{Router, Json, extract::Extension, response::Html, http::StatusCode, extract::Path, response::IntoResponse};
use axum::routing::{get, post};
use sqlx::SqlitePool;

use crate::strs::CartPayload;
use hyper::HeaderMap;
use serde_json::json;
use crate::strs::get_jwt_identity;
use axum::handler::Handler;
use serde_json::Value;

pub fn cart_routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/cart", get(cart_handler).layer(Extension(pool.clone())))
        .route("/api/cart", post(cart_handler_post).layer(Extension(pool.clone())))
}






#[axum::debug_handler]
async fn cart_handler(Extension(pool):Extension<SqlitePool>,header:HeaderMap) ->Result<Json<Value>,StatusCode> {
    let idenity=get_jwt_identity(header).await.map_err(|_| StatusCode::UNAUTHORIZED)?;
    match idenity.get("user_id"){
        Some(user_id)=>{

            let cartjson = sqlx::query!(
                r#"
                SELECT 
                    cart.UserID, 
                    cart.ProductID,
                    cart.Quantity,
                    Products.name,
                    Products.Description,
                    Categories.CategoryName
                FROM cart
                LEFT JOIN products ON Products.ProductID = cart.ProductID
                INNER JOIN Categories ON Categories.categoryid = Products.CategoryID
                WHERE cart.UserID = ?"#,
                user_id
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                eprintln!("Failed to execute query: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        
            
            let result: Vec<Value> = cartjson.into_iter().map(|row| {
                json!({
                    "user_id": row.UserID,
                    "product_id": row.ProductID,
                    "quantity": row.Quantity,
                    "name": row.Name,
                    
                    "description": row.Description.unwrap_or_default(), 
                    "category": row.CategoryName
                })
            }).collect();
        
            
            Ok(Json(json!(result)))


        }
        None=>{
            Err(StatusCode::UNAUTHORIZED)
        }
    }

}



async fn cart_handler_post(Extension(pool):Extension<SqlitePool>,header:HeaderMap,payload:Json<CartPayload>) ->Result<Json<Value>,StatusCode>{
    let idenity=get_jwt_identity(header).await.map_err(|_| StatusCode::UNAUTHORIZED)?;
    match idenity.get("user_id"){
        Some(user_id)=>{
            let user_id=idenity.get("user_id").and_then(|i| i.as_i64())
            .ok_or(StatusCode::UNAUTHORIZED)? as i32;
            
            println!("User ID: {} , product_id : {} , {}", user_id,payload.product_id,payload.quantity);

            let insert = sqlx::query!("INSERT INTO cart (userid,productid,quantity) VALUES (?,?,?)"
            ,user_id,payload.product_id,payload.quantity)
            .fetch_all(&pool)
            .await;
            Ok(Json(json!({"message":"Added to cart"})))
        }
        None=>{
            Err(StatusCode::UNAUTHORIZED)
    }
    
}}