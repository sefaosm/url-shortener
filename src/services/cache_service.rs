// src/services/cache_service.rs

use redis::AsyncCommands;

/// Cache key prefix for URL mappings: shorturl:{code} → original_url
const URL_CACHE_PREFIX: &str = "shorturl:";

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