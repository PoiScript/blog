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
use orgize::Org;

use crate::handlers::SolomonHtmlHandler;
use crate::partials::{footer, header, script, style, title};

pub struct OrgHtml<'a>(pub &'a str);

impl<'a> Render for OrgHtml<'a> {
    fn render_to(&self, buffer: &mut String) {
        let org = Org::parse(&self.0);

        let _ = org.write_html_custom(
            unsafe { &mut buffer.as_mut_vec() },
            &mut SolomonHtmlHandler::default(),
        );
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
