use crate::error::ResponseError;

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