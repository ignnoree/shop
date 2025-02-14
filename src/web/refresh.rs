use axum::{
    extract::State,
    http::{response, HeaderMap},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::{Duration, Utc};
use hyper::StatusCode;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value,to_string};
use sqlx::Sqlite;
use crate::strs::get_jwt_identity;

use crate::strs::generate_tokens;
use sqlx::sqlite::SqlitePool;
use axum::Extension;
use axum::handler::Handler;

use crate::strs::Claims;

pub fn routes(pool:SqlitePool) -> Router {
    Router::new().route("/api/refresh", post(refresh_token.layer(Extension(pool.clone()))))
}


#[derive(Deserialize)]
struct RefreshRequest {
    token: String,
}


async fn refresh_token(Extension(pool): Extension<SqlitePool>,Json(payload): Json<RefreshRequest>) -> Response {
    let secret = b"secret";

    let token_data = match decode::<Claims>(
        &payload.token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    ) {
        Ok(data) => data,
        Err(_) => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid token"}))).into_response(),
    };

    let (new_access_token,_) = generate_tokens(&token_data.claims.sub,&pool).await;

    (StatusCode::OK, Json(json!({"access_token": new_access_token}))).into_response()
}