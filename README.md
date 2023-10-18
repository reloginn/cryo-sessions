# cryo-sessions
This crate is intended for people who want to store user sessions locally in Redis.

# Example
```rust
let uuid = Uuid::new();
let redis = Redis::from_env().await?; // this method takes the redis url from the REDIS_URL environment variable
let session = Session::new();
redis.new_session(session.clone(), SessionInfo::new(uuid.clone(), "Mozilla(5.0)".into()), Duration::from_secs(2400)).await?;
redis.new_session(session.clone(), SessionInfo::new(uuid.clone(), "Apple Safari".into()), Duration::from_secs(3400)).await?;
let info = redis.get_information_by_session(session).await.unwrap();
println!("{:?}", info);
```