use std::time::Duration;

use redis::{AsyncCommands, Client, RedisResult};

use crate::{session::Session, uuid::Uuid};

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
    /// redis.insert_session(Session::new(Uuid::new()), Duration::from_secs(3400)).await.is_ok(); // Duration indicates how long this record will live in the database
    /// ```
    pub async fn insert_session(&self, session: Session, duration: Duration) -> RedisResult<()> {
        let mut connection = self.client.get_tokio_connection().await?;
        connection
            .set_ex(
                format!("{}:{}", session.uuid.uuid, session.session.trim()),
                session.uuid.uuid,
                duration.as_secs() as usize,
            )
            .await?;
        Ok(())
    }
    /// This method gets Uuid by session id.
    /// 
    /// # Example
    /// ```rust
    /// let redis = Redis::from_env();
    /// const SESS: String = String::from("put into this session id");
    /// let uuid = redis.get_uuid_by_session(SESS).await.unwrap().unwrap();
    /// println!("{:?}", uuid);
    /// ```
    pub async fn get_uuid_by_session(&self, session: String) -> RedisResult<Option<Uuid>> {
        let mut connection = self.client.get_tokio_connection().await?;
        let mut scan = connection
            .scan_match::<_, String>(format!("*:{}", session.trim()))
            .await?;
        Ok(scan.next_item().await.and_then(|next| {
            let (uuid, _) = next.split_once(':').unwrap_or_default();
            Some(Uuid::from(uuid))
        }))
    }
    /// This method gets Session by Uuid.
    /// 
    /// # Example
    /// ```rust
    /// let redis = Redis::from_env();
    /// let uuid = Uuid::from("put into this uuid");
    /// let session = redis.get_session_by_uuid(uuid).await.unwrap().unwrap();
    /// println!("{:?}", session);
    /// ```
    pub async fn get_session_by_uuid(&self, uuid: Uuid) -> RedisResult<Option<Session>> {
        let mut connection = self.client.get_tokio_connection().await?;
        let mut scan = connection
            .scan_match::<_, String>(format!("{}:*", uuid.uuid))
            .await?;
        Ok(scan.next_item().await.and_then(|next| {
            let (_, session) = next.split_once(':').unwrap_or_default();
            Some(Session::from_values(session.into(), uuid))
        }))
    }
    /// This method gets all available user sessions.
    /// 
    /// # Example
    /// ```rust
    /// let redis = Redis::from_env();
    /// let uuid = Uuid::from("put into this uuid");
    /// let sessions = redis.get_sessions_by_uuid(uuid).await.unwrap();
    /// sessions.iter().for_each(|val| println!("{}", val));
    /// ``` 
    pub async fn get_sessions_by_uuid(&self, uuid: Uuid) -> RedisResult<Vec<String>> {
        let mut connection = self.client.get_tokio_connection().await?;
        let mut sessions = Vec::new();
        let mut scan = connection
            .scan_match::<_, String>(format!("{}:*", &uuid.uuid))
            .await?;
        while let Some(next) = scan.next_item().await {
            sessions.push(next)
        }
        Ok(sessions)
    }
}
