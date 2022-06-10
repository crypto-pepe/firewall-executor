use std::fmt::{Debug, Formatter};
use std::{collections::BTreeMap, fmt::Display};

use crate::error::BanError;
use crate::model::BanTargetConversionError;

use actix_web::body::BoxBody;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub(crate) reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<BTreeMap<String, String>>, // field name -> description,
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

pub enum UnBanRequestConversionError {
    EmptyTarget,
    PatternUnsupported,
}

impl Display for UnBanRequestConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            UnBanRequestConversionError::EmptyTarget => "target requires at lease one descriptor",
            UnBanRequestConversionError::PatternUnsupported => "only allowed pattern is \"*\"",
        })
    }
}

impl From<UnBanRequestConversionError> for ErrorResponse {
    fn from(e: UnBanRequestConversionError) -> Self {
        ErrorResponse {
            code: 400,
            reason: e.to_string(),
            details: None,
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

pub enum HeaderError {
    HeaderRequired(String),
    HeaderIsEmpty(String),
    HeaderIsNotString(String),
}

impl From<HeaderError> for ErrorResponse {
    fn from(e: HeaderError) -> Self {
        ErrorResponse {
            code: 400,
            reason: match e {
                HeaderError::HeaderRequired(s) => format!("header {} is required", s),
                HeaderError::HeaderIsEmpty(s) => format!("header {} is empty", s),
                HeaderError::HeaderIsNotString(s) => format!("header {} is not string", s),
            },
            details: None,
        }
    }
}

impl From<BanError> for ErrorResponse {
    fn from(e: BanError) -> Self {
        match e {
            BanError::Error(e) => ErrorResponse {
                code: 500,
                reason: e.to_string(),
                details: None,
            },
            BanError::NotFound(t) => ErrorResponse {
                code: 404,
                reason: format!("target {} not found", t),
                details: None,
            },
        }
    }
}
