use std::fmt::{Debug, Display, Error, Formatter};
use std::net::IpAddr;

use num_traits::Zero;
use serde::{Deserialize, Serialize};

use crate::api::routes::BanRequest;

#[derive(Debug, PartialEq)]
pub enum BanTargetConversionError {
    FieldRequired(String),
    EmptyField(String),
    BadTTL,
    NotEnoughFields,
}

impl Display for BanTargetConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BanTargetConversionError::FieldRequired(field_name) => {
                f.write_str(&*format!("field {} required", field_name))
            }
            BanTargetConversionError::EmptyField(field_name) => {
                f.write_str(&*format!("field {} is empty", field_name))
            }
            BanTargetConversionError::NotEnoughFields => {
                f.write_str("at least on field required: 'ip', 'user_agent'")
            }
            BanTargetConversionError::BadTTL => f.write_str("ttl must be above 0"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct BanTarget {
    pub ip: Option<IpAddr>,
    pub user_agent: Option<String>,
}

const SEPARATOR: &str = "__";

impl Display for BanTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ss = vec![];

        if self.user_agent.is_none() && self.ip.is_none() {
            return Err(Error);
        }

        if let Some(ip) = &self.ip {
            ss.push(format!("ip:{}", ip));
        }

        if let Some(user_agent) = &self.user_agent {
            ss.push(format!("user_agent:{}", user_agent));
        }

        f.write_str(&ss.join(SEPARATOR))
    }
}

impl BanTarget {
    pub fn verify(&self) -> Result<(), BanTargetConversionError> {
        if self.ip.is_none() && self.user_agent.is_none() {
            return Err(BanTargetConversionError::NotEnoughFields);
        }
        if let Some(ua) = &self.user_agent {
            if ua.is_empty() {
                return Err(BanTargetConversionError::EmptyField(ua.to_string()));
            }
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
            .ok_or_else(|| BanTargetConversionError::FieldRequired("reason".to_string()))
            .and_then(|r| {
                if r.is_empty() {
                    Err(BanTargetConversionError::EmptyField("reason".to_string()))
                } else {
                    Ok(r)
                }
            })?;
        let ttl = br
            .ttl
            .ok_or_else(|| BanTargetConversionError::FieldRequired("ttl".to_string()))
            .and_then(|t| {
                if t.is_zero() {
                    Err(BanTargetConversionError::BadTTL)
                } else {
                    Ok(t)
                }
            })?;

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
#[serde(untagged)]
pub enum UnBanEntity {
    Target(BanTarget),
    Pattern(String),
}

impl Display for UnBanEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(serde_json::to_string(self).map_err(|_| Error)?.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::BanTarget;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    use std::str::FromStr;

    struct TestCase {
        pub input: BanTarget,
        pub want: String,
    }

    #[test]
    fn target_to_key_ipv4() {
        let tc = TestCase {
            input: BanTarget {
                ip: Some(IpAddr::V4(Ipv4Addr::from_str("1.1.1.1").unwrap())),
                user_agent: None,
            },
            want: "ip:1.1.1.1".into(),
        };

        assert_eq!(tc.input.to_string(), tc.want);
    }

    #[test]
    fn target_to_key_ipv6() {
        let tc = TestCase {
            input: BanTarget {
                ip: Some(IpAddr::V6(
                    Ipv6Addr::from_str("2001:db8:11a3:9d7:1f34:8a2e:7a0:765d").unwrap(),
                )),
                user_agent: None,
            },
            want: "ip:2001:db8:11a3:9d7:1f34:8a2e:7a0:765d".into(),
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
                ip: Some(IpAddr::V4(Ipv4Addr::from_str("1.1.1.1").unwrap())),
                user_agent: Some("abc".into()),
            },
            want: "ip:1.1.1.1__user_agent:abc".into(),
        };

        assert_eq!(tc.input.to_string(), tc.want);
    }
}
