use json::object;
use maud::{html, Render};
use wasm_bindgen::prelude::*;

use super::{AmpPage, OrgAmp};
use crate::partials::title_section;
use crate::Store;

#[wasm_bindgen(js_name = ampAbout)]
pub fn amp_about(store: Store) -> Result<String, JsValue> {
    let post = &store.get_about()?;

    let schema = object! {
        "@context": "http://schema.org",
        "@type": "BlogPosting",
        "url": "https://blog.poi.cat/amp/about",
        "name": "Solomon",
        "headline": "About☆Solomon",
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
        title: "About",
        canonical: "/about",
        custom_css: &store.amp_custom_css,
        schema,
        main: html! {
            ( title_section("About", Some(&post.published.format("%F").to_string())) )
            article { (OrgAmp::new(&post.content, &store)) }
        },
    };

    Ok(amp.render().0)
}
