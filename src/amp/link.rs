use maud::{html, Render};
use web_sys::*;

use super::AmpPage;
use crate::constants::LINKS;
use crate::partials::title_section;
use crate::utils::html_response;

pub async fn link() -> Response {
    let amp = AmpPage {
        title: "Link",
        canonical: "/link",
        main: html! {
            (title_section("Link", None))
            ."link-list" {
                @for link in LINKS.iter() {
                    a.item target="_blank" href=(link.0) {
                        amp-img.profile
                            src={ "/assets/avatars/"(link.1)".jpg"}
                            width="40"
                            height="40"
                            layout="responsive"
                            alt={ "avatar for "(link.2) } { }
                        .text {
                            .title { (link.2) }
                        }
                    }
                }
            }
        },
    };

    html_response(&amp.render().into_string())
}
