use axum::{
    extract::Path,
    http::header::CONTENT_TYPE,
    http::StatusCode,
    response::{AppendHeaders, Html, IntoResponse, Redirect},
};
use rand::Rng;
use std::error::Error;
use tokio::fs;
use include_dir::{include_dir, Dir};

use crate::templates;

static QUOTES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/quotes");

// TODO: set this somewhere else lol
static MIN: u32 = 0;
static MAX: u32 = 69;

pub fn get_min() -> u32 {MIN}
pub fn get_max() -> u32 {MAX}

pub async fn css() -> impl IntoResponse {
    (
        AppendHeaders([(CONTENT_TYPE, "text/css")]),
        templates::StyleCss {}.to_string(),
    )
}

pub async fn root() -> Html<String> {
    let output = templates::BaseHtml {
        title: "ircqrs".to_string(),
        content: "<p>welcome to the ircqrs quote database!</p>
            <p>check out a <a href='/random'>random quote</a></p>".to_string(),
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

pub async fn random() -> Redirect {
    let mut rng = rand::thread_rng();
    let randnum: u32 = rng.gen_range(get_min()..get_max());
    //let uri: str = format!("/{}",randnum);
    Redirect::temporary(&*format!("/{}", randnum))
}

async fn read_file(filename: String) -> Result<Vec<u8>, Box<dyn Error>> {
    match fs::read(filename).await {
        Ok(contents) => Ok(contents),
        Err(e) => Err(Box::new(e)),
    }
}

fn make_quote_page(quoteid: u32, content: String) -> Html<String> {
    let mut previous = quoteid;
    let mut next = quoteid;
    if quoteid > get_min() {
        previous = quoteid - 1;
    }
    if quoteid < get_max() {
        next = quoteid + 1;
    }
    let previous = previous; // un-mut
    let next = next;

    Html(
        templates::BaseHtml {
            title: format!("quote #{}", quoteid),
            content: templates::QuoteHtml {
                first: get_min(),
                last: get_max(),
                previous,
                next,
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
