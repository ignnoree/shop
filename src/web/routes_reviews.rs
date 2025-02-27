use std::{convert::identity, result};

use hyper::header;
use sqlx::{pool, sqlite::SqliteColumn, Sqlite};
use axum::{
    extract::{Extension, FromRequest, },
    http::{status, HeaderMap, Response, StatusCode},
    response::{IntoResponse, Json},
    routing::{get,post,Router,delete}
    
};
use sqlx::sqlite::SqlitePool;
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use serde_json::json;
use axum::extract::Path;
use tokio::time::error::Elapsed;
use crate::strs::get_jwt_identity;
use axum::handler::Handler;


pub fn reviews_routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/{id}/reviews", get(get_reviews))
        .route("/api/{id}/reviews", post(send_reviews))
        .route("/api/{id}/reviews", delete(delete_reviews))
        .layer(Extension(pool)) 
}
use sqlx::query;
#[derive(Serialize,Deserialize)]
struct ReviewPost{
    rating:u32,
    comment:String
    
}
#[derive(serde::Serialize)]
struct ReviewGet{
    review_id: i64,
    username:String,
    user_id:i64,
    product_id:i64,
    rating:Option<i64>,
    comment:Option<std::string::String>
}



#[axum::debug_handler]
async fn get_reviews(Extension(pool): Extension<SqlitePool>,Path(id):Path<String>,header:HeaderMap) 
-> Result<Json<serde_json::Value>,StatusCode> {
    let user = get_jwt_identity(header).await.map_err(|_| StatusCode::UNAUTHORIZED)?;
    match user.get("user_id"){

        Some(_)=>{

            let rows=sqlx::query!(r#"SELECT 
            users.UserId AS user_id,
            users.username,
            reviews.reviewId AS review_id,
            reviews.productId AS product_id,
            reviews.rating,
            reviews.comment
        FROM users 
        JOIN reviews ON users.UserId = reviews.UserId where reviews.ProductId = ?"#,id).fetch_all(&pool).await.map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?;

        let reviews:Vec<ReviewGet>=rows.into_iter().map(|row|{
            ReviewGet{
                review_id:row.review_id,
                username:row.Username,
                user_id:row.user_id,
                product_id:row.product_id,
                rating:row.Rating,
                comment:row.Comment
            }
        }).collect();

        Ok(Json(json!({"reviews":reviews})))
    
    }




        None=>Err(StatusCode::UNAUTHORIZED),

    }}


#[axum::debug_handler]
async fn send_reviews(Extension(pool): Extension<SqlitePool>,header:HeaderMap,Path(id):Path<String>,payload:Json<ReviewPost>) 
-> Result<Json<serde_json::Value>,StatusCode> {
    let idenity= get_jwt_identity(header).await.map_err(|_| StatusCode::UNAUTHORIZED)?;
    match idenity.get("user_id") {
        Some(user_id) => {
            let query = "INSERT INTO Reviews(UserID,ProductID,Rating,Comment) VALUES(?,?,?,?)";
            let review = sqlx::query(query)
            .bind(user_id)
            .bind(id)
            .bind(payload.rating)
            .bind(payload.comment.clone())
            .execute(&pool)
            .await
            .unwrap();
            Ok(Json(json!({"message":"Review added successfully"})))

        },
        None => {
            Err(StatusCode::UNAUTHORIZED)
        }
    }}
        


#[derive(serde::Serialize,serde::Deserialize)]
struct Deleterev{
    review_id:i64,
}




#[axum::debug_handler]
async fn delete_reviews(Extension(pool):Extension<SqlitePool> , Path(id):Path<String>,header:HeaderMap,payload:Json<Deleterev>)
->Result<Json<serde_json::Value>,StatusCode>{
    let idenity=get_jwt_identity(header).await.map_err(|_| StatusCode::UNAUTHORIZED)?;

    match idenity.get("user_id"){
        Some(s)=>{
            let query_result=sqlx::query!("DELETE from reviews where reviewid = ? and userid = ?",payload.review_id,s).execute(&pool).await;


            match query_result{
                Ok(res)=>{
                    if res.rows_affected()>0{
                        Ok(Json(json!({ "message": "Review deleted successfully" })))
                    }else {
                        Err(StatusCode::NOT_FOUND)
                    }
                }
                Err(_)=>{
                    Err(StatusCode::NOT_FOUND)
                }

                }

            }
            None =>{ 
                Err(StatusCode::UNAUTHORIZED)
            }

        } 
    }
