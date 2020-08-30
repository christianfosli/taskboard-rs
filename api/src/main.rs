use warp::{Filter, Rejection, Reply};
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("api starting...");
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    let health = warp::path!("healthz").and_then(health_handler);
    let routes = hello.or(health).with(warp::log("taskboard_api"));
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

async fn health_handler() -> Result<impl Reply, Rejection> {
    Ok("OK")
}
