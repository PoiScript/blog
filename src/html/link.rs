use maud::{html, Render};
use web_sys::*;

use super::HtmlPage;
use crate::constants::LINKS;
use crate::partials::title_section;

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

    return Response::new_with_opt_str_and_init(
        Some(&html.render().into_string()),
        ResponseInit::new().status(200).headers(
            &headers!(
                "content-type" => "text/html; charset=utf-8"
            )
            .into(),
        ),
    )
    .unwrap();
}
