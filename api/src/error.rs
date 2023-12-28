use actix_web::http::{header, StatusCode};
use actix_web::HttpResponse;
use sea_orm::DbErr;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ResponseError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>
}

impl ResponseError {
    pub fn common_error(error: &str) -> Self {
        Self {
            error: None,
            message: Some(error.to_owned()),
            field: None
        }
    }

    pub fn for_validation(field: &str, message: &str) -> Self {
        Self {
            error: None,
            message: Some(message.to_owned()),
            field: Some(field.to_owned())
        }
    }
}

#[derive(Serialize, Default, Debug)]
pub struct HttpResponseError {
    pub code: Option<u16>,
    pub errors: Vec<ResponseError>,
}

impl From<DbErr> for HttpResponseError {
    fn from(db_err: DbErr) -> Self {
        tracing::error!("DbErr: {:?}", db_err);
        HttpResponseError::internal_server_error()
    }
}

impl std::fmt::Display for HttpResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.errors)
    }
}

impl From<HttpResponseError> for HttpResponse {
    fn from(service_exception: HttpResponseError) -> Self {
        HttpResponse::build(from_code_to_status_code(service_exception.code.unwrap()))
            .insert_header(header::ContentType::json())
            .json(service_exception)
    }
}

pub fn from_code_to_status_code(code: u16) -> StatusCode {
    match code {
        400 => StatusCode::BAD_REQUEST,
        401 => StatusCode::UNAUTHORIZED,
        403 => StatusCode::FORBIDDEN,
        404 => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

impl HttpResponseError {
    pub fn internal_server_error() -> Self {
        Self {
            code: Some(500),
            errors: vec![
                ResponseError::common_error("Internal Server Error")
            ]
        }
    }

    pub fn set_code(&mut self, code: u16) -> &Self {
        self.code = Some(code);
        self
    }

    pub fn set_error_message(&mut self, error: &'static str) -> &Self {
        let err = ResponseError::common_error(error);
        self.errors = vec![err];
        self
    }

    pub fn set_validation_error(&mut self, field: &str, message: &str) -> &Self {
        let err = ResponseError::for_validation(field, message);

        self.errors.push(err);
        self
    }

}