use std::sync::Arc;
use tokio::time::{Duration, interval};

use crate::AppState;
use crate::repositories::url_repository;

/// Periodically removes expired URLs by soft-deleting them.
/// Runs every hour in an infinite loop.
pub async fn run_expired_url_cleanup(state: Arc<AppState>) {
    let mut ticker = interval(Duration::from_secs(3600));

    loop {
        ticker.tick().await;

        match url_repository::cleanup_expired_urls(&state.db).await {
            Ok(0) => {
                tracing::debug!("Expired URL cleanup: no expired URLs found");
            }
            Ok(count) => {
                tracing::info!("Expired URL cleanup: deactivated {} expired URLs", count);
            }
            Err(e) => {
                tracing::error!("Expired URL cleanup failed: {}", e);
            }
        }
    }
}
