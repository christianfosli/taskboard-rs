use std::convert::Infallible;

use warp::Reply;

use crate::store::ProjectStore;

pub async fn handle_health(store: impl ProjectStore) -> Result<impl Reply, Infallible> {
    match store.ping().await {
        Ok(_) => Ok("OK"),
        Err(e) => {
            warn!("ProjectStore ping failed: {}", e);
            Ok("Degraded: Server up, ProjectStore unreachable")
        }
    }
}
