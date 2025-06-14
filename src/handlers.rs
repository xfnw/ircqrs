use axum::{
    extract::Path,
    http::header::CONTENT_TYPE,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use include_dir::{include_dir, Dir};
use rand::prelude::IndexedRandom;
use std::cmp::{max, min};
use std::collections::BTreeMap;
use std::env;
use std::sync::LazyLock;

use crate::templates;

#[cfg(not(test))]
static QUOTES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/quotes");

#[cfg(test)]
static QUOTES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/testquotes");

pub static QUOTEENTRIES: LazyLock<Vec<u32>> = LazyLock::new(index_quoteentries);
pub static PARTICIPANTS: LazyLock<BTreeMap<String, Vec<u32>>> = LazyLock::new(index_participants);
pub static MIN: LazyLock<u32> = LazyLock::new(|| *QUOTEENTRIES.first().unwrap_or(&0));
pub static MAX: LazyLock<u32> = LazyLock::new(|| *QUOTEENTRIES.last().unwrap_or(&0));
static BINPATH: LazyLock<String> =
    LazyLock::new(|| env::current_exe().unwrap().to_str().unwrap().to_string());

fn index_quoteentries() -> Vec<u32> {
    let mut out: Vec<u32> = vec![];
    for i in QUOTES.entries() {
        let name = i.path().to_str().unwrap();
        let name = &name[..name.len() - 4];
        out.push(name.parse::<u32>().unwrap());
    }
    out.sort_unstable();
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
                        if c == '\n' {
                            // names may not span multiple lines, reset
                            go = false;
                            person.clear();
                            (p1, p2) = ('\0', '\0');
                            continue;
                        }
                        if p1 == '\0' {
                            if c == '<' || c == '*' {
                                p1 = c;
                            } else {
                                go = false;
                                continue;
                            }
                        } else if p1 == '<' {
                            // <name> says something lines
                            if c == '>' {
                                go = false;
                            } else {
                                person.push(c);
                            }
                        } else if p1 == '*' {
                            // * name does an action lines
                            if p2 == ' ' {
                                if c == ' ' {
                                    go = false;
                                } else {
                                    person.push(c);
                                }
                            } else if c == ' ' {
                                // action lines do not have a character for
                                // name ending, so we end on a space
                                p2 = c;
                            } else {
                                go = false;
                                p1 = '\0';
                                continue;
                            }
                        }
                        if !go {
                            // go was set to false during this iteration
                            // we successfully parsed out a name

                            // there is not a good way to convert from
                            // Vec<char> to String
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
                        // reset for the next line
                        go = true;
                        (p1, p2) = ('\0', '\0');
                    }
                }
            }
            Err(_) => continue, // skip unreadable quotes
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
        title: "ircqrs",
        content: templates::HomepageHtml {
            first: *MIN,
            last: *MAX,
            binpath: &BINPATH,
            people: &PARTICIPANTS,
        }
        .to_string(),
        relpath: "",
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
                title: "404 not found",
                content: templates::QuoteHtml {
                    first: *MIN,
                    last: *MAX,
                    previous,
                    next,
                    quote: "the requested quote does not exist".to_string(),
                }
                .to_string(),
                relpath: "",
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
                title: "404 not found",
                content: "the requested page does not exist".to_string(),
                relpath: "",
            }
            .to_string(),
        ),
    )
}

pub async fn handler_404() -> impl IntoResponse {
    tuple_404()
}

pub async fn random() -> Redirect {
    let mut rng = rand::rng();
    let randnum: u32 = *QUOTEENTRIES.choose(&mut rng).unwrap_or(&0);
    //let uri: str = format!("{}",randnum);
    Redirect::temporary(&format!("{randnum}"))
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
            title: format!("quote #{quoteid}").as_ref(),
            content: templates::QuoteHtml {
                first: *MIN,
                last: *MAX,
                previous,
                next,
                quote: content,
            }
            .to_string(),
            relpath: "",
        }
        .to_string(),
    )
}

fn get_quote_content(quoteid: u32) -> Result<String, StatusCode> {
    match QUOTES.get_file(format!("{quoteid}.txt")) {
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
                        title: "500 internal server error",
                        content: format!("there was an error converting quote {quoteid} to utf8"),
                        relpath: "",
                    }
                    .to_string(),
                ),
            ),
        },
        Err(_) => tuple_404(),
    }
}

pub async fn view_participant(Path(person): Path<String>) -> (StatusCode, Html<String>) {
    let (returncode, participating) = match (*PARTICIPANTS).get(&person) {
        Some(quotes) => (StatusCode::OK, quotes.as_slice()),
        None => (StatusCode::NOT_FOUND, [].as_slice()),
    };

    (
        returncode,
        Html(
            templates::BaseHtml {
                title: format!("{} quotes featuring {}", participating.len(), person).as_ref(),
                content: templates::ParticipantHtml {
                    person: &person,
                    participating,
                }
                .to_string(),
                relpath: "../",
            }
            .to_string(),
        ),
    )
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
    let got = view_quote(Path("5".to_string())).await;
    assert_eq!(got.0, expected.0);
    assert_eq!(got.1 .0, expected.1 .0);
}
