use maud::{html, PreEscaped, Render};
use orgize::Org;
use web_sys::*;

use super::HtmlPage;
use crate::handlers::SolomonHtmlHandler;
use crate::partials::{title_section, up_next};
use crate::store::get_posts_list;

pub async fn post(slug: &str) -> Response {
    let posts = get_posts_list().await;

    let post = posts.into_iter().find(|p| p.slug == slug);

    if let Some(post) = post {
        let org = Org::parse(&post.content);

        let mut output = Vec::new();

        org.write_html_custom(&mut output, &mut SolomonHtmlHandler::default())
            .unwrap_or_default();

        let output = String::from_utf8_lossy(&output);

        let html = HtmlPage {
            title: &post.title,
            main: html! {
                ( title_section(&post.title, None) )
                article { (PreEscaped(output)) }
                ( up_next(post.prev, post.next) )
            },
        };

        Response::new_with_opt_str_and_init(
            Some(&html.render().into_string()),
            ResponseInit::new().status(200).headers(
                &headers!(
                    "content-type" => "text/html; charset=utf-8"
                )
                .into(),
            ),
        )
        .unwrap()
    } else {
        Response::new_with_opt_str_and_init(
            Some("Redirecting to /404"),
            ResponseInit::new().status(302).headers(
                &headers!(
                    "location" => "/404",
                    "content-type" => "text/plain; charset=utf-8"
                )
                .into(),
            ),
        )
        .unwrap()
    }
}
