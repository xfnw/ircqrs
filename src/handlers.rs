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
use std::collections::BTreeMap;
use std::env;

use crate::templates;

#[cfg(not(test))]
static QUOTES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/quotes");

#[cfg(test)]
static QUOTES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/testquotes");

lazy_static! {
    pub static ref QUOTEENTRIES: Vec<u32> = index_quoteentries();
    pub static ref PARTICIPANTS: BTreeMap<String, Vec<u32>> = index_participants();
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

fn index_quoteentries() -> Vec<u32> {
    let mut out: Vec<u32> = vec![];
    for i in QUOTES.entries() {
        let name = i.path().to_str().unwrap();
        let name = &name[..name.len() - 4];
        out.push(name.parse::<u32>().unwrap());
    }
    out.sort();
    out
}

fn index_participants() -> BTreeMap<String, Vec<u32>> {
    let mut participants: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    for quoteid in &*QUOTEENTRIES {
        match get_quote_content(*quoteid) {
            Ok(quote) => {
                let mut go = true;
                let mut p1 = '\0';
                let mut p2 = '\0';
                let mut person: Vec<char> = vec![];
                for c in quote.chars() {
                    if go {
                        if p1 == '\0' {
                            if c == '<' || c == '*' {
                                p1 = c;
                            }
                        } else if p1 == '<' {
                            if c == '>' {
                                go = false;
                            } else {
                                person.push(c);
                            }
                        } else if p1 == '*' {
                            if p2 == ' ' {
                                if c == ' ' {
                                    go = false;
                                } else {
                                    person.push(c);
                                }
                            } else if c == ' ' {
                                p2 = c;
                            }
                        }
                        if !go {
                            let perstr: String = person.iter().collect();
                            person.clear();
                            (p1, p2) = ('\0', '\0');

                            match participants.get_mut(&perstr) {
                                Some(inside) => {
                                    if inside.last().unwrap_or(&0) != quoteid {
                                        inside.push(*quoteid);
                                    }
                                }
                                None => {
                                    participants.insert(perstr, vec![*quoteid]);
                                }
                            }
                        }
                    } else if c == '\n' {
                        go = true;
                        (p1, p2) = ('\0', '\0');
                    }
                }
            }
            Err(_) => continue,
        }
    }
    participants
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
    //let uri: str = format!("{}",randnum);
    Redirect::temporary(&format!("{}", randnum))
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

fn get_quote_content(quoteid: u32) -> Result<String, StatusCode> {
    match QUOTES.get_file(format!("{}.txt", quoteid)) {
        Some(content) => match content.contents_utf8() {
            Some(ucontent) => Ok(ucontent.to_string()),
            None => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn view_quote(param: Path<String>) -> (StatusCode, Html<String>) {
    //(StatusCode::OK,Html("meow".to_string()))
    match param.parse::<u32>() {
        Ok(quoteid) => match get_quote_content(quoteid) {
            Ok(content) => (StatusCode::OK, make_quote_page(quoteid, content)),
            Err(StatusCode::NOT_FOUND) => tuple_404_validid(quoteid),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(
                    templates::BaseHtml {
                        title: "500 internal server error".to_string(),
                        content: format!("there was an error converting quote {} to utf8", quoteid),
                    }
                    .to_string(),
                ),
            ),
        },
        Err(_) => tuple_404(),
    }
}

#[test]
fn check_indexed_quoteentries() {
    assert_eq!(*QUOTEENTRIES, vec![5, 6, 9, 10]);
}

#[test]
fn check_indexed_participants() {
    let mut expected = BTreeMap::new();
    expected.insert("blåhaj".to_string(), vec![9]);
    expected.insert("person1".to_string(), vec![5, 9]);
    expected.insert("person2".to_string(), vec![9]);

    assert_eq!(*PARTICIPANTS, expected);
}

#[tokio::test]
async fn test_quote_retrieval() {
    let expected = (
        StatusCode::OK,
        make_quote_page(5, "<person1> hello there!\n".to_string()),
    );
    let got = view_quote(Path { 0: "5".to_string() }).await;
    assert_eq!(got.0, expected.0);
    assert_eq!(got.1 .0, expected.1 .0);
}
