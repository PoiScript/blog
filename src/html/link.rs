use maud::{html, Render};
use wasm_bindgen::prelude::*;

use super::HtmlPage;
use crate::constants::LINKS;
use crate::partials::title_section;

#[wasm_bindgen(js_name = htmlLink)]
pub fn html_link() -> String {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let html = HtmlPage {
        title: "Link",
        amphtml: Some("/amp/link"),
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

    html.render().0
}
