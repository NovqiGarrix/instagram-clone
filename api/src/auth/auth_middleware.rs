use actix_web::{FromRequest, HttpRequest, web::Data};
use actix_web::dev::Payload;
use actix_web::http::StatusCode;
use crate::AppState;
use crate::error::HttpResponseError;
use super::JwtTokenPayload;
use crate::utils::jwt;

impl FromRequest for JwtTokenPayload {
    type Error = HttpResponseError;
    type Future = std::pin::Pin<Box<dyn futures::Future<Output=Result<JwtTokenPayload, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let authorization_header = req.headers().get("Authorization");

        if authorization_header.is_none() {
            return Box::pin(async {
                Err(HttpResponseError::default()
                    .set_code(StatusCode::BAD_REQUEST.as_u16())
                    .set_error_message("Missing Authorization from the header"))
            });
        }

        let authorization_header = authorization_header.unwrap();

        match authorization_header.to_str() {
            Ok(auth_header) => {
                let bearer_token = auth_header.split_once("Bearer ");

                if bearer_token.is_none() {
                    return Box::pin(async {
                        Err(
                            HttpResponseError::default()
                                .set_code(StatusCode::BAD_REQUEST.as_u16())
                                .set_error_message("Missing bearer token in authorization header")
                        )
                    });
                }

                let (_, bearer_token) = bearer_token.unwrap();

                // Validate jwt token
                let settings = req.app_data::<Data<AppState>>().expect("app_data should exist here");
                match jwt::verify::<JwtTokenPayload>(bearer_token, &settings.config.jwt) {
                    Ok(payload) => {
                        Box::pin(async move { Ok(payload) })
                    }

                    Err(e) => {
                        tracing::error!("Failed to verify user's jwt token: {:?}", e);
                        Box::pin(async move { Err(e) })
                    }
                }
            }

            Err(e) => {
                tracing::error!("Failed to parse authorization_header to str: {:?}", e);
                Box::pin(async {
                    Err(
                        HttpResponseError::default()
                            .set_code(StatusCode::BAD_REQUEST.as_u16())
                            .set_error_message("Invalid Authorization header value")
                    )
                })
            }
        }
    }
}