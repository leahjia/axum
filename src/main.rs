use axum::{extract::{Path, Query}, response::{Html, IntoResponse}, routing::get, Router};
use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // take a path, make a route
    let routes_hello = Router::new()
        .route( "/hello", get(handler_hello))
        .route( "/helloagain/:name", get(handler_helloagain));

    // start server
    let addr = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("-> LISTENING ON: {}", addr.local_addr().unwrap());
    axum::serve(addr, routes_hello.into_make_service()).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

// handler hello
// e.g. `/hello?name=Jane`
// side note - destructuring the Query struct using Query(params) 
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("-> {:?} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("Leah");
    Html(format!("Hello <string>{name}</string>!!"))
}

// e.g. `/helloagain/Mike`
async fn handler_helloagain(Path(name): Path<String>) -> impl IntoResponse {
    println!("-> {:?} - handler_helloagain - {:?}", "HANDLER", name);
    Html(format!("Hello <string>{name}</string>!!"))
}
