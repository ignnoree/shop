use axum::extract::Path;
use axum::http::{StatusCode};
use axum::Router;
use axum::routing::{get, post};
use hyper::header::PRAGMA;
use hyper::HeaderMap;
use jsonwebtoken::Header;
use axum::Json;
use serde_json::Value;
use crate::strs::{get_jwt_identity, Product};
use crate::strs::query_prods_by_category;
pub fn routes_reviews() -> Router {
    Router::new()
    .route("/api/reviews/{product_id}", get(reviews_handler))
    .route("/api/reviews/{product_id}", post(reviews_handler_post))
}

use rusqlite::{params, Connection, Row};
use serde_json::json;
use crate::strs::ReviewPayload;



async fn reviews_handler(Path(product_id):Path<u32>,headers:HeaderMap) -> Result<Json<serde_json::Value>,StatusCode> {
    println!("reviews handler");

    let idenity:Json<Value> = get_jwt_identity(headers)
    .await.map_err(|_| StatusCode::UNAUTHORIZED)?;


    match idenity.get("role"){


        Some(role)=>{
            let reviews="select * from reviews where productid=?".to_string();
            let conn = Connection::open("database.db").unwrap();
            let mut stmt = conn.prepare(&reviews).unwrap();
            let reviewsjson  = stmt.query_map(params![product_id],|row|{
                Ok(json!({
                    "review_id":row.get::<_,i32>(0).unwrap(),
                    "user_id":row.get::<_,i32>(1).unwrap(),
                    "product_id":row.get::<_,i32>(2).unwrap(),
                    "rating":row.get::<_,i32>(3).unwrap(),
                    "review":row.get::<_,String>(4).unwrap(),

                }))
            }).map_err(|e| {
                println!("Failed to execute query: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            let mut products_json = Vec::new();
            for product in reviewsjson {
                products_json.push(product.map_err(|e| {
                    println!("Failed to get product: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?);
            }
            Ok(Json(json!({"reviews": products_json})))
                }

    None=>Err(StatusCode::UNAUTHORIZED)
    }
}

#[axum::debug_handler]
async fn reviews_handler_post(Path(product_id):Path<u32>,headers:HeaderMap,payload:Json<ReviewPayload>) -> Result<Json<serde_json::Value>,StatusCode> {

    let idenity:Json<Value> = get_jwt_identity(headers)
    .await.map_err(|_| StatusCode::UNAUTHORIZED)?;

    match idenity.get("role"){

        Some(role)=>{
            let conn = Connection::open("database.db").unwrap();

            let reviews="insert into reviews (userid,productid,rating,comment) values (?,?,?,?)".to_string();
            let mut stmt = conn.prepare(&reviews).unwrap();

            let username=idenity.get("sub").unwrap().as_str().unwrap().to_string();
            let userid=idenity.get("user_id").unwrap().as_i64().unwrap();
    
        

            println!("userid is {} , proid is {} ,{} {} ",userid,product_id,payload.rating,payload.review);
            
            stmt.execute(params![userid,product_id,payload.rating,payload.review]).map_err(|e|{
                println!("Failed to execute query: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR

            
            })?;
            Ok(Json(json!({"message":"review added"}))
            )
        }
        None=>Err(StatusCode::UNAUTHORIZED)
    }



}