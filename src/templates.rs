use boilerplate::Boilerplate;
use html_escaper::{Escape, Trusted};
use std::collections::BTreeMap;

#[derive(Boilerplate)]
pub struct BaseHtml {
    pub title: String,
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
