
use core::error;
use std::{clone, sync::Arc};

use axum::{
    extract::{FromRef, State},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use axum_jwt_auth::{JwtDecoderState, LocalDecoder};
use chrono::{Duration, Utc};
use hyper::StatusCode;
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection, Result};
use serde_json::{json, StreamDeserializer, Value};
use crate::errors::{Error};  
use rand::rngs::OsRng;
use tokio::task;
use crate::strs::LoginResponse;
use crate::hashing::verify_password;

pub fn routes()->Router{
    Router::new().route("/api/login",post(api_login))
}

use crate::strs::LoginPayload;
use crate::strs::Claims;
use crate::strs::generate_tokens;
use crate::hashing::hash_password;
use bcrypt::verify;
#[axum::debug_handler]
async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>, Error> {
    let username = payload.username.clone();
    let password = payload.password.clone();
    let username_2 = username.clone();

    
    let (is_valid, user_id): (bool, i64) = task::spawn_blocking(move || -> Result<(bool, i64), rusqlite::Error> {
        let connection = Connection::open("database.db")?;
        let query = "SELECT PasswordHash, UserID FROM Users WHERE Username = ?1";

        let mut stmt = connection.prepare(query)?;
        let mut rows = stmt.query(params![username])?;

        
        if let Some(row) = rows.next()? {
            let hashed_password: String = row.get(0)?;
            let user_id: i64 = row.get(1)?;

            
            let is_valid = verify_password(&password, &hashed_password);

            Ok((is_valid, user_id))
        } else {
            Ok((false, 0)) 
        }
    })
    .await
    .map_err(|_| Error::InternalServerError)??;

   
    if is_valid {
        let (access_token, refresh_token) = generate_tokens(&username_2).await;
        let response = json!( {
            "access_token": access_token,
            "refresh_token": refresh_token,
        });

        Ok(Json(response))
    } else {
        Err(Error::LoginFail)
    }
}



