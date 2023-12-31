pub mod jwt;
pub mod password;

use std::str::FromStr;
use actix_web::http::StatusCode;

use uuid::Uuid;
use validator::ValidationError;
use crate::error::{HttpResponseError, ResponseError};

pub fn check_valid_uuid(value: &str) -> Result<(), ValidationError> {
    match Uuid::from_str(value) {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("invalid_uuid")),
    }
}

pub fn parse_validation_errors(validation_errors: validator::ValidationErrors) -> Vec<ResponseError> {
    let field_errors: Vec<ResponseError> = validation_errors
        .field_errors()
        .into_iter()
        .map(|(field_key, errors)| {
            let messages = errors
                .iter()
                .map(|err| match &err.message {
                    Some(hello) => hello.to_string(),
                    _ => String::from("Bad value"),
                })
                .collect::<Vec<String>>();

            let default_value = String::from("Bad value");
            let messages = messages.first().unwrap_or(&default_value);

            ResponseError::for_validation(field_key, messages)
        })
        .collect();

    field_errors
}

pub fn validate_data<T>(data: &T) -> crate::Result<()>
    where
        T: validator::Validate,
{
    match data.validate() {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Client sends invalid data: {:?}", e);

            let formatted_validation_errors = parse_validation_errors(e);

            let http_response_error = HttpResponseError::default()
                .set_code(StatusCode::BAD_REQUEST.as_u16())
                .set_validation_errors(formatted_validation_errors);

            Err(http_response_error)
        }
    }
}

// Caution when using this function
// No error handler here...
// Make sure you know what you are doing!
pub fn from_value_to_string(data: &serde_json::Value, field: &str) -> String {
    data[field].as_str().unwrap().to_string()
}