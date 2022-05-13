use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::http_error::FieldRequiredError;

#[derive(Debug, Serialize, Deserialize)]
pub enum TargetType {
    #[serde(rename = "ip")]
    IP,
    #[serde(rename = "user-agent")]
    UserAgent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BanTarget {
    #[serde(rename = "type")]
    pub target_type: TargetType,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct BanEntity {
    pub analyzer: String,
    pub target: Vec<BanTarget>,
    pub reason: String,
    pub ttl: u32,
}

impl BanEntity {
    pub fn from(br: BanRequest, analyzer: String) -> Result<Self, FieldRequiredError> {
        let target = br
            .target
            .ok_or(FieldRequiredError::Field("target".to_string()))?;
        let reason = br
            .reason
            .ok_or(FieldRequiredError::Field("reason".to_string()))?;
        let ttl = br.ttl.ok_or(FieldRequiredError::Field("ttl".to_string()))?;

        Ok(BanEntity {
            analyzer,
            ttl,
            target,
            reason,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct BanRequest {
    pub target: Option<Vec<BanTarget>>,
    pub reason: Option<String>,
    pub ttl: Option<u32>,
}
