use maud::{html, Render};
use wasm_bindgen::prelude::*;

use super::HtmlPage;
use crate::partials::title_section;

#[wasm_bindgen(js_name = htmlNotFound)]
pub fn html_not_found() -> String {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let html = HtmlPage {
        title: "Not Found",
        amphtml: None,
        main: html! {
            (title_section("Not Found", None))
            "Not Found"
        },
    };

    html.render().0
}
