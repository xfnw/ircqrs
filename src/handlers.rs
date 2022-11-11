use axum::{
    extract::Path,
    http::header::CONTENT_TYPE,
    http::StatusCode,
    response::{AppendHeaders, Html, IntoResponse},
};
use std::error::Error;
use tokio::fs;

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

fn tuple_404() -> (StatusCode, Html<String>) {
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

pub async fn handler_404() -> impl IntoResponse {
    tuple_404()
}

pub async fn random() -> Html<String> {
    Html("not yet implemented".to_string())
}

async fn read_file(filename: String) -> Result<Vec<u8>, Box<dyn Error>> {
    match fs::read(filename).await {
        Ok(contents) => Ok(contents),
        Err(e) => Err(Box::new(e)),
    }
}

pub async fn view_quote(param: Path<String>) -> impl IntoResponse {
    //(StatusCode::OK,Html("meow".to_string()))
    match param.parse::<u16>() {
        Ok(quoteid) => match read_file(format!("quotes/{}.txt", quoteid)).await {
            Ok(content) => (StatusCode::OK,Html("meow".to_string())),
            Err(_) => tuple_404(),
        },
        Err(_) => tuple_404(),
    }
}
