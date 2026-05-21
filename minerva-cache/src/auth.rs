//! This submodule contains implementations related to saving and recovering
//! session objects on the Redis cache.

use redis::AsyncCommands;
use redis::Client;
use redis::RedisResult;

/// Session data expiration time. Counts exactly to 24 hours.
const EXPIRATION: usize = 24 * 60 * 60;

/// Generates a Redis key for the session to be stored on Redis. This key name
/// manipulation occurs mostly to avoid conflicts in multi-tenant scenarios.
/// The output has a format such as `<TENANT>$SESSION:<TOKEN>`.
fn gen_session_key(tenant: &str, token: &str) -> String {
    let clean_info = |info: &str| info.replace('$', "-").replace(':', "_");
    base64::encode(format!(
        "{}$SESSION:{}",
        clean_info(tenant),
        clean_info(token)
    ))
}

/// Retrieves session data from Redis, given a tenant and the session token.
/// If exists, returns the session data in JSON format, as a string.
pub async fn get_session(client: &Client, tenant: &str, token: &str) -> RedisResult<String> {
    let key = gen_session_key(tenant, token);
    let mut conn = client.get_async_connection().await?;
    conn.get(key).await
}

/// Saves session data to Redis, given a tenant, the session token, and its data
/// formatted as a JSON string.
pub fn save_session(client: &Client, tenant: &str, token: &str, json: &str) -> RedisResult<()> {
    let key = gen_session_key(tenant, token);
    let mut conn = client.get_connection()?;
    redis::transaction(&mut conn, &[std::slice::from_ref(&key)], |conn, pipe| {
        pipe.set(key.clone(), json)
            .expire(key.clone(), EXPIRATION)
            .query(conn)
    })
}

/// Removes session data from Redis. Will attempt to delete the data from a
/// single session, given a tenant and the session token. If the key doesn't
/// exist on Redis cache, returns a successful result anyway.
pub async fn remove_session(client: &Client, tenant: &str, token: &str) -> RedisResult<()> {
    let key = gen_session_key(tenant, token);
    let mut conn = client.get_async_connection().await?;
    conn.del(key).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
    async fn save_then_remove() {
        let server = "localhost:6379";
        let token = "save_then_remove";
        let json = "{\"foo\": \"bar\"}";
        let tenant = "teste";

        let client = crate::build_client(server).unwrap();

        // Remove session before anything. Should not error.
        // Running it thrice as a guarantee.
        remove_session(&client, tenant, token).await.unwrap();
        remove_session(&client, tenant, token).await.unwrap();
        remove_session(&client, tenant, token).await.unwrap();

        // Make sure session does not exist on server
        assert!(get_session(&client, tenant, token).await.is_err());

        // Save session, expect non-failure result
        save_session(&client, tenant, token, json).unwrap();

        // Fetch saved JSON and expect it to be equals to what we sent
        let json_return = get_session(&client, tenant, token).await.unwrap();
        assert_eq!(json, json_return);

        // Remove session expecting success
        remove_session(&client, tenant, token).await.unwrap();
    }
}
