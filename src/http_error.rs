use std::{collections::BTreeMap, fmt::Display};

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    reason: String,
    details: Option<BTreeMap<String, String>>, // field name -> description,
}

#[derive(Debug)]
pub enum BanTargetConversionError {
    FieldRequired(String),
    InvalidTypeCount,
}

impl Display for BanTargetConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            BanTargetConversionError::FieldRequired(field_name) => f.write_str(field_name),
            BanTargetConversionError::InvalidTypeCount => f.write_str("invalid type count"),
        };
    }
}

impl From<BanTargetConversionError> for HttpResponse {
    fn from(v: BanTargetConversionError) -> Self {
        v.error_response()
    }
}

impl ResponseError for BanTargetConversionError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        let err_resp = match self {
            BanTargetConversionError::FieldRequired(field_name) => {
                let mut details = BTreeMap::new();
                details.insert(field_name.to_string(), "This field is required".to_string());
                ErrorResponse {
                    code: 100,
                    reason: "Provided request does not match the constraints".into(),
                    details: Some(details),
                }
            }
            BanTargetConversionError::InvalidTypeCount => ErrorResponse {
                code: 400,
                reason: self.to_string(),
                details: None,
            },
        };

        HttpResponse::build(StatusCode::BAD_REQUEST).json(err_resp)
    }
}
