mod parse_validation_errors;
pub mod jwt;

use std::str::FromStr;

use uuid::Uuid;
use validator::ValidationError;

pub fn check_valid_uuid(value: &str) -> Result<(), ValidationError> {
    match Uuid::from_str(value) {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("invalid_uuid")),
    }
}