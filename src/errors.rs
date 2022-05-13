use bb8::RunError;
use redis::RedisError;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BanError {
    #[error(transparent)]
    Error(#[from] Redis),
}
#[derive(Error, Debug)]
pub enum CheckBanError {
    #[error(transparent)]
    Error(#[from] Redis),
}

#[derive(Error, Debug)]
pub enum Redis {
    #[error("execute '{1}': {0:?}")]
    CMD(Arc<RedisError>, String),

    #[error("get connection: {0:?}")]
    GetConnection(Arc<RunError<RedisError>>),

    #[error("build pool: {0:?}")]
    BuildPool(Arc<RedisError>),

    #[error("create connection manager: {0:?}")]
    CreateConnManager(Arc<RedisError>),

    #[error("timeout")]
    Timeout,
}
