use maud::{html, Render};
use web_sys::*;

use super::HtmlPage;
use crate::partials::title_section;
use crate::utils::html_response;

pub async fn not_found() -> Response {
    let html = HtmlPage {
        title: "Not Found",
        main: html! {
            (title_section("Not Found", None))
            "Not Found"
        },
    };

    html_response(&html.render().into_string())
}
