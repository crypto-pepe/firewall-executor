use std::fmt::{Debug, Display, Error, Formatter};

use serde::{Deserialize, Serialize};

use crate::http_error::BanTargetConversionError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BanTarget {
    pub ip: Option<String>,
    pub user_agent: Option<String>,
}

const SEPARATOR: &str = "__";

impl Display for BanTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut vv: Vec<String> = Vec::new();

        if self.user_agent.is_none() && self.ip.is_none() {
            return Err(Error);
        }
        if self.ip.is_some() {
            vv.push(format!("ip:{}", &*self.ip.as_ref().unwrap()));
        }
        if self.user_agent.is_some() {
            vv.push(format!(
                "user_agent:{}",
                &*self.user_agent.as_ref().unwrap()
            ));
        }
        f.write_str(&*vv.join(SEPARATOR))
    }
}

impl BanTarget {
    pub fn verify(&self) -> Result<(), BanTargetConversionError> {
        if self.ip.is_none() && self.user_agent.is_none() {
            return Err(BanTargetConversionError::NotEnoughFields);
        }
        Ok(())
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

        target.verify()?;
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

#[derive(Debug, Deserialize, Serialize)]
pub struct UnBanRequest {
    pub target: UnBanEntity,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum UnBanEntity {
    Target(BanTarget),
    Pattern(String),
}

impl Display for UnBanEntity {
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
                ip: Some("1.1.1.1".into()),
                user_agent: None,
            },
            want: "ip:1.1.1.1".into(),
        };

        assert_eq!(tc.input.to_string(), tc.want);
    }

    #[test]
    fn target_to_key_user_agent() {
        let tc = TestCase {
            input: BanTarget {
                ip: None,
                user_agent: Some("abc".into()),
            },
            want: "user_agent:abc".into(),
        };

        assert_eq!(tc.input.to_string(), tc.want);
    }

    #[test]
    fn target_to_key_ip_and_user_agent() {
        let tc = TestCase {
            input: BanTarget {
                ip: Some("1.1.1.1".into()),
                user_agent: Some("abc".into()),
            },
            want: "ip:1.1.1.1__user_agent:abc".into(),
        };

        assert_eq!(tc.input.to_string(), tc.want);
    }
}
