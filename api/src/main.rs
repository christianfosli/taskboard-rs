use warp::Filter;
#[macro_use]
extern crate log;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("api starting...");

    let routes = routes::task_routes()
        .or(routes::health_check_route())
        .with(warp::log("taskboard_api"));

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
