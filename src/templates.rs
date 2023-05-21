use boilerplate::Boilerplate;
use html_escaper::{Escape, Trusted};

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
