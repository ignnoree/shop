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
use crate::strs::get_jwt_identity;

use crate::strs::generate_tokens;


use crate::strs::Claims;

pub fn routes() -> Router {
    Router::new().route("/api/refresh", post(refresh_token))
}


#[derive(Deserialize)]
struct RefreshRequest {
    token: String,
}


async fn refresh_token(Json(payload): Json<RefreshRequest>) -> Response {
    let secret = b"secret";

    let token_data = match decode::<Claims>(
        &payload.token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    ) {
        Ok(data) => data,
        Err(_) => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid token"}))).into_response(),
    };

    let (new_access_token,_) = generate_tokens(&token_data.claims.sub).await;

    (StatusCode::OK, Json(json!({"access_token": new_access_token}))).into_response()
}