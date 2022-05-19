use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    host: String,
    port: u16,
    client_id: Option<String>,
    password: Option<String>,
    pub namespace: String,
    pub timeout_sec: u64,
}

impl Config {
    pub fn connection_string(&self) -> String {
        let mut auth = self.client_id.clone().unwrap_or_else(|| "".to_string());
        if auth.is_empty() {
            auth = self.password.clone().unwrap_or_else(|| "".to_string());
        } else {
            auth.push(':');
            auth.push_str(&*self.password.clone().unwrap_or_else(|| "".to_string()));
        }

        format!(
            "redis://{}{}:{}",
            if auth.is_empty() { auth } else { auth + "@" },
            self.host,
            self.port
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::redis::Config;

    #[test]
    fn connection_string_host_port() {
        let cfg = Config {
            namespace: "".to_string(),
            host: "localhost".to_string(),
            port: 6379,
            client_id: None,
            password: None,
            timeout_sec: 0,
        };
        assert_eq!(
            cfg.connection_string(),
            "redis://localhost:6379".to_string()
        )
    }

    #[test]
    fn connection_string_host_port_password() {
        let cfg = Config {
            namespace: "".to_string(),
            host: "localhost".to_string(),
            port: 6379,
            client_id: None,
            password: Some("password".to_string()),
            timeout_sec: 0,
        };
        assert_eq!(
            cfg.connection_string(),
            "redis://password@localhost:6379".to_string()
        )
    }

    #[test]
    fn connection_string_host_port_user_password() {
        let cfg = Config {
            namespace: "".to_string(),
            host: "localhost".to_string(),
            port: 6379,
            client_id: Some("user".to_string()),
            password: Some("password".to_string()),
            timeout_sec: 0,
        };
        assert_eq!(
            cfg.connection_string(),
            "redis://user:password@localhost:6379".to_string()
        )
    }
}
