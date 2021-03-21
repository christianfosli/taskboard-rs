use percent_encoding::percent_decode_str;
use tracing::{debug, error};
use warp::{
    reject::{self, Reject},
    reply, Rejection, Reply,
};

use crate::store::ProjectStore;

pub async fn handle_search_project(
    store: impl ProjectStore,
    search_text: String,
) -> Result<impl Reply, Rejection> {
    // decoding the search_text might be done automatically by warp in the future
    // see https://github.com/seanmonstar/warp/issues/242
    let search_text = percent_decode_str(&search_text)
        .decode_utf8()
        .map_err(|e| {
            error!("Failed to decode search text: {}", e);
            reject::custom(SearchProjectError {})
        })?
        .to_string();

    debug!("Search text decoded to {}", &search_text);

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
