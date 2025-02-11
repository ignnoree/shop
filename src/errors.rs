use axum::{
    response::{Response, IntoResponse},
    http::StatusCode,
};
use rusqlite::Error as RusqliteError;
use std::fmt::{self, write};


#[derive(Debug)]
pub enum Error {
    LoginFail,
    SqliteError(rusqlite::Error),
    UserAlreadyExists,
    InternalServerError,
    UNAUTHORIZED,

}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UNAUTHORIZED => write!(f, "Unauthorized"),
            Error::SqliteError(err) => write!(f, "Database error: {}", err),
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
            Error::SqliteError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
            Error::UserAlreadyExists => {
                (StatusCode::BAD_REQUEST, "User Already Exists").into_response()
            }
            Error::InternalServerError => {
                (StatusCode::BAD_REQUEST, "Server Error").into_response()
            }
            Error::UNAUTHORIZED => {
                (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
            }
        }

    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Error::SqliteError(err)
    }
}