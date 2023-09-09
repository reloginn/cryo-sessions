use crate::Uuid;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

#[derive(Debug, Clone)]
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
    /// This method creates a Session structure from two values (this is done for more convenient work)
    /// 
    /// # Example
    /// ```rust
    /// let session = Session::from_values(sess, Uuid::new()); // or Uuid::from("enter uuid here")
    /// ```
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
