use json::object;
use maud::{html, Render};
use web_sys::*;

use super::{AmpPage, OrgAmp};
use crate::partials::title_section;
use crate::store::{get_about, get_css};
use crate::utils::{html_response, to_datetime};

pub async fn about() -> Response {
    let post = get_about().await;
    let css = get_css().await;

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
        "datePublished": to_datetime(post.published).to_rfc2822(),
        "dateModified": post.updated.map(|dt| to_datetime(dt).to_rfc2822()),
        "author": {
            "@type": "Person",
            "name": "PoiScript"
        }
    };

    let amp = AmpPage {
        title: "About",
        canonical: "/about",
        custom_css: &css,
        schema,
        main: html! {
            ( title_section("About", Some(&post.published.format("%F").to_string())) )
            article { (OrgAmp(&post.content)) }
        },
    };

    html_response(&amp.render().into_string())
}
