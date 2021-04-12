use maud::{html, Render};
use wasm_bindgen::prelude::*;

use super::{HtmlPage, OrgHtml};
use crate::partials::title_section;
use crate::Store;

#[wasm_bindgen(js_name = htmlAbout)]
pub fn html_about(store: Store) -> Result<String, JsValue> {
    let post = &store.get_about()?;

    let html = HtmlPage {
        title: "About",
        amphtml: Some("/amp/about"),
        main: html! {
            ( title_section("About", Some(&post.published.format("%F").to_string())) )
            article { (OrgHtml::new(&post.content, &store)) }
        },
    };

    Ok(html.render().0)
}
