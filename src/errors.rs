use axum::{
    response::{Response, IntoResponse},
    http::StatusCode,
};

use std::fmt::{self, write};


#[derive(Debug)]
pub enum Error {
    LoginFail,
    UserAlreadyExists,
    InternalServerError,
    Unauthorized,
    //UNAUTHORIZED,

}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error:: Unauthorized => write!(f, "Unauthorized"),
    
            Error::UserAlreadyExists => write!(f, "User already exists"),
            Error::InternalServerError => write!(f, "Internal server error"),
            Error::LoginFail=>write!(f,"login failed")

        }
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::LoginFail => {
                (StatusCode::UNAUTHORIZED, "Login Fail").into_response()
            }
            
            Error::UserAlreadyExists => {
                (StatusCode::BAD_REQUEST, "User Already Exists").into_response()
            }
            Error::InternalServerError => {
                (StatusCode::BAD_REQUEST, "Server Error").into_response()
            }
            Error:: Unauthorized => {
                (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
            }
        }

    }
}

