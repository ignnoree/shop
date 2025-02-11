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
use crate::strs::Claims;
use crate::errors::Error;
pub fn info_routes() -> Router {
    Router::new().route("/api/info", post(get_info))
}

async fn get_info(headers: HeaderMap) -> Result<Json<serde_json::Value>, StatusCode> {
    match get_jwt_identity(headers).await{
        Ok(response)=>{
            println!("{:?}", response);
            Ok(response)
        
        }
        Err(status) => Err(StatusCode::UNAUTHORIZED),
    }
    
}