
use axum::{
    extract::Path,
    response::{Json, IntoResponse},
    Router,
    routing::{get,post},
    http::HeaderMap,
};
use jsonwebtoken::Header;
use rusqlite::{params, Connection};
use serde_json::json;
use hyper::StatusCode;
use serde_json::Value;
use crate::strs::get_jwt_identity;
use crate::strs::query_prods_by_category;


pub fn category_routes() -> Router {
    Router::new().route("/api/category/{category_name}", get(category_api_handler))
}


async fn category_api_handler(Path(category_name): Path<String>,headers:HeaderMap)
->Result<Json<serde_json::Value>,StatusCode>{
    let idenity:Json<Value>=get_jwt_identity(headers).await.map_err(|_| StatusCode::UNAUTHORIZED)?;
    match idenity.get("role"){
        Some(role)=>{
            let prods=query_prods_by_category(&category_name).await;
            match prods{
                Ok(prods)=>{
                    Ok(prods)
                }
                Err(_)=>{
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        None=>Err(StatusCode::UNAUTHORIZED)
        }
}
    

