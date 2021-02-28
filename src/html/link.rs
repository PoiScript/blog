use maud::{html, Render};
use web_sys::*;

use super::HtmlPage;
use crate::constants::LINKS;
use crate::partials::title_section;
use crate::utils::html_response;

pub async fn link() -> Response {
    let html = HtmlPage {
        title: "Link",
        main: html! {
            (title_section("Link", None))
            ."link-list" {
                @for link in LINKS.iter() {
                    a.item target="_blank" href=(link.0) {
                        img.profile
                            src={ "/assets/avatars/"(link.1)".jpg"}
                            alt={ "avatar for "(link.2) };
                        .text {
                            .title { (link.2) }
                        }
                    }
                }
            }
        },
    };

    html_response(&html.render().into_string())
}
