use maud::{html, Render};
use web_sys::*;

use super::{AmpPage, OrgAmp};
use crate::partials::title_section;
use crate::store::get_about;
use crate::utils::html_response;

pub async fn about() -> Response {
    let post = get_about().await;

    let amp = AmpPage {
        title: "About",
        canonical: "/about",
        main: html! {
            ( title_section("About", None) )
            article { (OrgAmp(&post)) }
        },
    };

    html_response(&amp.render().into_string())
}
