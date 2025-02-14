
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
use serde_json::{json, StreamDeserializer, Value};
use crate::errors::{Error};  
use rand::rngs::OsRng;
use tokio::task;
use crate::strs::LoginResponse;
use crate::hashing::verify_password;
use sqlx::SqlitePool;
use axum::handler::Handler;
pub fn routes(pool:SqlitePool)->Router{
    Router::new().route("/api/login",post(api_login.layer(Extension(pool.clone()))))
}


use axum::Extension;
use crate::strs::LoginPayload;
use crate::strs::Claims;
use crate::strs::generate_tokens;
use crate::hashing::hash_password;
use bcrypt::verify;



pub async fn api_login(
    Extension(pool): Extension<SqlitePool>,
    payload: Json<LoginPayload>,
) -> Result<Json<Value>, Error> {
    let username = payload.username.clone();
    let password = payload.password.clone();

    let row = sqlx::query!("SELECT PasswordHash, UserID FROM Users WHERE Username = ?", username)
        .fetch_optional(&pool)
        .await
        .map_err(|_| Error::InternalServerError)?;

    if let Some(row) = row {
        let is_valid = verify_password(&password, &row.PasswordHash);
        if is_valid {
            let (access_token, refresh_token) = generate_tokens(&username,&pool).await;
            let response = json!( {
                "access_token": access_token,
                "refresh_token": refresh_token,
            });
            return Ok(Json(response));
        }
    }

    Err(Error::LoginFail)
}
