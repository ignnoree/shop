
use std::{f32::consts::E, result};

use serde::{Deserialize, Serialize};
use axum::{
    extract::State,
    http::{response, HeaderMap},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use rusqlite::{params, Connection, Result};
use chrono::{Duration, Utc};
use hyper::StatusCode;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde_json::{json, Value,to_string};
use tokio::net::windows::named_pipe;

#[derive(Debug,Deserialize)]
pub struct LoginPayload{
    pub username:String,
    pub password:String,
} 


#[derive(Debug,Serialize)]
pub struct LoginResponse{
    pub token:String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Claims{
    pub user_id:i32,
    pub sub:String,
    pub exp:usize,
    pub role:String,

}

pub async fn get_jwt_identity(headers: HeaderMap) -> Result<Json<serde_json::Value>, StatusCode> {
    
    let auth_header = match headers.get("Authorization") {
        Some(auth) => auth,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    
    let auth_header_str = match auth_header.to_str() {
        Ok(auth_str) => auth_str,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    
    if !auth_header_str.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    
    let token = auth_header_str.trim_start_matches("Bearer ");

    
    let secret = b"secret"; 
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(decoded_claims) => {
            
            let response= serde_json::to_value(&decoded_claims.claims).unwrap();
            Ok(Json(response))
        }
        Err(e) => {
            eprintln!("JWT decode error: {}", e);
            Err(StatusCode::UNAUTHORIZED)
            
        }
    }
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Product{
    pub product_name:String,
    pub product_description:String,
    pub product_price:f64,
    pub product_quantity:i32,
    pub product_category_id:i32,
}


#[derive(Debug,Deserialize,Serialize)]
pub struct ReviewPayload{
    pub rating:i32,
    pub review:String,
}

pub async fn generate_tokens(username: &str) -> (String, String) {
    let secret = b"secret";
    let refresh_secret = b"secret";
    let connection=Connection::open("database.db").unwrap();
    let query = "SELECT * FROM admins WHERE user_id = ?1";
    let query2:&str="select userid from users where username = ?1";
    let mut statement = connection.prepare(query).unwrap();
    let mut statement2 = connection.prepare(query2).unwrap();
    let user_id:i32 =statement2.query_row(params![username], |row| row.get(0)).unwrap();
    
    let exists = statement.exists(params![user_id]).unwrap();
    println!("exists: {}", exists);
    let role = if exists{
        "admin"
    }
    else{
        "user"
    };
    let access_claims = Claims {
        user_id:user_id,
        sub:username.to_string(),
        exp: (Utc::now() + Duration::minutes(15)).timestamp() as usize, // 15 min
        role: role.to_string(),
    };

    let refresh_claims = Claims {
        user_id:user_id,
        sub: username.to_string(),
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize, // 7 days
        role: role.to_string(),
    };

    let access_token = encode(&Header::default(), &access_claims, &EncodingKey::from_secret(secret)).unwrap();
    let refresh_token = encode(&Header::default(), &refresh_claims, &EncodingKey::from_secret(refresh_secret)).unwrap();

    (access_token, refresh_token)
}


pub async fn check_admin(headers: HeaderMap) -> Result<bool, StatusCode> {
    let identity = get_jwt_identity(headers)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let sub = identity
        .get("sub") //
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::UNAUTHORIZED)?;

        let connection = Connection::open("database.db").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let query = "SELECT * FROM admin WHERE Username = ?1";
        let mut statement = connection.prepare(query).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let exists = statement.exists(params![sub]).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        if exists{
            Ok((true))
        }
        else{
            Err(StatusCode::UNAUTHORIZED)
}}

#[derive(Debug,Deserialize,Serialize)]
pub struct CartPayload{
    pub product_id:i32,
    pub quantity:i32,
}




pub async fn query_prods_by_category(category:&str)->Result<Json<serde_json::Value>, StatusCode>{
    let connection = Connection::open("database.db").map_err(|e| {
        println!("Failed to open database connection: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    println!("Connected to database");

    let mut stmt = connection.prepare("SELECT * FROM products WHERE categoryid = (SELECT categoryid FROM categories WHERE categoryname = ?)").map_err(|e| {
        println!("Failed to prepare query: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let products = stmt.query_map(params![&category], |row| {
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