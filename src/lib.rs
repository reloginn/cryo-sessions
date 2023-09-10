//! # cryo-sessions
//! This crate is intended for people who want to store user sessions locally in Redis.
//! 
//! # Example
//! ```rust
//! let uuid = Uuid::new();
//! let redis = Redis::from_env(); // this method takes the redis url from the REDIS_URL environment variable
//! let session = Session::new();
//! redis.new_session(session.to_owned(), SessionInfo::new(uuid.to_owned(), "Mozilla(5.0)".into()), Duration::from_secs(2400)).await.is_ok();
//! redis.new_session(session.to_owned(), SessionInfo::new(uuid.to_owned(), "Apple Safari".into()), Duration::from_secs(3400)).await.is_ok();
//! let info = redis.get_information_by_session(session).await.unwrap();
//! println!("{:?}", info);
//! ```

mod redis;
mod session;
mod uuid;

pub use crate::{redis::Redis, session::Session, uuid::Uuid};
