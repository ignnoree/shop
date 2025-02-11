use axum::{
    extract::{Query, State},
    http::{response, HeaderMap},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use crate::strs::get_jwt_identity;

use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use hyper::StatusCode;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde_json::{json, Value,to_string};

use crate::strs::Claims;
use rusqlite::{params, Connection, Result};
pub fn admin_routes_add_products() -> Router {
    Router::new().route("/api/admin/add_Products", post(add_products))
}
pub fn admin_routes_add_categorys() -> Router {
    Router::new().route("/api/admin/add_category", post(add_category))
}



use crate::strs::Product;
use crate::errors::{Error};

#[axum::debug_handler]
async fn add_products(headers: HeaderMap,payload: Json<Product>) -> Result<Json<Value>, Error> {
    let identity = get_jwt_identity(headers)
        .await
        .map_err(|_| Error::UNAUTHORIZED)?;

    let role = identity
    .get("role") //
    .and_then(|v| v.as_str())
    .ok_or(Error::UNAUTHORIZED)?; 

    if role != "admin"{
        return Err(Error::UNAUTHORIZED);
        
    }  
    let product = Product{ 
        product_name:payload.product_name.clone(),
        product_description:payload.product_description.clone(),
        product_price:payload.product_price,
        product_quantity:payload.product_quantity,
        product_category_id:payload.product_category_id,

    };


    let connection = Connection::open("database.db").map_err(|e| {
        println!("Failed to open database connection: {:?}", e);
        Error::InternalServerError
    })?;
    println!("Connected to database");



    let query = "INSERT INTO Products (Name, Description, Price, StockQuantity, CategoryID) VALUES (?1, ?2, ?3, ?4, ?5)";
    match connection.execute(
        query,
        rusqlite::params![
            &product.product_name,
            &product.product_description,
            &product.product_price,
            &product.product_quantity,
            &product.product_category_id,
        ],
    ) {
        Ok(_) => Ok(Json(json!({"message": "Product Added"}))),
        Err(e) => {
            // Print the error and return it wrapped in a JSON response
            println!("Error executing query: {}", e);
            Err(Error::InternalServerError)
        }
    } 
}


#[derive(Debug,Deserialize)]
struct Category{
    category_name:String,
    parent_category_id:Option<String>

}

#[axum::debug_handler]
async fn add_category(headers: HeaderMap,payload: Json<Category>) -> Result<Json<Value>, Error> {
    let identity = get_jwt_identity(headers)
        .await
        .map_err(|_| Error::UNAUTHORIZED)?;

    let role = identity
    .get("role") //
    .and_then(|v| v.as_str())
    .ok_or(Error::UNAUTHORIZED)?; 

    if role != "admin"{
        return Err(Error::UNAUTHORIZED);
    }  
    let category = Category{ 
        category_name:payload.category_name.clone(),
        parent_category_id:payload.parent_category_id.clone(),
    };
    let connection = Connection::open("database.db").map_err(|e| {
        println!("Failed to open database connection: {:?}", e);
        Error::InternalServerError
    })?;
    println!("Connected to database");
    let query="INSERT INTO Categories (CategoryName, ParentCategoryID) VALUES (?1, ?2)";
    match connection.execute(
        query,
        rusqlite::params![
            &category.category_name,
            &category.parent_category_id, 
        ],
    ) {
        Ok(_) => Ok(Json(json!({"message": "Category Added"}))),
        Err(e) => {
            // Print the error and return it wrapped in a JSON response
            println!("Error executing query: {}", e);
            Err(Error::InternalServerError)
        }
    } 
}

