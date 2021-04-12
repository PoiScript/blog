use chrono::Utc;
use maud::{html, Render};
use orgize::{export::HtmlHandler, Element, Event, Org};
use std::borrow::Cow;
use std::io::Write;
use wasm_bindgen::prelude::*;

use crate::{handlers::SolomonBaseHandler, Store};

pub struct OrgRss<'a> {
    pub content: &'a str,
    pub store: &'a Store,
}

impl<'a> OrgRss<'a> {
    fn new(content: &'a str, store: &'a Store) -> Self {
        OrgRss { content, store }
    }
}

impl<'a> Render for OrgRss<'a> {
    fn render_to(&self, buffer: &mut String) {
        let org = Org::parse(self.content);
        let mut handler = SolomonBaseHandler::default();

        let mut writer = unsafe { buffer.as_mut_vec() };

        let _ = write!(&mut writer, "<![CDATA[");

        for event in org.iter() {
            match event {
                Event::Start(Element::Link(link)) if link.path.starts_with("file:") => {
                    let path = &link.path[5..];
                    let alt = link.desc.as_ref().unwrap_or_else(|| &Cow::Borrowed(""));

                    let size = path
                        .strip_prefix("/assets/")
                        .and_then(|key| self.store.get_size(key));

                    if let Some((width, height)) = size {
                        let _ = write!(
                            &mut writer,
                            "<img alt=\"{}\" width=\"{}\" height=\"{}\" src=\"{}\"/>",
                            alt, width, height, path
                        );
                    } else {
                        let _ = write!(&mut writer, "<img alt=\"{}\" src=\"{}\"/>", alt, path);
                    }
                }
                Event::Start(element) => {
                    let _ = handler.start(&mut writer, element);
                }
                Event::End(element) => {
                    let _ = handler.end(&mut writer, element);
                }
            }
        }

        let _ = write!(&mut writer, "]]>");
    }
}

#[wasm_bindgen]
pub fn rss(store: Store) -> String {
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
                @for post in &store.posts {
                    item {
                        title { (&post.title) }
                        author { "PoiScript" }
                        link { "https://blog.poi.cat/post/"(post.slug) }
                        guid isPermaLink="false" { (post.slug) }
                        @for tag in &post.tags {
                            category { (tag) }
                        }
                        pubDate { (post.published.to_rfc2822()) }
                        description { ( OrgRss::new(&post.content, &store) ) }
                    }
                }
            }
        }
    };

    body.0
}
