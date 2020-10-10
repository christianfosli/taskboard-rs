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
    info!("project-service starting...");

    let es_client = store::es::create_client()?;

    let routes = routes::health_check_route(&es_client)
        .or(routes::project_routes(&es_client))
        .with(cors::cors())
        .with(warp::log("project-service"));

    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;

    Ok(())
}
