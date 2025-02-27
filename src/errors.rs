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
    UserIsNotAdmin,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error:: Unauthorized => write!(f, "Unauthorized"),
            Error::UserIsNotAdmin=>write!(f,"User is not a admin or doest exists"),
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
            Error::UserIsNotAdmin=>{
                (StatusCode::BAD_REQUEST, "User is not a admin or doest exists").into_response()
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

