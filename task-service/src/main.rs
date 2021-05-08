use std::env;

use errors::handle_rejection;
use services::project_service::ProjectService;
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
        .with_span_events(FmtSpan::CLOSE) // times requests
        .init();

    let es_client = store::es::create_client()?;

    let project_service_url = env::var("PROJECT_SERVICE_URL")?;
    let project_service_client = ProjectService::new(reqwest::Client::new(), project_service_url);

    let routes = routes::task_routes(&es_client, &project_service_client)
        .or(routes::health_routes(&es_client))
        .recover(handle_rejection)
        .with(cors::cors())
        .with(warp::trace::request());

    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;

    Ok(())
}
