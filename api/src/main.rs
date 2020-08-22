use warp::Filter;

#[tokio::main]
async fn main() {
    println!("todo-api starting up...");

    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([0, 0, 0, 0], 8000))
        .await;
}
