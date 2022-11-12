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

fn make_quote_page(quoteid: u32, content: String) -> Html<String> {
    // TODO: set this somewhere else lol
    let min = 0;
    let max = 69;

    let mut previous = quoteid;
    let mut next = quoteid;
    if quoteid > min {
        previous = quoteid - 1;
    }
    if quoteid < max {
        next = quoteid + 1;
    }
    let previous = previous; // un-mut
    let next = next;

    Html(
        templates::BaseHtml {
            title: format!("quote #{}", quoteid),
            content: templates::QuoteHtml {
                first: min,
                last: max,
                previous: previous,
                next: next,
                quote: content,
            }
            .to_string(),
        }
        .to_string(),
    )
}

pub async fn view_quote(param: Path<String>) -> impl IntoResponse {
    //(StatusCode::OK,Html("meow".to_string()))
    match param.parse::<u32>() {
        Ok(quoteid) => match read_file(format!("quotes/{}.txt", quoteid)).await {
            Ok(content) => match String::from_utf8(content) {
                Ok(ucontent) => (StatusCode::OK, make_quote_page(quoteid, ucontent)),
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(
                        templates::BaseHtml {
                            title: "500 internal server error".to_string(),
                            content: format!(
                                "there was an error converting quote {} to utf8",
                                quoteid
                            ),
                        }
                        .to_string(),
                    ),
                ),
            },
            Err(_) => tuple_404(),
        },
        Err(_) => tuple_404(),
    }
}
