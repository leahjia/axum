use axum::{response::{Html, IntoResponse}, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // take a path, make a route
    let routes_hello = Router::new().route( "/hello", get(handler_hello));

    // start server
    let addr = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("-> LISTENING ON: {}", addr.local_addr().unwrap());
    axum::serve(addr, routes_hello.into_make_service()).await.unwrap();
}

// handler hello
async fn handler_hello() -> impl IntoResponse {
    println!("-> {:?} - handler_hello", "HANDLER");
    Html("Hello <string>Leah!</string>")
}
