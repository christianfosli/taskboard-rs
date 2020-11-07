use warp::{
    reject::{self, Reject},
    reply, Rejection, Reply,
};

use crate::store::ProjectStore;

pub async fn handle_search_project(
    store: impl ProjectStore,
    search_text: String,
) -> Result<impl Reply, Rejection> {
    let projects = store.search(&search_text).await.map_err(|e| {
        error!(
            "Failed to get search results for {} from store: {}",
            search_text, e
        );
        reject::custom(SearchProjectError {})
    })?;

    Ok(reply::json(&projects))
}

#[derive(Clone, Debug)]
struct SearchProjectError {}
impl Reject for SearchProjectError {}
