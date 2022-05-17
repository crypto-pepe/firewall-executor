use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::http_error::BanTargetConversionError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BanTarget {
    pub ip: String,
    pub user_agent: Option<String>,
}

impl Display for BanTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.user_agent.is_none() {
            f.write_str(&*self.ip)
        } else {
            f.write_str(&*format!(
                "{}_{}",
                &*self.ip,
                self.user_agent.as_ref().unwrap()
            ))
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct BanEntity {
    pub analyzer: String,
    pub target: String,
    pub reason: String,
    pub ttl: u32,
}

impl BanEntity {
    pub fn new(br: BanRequest, analyzer: String) -> Result<Self, BanTargetConversionError> {
        let target = br
            .target
            .ok_or_else(|| BanTargetConversionError::FieldRequired("target".to_string()))?;
        let reason = br
            .reason
            .ok_or_else(|| BanTargetConversionError::FieldRequired("reason".to_string()))?;
        let ttl = br
            .ttl
            .ok_or_else(|| BanTargetConversionError::FieldRequired("ttl".to_string()))?;

        let target = target.to_string();
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
    pub target: Option<BanTarget>,
    pub reason: Option<String>,
    pub ttl: Option<u32>,
}

impl Display for BanRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::BanTarget;

    struct TestCase {
        pub input: BanTarget,
        pub want: String,
    }

    #[test]
    fn target_to_key_ip() {
        let tc = TestCase {
            input: BanTarget {
                ip: "1.1.1.1".into(),
                user_agent: None,
            },
            want: "1.1.1.1".into(),
        };

        assert_eq!(tc.input.to_string(), tc.want);
    }

    #[test]
    fn target_to_key_ip_and_user_agent() {
        let tc = TestCase {
            input: BanTarget {
                ip: "1.1.1.1".into(),
                user_agent: Some("abc".into()),
            },
            want: "1.1.1.1_abc".into(),
        };

        assert_eq!(tc.input.to_string(), tc.want);
    }
}
