use maud::{html, Markup, Render};
use wasm_bindgen::prelude::*;

use crate::post::UpNext;

#[wasm_bindgen]
extern "C" {
    type Global;

    #[wasm_bindgen(getter, static_method_of = Global, js_class = globalThis, js_name = CSS_ASSET)]
    fn css_asset() -> JsValue;

    #[wasm_bindgen(getter, static_method_of = Global, js_class = globalThis, js_name = JS_ASSET)]
    fn js_asset() -> JsValue;
}

pub struct Meta<'a>(&'a str, &'a str);

impl<'a> Render for Meta<'a> {
    fn render(&self) -> Markup {
        html! { meta name=(self.0) content=(self.1); }
    }
}

pub struct Title<'a>(&'a str);

impl<'a> Render for Title<'a> {
    fn render(&self) -> Markup {
        html! { title { (self.0) "☆Solomon" } }
    }
}

pub fn title(title: &str) -> Markup {
    html! { title { (title) "☆Solomon" } }
}

pub fn header() -> Markup {
    let icon = html! {
        svg xmlns="http://www.w3.org/2000/svg"
            width="100%" height="100%"
        {
            g fill="none" stroke="currentColor" stroke-width="1.6" {
                path d="M11.6 17.2H4L14.4 1.6l-10 6 7.2 9.6z" { }
                path d="M12.4 6.8H20L9.6 22.4l10-6-7.2-9.6z" { }
            }
        }
    };

    html! {
        header.header.toolbar {
            div.container {
                a.homepage.link  href="/" {
                    span.logo { (icon) }
                    span { "Solomon" }
                }
                span.spacer { }
                a.link href="/about"{ "About" }
                span.separator { "/" }
                a.link href="/link"{ "Link" }
            }
        }
    }
}

pub fn title_section(title: &str, subtitle: Option<&str>) -> Markup {
    html! {
        div."title-section" {
            h1.title { (title) }

            @if let Some(subtitle) = subtitle {
                h2.subtitle { (subtitle) }
            }
        }
    }
}

pub fn up_next_prev(title: &str, slug: &str) -> Markup {
    html! {
        a.link.start href={ "/post/"(slug) } {
            .icon.left {
                svg fill="currentColor"
                    focusable="false"
                    height="100%"
                    preserveAspectRatio="xMidYMid meet"
                    width="100%"
                    xmlns="http://www.w3.org/2000/svg"
                {
                    path d="M15.4 16.6L10.8 12l4.6-4.6L14 6l-6 6 6 6 1.4-1.4z" {  }
                }
            }
            div {
                .label { "Prev" }
                .title { (title) }
            }
        }
    }
}

pub fn up_next_next(title: &str, slug: &str) -> Markup {
    html! {
        a.link.end href={ "/post/"(slug) } {
            div {
                .label { "Next" }
                .title { (title) }
            }
            .icon.right {
                svg fill="currentColor"
                    focusable="false"
                    height="100%"
                    width="100%"
                    preserveAspectRatio="xMidYMid meet"
                    xmlns="http://www.w3.org/2000/svg"
                {
                    path d="M8.6 16.3l4.6-4.6-4.6-4.5L10 5.7l6 6-6 6z" { }
                }
            }
        }
    }
}

pub fn up_next(prev: Option<UpNext>, next: Option<UpNext>) -> Markup {
    html! {
        .up-next {
            .nav.start {
                @if let Some(prev) = prev {
                    ( up_next_prev(&prev.title, &prev.slug) )
                }
            }
            .nav.end {
                @if let Some(next) = next {
                    ( up_next_next(&next.title, &next.slug) )
                }
            }
        }
    }
}

pub fn footer() -> Markup {
    html! {
        footer.footer.toolbar {
            div.container {
                div.links {
                    a.link href="/rss" { "RSS" }
                    span.separator { "/" }
                    a.link href="https://github.com/PoiScript/solomon" { "GitHub" }
                }
                span.spacer { }
                span.license { "CC-BY-SA-4.0" }
            }
        }
    }
}

pub fn style() -> Markup {
    let url = Global::css_asset();

    html! {
        link rel="stylesheet" href={"/assets/"(url.as_string().unwrap())};
    }
}

pub fn script() -> Markup {
    let url = Global::js_asset();

    html! {
        script src={"/assets/"(url.as_string().unwrap())} {}
    }
}
