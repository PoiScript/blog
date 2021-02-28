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
        main: html! {
            ( title_section("About", None) )
            article { (OrgHtml(&post)) }
        },
    };

    html_response(&html.render().into_string())
}
