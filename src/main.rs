use axum::{handler::Handler, routing::get, Router};

use std::net::SocketAddr;

pub mod handlers;
pub mod templates;

fn get_listen() -> SocketAddr {
    // TODO: allow customizing this via args or environment variable
    SocketAddr::from(([127, 0, 0, 1], 8326))
}

#[tokio::main]
async fn main() {
    // force lazy_static to initalize
    eprintln!("loaded {} quotes", (*handlers::QUOTEENTRIES).len());

    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/style.css", get(handlers::css))
        .route("/robots.txt", get(handlers::robots))
        .route("/random", get(handlers::random))
        .route("/:quote", get(handlers::view_quote))
        .fallback(handlers::handler_404.into_service());

    let addr = get_listen();
    eprintln!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
