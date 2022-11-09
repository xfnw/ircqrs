use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use boilerplate::Boilerplate;
use html_escaper::{Escape, Trusted};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Boilerplate)]
struct BaseHtml {
    title: &'static str,
    content: &'static str,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8326));
    eprintln!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<String> {
    let output = BaseHtml {
        title: "hello, world",
        content: "none",
    }
    .to_string();
    Html(output)
}
