use std::sync::Arc;
use tokio::time::{Duration, interval};

use crate::AppState;
use crate::repositories::url_repository;
use crate::services::cache_service;

/// Periodically flushes accumulated click counts from Redis to PostgreSQL.
/// Runs every 30 seconds. Uses GETDEL for atomic read-and-reset per key.
pub async fn run_click_count_flush(state: Arc<AppState>) {
    let mut ticker = interval(Duration::from_secs(30));

    loop {
        ticker.tick().await;

        let mut redis = state.redis.clone();
        let pending_counts = cache_service::get_and_reset_all_click_counts(&mut redis).await;

        if pending_counts.is_empty() {
            tracing::debug!("Click count flush: no pending counts");
            continue;
        }

        let total_keys = pending_counts.len();
        let mut success_count = 0u64;
        let mut total_clicks = 0i64;

        for (short_code, count) in pending_counts {
            match url_repository::increment_click_count_by_code(&state.db, &short_code, count).await
            {
                Ok(()) => {
                    success_count += 1;
                    total_clicks += count;
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to flush click count for '{}' (count={}): {}",
                        short_code,
                        count,
                        e
                    );
                }
            }
        }

        tracing::info!(
            "Click count flush: updated {}/{} codes, {} total clicks flushed",
            success_count,
            total_keys,
            total_clicks
        );
    }
}
