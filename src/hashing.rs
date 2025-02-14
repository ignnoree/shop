use bcrypt::{hash, DEFAULT_COST};
use crate::strs::LoginPayload;

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).expect("Failed to hash password")
}


use bcrypt::verify;

pub fn verify_password(password: &str, hashed: &str) -> bool {
    verify(password, hashed).unwrap_or(false)
}