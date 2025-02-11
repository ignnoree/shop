

use std::result;

use axum::{Router, Json, extract::Extension, response::Html, http::StatusCode, extract::Path, response::IntoResponse};
use axum::routing::{get, post};

use crate::strs::CartPayload;
use hyper::HeaderMap;
use serde_json::json;
use crate::strs::get_jwt_identity;
use rusqlite::{params, Connection};
use serde_json::Value;

fn cart_routes() -> Router {
    Router::new()
        .route("/api/cart", get(cart_handler))
        .route("/api/cart", post(cart_handler_post))
}



async fn cart_handler(header:HeaderMap) -> Result<Json<Value>,StatusCode>{
    let idenity=get_jwt_identity(header).await.map_err(|_| StatusCode::UNAUTHORIZED)?;
    match idenity.get("user_id"){
        Some(user_id)=>{
            let user_id=idenity.get("user_id").and_then(|i| i.as_i64())
            .ok_or(StatusCode::UNAUTHORIZED)? as i32;




            let cart="select * from cart where user_id=?".to_string();
            let conn = Connection::open("database.db").unwrap();
            let mut stmt = conn.prepare(&cart).unwrap();
            let cartjson  = stmt.query_map(params![user_id],|row|{
                Ok(json!({
                    "cart_id":row.get::<_,i32>(0).unwrap(),
                    "user_id":row.get::<_,i32>(1).unwrap(),
                    "product_id":row.get::<_,i32>(2).unwrap(),
                    "quantity":row.get::<_,i32>(3).unwrap(),
                    "price":row.get::<_,i32>(4).unwrap(),

                }))
            }).map_err(|e| {
                println!("Failed to execute query: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            let mut cart_json = Vec::new();
            for product in cartjson {
                cart_json.push(product.map_err(|e| {
                    println!("Failed to get product: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?);
            }
            Ok(Json(json!({"cart": cart_json})))
        }
        None=>{
            Err(StatusCode::UNAUTHORIZED)
    }
}}


#[axum::debug_handler]
async fn cart_handler_post(header:HeaderMap,payload:Json<CartPayload>) ->Result<Json<Value>,StatusCode>{
    let idenity=get_jwt_identity(header).await.map_err(|_| StatusCode::UNAUTHORIZED)?;
    match idenity.get("user_id"){
        Some(user_id)=>{


            let user_id=idenity.get("user_id").and_then(|i| i.as_i64())
            .ok_or(StatusCode::UNAUTHORIZED)? as i32;


            let conn = Connection::open("database.db").unwrap();
            let insert = "INSERT INTO cart (user_id,product_id,quantity,price) VALUES (?1,?2,?3,?4)";
            conn.execute(insert,params![user_id,payload.product_id,payload.quantity]).map_err(|e|{
                println!("Failed to insert into cart: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            Ok(Json(json!({"message":"Added to cart"})))
        }
        None=>{
            Err(StatusCode::UNAUTHORIZED)
    }
    
}}