use json::object;
use maud::{html, Render};
use web_sys::*;

use super::AmpPage;
use crate::constants::LINKS;
use crate::partials::title_section;
use crate::store::get_css;
use crate::utils::html_response;

pub async fn link() -> Response {
    let css = get_css().await;

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
        custom_css: &css,
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

    html_response(&amp.render().into_string())
}
