use json::object;
use maud::{html, Render};
use wasm_bindgen::prelude::*;

use super::{AmpPage, OrgAmp};
use crate::partials::{title_section, up_next};
use crate::Store;

#[wasm_bindgen(js_name = ampPost)]
pub fn amp_post(store: Store, slug: String) -> Result<String, JsValue> {
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

    let schema = object! {
        "@context": "http://schema.org",
        "@type": "BlogPosting",
        "url": format!("https://blog.poi.cat/amp/{}", post.slug),
        "name": "Solomon",
        "headline": format!("{}☆Solomon", post.title),
        "description": "PoiScript's Blog",
        "mainEntityOfPage": "https://blog.poi.cat",
        "publisher": {
            "@type": "Organization",
            "name": "Solomon",
            "logo": {
                "@type": "ImageObject",
                "url": "https://blog.poi.cat/assets/amp-logo.jpg",
                "height": 60,
                "width": 600
            }
        },
        "datePublished": post.published.to_rfc2822(),
        "dateModified": post.updated.map(|dt| dt.to_rfc2822()),
        "author": {
            "@type": "Person",
            "name": "PoiScript"
        }
    };

    let amp = AmpPage {
        title: &post.title,
        canonical: &format!("/post/{}", post.slug),
        custom_css: &store.amp_custom_css,
        schema,
        main: html! {
            ( title_section(&post.title, Some(&subtitle)) )
            article { (OrgAmp::new(&post.content, &store)) }
            ( up_next(&post.prev, &post.next) )
        },
    };

    Ok(amp.render().0)
}
