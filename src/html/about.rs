use maud::{html, Render};
use web_sys::*;

use super::{HtmlPage, OrgHtml};
use crate::partials::title_section;
use crate::store::get_about;
use crate::utils::html_response;

pub async fn about() -> Response {
    let post = get_about().await;

    let html = HtmlPage {
        title: "About",
        amphtml: Some("/amp/about"),
        main: html! {
            ( title_section("About", Some(&post.published.format("%F").to_string())) )
            article { (OrgHtml(&post.content)) }
        },
    };

    html_response(&html.render().into_string())
}
