mod about;
mod home;
mod link;
mod post;

pub use about::*;
pub use home::*;
pub use link::*;
pub use post::*;

use crate::partials::{footer, header};
use maud::{html, Markup, Render, DOCTYPE};

pub struct AmpPage<'a> {
    title: &'a str,
    canonical: &'a str,
    main: Markup,
}

impl<'a> Render for AmpPage<'a> {
    fn render(&self) -> Markup {
        html! {
            (DOCTYPE)
            html amp? lang="zh-Hans" i-amphtml-layout? i-amphtml-no-boilerplate? transformed="self;v=1" {
                head {
                    meta charset="utf-8";
                    style amp-runtime? i-amphtml-version="011909181902540" {
                        // (PreEscaped(include_str!("../etc/amp-runtime.011909181902540.css")))
                    }
                    link rel="preload" href="https://cdn.ampproject.org/v0.js" as="script";
                    meta name="viewport" content="width=device-width,minimum-scale=1";
                    script async? src="https://cdn.ampproject.org/v0.js" {  }
                    style amp-custom? {
                        // (PreEscaped(include_str!("../etc/amp-custom.css")))
                    }
                    link rel="canonical" href={ "https://blog.poi.cat/post/" (self.canonical) };
                    title { (self.title) "☆Solomon" }
                    script type="application/ld+json" {
                        // (PreEscaped(json.to_string()))
                    }
                }
                body {
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
