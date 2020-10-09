use warp::Filter;
#[macro_use]
extern crate log;
mod cors;
mod handlers;
mod routes;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    pretty_env_logger::init();
    info!("project-service starting...");

    let routes = routes::health_check_route()
        .with(cors::cors())
        .with(warp::log("project-service"));

    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;

    Ok(())
}
