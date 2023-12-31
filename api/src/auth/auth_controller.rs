use actix_web::{post, HttpResponse, web::{Data, Json}, http::StatusCode, HttpRequest, get};
use serde_json::json;
use chrono::{Utc, Duration};
use crate::AppState;
use crate::auth::{GetNewTokenPayload, JwtRefreshTokenPayload, JwtTokenPayload, SignInPayload, SignUpPayload};
use crate::utils::{from_value_to_string, validate_data};
use crate::Result;
use crate::utils::jwt;
use super::auth_service::{get_new_token, sign_in, signup};

#[post("/signup")]
pub async fn signup_handler(ctx: Data<AppState>, payload: Json<SignUpPayload>) -> Result<HttpResponse> {
    let payload = payload.into_inner();

    // Validate request body
    validate_data(&payload)?;

    // Run the signup function
    signup(&ctx.db, payload).await?;

    Ok(HttpResponse::Created().json(
        json!({
            "code": StatusCode::CREATED.as_u16()
        })
    ))
}

#[post("")]
pub async fn sign_in_handler(ctx: Data<AppState>, req: HttpRequest, payload: Json<SignInPayload>) -> Result<HttpResponse> {
    let payload = payload.into_inner();

    validate_data(&payload)?;

    let user_data = sign_in(&ctx.db, payload).await?;

    let token_payload = JwtTokenPayload {
        aud: JwtTokenPayload::get_audience(),
        exp: JwtTokenPayload::get_exp(),
        id: from_value_to_string(&user_data, "id"),
        email: from_value_to_string(&user_data, "email"),
        full_name: from_value_to_string(&user_data, "fullName"),
        username: from_value_to_string(&user_data, "username"),
        picture_url: from_value_to_string(&user_data, "pictureUrl"),
    };

    let token = jwt::sign(
        &token_payload,
        &ctx.config.jwt,
    )?;

    let uip = req.connection_info();
    let uip = uip.peer_addr().unwrap();

    let refresh_token_payload = JwtRefreshTokenPayload {
        aud: jwt::JWT_AUDIENCE.to_string(),
        exp: (Utc::now() + Duration::days(3 * 365)).timestamp(),
        username: from_value_to_string(&user_data, "username"),
        used_for: "refreshToken".to_string(),
        user_ip: uip.to_string(),
    };

    let refresh_token = jwt::sign(
        &refresh_token_payload,
        &ctx.config.jwt,
    )?;

    Ok(HttpResponse::Ok().json(
        json!({
            "code": StatusCode::OK.as_u16(),
            "data": user_data,
            "token": token,
            "refreshToken": refresh_token
        })
    ))
}

#[post("/token")]
pub async fn get_new_token_handler(ctx: Data<AppState>, req: HttpRequest, payload: Json<GetNewTokenPayload>) -> Result<HttpResponse> {
    let payload = payload.into_inner();

    validate_data(&payload)?;

    let connection_info = req.connection_info().clone();
    let uip = connection_info.peer_addr().unwrap();

    let new_token = get_new_token(&ctx.db, &ctx.config.jwt, payload, uip).await?;

    Ok(
        HttpResponse::Ok()
            .json(json!({
                "code": StatusCode::OK.as_u16(),
                "token": new_token
            }))
    )
}

#[get("/me")]
pub async fn get_me_handler(jwt_payload: JwtTokenPayload) -> HttpResponse {
    HttpResponse::Ok()
        .json(json!({
           "code": StatusCode::OK.as_u16(),
            "data": jwt_payload
        }))
}