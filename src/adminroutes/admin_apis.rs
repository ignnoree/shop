use std::{path::PathBuf, result};

use axum::{
    extract::{Query, State}, handler::Handler, http::{response, HeaderMap}, response::{IntoResponse, Response}, 
    routing::{get, post,delete}, Extension, Json, Router
};
use rsa::pkcs8::der::asn1::Int;
use crate::strs::get_jwt_identity;

use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use hyper::StatusCode;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde_json::{json, Value,to_string};
use sqlx::{sqlite::SqlitePool, Sqlite};

use crate::strs::Claims;

pub fn admin_routes(pool:SqlitePool) -> Router {
    Router::new()
    .route("/api/admin/Product",  delete(add_category))
    .route("/api/admin/Product", post(add_products))
    .route("/api/admin/admins", post(add_admin))
    .route("/api/admin/admins", delete(delete_admin))
    .route("/api/admin/user",  delete(delete_user))
    .route("/api/admin/category",  post(add_category))
    .route("/api/admin/category",  delete(delete_category))
    .layer(Extension(pool.clone()))
}


async fn check_admin(header:HeaderMap)->Result<String,Error>{
    let idenity=get_jwt_identity(header)
    .await
    .map_err(|_| Error::Unauthorized)?;

    let role= idenity.get("role")
    .and_then(|v|v.as_str())
    .ok_or(Error::Unauthorized)?;

    if role == "admin"{
        return Ok(role.to_string())
    }

    return Err(Error::Unauthorized);

}

use crate::strs::Product;
use crate::errors::{Error};


struct DeleteProduct{
    product_id:i32
}

struct DeleteAdmin{

    username:str
}

async fn delete_admin(header:HeaderMap,Extension(pool):Extension<SqlitePool>,payload:Json<Username>)
->Result<Json<Value>,Error>{
    let idenity=check_admin(header).await?;
    if idenity != "admin"{
        return Err(Error::Unauthorized);
    }
    let username = &payload.username;

    let check_role =sqlx::query!("select role from admins where user_id = (select userid from users where username = ?)",username)
    .fetch_one(&pool)
    .await
    .map_err(|_| Error::InternalServerError)?;

    let role = check_role.role;
    match role{
        Some(role)=>{

            if role == "admin"{
                return Err(Error::Unauthorized);
            }
            sqlx::query!("delete from admins where user_id = (select userid from users where username = ?)",username)
            .execute(&pool)
            .await
            .map_err(|_| Error::InternalServerError)?;
            return Ok(Json(json!({"msg":"admin removed"})))   
        }
        None=>{
            Err(Error::UserIsNotAdmin)
        }

    }
    }




async fn delete_product(header:HeaderMap,Extension(pool):Extension<SqlitePool>,payload: Json<DeleteProduct>)
->Result<Json<Value>, Error>{
    let idenity=get_jwt_identity(header)
    .await
    .map_err(|_| Error::Unauthorized)?;

    let role= idenity.get("role")
    .and_then(|v|v.as_str())
    .ok_or(Error::Unauthorized)?;


    if role !="admin"{
        return Err(Error::Unauthorized)
    }
    let product_id = payload.product_id.clone();

    sqlx::query!("delete from reviews where productid = ? ",product_id)
    .execute(&pool)
    .await.map_err(|_|Error::InternalServerError);

    sqlx::query!("delete from products where productid = ? ",product_id)
    .execute(&pool)
    .await.map_err(|_|Error::InternalServerError);



    Ok(Json(json!({"MSG":"product deleted "})))

}


#[axum::debug_handler]
async fn add_products(Extension(pool):Extension<SqlitePool>,headers: HeaderMap,payload: Json<Product>) 
-> Result<Json<Value>, Error> {
    let identity = get_jwt_identity(headers)
        .await
        .map_err(|_| Error:: Unauthorized)?;

    let role = identity
    .get("role") //
    .and_then(|v| v.as_str())
    .ok_or(Error:: Unauthorized)?; 

    if role != "admin"{
        return Err(Error:: Unauthorized);
        
    }  
    
    let product_name=payload.product_name.clone();
    let product_description=payload.product_description.clone();
    let product_price=payload.product_price.clone();
    let product_quantity=payload.product_quantity.clone();
    let product_category_id=payload.product_category_id.clone();
    



    
    let execute=sqlx::query!("INSERT INTO Products (Name, Description, Price, StockQuantity, CategoryID) VALUES (?, ?, ?, ?, ?)",product_name,product_description,product_price,product_quantity,product_category_id) 
    .execute(&pool)
    .await;
    Ok(Json(json!({"message": "executed"})))
    
            
    } 






#[derive(Debug,Deserialize)]
struct Category{
    category_name:String,
    parent_category_id:Option<String>

}

#[derive(Debug,Deserialize)]
struct CategoryID{
    category_id:u32,
}


#[axum::debug_handler]
async fn add_category(Extension(pool):Extension<SqlitePool>,headers: HeaderMap,payload: Json<Category>)
 -> Result<Json<Value>, Error> {
    let identity = get_jwt_identity(headers)
        .await
        .map_err(|_| Error:: Unauthorized)?;

    let role = identity
    .get("role") //
    .and_then(|v| v.as_str())
    .ok_or(Error:: Unauthorized)?; 

    if role != "admin"{
        
    }  
    
    let categoryname= payload.category_name.clone();
    let parentcategoryid=payload.parent_category_id.clone();
    
    let execute=sqlx::query!
    ("INSERT INTO Categories (CategoryName, ParentCategoryID) VALUES (?, ?)",categoryname,parentcategoryid).execute(&pool).await; 
    {
    Ok(Json(json!({"message": "Category Added"})))

        }
    } 



#[derive(Debug,Deserialize)]
struct Username{
    username:String
}


async fn delete_user(header:HeaderMap,Extension(pool):Extension<SqlitePool>, payload: Json<Username>)
->Result<Json<Value>,Error>{
    let idenity=get_jwt_identity(header)
    .await.map_err(|_| Error::Unauthorized)?;

    let role =idenity.get("role").and_then(|v| v.as_str()).ok_or(Error:: Unauthorized)?;

    if role != "admin"{
        return Err(Error:: Unauthorized);
    }
    let user_id=&payload.username;
    let res=sqlx::query!("delete from users where userid = ? ",user_id).execute(&pool)
    .await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR );

    sqlx::query!("delete from reviews where userid = ?",user_id).execute(&pool).await.
    map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    Ok(Json(json!({"msg":"user deleted"})))
    
}




async fn add_admin(header:HeaderMap,Extension(pool):Extension<SqlitePool>,payload: Json<Username>)
->Result<Json<Value>,Error>{
    let res= check_admin(header).await?;

    let new_admin_username=&payload.username;


    let res = sqlx::query!(
        "INSERT INTO admins (user_id) 
         SELECT userid FROM users WHERE username = ?",
        new_admin_username
    )
    .execute(&pool)
    .await
    .map_err(|_| Error::InternalServerError)?;
    
    Ok(Json(json!({"msg": "admin added"})))
    
}

#[axum::debug_handler]
async fn delete_category(header:HeaderMap,Extension(pool):Extension<SqlitePool>,payload:Json<CategoryID>)
->Result<Json<Value>,Error>{
    let check_admin=check_admin(header).await?;
    let category_id=&payload.category_id;

    sqlx::query!("delete from categories where parentcategoryid = ?",category_id).execute(&pool)
    .await.map_err(|_| Error::InternalServerError)?;
    
    sqlx::query!("delete from categories where categoryid = ?",category_id).execute(&pool)
    .await.map_err(|_| Error::InternalServerError)?;


    Ok(Json(json!({"msg":"category deleted ! "})))
}


