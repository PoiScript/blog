use maud::{html, Render};
use wasm_bindgen::prelude::*;

use super::{HtmlPage, OrgHtml};
use crate::partials::{title_section, up_next};
use crate::Store;

#[wasm_bindgen(js_name = htmlPost)]
pub fn html_post(store: Store, slug: String) -> Result<String, JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let post = store.get_post(&slug)?;

    let subtitle = html! {
        (post.published.format("%F"))
        " ·"
        @for tag in &post.tags {
            " #" (tag)
        }
    }
    .render()
    .0;

    let amphtml = format!("/amp/post/{}", post.slug);

    let html = HtmlPage {
        title: &post.title,
        amphtml: Some(&amphtml),
        main: html! {
            ( title_section(&post.title, Some(&subtitle)) )
            article { (OrgHtml::new(&post.content, &store)) }
            ( up_next(&post.prev, &post.next) )
        },
    };

    Ok(html.render().0)
}
