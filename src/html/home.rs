use maud::{html, Render};
use wasm_bindgen::prelude::*;

use super::HtmlPage;
use crate::Store;

#[wasm_bindgen(js_name = htmlHome)]
pub fn html_home(store: Store) -> String {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let html = HtmlPage {
        title: "Home",
        amphtml: Some("/amp"),
        main: html! {
            @for post in store.posts {
                ."post-item" {
                    a.title href={ "/post/"(post.slug) } { (post.title) }
                    .subtitle {
                        (post.published.format("%F"))
                        " ·"
                        @for tag in post.tags {
                            " #" (tag)
                        }
                    }
                }
            }
        },
    };

    html.render().0
}
