use std::net::SocketAddr;
use tower_http::services::ServeDir;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static());

    // Start Server --->
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("--->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

//#region Routes
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello/:name", get(handler_hello2))
}

fn routes_static() -> Router {
    //todo create actuall fallback route
    Router::new().nest_service("/", ServeDir::new("./fallback"))
}
//#endregion

//#region HelloHandler
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}</strong"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong"))
}
//#endregion
