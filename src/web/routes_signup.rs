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




#[axum::debug_handler]
pub async fn api_signup(
    Extension(pool): Extension<SqlitePool>,
    payload: Json<SignupPayload>,
) -> Result<Json<Value>, Error> {
    // Check if the username already exists
    let query = sqlx::query!("SELECT COUNT(*) AS count FROM Users WHERE Username = ?", payload.username)
        .fetch_one(&pool)
        .await
        .map_err(|_| Error::InternalServerError)?; // Handle error if query fails

    // If the count is greater than 0, the user already exists
    if query.count > 0 {
        return Err(Error::UserAlreadyExists);
    } else {
        println!("User does not exist, proceeding with signup...");
    }

    // Hash the password
    let password_hash = hash_password(&payload.password);

    // Insert the new user into the database
    let query = "INSERT INTO Users (Username, PasswordHash, Email, FirstName, LastName, Address, City, State, Zipcode, Country, PhoneNumber) 
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

    sqlx::query(query)
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