use std::borrow::Cow;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use regex::Regex;
use crate::utils::jwt;

pub mod auth_service;
pub mod auth_controller;
pub mod auth_routes;
mod auth_middleware;

fn no_symbols(username: &str) -> Result<(), ValidationError> {
    let symbols_regex = Regex::new(r"[-!$%^&@*()+|~=`{}\[\]:;<>?,/]").expect("should parse the symbols regex");

    if symbols_regex.is_match(username) {
        let mut val_error = ValidationError::new("invalid_username");
        val_error.message = Some(Cow::from("Username should not contains non-allowed character"));
        return Err(val_error);
    }

    Ok(())
}



// ---- AUTH STRUCTS ----

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtTokenPayload {
    pub aud: String,
    pub exp: i64,
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub username: String,
    pub picture_url: String,
}

impl JwtTokenPayload {
    pub fn get_audience() -> String {
        jwt::JWT_AUDIENCE.to_string()
    }

    pub fn get_exp() -> i64 {
        (Utc::now() + Duration::minutes(60)).timestamp()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtRefreshTokenPayload {
    pub aud: String,
    pub exp: i64,
    pub username: String,
    pub used_for: String,
    pub user_ip: String,
}

// ---- END OF AUTH STRUCTS ----



// ---- REQUEST PAYLOAD ----

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SignUpPayload {
    #[validate(email(message = "Please provide proper email"), required(message = "This field is required"))]
    pub email: Option<String>,

    #[serde(rename = "fullName")]
    #[validate(length(min = 4, message = "Full name must be at least 4 characters"), required(message = "This field is required"))]
    pub full_name: Option<String>,

    #[validate(
    length(
    min = 4,
    message = "Username must be at least 4 characters"
    ),
    required(
    message = "This field is required"
    ),
    custom = "no_symbols"
    )]
    pub username: Option<String>,

    pub bio: Option<String>,

    #[validate(length(min = 3, message = "Password must be at least 3 characters"), required(message = "This field is required"))]
    pub password: Option<String>,

    #[serde(rename = "confirmPassword")]
    #[validate(required(message = "This field is required"), must_match(other = "password", message = "Password confirmation must match password"))]
    _confirm_password: Option<String>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct SignInPayload {
    #[serde(rename = "emailUsername")]
    #[validate(required(message = "This field is required"))]
    pub email_username: Option<String>,

    #[validate(required(message = "This field is required"))]
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct GetNewTokenPayload {
    #[serde(rename = "refreshToken")]
    #[validate(required(message = "Please provide your refresh token to get a new access token"))]
    pub refresh_token: Option<String>,
}

// ---- END OF REQUEST PAYLOAD ----