use std::convert::Infallible;

use tracing::warn;
use warp::Reply;

use crate::store::TaskStore;

pub async fn handle_health(store: impl TaskStore) -> Result<impl Reply, Infallible> {
    match store.ping().await {
        Ok(_) => Ok("OK"),
        Err(e) => {
            warn!("TaskStore ping failed: {}", e);
            Ok("Degraded: Server up, TaskStore unreachable")
        }
    }
}
