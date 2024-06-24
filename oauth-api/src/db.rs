use r2d2::PooledConnection;
use redis::{Client, RedisError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OauthDbError {
    #[error("RedisPoolError: {0}")]
    RedisPoolError(#[from] r2d2::Error),

    #[error("RedisError: {0}")]
    RedisError(#[from] RedisError),
}

type OauthDbResult<T> = Result<T, OauthDbError>;

pub trait OauthRedisDb {
    async fn get_auth_code(
        &mut self,
        code_verifier: &str,
        client_id: &str,
    ) -> OauthDbResult<Option<String>>;
}

impl OauthRedisDb for PooledConnection<Client> {
    async fn get_auth_code(
        &mut self,
        code_verifier: &str,
        client_id: &str,
    ) -> OauthDbResult<Option<String>> {
        let key = format!("{client_id}:{code_verifier}");
        let auth_code = redis::cmd("GET").arg(key).query(self)?;
        Ok(auth_code)
    }
}
