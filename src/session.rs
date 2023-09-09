use crate::Uuid;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub struct Session {
    pub session: String,
    pub uuid: Uuid,
}

impl Session {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            session: thread_rng()
                .sample_iter(&Alphanumeric)
                .take(64)
                .map(char::from)
                .collect(),
            uuid,
        }
    }
    pub fn from_values(session: String, uuid: Uuid) -> Self {
        Self { session, uuid }
    }
}

impl From<Uuid> for Session {
    fn from(value: Uuid) -> Self {
        Self {
            session: thread_rng()
                .sample_iter(&Alphanumeric)
                .take(64)
                .map(char::from)
                .collect(),
            uuid: value,
        }
    }
}
