use std::{time::Duration, collections::HashSet};

use redis::{AsyncCommands, Client, RedisResult};

use crate::{session::{Session, SessionInfo}, uuid::Uuid};

#[derive(Debug, Clone)]
pub struct Redis {
    client: Client,
}

impl Redis {
    pub fn new(url: &str) -> Self {
        Self {
            client: Client::open(url).expect("Can not create redis client"),
        }
    }
    pub fn from_env() -> Self {
        Self {
            client: Client::open(std::env::var("REDIS_URL").unwrap_or("redis://127.0.0.1/".into())).expect("Can not create redis client")
        }
    }
    /// This method inserts Session into the Redis.
    /// 
    /// # Example
    /// ```rust
    /// let redis = Redis::from_env();
    /// redis.new_session(Session::new(), SessionInfo::new(Uuid::new(), "Mozilla(5.0)".into()), Duration::from_secs(3400)).await.is_ok(); // Duration indicates how long this record will live in the database
    /// ```
    pub async fn new_session(&self, session: Session, session_info: SessionInfo, duration: Duration) -> RedisResult<()> {
        let mut connection = self.client.get_tokio_connection().await?;
        let key = session.session().trim();
        redis::pipe()
            .hset(key, "uuid", &session_info.uuid().uuid)
            .hset(key, "user_agent", session_info.user_agent())
            .ignore()
            .expire(key, duration.as_secs() as usize)
            .query_async(&mut connection)
            .await?;
        Ok(())
    }
    /// This method gets Uuid by session id.
    /// 
    /// # Example
    /// ```rust
    /// let redis = Redis::from_env();
    /// let info = redis.get_information_by_session(Session::from("put into this session")).await.unwrap().unwrap();
    /// println!("{:?}", info);
    /// ```
    pub async fn get_information_by_session(&self, session: Session) -> RedisResult<SessionInfo> {
        let mut connection = self.client.get_tokio_connection().await?;
        let info: HashSet<String> = connection
            .hvals(session.session().trim()).await?;
        Ok(SessionInfo::new(Uuid::from(info.get("uuid").expect("Can not get uuid").as_str()), info.get("user_agent").expect("Can not get user agent").into()))
    }
}
