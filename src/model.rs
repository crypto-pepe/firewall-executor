use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::http_error::BanTargetConversionError;

const TARGET_TYPE_ORDER: &'static [TargetType] = &[TargetType::IP, TargetType::UserAgent];

#[derive(Debug, Serialize, Deserialize)]
pub enum TargetType {
    #[serde(rename = "ip")]
    IP,
    #[serde(rename = "user-agent")]
    UserAgent,
}

impl Display for TargetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TargetType::IP => "ip",
            TargetType::UserAgent => "user-agent",
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BanTarget {
    #[serde(rename = "type")]
    pub target_type: TargetType,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BanTargetRequest {
    pub target: Vec<BanTarget>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct BanEntity {
    pub analyzer: String,
    pub target: String,
    pub reason: String,
    pub ttl: u32,
}

pub fn target_to_key(bt: &Vec<BanTarget>) -> Result<String, BanTargetConversionError> {
    let mut bt_value = BTreeMap::new();

    for t in bt {
        if bt_value.get(&t.target_type.to_string()).is_none() {
            bt_value.insert(t.target_type.to_string(), t.value.to_string());
        } else {
            return Err(BanTargetConversionError::InvalidTypeCount);
        }
    }

    let target = TARGET_TYPE_ORDER.into_iter().fold(String::new(), |res: String, t| {
        if let Some(v) = bt_value.get(&*t.to_string()) {
            format!("{}{}{}", res, t, v)
        } else { res }
    });

    if target.is_empty() {
        return Err(BanTargetConversionError::InvalidTypeCount);
    }
    Ok(target)
}

impl BanEntity {
    pub fn from(br: BanRequest, analyzer: String) -> Result<Self, BanTargetConversionError> {
        let target = br.target.ok_or(BanTargetConversionError::FieldRequired(
            "target".to_string(),
        ))?;
        let reason = br.reason.ok_or(BanTargetConversionError::FieldRequired(
            "reason".to_string(),
        ))?;
        let ttl = br
            .ttl
            .ok_or(BanTargetConversionError::FieldRequired("ttl".to_string()))?;

        let target = target_to_key(&target)?;
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
