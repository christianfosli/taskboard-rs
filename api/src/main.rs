use warp::{Filter, Rejection, Reply};

#[tokio::main]
async fn main() {
    println!("todo-api starting up...");
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    let health = warp::path!("healthz").and_then(health_handler);
    let routes = hello.or(health);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

async fn health_handler() -> Result<impl Reply, Rejection> {
    Ok("OK")
}
