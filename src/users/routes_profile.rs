use axum::{extract::Extension, Router,http::HeaderMap};
use jsonwebtoken::Header;
use sqlx::SqlitePool;
use axum::handler::Handler;
use axum::response::IntoResponse;
use axum::response::Json;
use serde_json::json;
use serde_json::Value;
use axum::http::StatusCode;
use crate::strs::get_jwt_identity;
use axum::routing::{get,post};
use sqlx::query;
use chrono::{DateTime, Utc};
use chrono::NaiveDateTime;


pub fn routes_profile_handler(pool:SqlitePool)->Router{
    Router::new()
    .route("/api/profile",post(update_profile))
    .route("/api/profile",get(profile_handler))
    .layer(Extension(pool.clone()))

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
use std::any::TypeId;
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
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct UpdateProfile {
    username: Option<String>,              
    email: Option<String>,         
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



#[axum::debug_handler]
async fn update_profile(header:HeaderMap,
    Extension(pool): Extension<SqlitePool>,
    payload:Json<UpdateProfile>)-> Result <Json<serde_json::Value> , StatusCode>{

        let idenity = get_jwt_identity(header).
        await.map_err(|_| StatusCode::UNAUTHORIZED)?;

        let user_id = idenity.get("user_id").ok_or(StatusCode::UNAUTHORIZED)?;
        let user_id_int=user_id.as_i64();



        let username = payload.username.clone();
        let email= payload.email.clone();
        let first_name=payload.first_name.clone();
        let last_name=payload.last_name.clone();
        let address=payload.address.clone();
        let city=payload.city.clone();
        let state=payload.state.clone();
        let zipcode= payload.zipcode.clone();
        let country= payload.country.clone();
        let phone_number=payload.phone_number.clone();
        
        


        sqlx::query!(
            "UPDATE users SET 
        email = COALESCE($1, email), 
        firstname = COALESCE($2, firstname), 
        lastname = COALESCE($3, lastname), 
        address = COALESCE($4, address), 
        city = COALESCE($5, city), 
        state = COALESCE($6, state), 
        zipcode = COALESCE($7, zipcode), 
        country = COALESCE($8, country), 
        phonenumber = COALESCE($9, phonenumber)
     WHERE userid = $10",
            email,
            first_name,
            last_name,
            address,
            city,
            state,
            zipcode,
            country,
            phone_number,
            user_id_int
        ).execute(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
            
                
        Ok(Json(json!({"msg": "profile updated"})))
            
                
                
            }
        
    