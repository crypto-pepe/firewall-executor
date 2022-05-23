use std::{collections::BTreeMap, fmt::Display};
use std::fmt::{Debug, Formatter};

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use actix_web::body::BoxBody;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub(crate) reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<BTreeMap<String, String>>, // field name -> description,
}

#[derive(Debug, PartialEq)]
pub enum BanTargetConversionError {
    FieldRequired(String),
    NotEnoughFields,
}

impl Display for BanTargetConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BanTargetConversionError::FieldRequired(field_name) => f.write_str(field_name),
            BanTargetConversionError::NotEnoughFields => {
                f.write_str("at least on field required: 'ip', 'user_agent'")
            }
        }
    }
}

impl From<BanTargetConversionError> for ErrorResponse {
    fn from(btce: BanTargetConversionError) -> Self {
        match btce {
            BanTargetConversionError::FieldRequired(field_name) => {
                let mut details = BTreeMap::new();
                details.insert(field_name, "This field is required".to_string());
                ErrorResponse {
                    code: 400,
                    reason: "Provided request does not match the constraints".into(),
                    details: Some(details),
                }
            }
            BanTargetConversionError::NotEnoughFields => ErrorResponse {
                code: 400,
                reason: btce.to_string(),
                details: None,
            },
        }
    }
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(self)
    }
}
