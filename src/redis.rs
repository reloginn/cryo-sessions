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
