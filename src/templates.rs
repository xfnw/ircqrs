use boilerplate::Boilerplate;
use html_escaper::{Escape, Trusted};
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use std::collections::BTreeMap;
pub const P_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC.remove(b'-').remove(b'_');

#[derive(Boilerplate)]
pub struct BaseHtml<'a> {
    pub title: &'a str,
    pub content: String,
    pub relpath: &'static str,
}

#[derive(Boilerplate)]
pub struct StyleCss {}

#[derive(Boilerplate)]
pub struct RobotsTxt {}

#[derive(Boilerplate)]
pub struct QuoteHtml {
    pub first: u32,
    pub last: u32,
    pub previous: u32,
    pub next: u32,
    pub quote: String,
}

#[derive(Boilerplate)]
pub struct ParticipantHtml<'a> {
    pub person: &'a String,
    pub participating: &'a Vec<u32>,
}

#[derive(Boilerplate)]
pub struct HomepageHtml<'a> {
    pub first: u32,
    pub last: u32,
    pub binpath: &'a String,
    pub people: &'a BTreeMap<String, Vec<u32>>,
}
