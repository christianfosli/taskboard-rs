use crate::store::TaskStore;
use warp::{reject, Rejection, Reply};

#[derive(Clone, Debug)]
struct TaskStoreUnreachable;

impl warp::reject::Reject for TaskStoreUnreachable {}

pub async fn handle_health(store: impl TaskStore) -> Result<impl Reply, Rejection> {
    store.ping().await.map_err(|e| {
        warn!("Healthcheck failed: {:?}", e);
        reject::custom(TaskStoreUnreachable {})
    })?;

    Ok("OK")
}
