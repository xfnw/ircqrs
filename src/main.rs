use axum::{
    handler::Handler,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use boilerplate::Boilerplate;
use html_escaper::{Escape, Trusted};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

pub mod handlers;
pub mod templates;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handlers::root))
        .fallback(handlers::handler_404.into_service());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8326));
    eprintln!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
