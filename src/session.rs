use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::Uuid;

#[derive(Debug, Clone)]
pub struct SessionInfo {
    uuid: Uuid,
    user_agent: String
}

impl SessionInfo {
    pub fn new(uuid: Uuid, user_agent: String) -> Self {
        Self {
            uuid,
            user_agent
        }
    }
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }
}

#[derive(Debug, Clone)]
pub struct Session {
    session: String,
}

impl Session {
    pub fn new() -> Self {
        Self {
            session: thread_rng()
                .sample_iter(&Alphanumeric)
                .take(64)
                .map(char::from)
                .collect(),
        }
    }
    pub fn session(&self) -> &str {
        &self.session
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for Session {
    fn from(value: String) -> Self {
        Self {
            session: value
        }
    }
}

impl From<&str> for Session {
    fn from(value: &str) -> Self {
        Self {
            session: value.into()
        }
    }
}
