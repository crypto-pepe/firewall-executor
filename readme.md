# Firewall-executor

Implements `POST /api/bans` from [this](https://github.com/crypto-pepe/firewall/wiki/Banned-Targets#ban-target)
api

## ENVs

| Name        | Required | Note                                                                     |
|-------------|----------|--------------------------------------------------------------------------|
| RUST_LOG    | No       | Log level. https://docs.rs/env_logger/0.9.0/env_logger/#enabling-logging |
| CONFIG_PATH | No       | Path to the `yaml` formatted config file                                 |

## Config

```yaml
redis:
  host: '127.0.0.1'
  port: 6379
  timeout_sec: 2
server:
  host: '127.0.0.1'
  port: 8000
```

| Name              | Required | Note                                    |
|-------------------|----------|-----------------------------------------|
| redis.host        | Yes      | Host, where redis is running            |
| redis.port        | Yes      | Port, on which redis is running         |
| redis.timeout_sec | Yes      | Timeout in seconds for redis connection |
| server.host       | Yes      | On what host run this app               |
| server.port       | Yes      | On which port to run this app           |

