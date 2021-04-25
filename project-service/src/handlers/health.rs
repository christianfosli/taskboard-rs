use tracing::warn;
use warp::{reject::Reject, Rejection, Reply};

use crate::store::ProjectStore;

pub fn handle_liveness() -> String {
    "OK".into()
}

pub async fn handle_readiness(store: impl ProjectStore) -> Result<impl Reply, Rejection> {
    store.ping().await.map_err(|e| {
        warn!("Ping store failed: {}", e);
        PingStoreError { inner_error: e }
    })?;

    Ok("OK. Store OK.")
}

#[derive(Debug)]
struct PingStoreError {
    inner_error: anyhow::Error,
}
impl Reject for PingStoreError {}
