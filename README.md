# cryo-sessions
This crate is intended for people who want to store user sessions locally in Redis.

# Example
```rust
use cryo_sessions::{Redis, Uuid, Session};

let uuid = Uuid::new();
let redis = Redis::from_env(); // this method takes the redis url from the REDIS_URL environment variable
redis.insert_session(Session::new(uuid.to_owned()), Duration::from_secs(2400)).await.is_ok();
redis.insert_session(Session::new(uuid.to_owned()), Duration::from_secs(3400)).await.is_ok();
let sessions = redis.get_sessions_by_uuid(uuid.to_owned()).await.unwrap();
sessions.iter().for_each(|val| println!("{}", val));
let session = redis.get_session_by_uuid(uuid.to_owned()).await.unwrap();
println!("{}", session);
```