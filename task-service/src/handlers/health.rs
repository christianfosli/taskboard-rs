use tracing::warn;
use warp::{Rejection, Reply};

use crate::errors::PingStoreError;
use crate::store::TaskStore;

pub fn handle_liveness() -> String {
    "OK".into()
}

pub async fn handle_readiness(store: impl TaskStore) -> Result<impl Reply, Rejection> {
    store.ping().await.map_err(|e| {
        warn!(error=?e, "TaskStore ping failed");
        PingStoreError { inner_error: e }
    })?;

    Ok("OK. Store OK.")
}
