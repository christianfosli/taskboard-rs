use std::convert::Infallible;

use warp::Reply;

pub async fn handle_health() -> Result<impl Reply, Infallible> {
    Ok("OK")
}
