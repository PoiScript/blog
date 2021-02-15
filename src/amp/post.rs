use maud::{html, PreEscaped, Render};
use orgize::Org;
use web_sys::*;

use super::AmpPage;
use crate::partials::{title_section, up_next};
use crate::store::get_posts_list;

pub async fn post(slug: &str) -> Response {
    let posts = get_posts_list().await;

    let post = posts.into_iter().find(|p| p.slug == slug);

    if let Some(post) = post {
        let org = Org::parse(&post.content);

        let mut output = Vec::new();

        org.write_html(&mut output).unwrap_or_default();

        let output = String::from_utf8_lossy(&output);

        let amp = AmpPage {
            title: &post.title,
            canonical: "/",
            main: html! {
                ( title_section(&post.title, None) )
                article { (PreEscaped(output)) }
                ( up_next(post.prev, post.next) )
            },
        };

        Response::new_with_opt_str_and_init(
            Some(&amp.render().into_string()),
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
                    "location" => "/404"
                )
                .into(),
            ),
        )
        .unwrap()
    }
}
