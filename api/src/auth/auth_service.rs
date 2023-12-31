use std::default::Default;
use actix_web::http::StatusCode;
use crate::utils::password;
use crate::utils::jwt;
use sea_orm::{Set, ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::auth::{GetNewTokenPayload, JwtRefreshTokenPayload, JwtTokenPayload, SignInPayload, SignUpPayload};
use crate::Result;
use serde_json::{json, Value};
use uuid::Uuid;
use entity::users::{Entity, Column, ActiveModel};
use crate::configuration::JwtSettings;
use crate::error::HttpResponseError;

const DEFAULT_PROFILE_PICTURE: &str = "https://bit.ly/3REd7XG";
const EMAIL_USERNAME_PASSWORD_WRONG_ERROR: &str = "Your email/username and password are wrong!";

pub async fn signup(db: &DatabaseConnection, data: SignUpPayload) -> Result<()> {
    let email = data.email.unwrap();
    let full_name = data.full_name.unwrap();
    let username = data.username.unwrap();
    let bio = data.bio;
    let password = data.password.unwrap();

    // Get the user with the email
    let user = Entity::find()
        .filter(Column::Email.eq(&email))
        .one(db)
        .await?;

    // Check if the user already exists
    if user.is_some() {
        return Err(
            HttpResponseError::default()
                .set_code(StatusCode::BAD_REQUEST.as_u16())
                .set_error_message("This email is already taken")
        );
    }

    // If not exists, create a new user
    ActiveModel {
        id: Set(Vec::from(Uuid::new_v4())),
        email: Set(email),
        name: Set(full_name),
        username: Set(username),
        bio: Set(bio),
        picture_url: Set(DEFAULT_PROFILE_PICTURE.to_owned()),
        password: Set(password::hash_password(&password)?),
        ..Default::default()
    }.insert(db).await?;

    Ok(())
}

pub async fn sign_in(db: &DatabaseConnection, data: SignInPayload) -> Result<Value> {
    let email_username = data.email_username.unwrap();
    let password = data.password.unwrap();

    let using_email = email_username.contains('@');

    let user = if using_email {
        Entity::find()
            .filter(
                Column::Email.eq(email_username)
            )
            .one(db)
            .await?
    } else {
        Entity::find()
            .filter(
                Column::Username.eq(email_username)
            )
            .one(db)
            .await?
    };

    if user.is_none() {
        return Err(
            HttpResponseError::default()
                .set_code(StatusCode::BAD_REQUEST.as_u16())
                .set_error_message(EMAIL_USERNAME_PASSWORD_WRONG_ERROR)
        );
    }

    let user = user.unwrap();

    match password::verify_password(&password, &user.password)? {
        // Password verified
        true => {
            let user_id = Uuid::from_slice(&user.id).unwrap();

            Ok(
                json!({
                    "id": user_id,
                    "email": user.email,
                    "fullName": user.name,
                    "username": user.username,
                    "pictureUrl": user.picture_url
                })
            )
        }

        // Else
        false => {
            Err(
                HttpResponseError::default()
                    .set_code(StatusCode::BAD_REQUEST.as_u16())
                    .set_error_message(EMAIL_USERNAME_PASSWORD_WRONG_ERROR)
            )
        }
    }
}

pub async fn get_new_token(db: &DatabaseConnection, jwt_config: &JwtSettings, data: GetNewTokenPayload, user_ip: &str) -> Result<String> {

    // Verify the refresh token first
    let refresh_token_payload: JwtRefreshTokenPayload = jwt::verify(data.refresh_token.as_ref().unwrap(), jwt_config)?;

    // If the user ip from the token is not match with the client user ip
    // then the client is using other user's refresh token
    if !refresh_token_payload.user_ip.eq(user_ip) {
        return Err(
            HttpResponseError::default()
                .set_code(StatusCode::BAD_REQUEST.as_u16())
                .set_error_message("Invalid refresh token")
        );
    }

    if !refresh_token_payload.used_for.eq("refreshToken") {
        return Err(
            HttpResponseError::default()
                .set_code(StatusCode::BAD_REQUEST.as_u16())
                .set_error_message("Invalid refresh token. Please re-login")
        );
    }

    // Get the user from db by using the username
    let user = Entity::find()
        .filter(
            Column::Username.eq(refresh_token_payload.username)
        )
        .one(db)
        .await?;

    // Check if the user is on db or not
    if user.is_none() {
        return Err(
            HttpResponseError::default()
                .set_code(StatusCode::BAD_REQUEST.as_u16())
                .set_error_message("Somehow your account is missing from our database")
        )
    }

    // Generate a new token
    let user = user.unwrap();
    let jwt_token_payload = JwtTokenPayload {
        aud: JwtTokenPayload::get_audience(),
        exp: JwtTokenPayload::get_exp(),
        id: Uuid::from_slice(&user.id).unwrap().to_string(),
        email: user.email,
        full_name: user.name,
        username: user.username,
        picture_url: user.picture_url
    };

    let new_token = jwt::sign(&jwt_token_payload, jwt_config)?;

    Ok(new_token)
}