use axum::{routing::get, Router};

use std::{env, net::SocketAddr};

pub mod handlers;
pub mod templates;

fn get_listen() -> SocketAddr {
    // TODO: add IRCQRS_SOCK environment variable for listening
    // on unix socket
    match env::var("IRCQRS_BIND") {
        Ok(value) => value.parse(),
        Err(_) => "127.0.0.1:8326".parse(),
    }
    .expect("failed to parse IRCQRS_BIND")
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

#[test]
fn listener_from_envvar() {
    env::set_var("IRCQRS_BIND", "1.2.3.4:3621");
    assert_eq!(get_listen(), "1.2.3.4:3621".parse().unwrap());
}

#[test]
fn listener_from_default() {
    env::remove_var("IRCQRS_BIND");
    assert_eq!(get_listen(), "127.0.0.1:8326".parse().unwrap());
}
