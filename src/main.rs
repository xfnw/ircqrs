use axum::{routing::get, Router};
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;

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
        .route("/by/:person", get(handlers::view_participant))
        .fallback(handlers::handler_404)
}

#[tokio::main]
async fn main() {
    // force lazy_static to initalize
    eprintln!("loaded {} quotes", (*handlers::QUOTEENTRIES).len());
    eprintln!("loaded {} participants", (*handlers::PARTICIPANTS).len());

    let app = create_router();

    let listener = TcpListener::bind(get_listen()).await.unwrap();
    eprintln!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[test]
fn test_get_listener() {
    env::set_var("IRCQRS_BIND", "1.2.3.4:3621");
    assert_eq!(get_listen(), "1.2.3.4:3621".parse().unwrap());
    env::remove_var("IRCQRS_BIND");
    assert_eq!(get_listen(), "127.0.0.1:8326".parse().unwrap());
}
