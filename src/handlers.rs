use axum::{
    extract::Path,
    http::header::CONTENT_TYPE,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use rand::prelude::SliceRandom;
use std::cmp::{max, min};
use std::env;

use crate::templates;

static QUOTES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/quotes");

lazy_static! {
    pub static ref QUOTEENTRIES: Vec<u32> = {
        let mut out: Vec<u32> = vec![];
        for i in QUOTES.entries() {
            let name = i.path().to_str().unwrap();
            let name = &name[..name.len() - 4];
            out.push(name.parse::<u32>().unwrap());
        }
        out.sort();
        out
    };
    pub static ref MIN: u32 = *QUOTEENTRIES.first().unwrap_or(&0);
    pub static ref MAX: u32 = *QUOTEENTRIES.last().unwrap_or(&0);

    // note that BINPATH will be included verbatim in html output,
    // so it may be an XSS vector.
    // we consider this not a non-issue because, presumably,
    // if an attacker has access to move the binary, they could
    // just replace it themselves.
    static ref BINPATH: String = env::current_exe().unwrap()
        .to_str().unwrap()
        .to_string();
}

pub async fn healthping() -> &'static str {
    "OK"
}

pub async fn css() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "text/css; charset=utf8")],
        templates::StyleCss {}.to_string(),
    )
}

pub async fn robots() -> String {
    templates::RobotsTxt {}.to_string()
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

fn tuple_404_validid(quoteid: u32) -> (StatusCode, Html<String>) {
    let mut previous = *MIN;
    let mut next = *MAX;
    if quoteid > *MIN {
        previous = min(*MAX, quoteid - 1);
    }
    if quoteid < *MAX {
        next = max(*MIN, quoteid + 1);
    }
    let previous = previous; // un-mut
    let next = next;

    (
        StatusCode::NOT_FOUND,
        Html(
            templates::BaseHtml {
                title: "404 not found".to_string(),
                content: templates::QuoteHtml {
                    first: *MIN,
                    last: *MAX,
                    previous,
                    next,
                    quote: "the requested quote does not exist".to_string(),
                }
                .to_string(),
            }
            .to_string(),
        ),
    )
}

fn tuple_404() -> (StatusCode, Html<String>) {
    (
        StatusCode::NOT_FOUND,
        Html(
            templates::BaseHtml {
                title: "404 not found".to_string(),
                content: "the requested page does not exist".to_string(),
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
    let randnum: u32 = *QUOTEENTRIES.choose(&mut rng).unwrap_or(&0);
    //let uri: str = format!("/{}",randnum);
    Redirect::temporary(&format!("/{}", randnum))
}

fn make_quote_page(quoteid: u32, content: String) -> Html<String> {
    let mut previous = *MIN;
    let mut next = *MAX;
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
        Ok(quoteid) => match QUOTES.get_file(format!("{}.txt", quoteid)) {
            Some(content) => match content.contents_utf8() {
                Some(ucontent) => (
                    StatusCode::OK,
                    make_quote_page(quoteid, ucontent.to_string()),
                ),
                None => (
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
            None => tuple_404_validid(quoteid),
        },
        Err(_) => tuple_404(),
    }
}
