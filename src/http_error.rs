use std::{collections::BTreeMap, fmt::Display};

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    reason: String,
    details: BTreeMap<String, String>, // field name -> description, 
}

#[derive(Debug)]
pub enum FieldRequiredError {
    Field(String)
}

impl Display for FieldRequiredError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let FieldRequiredError::Field(field_name) = self;
        f.write_str(field_name)
    }
}

impl From<FieldRequiredError> for HttpResponse {
    fn from(v: FieldRequiredError) -> Self {
        v.error_response()
    }
}

impl ResponseError for FieldRequiredError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        let FieldRequiredError::Field(field_name) = self;

        let mut details = BTreeMap::new();
        details.insert(field_name.to_string(), "This field is required".to_string());

        HttpResponse::build(StatusCode::BAD_REQUEST).json(
            ErrorResponse {
                code: 100,
                reason: "Provided request does not match the constraints".into(),
                details,
            })
    }
}
