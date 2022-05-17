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

| Name              | Required | Note                               |
|-------------------|----------|------------------------------------|
| redis.host        | Yes      | Redis service host                 |
| redis.port        | Yes      | Redis service port                 |
| redis.timeout_sec | Yes      | Redis connection timeout (seconds) |
| server.host       | Yes      | Firewall service host              |
| server.port       | Yes      | Firewall service port              |

