use axum::{
    extract::Path,
    http::header::CONTENT_TYPE,
    http::StatusCode,
    response::{AppendHeaders, Html, IntoResponse, Redirect},
};
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use rand::Rng;
use std::{env, error::Error};
use tokio::fs;

use crate::templates;

static QUOTES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/quotes");

lazy_static! {
    static ref MIN: u32 = 0;
    static ref MAX: u32 = 69;

    // note that BINPATH will be included verbatim in html output,
    // so it may be an XSS vector.
    // we consider this not a non-issue because, presumably,
    // if an attacker has access to move the binary, they could
    // just replace it themselves.
    static ref BINPATH: String = env::current_exe().unwrap()
        .to_str().unwrap()
        .to_string();
}

pub async fn css() -> impl IntoResponse {
    (
        AppendHeaders([(CONTENT_TYPE, "text/css")]),
        templates::StyleCss {}.to_string(),
    )
}

pub async fn root() -> Html<String> {
    let output = templates::BaseHtml {
        title: "ircqrs".to_string(),
        content: format!(
            "<p>welcome to the ircqrs quote database!</p>
            <p>check out a <a href='/random'>random quote</a></p>
            <p>served by {}. <a href='https://github.com/xfnw/ircqrs'>
            source</a></p>",
            *BINPATH
        ),
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
    let randnum: u32 = rng.gen_range(*MIN..*MAX);
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
    if quoteid > *MIN {
        previous = quoteid - 1;
    }
    if quoteid < *MAX {
        next = quoteid + 1;
    }
    let previous = previous; // un-mut
    let next = next;

    Html(
        templates::BaseHtml {
            title: format!("quote #{}", quoteid),
            content: templates::QuoteHtml {
                first: *MIN,
                last: *MAX,
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
