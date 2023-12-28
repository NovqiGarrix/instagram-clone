use base64::{engine::general_purpose, Engine as _};
use jsonwebtoken::*;
use serde::de::DeserializeOwned;
use crate::configuration::JwtSettings;

use crate::error::HttpResponseError;

pub const JWT_AUDIENCE: &str = "instaclone";

pub fn sign<TPayload: serde::Serialize>(
    payload: &TPayload,
    config: &JwtSettings,
) -> Result<String, HttpResponseError> {
    let header = Header::new(Algorithm::RS256);

    let private_key = match general_purpose::STANDARD.decode(&config.private_key) {
        Ok(_key) => _key,
        Err(e) => {
            tracing::error!("Failed to decode Base64 JWT private key: {:?}", e);
            return Err(HttpResponseError::internal_server_error());
        }
    };

    let encoding_key = match EncodingKey::from_rsa_pem(&private_key) {
        Ok(key) => key,
        Err(e) => {
            tracing::error!("Failed to parse JWT private key: {:?}", e);
            return Err(HttpResponseError::internal_server_error());
        }
    };

    match encode(&header, payload, &encoding_key) {
        Ok(token) => Ok(token),
        Err(e) => {
            tracing::error!("Failed to encode JWT token: {:?}", e);
            Err(HttpResponseError::internal_server_error())
        },
    }
}

#[tracing::instrument(name = "Verify JWT Token", skip(config))]
pub fn verify<TPayload: DeserializeOwned>(
    token: &str,
    config: &JwtSettings,
) -> Result<TPayload, HttpResponseError> {
    let decoded_base64_pubkey = match general_purpose::STANDARD.decode(&config.public_key) {
        Ok(pub_key) => pub_key,
        Err(e) => {
            tracing::error!("Failed to decode Base64 JWT public key: {:?}", e);
            return Err(HttpResponseError::internal_server_error());
        }
    };

    let key = match DecodingKey::from_rsa_pem(&decoded_base64_pubkey) {
        Ok(key) => key,
        Err(e) => {
            tracing::error!("Failed to create DecodingKey from JWT public key: {:?}", e);
            return Err(HttpResponseError::internal_server_error());
        }
    };

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[JWT_AUDIENCE]);

    match decode::<TPayload>(token, &key, &validation) {
        Ok(token) => Ok(token.claims),
        Err(e) => {
            tracing::error!("Decoding user's JWT (Kind: {:?}): {:?}", &e.kind(), e);
            // TODO: You can do better
            Err(HttpResponseError::internal_server_error())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{sign, verify};
    use crate::Settings;
    use chrono::{Duration, Utc};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct Payload {
        aud: String,
        name: String,
        exp: i64,
    }

    #[test]
    fn should_sign_jwt() {
        let payload = gen_payload();

        let config = Settings::get_configuration();
        let token = sign(&payload, &config.jwt).unwrap();

        assert!(token.contains("ey"));
    }

    fn gen_payload() -> Payload {
        Payload {
            aud: "sicantikbangsa.com".into(),
            exp: (Utc::now() + Duration::hours(2)).timestamp(),
            name: String::from("SiCantikBangsa"),
        }
    }

    #[test]
    fn should_verify_token() {
        let payload = gen_payload();

        let config = Settings::get_configuration();
        let token = sign(&payload, &config.jwt).unwrap();

        let decoded = verify::<Payload>(&token, &config.jwt).unwrap();

        assert_eq!(decoded.name, payload.name);
    }
}