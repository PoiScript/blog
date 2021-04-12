mod about;
mod home;
mod link;
mod not_found;
mod post;

pub use about::*;
pub use home::*;
pub use link::*;
pub use not_found::*;
pub use post::*;

use maud::{html, Markup, Render, DOCTYPE};
use orgize::{export::HtmlHandler, Element, Event, Org};
use std::borrow::Cow;
use std::io::Write;

use crate::partials::{footer, header, script, style, title};
use crate::{handlers::SolomonBaseHandler, Store};

pub struct OrgHtml<'a> {
    pub content: &'a str,
    pub store: &'a Store,
}

impl<'a> OrgHtml<'a> {
    fn new(content: &'a str, store: &'a Store) -> Self {
        OrgHtml { content, store }
    }
}

impl<'a> Render for OrgHtml<'a> {
    fn render_to(&self, buffer: &mut String) {
        let org = Org::parse(&self.content);
        let mut handler = SolomonBaseHandler::default();

        let mut writer = unsafe { buffer.as_mut_vec() };

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
                            "<div class=\"image-container\">\
                            <div class=\"image-wrapper\">\
                            <img alt=\"{}\" width=\"{}\" height=\"{}\" src=\"{}\" loading=\"lazy\"/>\
                            </div></div>",
                            alt, width, height, path
                        );
                    } else {
                        let _ = write!(
                            &mut writer,
                            "<div class=\"image-container\">\
                            <div class=\"image-wrapper\">\
                            <img alt=\"{}\" src=\"{}\" loading=\"lazy\"/>\
                            </div></div>",
                            alt, path
                        );
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
    }
}

pub struct HtmlPage<'a> {
    title: &'a str,
    amphtml: Option<&'a str>,
    main: Markup,
}

impl<'a> Render for HtmlPage<'a> {
    fn render(&self) -> Markup {
        html! {
            (DOCTYPE)
            html lang="zh-Hans" {
                head {
                    meta charset="utf-8";
                    (title(self.title))
                    @if let Some(amphtml) = self.amphtml {
                        link rel="amphtml" href={ "https://blog.poi.cat" (amphtml) };
                    }
                    meta name="description" content="PoiScript's Blog";
                    meta name="viewport" content="width=device-width,initial-scale=1";
                    meta name="application-name" content="solomon";
                    meta name="theme-color" content="#673ab7";
                    meta name="apple-mobile-web-app-title" content="solomon";
                    meta name="apple-mobile-web-app-capable" content="yes";
                    meta name="apple-mobile-web-app-status-bar-style" content="black";
                    link rel="apple-touch-icon" sizes="120x120" href="/assets/favicon/touch-icon.png";
                    link rel="shortcut icon" sizes="32x32" href="/assets/favicon/favicon.ico";
                    link rel="icon" sizes="192x192" href="/assets/favicon/favicon-192x192.png";
                    (style())
                    (script())
                }
                body.root {
                    (header())
                    main.main { (self.main) }
                    (footer())
                }
            }
        }
    }
}
