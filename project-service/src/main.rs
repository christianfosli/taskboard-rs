use std::env;

use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;
mod cors;
mod handlers;
mod routes;
mod store;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(env::var("RUST_LOG").unwrap_or("tracing=info".to_owned()))
        .with_span_events(FmtSpan::CLOSE) // time requests
        .init();

    let es_client = store::es::create_client()?;

    let routes = routes::health_check_route(&es_client)
        .or(routes::project_routes(&es_client))
        .or(warp::path!("print-correlation-id")
            .and(warp::get())
            .and(warp::header::optional::<String>("X-Correlation-Id"))
            .map(|correlation_id| format!("correlation-id: {:?}", correlation_id)))
        .with(cors::cors())
        .with(warp::trace::request());

    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;

    Ok(())
}
