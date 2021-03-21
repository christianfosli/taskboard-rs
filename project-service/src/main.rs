use std::env;

use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;
mod cors;
mod handlers;
mod routes;
mod services;
mod store;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(env::var("RUST_LOG").unwrap_or("tracing=info".to_owned()))
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

/// Work-around for CORS not working on rejected requests (warp issue #518)
async fn handle_rejection(
    err: warp::Rejection,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    tracing::error!("{:?}", err);
    Ok(warp::reply::with_status(
        format!("{:?}", err),
        reqwest::StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
