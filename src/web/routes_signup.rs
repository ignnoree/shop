use axum::{
    extract::Json,
    response::IntoResponse,
    routing::post,
    Router,
};
use rusqlite::{params, Connection, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::errors::{Error}; 
use crate::hashing::{hash_password, verify_password};

pub fn routes()->Router{
    Router::new().route("/api/signup",post(api_signup))
}

#[derive(Debug, Deserialize)]
struct SingupPayload{
    username:String,
    password:String,
    email:String,
    first_name:String,
    last_name:String,
    address:String,
    city :String,
    state:String,
    zipcode:String,
    country :String,
    phonenumber:String
}


#[axum::debug_handler]
async fn api_signup(payload:Json<SingupPayload>)->Result<Json<Value> ,Error>{
    let connection =  Connection::open("database.db")?;
    let query_check = "SELECT COUNT(*) FROM users WHERE username = ?1";
    let mut stmt = connection.prepare(query_check)?;
    let count: i64 = stmt.query_row([&payload.username], |row| row.get(0))?;

    if count > 0 {
        return Err(Error::UserAlreadyExists);
    } else {
    let password_hash =hash_password(&payload.password);
    
    let query = "INSERT INTO Users (Username, PasswordHash, Email, FirstName, LastName, Address, City, State, Zipcode, Country, PhoneNumber) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)";
    connection.execute(query, [&payload.username, &password_hash, &payload.email, &payload.first_name, &payload.last_name, &payload.address, &payload.city, &payload.state, &payload.zipcode, &payload.country, &payload.phonenumber])?;
    println!("succsessfully reached ! ");
    Ok(Json(json!({ "message": "Singup Success" })))}
}

