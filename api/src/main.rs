use warp::Filter;
#[macro_use]
extern crate log;
mod cors;
mod handlers;
mod routes;
mod store;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    pretty_env_logger::init();
    info!("api starting...");

    let es_client = store::es::create_client()?;

    let routes = routes::task_routes()
        .or(routes::health_check_route(&es_client))
        .with(cors::cors())
        .with(warp::log("taskboard_api"));

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;

    Ok(())
}
