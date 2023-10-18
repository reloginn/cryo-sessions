use std::{time::Duration, collections::HashSet};

use redis::{AsyncCommands, Client, RedisResult, aio::Connection};

use crate::{session::{Session, SessionInfo}, uuid::Uuid};

pub struct Redis {
    connection: Connection,
}

impl Redis {
    pub async fn new(url: &str) -> RedisResult<Self> {
        Ok(Self {
            connection: Client::open(url)?.get_tokio_connection().await?,
        })
    }
    pub async fn from_env() -> RedisResult<Self> {
        Ok(Self {
            connection: Client::open(std::env::var("REDIS_URL").unwrap_or("redis://127.0.0.1/".into()))?.get_tokio_connection().await?
        })
    }
    /// This method inserts Session into the Redis.
    /// 
    /// # Example
    /// ```rust
    /// let redis = Redis::from_env().await?;
    /// redis.new_session(Session::new(), SessionInfo::new(Uuid::new(), "Mozilla(5.0)".into()), Duration::from_secs(3400)).await.is_ok(); // Duration indicates how long this record will live in the database
    /// ```
    pub async fn new_session(&mut self, session: Session, session_info: SessionInfo, duration: Duration) -> RedisResult<()> {
        let key = session.session().trim();
        redis::pipe()
            .hset(key, "uuid", &session_info.uuid().uuid)
            .hset(key, "user_agent", session_info.user_agent())
            .ignore()
            .expire(key, duration.as_secs() as usize)
            .query_async(&mut self.connection)
            .await?;
        Ok(())
    }
    /// This method gets Uuid by session id.
    /// 
    /// # Example
    /// ```rust
    /// let redis = Redis::from_env().await?;
    /// let info = redis.get_information_by_session(Session::from("put into this session id")).await?;
    /// println!("{:?}", info);
    /// ```
    pub async fn get_information_by_session(&mut self, session: Session) -> Option<SessionInfo> {
        let info: HashSet<String> = self.connection
            .hvals(session.session().trim()).await.expect("Failed to get hvals");
        Some(SessionInfo::new(Uuid::from(info.get("uuid")?.as_str()), info.get("user_agent")?.into()))
    }
}
