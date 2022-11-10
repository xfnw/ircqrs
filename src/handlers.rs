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

//mod templates;
use crate::templates;

pub async fn root() -> Html<String> {
    let output = templates::BaseHtml {
        title: "hello, world".to_string(),
        content: "none".to_string(),
    }
    .to_string();
    Html(output)
}

pub async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Html(
            templates::BaseHtml {
                title: "404 not found".to_string(),
                content: "the requested quote does not exist".to_string(),
            }
            .to_string(),
        ),
    )
}
