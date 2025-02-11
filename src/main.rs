#![allow(unused)]
use std::net::SocketAddr;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower_http::services::ServeDir; // ✅ Import ServeDir for serving static files
use axum::routing::get_service;

pub use self::errors::{Error};
mod hashing;
mod errors;
mod web;
mod strs;
mod adminroutes;
#[tokio::main]
async fn main() {
    let routes_all=Router::new()
    .merge(web::routes_login::routes())
    .merge(web::routes_signup::routes())
    .merge(web::routes_info::info_routes())
    .merge(web::refresh::routes())
    .merge(adminroutes::admin_apis::admin_routes_add_products())
    .merge(adminroutes::admin_apis::admin_routes_add_categorys())
    .merge(web::routes_home::home_routes())
    .merge(web::routes_categorys::category_routes())
    .merge(web::routes_reviews::routes_reviews());



    let addr=SocketAddr::from(([127,0,0,1],3000)); 
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

}

