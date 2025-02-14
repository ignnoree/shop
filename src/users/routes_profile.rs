use axum::{extract::Extension, Router,http::HeaderMap};
use sqlx::SqlitePool;
use axum::handler::Handler;
use axum::response::IntoResponse;
use axum::response::Json;
use serde_json::json;
use serde_json::Value;
use axum::http::StatusCode;
use crate::strs::get_jwt_identity;
use axum::routing::get;
use sqlx::query;
use chrono::{DateTime, Utc};
use chrono::NaiveDateTime;


pub fn routes_profile_handler(pool:SqlitePool)->Router{
    Router::new().route("/api/profile",get(profile_handler.layer(Extension(pool.clone()))))
}


#[derive(serde::Serialize, serde::Deserialize)]
struct ProfileResponse {
    user_id: i64,
    username: String,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
    address: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zipcode: Option<String>,
    country: Option<String>,
    phone_number: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct UserRecord {
    user_id: i64,
    username: String,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
    address: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zipcode: Option<String>,
    country: Option<String>,
    phone_number: Option<String>,

}




async fn profile_handler(
  Extension(pool): Extension<SqlitePool>,
  header: HeaderMap
) -> Result<Json<ProfileResponse>, StatusCode> {
  let identity = get_jwt_identity(header)
      .await
      .map_err(|_| StatusCode::UNAUTHORIZED)?;

  let user_id = identity.get("user_id")
      .ok_or(StatusCode::UNAUTHORIZED)?;

  // Use query_as! with explicit column selection
  let user = sqlx::query_as!(
      UserRecord,
      r#"SELECT UserID as user_id,
       Username as username,
       Email as email,
       FirstName as first_name,
       LastName as last_name,
       Address as address,
       City as city,
       State as state,
       Zipcode as zipcode,
       Country as country,
       PhoneNumber as phone_number
FROM Users 
WHERE UserID = ?"#,
      user_id
  )
  .fetch_one(&pool)
  .await
  .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

  Ok(Json(ProfileResponse {
      user_id: user.user_id,
      username: user.username,
      email: user.email,
      first_name: user.first_name,
      last_name: user.last_name,
      address: user.address,
      city: user.city,
      state: user.state,
      zipcode: user.zipcode,
      country: user.country,
      phone_number: user.phone_number,
  }))
}