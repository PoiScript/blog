use json::object;
use maud::{html, Render};
use wasm_bindgen::prelude::*;

use super::AmpPage;
use crate::Store;

#[wasm_bindgen(js_name = ampHome)]
pub fn amp_home(store: Store) -> String {
    let schema = object! {
        "@context": "http://schema.org",
        "@type": "Webpage",
        "url": "https://blog.poi.cat/amp/",
        "name": "Solomon",
        "headline": "Home☆Solomon",
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
        }
    };

    let amp = AmpPage {
        title: "Home",
        canonical: "/",
        custom_css: &store.amp_custom_css,
        schema,
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

    amp.render().0
}
