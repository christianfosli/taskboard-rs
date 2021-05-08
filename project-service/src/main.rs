use std::env;

use errors::handle_rejection;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;
mod cors;
mod errors;
mod handlers;
mod routes;
mod services;
mod store;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info".to_owned()))
        .with_span_events(FmtSpan::CLOSE) // time requests
        .init();

    let es_client = store::es::create_client()?;

    let task_service_url = env::var("TASK_SERVICE_URL")?;
    let task_service_client =
        services::task_service::TaskService::new(reqwest::Client::new(), task_service_url);

    let routes = routes::health_check_route(&es_client)
        .or(routes::project_routes(&es_client, &task_service_client))
        .recover(handle_rejection)
        .with(cors::cors())
        .with(warp::trace::request());

    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;

    Ok(())
}
