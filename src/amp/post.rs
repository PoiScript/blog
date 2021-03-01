use json::object;
use maud::{html, Render};
use web_sys::*;

use super::{AmpPage, OrgAmp};
use crate::partials::{title_section, up_next};
use crate::store::{get_css, get_posts_list};
use crate::utils::{html_response, redirect_404_response, to_datetime};

pub async fn post(slug: &str) -> Response {
    let posts = get_posts_list().await;
    let css = get_css().await;

    let post = posts.into_iter().find(|p| p.slug == slug);

    if let Some(post) = post {
        let subtitle = html! {
            (post.published.format("%F"))
            " ·"
            @for tag in post.tags {
                " #" (tag)
            }
        }
        .render()
        .into_string();

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
            "datePublished": to_datetime(post.published).to_rfc2822(),
            "dateModified": post.updated.map(|dt| to_datetime(dt).to_rfc2822()),
            "author": {
                "@type": "Person",
                "name": "PoiScript"
            }
        };

        let amp = AmpPage {
            title: &post.title,
            canonical: &format!("/post/{}", post.slug),
            custom_css: &css,
            schema,
            main: html! {
                ( title_section(&post.title, Some(&subtitle)) )
                article { (OrgAmp(&post.content)) }
                ( up_next(post.prev, post.next) )
            },
        };

        html_response(&amp.render().into_string())
    } else {
        redirect_404_response()
    }
}
