use json::object;
use maud::{html, Render};
use wasm_bindgen::prelude::*;

use super::AmpPage;
use crate::constants::LINKS;
use crate::partials::title_section;
use crate::Store;

#[wasm_bindgen(js_name = ampLink)]
pub fn amp_link(store: Store) -> String {
    let schema = object! {
        "@context": "http://schema.org",
        "@type": "Webpage",
        "url": "https://blog.poi.cat/amp/link",
        "name": "Solomon",
        "headline": "Link☆Solomon",
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
        title: "Link",
        canonical: "/link",
        custom_css: &store.amp_custom_css,
        schema,
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

    amp.render().0
}
