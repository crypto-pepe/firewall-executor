use std::sync::Arc;

use bb8::RunError;
use redis::RedisError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BanError {
    #[error(transparent)]
    Error(#[from] Redis),
    #[error("ban target '{0:?}' not found")]
    NotFound(String),
}

#[derive(Error, Debug)]
pub enum Redis {
    #[error("get keys '{1:?}': {0:?}")]
    GetKeys(Arc<RedisError>, String),

    #[error("delete keys '{1:?}': {0:?}")]
    DeleteKeys(Arc<RedisError>, Vec<String>),

    #[error("execute '{1:?}': {0:?}")]
    Pipeline(Arc<RedisError>, Vec<String>),

    #[error("get connection: {0:?}")]
    GetConnection(Arc<RunError<RedisError>>),

    #[error("build pool: {0:?}")]
    BuildPool(Arc<RedisError>),

    #[error("create connection manager: {0:?}")]
    CreateConnManager(Arc<RedisError>),

    #[error("timeout")]
    Timeout,
}
