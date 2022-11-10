use boilerplate::Boilerplate;
use html_escaper::{Escape, Trusted};

#[derive(Boilerplate)]
pub struct BaseHtml {
    pub title: String,
    pub content: String,
}
