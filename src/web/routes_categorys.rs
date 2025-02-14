
use axum::{
    extract::Path, handler::Handler, http::HeaderMap, response::{IntoResponse, Json}, routing::{get,post}, Router
};
use jsonwebtoken::Header;
use serde_json::json;
use hyper::StatusCode;
use serde_json::Value;
use crate::strs::get_jwt_identity;
use crate::strs::get_products_by_category;
use axum::Extension;
use sqlx::{Sqlite, SqlitePool};

pub fn category_routes(pool:SqlitePool) -> Router {
    Router::new().route("/api/category/{category_name}", get(category_api_handler.layer(Extension(pool.clone()))))
}


async fn category_api_handler(Extension(pool): Extension<SqlitePool>,Path(category_name): Path<String>,headers:HeaderMap)
->Result<Json<serde_json::Value>,StatusCode>{
    let idenity:Json<Value>=get_jwt_identity(headers).await.map_err(|_| StatusCode::UNAUTHORIZED)?;
    match idenity.get("role"){
        Some(role)=>{
            let prods=get_products_by_category(&pool,&category_name).await;
            match prods{
                Ok(prods)=>{
                    Ok(Json(json!(prods)))
                }
                Err(_)=>{
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        None=>Err(StatusCode::UNAUTHORIZED)
        }
}
    

