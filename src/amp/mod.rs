mod about;
mod home;
mod link;
mod post;

pub use about::*;
pub use home::*;
pub use link::*;
pub use post::*;

use json::JsonValue;
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use orgize::Org;

use crate::handlers::SolomonHtmlHandler;
use crate::partials::{footer, header, title};

pub struct OrgAmp<'a>(pub &'a str);

impl<'a> Render for OrgAmp<'a> {
    fn render_to(&self, buffer: &mut String) {
        let org = Org::parse(&self.0);

        let _ = org.write_html_custom(
            unsafe { &mut buffer.as_mut_vec() },
            &mut SolomonHtmlHandler::default(),
        );
    }
}

pub struct AmpPage<'a> {
    title: &'a str,
    canonical: &'a str,
    custom_css: &'a str,
    schema: JsonValue,
    main: Markup,
}

impl<'a> Render for AmpPage<'a> {
    fn render(&self) -> Markup {
        html! {
            (DOCTYPE)
            html amp? lang="zh-Hans" {
                head {
                    meta charset="utf-8";
                    script async src="https://cdn.ampproject.org/v0.js" { }
                    (title(self.title))
                    link rel="canonical" href={ "https://blog.poi.cat" (self.canonical) };
                    meta name="description" content="PoiScript's Blog";
                    meta name="viewport" content="width=device-width,minimum-scale=1";
                    meta name="application-name" content="solomon";
                    meta name="theme-color" content="#673ab7";
                    meta name="apple-mobile-web-app-title" content="solomon";
                    meta name="apple-mobile-web-app-capable" content="yes";
                    meta name="apple-mobile-web-app-status-bar-style" content="black";
                    link rel="apple-touch-icon" sizes="120x120" href="/assets/favicon/touch-icon.png";
                    link rel="shortcut icon" sizes="32x32" href="/assets/favicon/favicon.ico";
                    link rel="icon" sizes="192x192" href="/assets/favicon/favicon-192x192.png";
                    (PreEscaped(include_str!("./boilerplate.html")))
                    style amp-custom? {
                        (PreEscaped(self.custom_css))
                    }
                    script type="application/ld+json" {
                        (self.schema.dump())
                    }
                }
                body.root {
                    (header())
                    main.main {
                        (self.main)
                    }
                    (footer())
                }
            }
        }
    }
}
