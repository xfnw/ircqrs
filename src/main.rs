use axum::{routing::get, Router};

use std::net::SocketAddr;

pub mod handlers;
pub mod templates;

fn get_listen() -> SocketAddr {
    // TODO: allow customizing this via args or environment variable
    SocketAddr::from(([127, 0, 0, 1], 8326))
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .route("/health", get(handlers::healthping))
        .route("/random", get(handlers::random))
        .route("/style.css", get(handlers::css))
        .route("/robots.txt", get(handlers::robots))
        .route("/:quote", get(handlers::view_quote))
        .fallback(handlers::handler_404)
}

#[tokio::main]
async fn main() {
    // force lazy_static to initalize
    eprintln!("loaded {} quotes", (*handlers::QUOTEENTRIES).len());

    let app = create_router();

    let addr = get_listen();
    eprintln!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
