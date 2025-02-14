use axum::{
    extract::{Query, State}, handler::Handler, http::{response, HeaderMap}, response::{IntoResponse, Response}, routing::{get, post}, Extension, Json, Router
};
use crate::strs::get_jwt_identity;

use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use hyper::StatusCode;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde_json::{json, Value,to_string};
use sqlx::{sqlite::SqlitePool, Sqlite};

use crate::strs::Claims;

pub fn admin_routes_add_products(pool:SqlitePool) -> Router {
    Router::new().route("/api/admin/add_Products", post(add_products.layer(Extension(pool.clone())))
    )
}
pub fn admin_routes_add_categorys(pool:SqlitePool) -> Router {
    Router::new().route("/api/admin/add_category", post(add_category.layer(Extension(pool.clone()))))
}



use crate::strs::Product;
use crate::errors::{Error};

#[axum::debug_handler]
async fn add_products(Extension(pool):Extension<SqlitePool>,headers: HeaderMap,payload: Json<Product>) -> Result<Json<Value>, Error> {
    let identity = get_jwt_identity(headers)
        .await
        .map_err(|_| Error:: Unauthorized)?;

    let role = identity
    .get("role") //
    .and_then(|v| v.as_str())
    .ok_or(Error:: Unauthorized)?; 

    if role != "admin"{
        return Err(Error:: Unauthorized);
        
    }  
    
    let product_name=payload.product_name.clone();
    let product_description=payload.product_description.clone();
    let product_price=payload.product_price.clone();
    let product_quantity=payload.product_quantity.clone();
    let product_category_id=payload.product_category_id.clone();
    



    
    let execute=sqlx::query!("INSERT INTO Products (Name, Description, Price, StockQuantity, CategoryID) VALUES (?, ?, ?, ?, ?)",product_name,product_description,product_price,product_quantity,product_category_id) 
    .fetch_one(&pool)
    .await;
    Ok(Json(json!({"message": "executed"})))
    
            
    } 






#[derive(Debug,Deserialize)]
struct Category{
    category_name:String,
    parent_category_id:Option<String>

}




#[axum::debug_handler]
async fn add_category(Extension(pool):Extension<SqlitePool>,headers: HeaderMap,payload: Json<Category>) -> Result<Json<Value>, Error> {
    let identity = get_jwt_identity(headers)
        .await
        .map_err(|_| Error:: Unauthorized)?;

    let role = identity
    .get("role") //
    .and_then(|v| v.as_str())
    .ok_or(Error:: Unauthorized)?; 

    if role != "admin"{
        return Err(Error:: Unauthorized);
    }  
    
    let categoryname= payload.category_name.clone();
    let parentcategoryid=payload.parent_category_id.clone();
    
    let execute=sqlx::query!("INSERT INTO Categories (CategoryName, ParentCategoryID) VALUES (?, ?)",categoryname,parentcategoryid).execute(&pool).await; 
    {
    Ok(Json(json!({"message": "Category Added"})))

        }
    } 


