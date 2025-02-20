#![allow(unused)]
use sqlx::SqlitePool;
use std::net::SocketAddr;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower_http::services::ServeDir; // 
use axum::routing::get_service;
use crate::db::create_connection_pool;

pub use self::errors::{Error};
mod hashing;
mod errors;
mod web;
mod strs;
mod adminroutes;
mod testing;
mod db;
mod dbquery;
mod users;
#[tokio::main]
async fn main() {
    let pool = create_connection_pool().await.expect("Failed to connect to DB");
    let routes_all=Router::new()
    .merge(web::routes_login::routes(pool.clone()))
    .merge(web::routes_signup::routes(pool.clone()))
    .merge(web::routes_info::info_routes())
    .merge(web::refresh::routes(pool.clone()))
    .merge(adminroutes::admin_apis::admin_routes_add_products(pool.clone()))
    .merge(adminroutes::admin_apis::admin_routes_add_categorys(pool.clone()))
    .merge(web::routes_home::home_routes(pool.clone()))
    .merge(web::routes_categorys::category_routes(pool.clone()))
    .merge(web::routes_cart::cart_routes(pool.clone()))
    .merge(users::routes_profile::routes_profile_handler(pool.clone()))
    .merge(web::routes_reviews::reviews_routes(pool.clone()));



    let addr=SocketAddr::from(([127,0,0,1],3000)); 
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

}

