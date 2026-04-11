use redis::AsyncCommands;

/// Cache key prefix for URL mappings: shorturl:{code} → original_url
const URL_CACHE_PREFIX: &str = "shorturl:";

/// Cache key prefix for click counters: clicks:{code} → count
const CLICK_COUNT_PREFIX: &str = "clicks:";

/// Default TTL for cached URLs: 1 hour
const URL_CACHE_TTL_SECONDS: u64 = 3600;

/// Attempts to get the original URL from Redis cache.
/// Returns None on cache miss OR on Redis error (graceful degradation).
pub async fn get_cached_url(
    redis: &mut redis::aio::ConnectionManager,
    short_code: &str,
) -> Option<String> {
    let key = format!("{}{}", URL_CACHE_PREFIX, short_code);

    match redis.get::<_, Option<String>>(&key).await {
        Ok(Some(value)) => {
            tracing::info!("Cache HIT for key '{}'", key);
            Some(value)
        }
        Ok(None) => {
            tracing::info!("Cache MISS for key '{}'", key);
            None
        }
        Err(e) => {
            tracing::warn!("Redis GET failed for key '{}': {}", key, e);
            None
        }
    }
}

/// Caches a URL mapping in Redis with TTL.
/// Silently logs errors — cache failure should never break the app.
pub async fn set_cached_url(
    redis: &mut redis::aio::ConnectionManager,
    short_code: &str,
    original_url: &str,
) {
    let key = format!("{}{}", URL_CACHE_PREFIX, short_code);

    if let Err(e) = redis
        .set_ex::<_, _, ()>(&key, original_url, URL_CACHE_TTL_SECONDS)
        .await
    {
        tracing::warn!("Redis SET failed for key '{}': {}", key, e);
    }
}

/// Removes a URL from the cache (used on delete or deactivation).
pub async fn delete_cached_url(
    redis: &mut redis::aio::ConnectionManager,
    short_code: &str,
) {
    let key = format!("{}{}", URL_CACHE_PREFIX, short_code);

    if let Err(e) = redis.del::<_, ()>(&key).await {
        tracing::warn!("Redis DEL failed for key '{}': {}", key, e);
    }
}

/// Increments the click counter for a short code in Redis.
/// Uses INCR which is atomic and creates the key with value 1 if it doesn't exist.
pub async fn increment_click_count(
    redis: &mut redis::aio::ConnectionManager,
    short_code: &str,
) {
    let key = format!("{}{}", CLICK_COUNT_PREFIX, short_code);

    if let Err(e) = redis.incr::<_, _, ()>(&key, 1i64).await {
        tracing::warn!("Redis INCR failed for key '{}': {}", key, e);
    }
}

/// Reads all pending click counts from Redis and atomically deletes them.
/// Returns a list of (short_code, count) pairs to be flushed to PostgreSQL.
/// Uses KEYS + GETDEL pattern — safe because click keys are bounded and short-lived.
pub async fn get_and_reset_all_click_counts(
    redis: &mut redis::aio::ConnectionManager,
) -> Vec<(String, i64)> {
    let pattern = format!("{}*", CLICK_COUNT_PREFIX);

    let keys: Vec<String> = match redis::cmd("KEYS")
        .arg(&pattern)
        .query_async(redis)
        .await
    {
        Ok(keys) => keys,
        Err(e) => {
            tracing::warn!("Redis KEYS failed for pattern '{}': {}", pattern, e);
            return Vec::new();
        }
    };

    let mut results = Vec::new();

    for key in keys {
        let short_code = key.trim_start_matches(CLICK_COUNT_PREFIX).to_string();

        // GETDEL: atomically read and delete — no clicks lost between read and delete
        match redis::cmd("GETDEL")
            .arg(&key)
            .query_async::<Option<i64>>(redis)
            .await
        {
            Ok(Some(count)) if count > 0 => {
                results.push((short_code, count));
            }
            Ok(_) => {}
            Err(e) => {
                tracing::warn!("Redis GETDEL failed for key '{}': {}", key, e);
            }
        }
    }

    results
}