
use std::{f32::consts::E, result};
use std::convert::TryInto;
use serde::{Deserialize, Serialize};
use axum::{
    extract::State,
    http::{response, HeaderMap},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use crate::db::create_connection_pool;
use chrono::{Duration, Utc};
use hyper::StatusCode;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde_json::{json, Value,to_string};
use tokio::net::windows::named_pipe;
use sqlx::sqlite::SqlitePool;
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
    pub user_id:i64,
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

pub async fn generate_tokens(username: &str, pool:&SqlitePool) -> (String, String) {
   
    let secret = b"secret";
    let refresh_secret = b"secret";

   
    let mut conn = pool.acquire().await.expect("Failed to acquire connection");

   
    let user_id_result = sqlx::query!("SELECT userid FROM users WHERE username = ? ", username).fetch_optional(pool)
    .await;

    let user_id = match user_id_result {
        Ok(Some(record)) => record.UserID,  
        Ok(None) => {
            eprintln!("User not found");
            return ("".to_string(), "".to_string());  
        }
        Err(e) => {
            eprintln!("Error retrieving user_id: {:?}", e);
            return ("".to_string(), "".to_string()); 
        }
    };
    

    let is_admin_result = sqlx::query!("SELECT id FROM admins WHERE user_id = ?", user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to execute query: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        });

    

        let role = match is_admin_result {
            Ok(Some(_)) => "admin",  // User is an admin
            Ok(None) => "user",      // User is a regular user
            Err(_) => "user",        // Default role in case of error
        };
    
    let access_claims = Claims {
        user_id: user_id.expect("User ID should be present"),
        sub: username.to_string(),
        exp: (Utc::now() + Duration::minutes(15)).timestamp() as usize, // 15 minutes expiry
        role: role.to_string(),
    };

    let refresh_claims = Claims {
        user_id: user_id.expect("User ID should be present"),
        sub: username.to_string(),
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize, // 7 days expiry
        role: role.to_string(),
    };

    // Encode tokens
    let access_token = encode(&Header::default(), &access_claims, &EncodingKey::from_secret(secret))
        .expect("Token encoding failed");
    let refresh_token = encode(&Header::default(), &refresh_claims, &EncodingKey::from_secret(refresh_secret))
        .expect("Token encoding failed");

    (access_token, refresh_token)
}



pub async fn check_admin(headers: HeaderMap, pool: &SqlitePool) -> Result<bool, StatusCode> {
    let identity = get_jwt_identity(headers)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let sub = identity
        .get("sub") //
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let user_id = identity.get("user_id").and_then(|i| i.as_i64()).ok_or(StatusCode::UNAUTHORIZED)? as i32;


        let exists = sqlx::query!("select id from admins where user_id = ? ",user_id).fetch_optional(pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        if exists.is_some(){
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

use sqlx::Row;

pub async fn get_products_by_category(pool: &SqlitePool, category: &str) -> Result<Vec<serde_json::Value>, StatusCode> {
    let query = "SELECT * FROM products WHERE category = ?"; // Replace with your actual query

    // Execute query with pool and map rows to JSON
    let products = sqlx::query(query)
        .bind(category)  // Bind category parameter to the query
        .fetch_all(pool) // Execute the query using the pool
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // Handle errors

    // Map each row into a JSON object
    let result: Vec<serde_json::Value> = products.into_iter().map(|row| {
        json!({
            "id": row.get::<i32, _>("id"),
            "name": row.get::<String, _>("name"),
            "price": row.get::<f64, _>("price"),
            "category": row.get::<String, _>("category"),
        })
    }).collect();

    Ok(result)
}


#[derive(Debug, Deserialize)]
pub struct SignupPayload{
    pub username:String,
    pub password:String,
    pub email:String,
    pub first_name:String,
    pub last_name:String,
    pub address:String,
    pub city :String,
    pub state:String,
    pub zipcode:String,
    pub country :String,
    pub phonenumber:String
}