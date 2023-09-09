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
    pub async fn get_uuid_by_session(&self, session: String) -> RedisResult<Uuid> {
        let mut connection = self.client.get_tokio_connection().await?;
        let mut scan = connection
            .scan_match::<_, String>(format!("*:{}", session.trim()))
            .await?;
        if let Some(next) = scan.next_item().await {
            let (uuid, _) = next.split_once(':').unwrap_or_default();
            Ok(Uuid::from(uuid))
        } else {
            Ok(Default::default())
        }
    }
    pub async fn get_session_by_uuid() {}
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
