use warp::{Filter, Rejection, Reply};

use crate::handlers::health::handle_health;

pub fn health_check_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("healthz").and_then(handle_health)
}
