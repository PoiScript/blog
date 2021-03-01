use json::object;
use maud::{html, Render};
use web_sys::*;

use super::AmpPage;
use crate::store::{get_css, get_posts_list};
use crate::utils::html_response;

pub async fn home() -> Response {
    let posts = get_posts_list().await;
    let css = get_css().await;

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
        custom_css: &css,
        schema,
        main: html! {
            @for post in posts {
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

    html_response(&amp.render().into_string())
}
