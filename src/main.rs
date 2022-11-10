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

#[derive(Boilerplate)]
struct BaseHtml {
    title: String,
    content: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .fallback(handler_404.into_service());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8326));
    eprintln!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<String> {
    let output = BaseHtml {
        title: "hello, world".to_string(),
        content: "none".to_string(),
    }
    .to_string();
    Html(output)
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Html(
            BaseHtml {
                title: "404 not found".to_string(),
                content: "the requested quote does not exist".to_string(),
            }
            .to_string(),
        ),
    )
}
