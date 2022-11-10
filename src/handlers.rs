use axum::{
    http::header::CONTENT_TYPE,
    http::StatusCode,
    response::{AppendHeaders, Html, IntoResponse},
};

use crate::templates;

pub async fn css() -> impl IntoResponse {
    (
        AppendHeaders([(CONTENT_TYPE, "text/css")]),
        templates::StyleCss {}.to_string(),
    )
}

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
