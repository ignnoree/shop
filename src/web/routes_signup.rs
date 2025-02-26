use axum::handler::Handler;
use axum::Extension;
use axum::{
    extract::Json,
    response::IntoResponse,
    routing::post,
    Router,
};

use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::query;
use crate::errors::{Error}; 
use crate::hashing::{hash_password, verify_password};
use crate::strs::SignupPayload;
use sqlx::sqlite::SqlitePool;


pub fn routes(pool:SqlitePool)->Router{
    Router::new().route("/api/signup",post(api_signup.layer(Extension(pool.clone())))
    )
}


use sqlx::sqlite::SqliteError;
use sqlx::Error as SqlxError;

#[axum::debug_handler]
pub async fn api_signup(
    Extension(pool): Extension<SqlitePool>,
    payload: Json<SignupPayload>,
) -> Result<Json<Value>, Error> {
    
    let query = sqlx::query!("SELECT COUNT(*) AS count FROM Users WHERE Username = ?", payload.username)
        .fetch_one(&pool)
        .await
        .map_err(|_| Error::InternalServerError)?; 

    
    if query.count > 0 {
        return Err(Error::UserAlreadyExists);
    } 


    let query = sqlx::query!("SELECT COUNT(*) AS count FROM Users WHERE email = ?", payload.email)
        .fetch_one(&pool)
        .await
        .map_err(|_| Error::InternalServerError)?; 

    
    if query.count > 0 {
        return Err(Error::UserAlreadyExists);
    } 

    let password_hash = hash_password(&payload.password);

    
    let query = "INSERT INTO Users (Username, PasswordHash, Email, FirstName, LastName, Address, City, State, Zipcode, Country, PhoneNumber) 
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

    let res =sqlx::query(query)
        .bind(&payload.username)
        .bind(&password_hash)
        .bind(&payload.email)
        .bind(&payload.first_name)
        .bind(&payload.last_name)
        .bind(&payload.address)
        .bind(&payload.city)
        .bind(&payload.state)
        .bind(&payload.zipcode)
        .bind(&payload.country)
        .bind(&payload.phonenumber)
        .execute(&pool)
        .await
        .map_err(|_| Error::InternalServerError)?; // Handle error if insert fails

    println!("Successfully reached!");

    // Return success response
    Ok(Json(json!({ "message": "Signup Success" })))
}