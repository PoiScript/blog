use chrono::Utc;
use maud::{html, Render};
use orgize::Org;
use std::fmt::Write;
use web_sys::*;

use crate::store::get_posts_list;
use crate::utils::to_datetime;

pub struct OrgRss<'a>(pub &'a str);

impl<'a> Render for OrgRss<'a> {
    fn render_to(&self, buffer: &mut String) {
        let _ = buffer.write_str("<![CDATA[");
        let _ = Org::parse(&self.0).write_html(unsafe { &mut buffer.as_mut_vec() });
        let _ = buffer.write_str("]]>");
    }
}

pub async fn rss() -> Response {
    let posts = get_posts_list().await;

    let body = html! {
        rss version="2.0"
            xmlns:atom="http://www.w3.org/2005/Atom"
            xmlns:content="http://purl.org/rss/1.0/modules/content/"
            xmlns:dc="http://purl.org/dc/elements/1.1/"
        {
            channel {
                title { "solomon" }
                description { "PoiScript's Blog" }
                link rel="self" href="https://blog.poi.cat/rss" {}
                link rel="alternate" href="https://blog.poi.cat" {}
                generator { "solomon "(env!("CARGO_PKG_VERSION")) }
                lastBuildDate { (Utc::now().to_rfc2822()) }
                language { "zh-Hans" }
                copyright { "Content licensed under CC-BY-SA-4.0." }
                @for post in posts {
                    item {
                        title { (&post.title) }
                        author { "PoiScript" }
                        link { "https://blog.poi.cat/post/"(post.slug) }
                        guid isPermaLink="false" { (post.slug) }
                        @for tag in &post.tags {
                            category { (tag) }
                        }
                        pubDate { (to_datetime(post.published).to_rfc2822()) }
                        description { ( OrgRss(&post.content) ) }
                    }
                }
            }
        }
    };

    return Response::new_with_opt_str_and_init(
        Some(&body.0),
        ResponseInit::new().status(200).headers(
            &headers!(
                "content-type" => "application/rss+xml; charset=utf-8"
            )
            .into(),
        ),
    )
    .unwrap();
}
