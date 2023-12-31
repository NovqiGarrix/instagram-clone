use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use crate::error::HttpResponseError;

pub fn hash_password(password: &str) -> crate::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(h) => Ok(h.to_string()),
        Err(e) => {
            tracing::error!("Failed to hash password: {:?}", e);
            Err(HttpResponseError::internal_server_error())
        }
    }
}

pub fn verify_password(password: &str, password_hash: &str) -> crate::Result<bool> {
    match PasswordHash::new(password_hash) {
        Ok(parsed_hash) => {

            let result = Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash);

            Ok(result.is_ok())
        },

        Err(e) => {
            tracing::error!("Failed to parse hashed password. Hashed Password: {}. ErrorDetails: {:?}", password_hash, e);
            Err(
                HttpResponseError::internal_server_error()
            )
        }
    }
}