use maud::{html, Render};
use web_sys::*;

use super::{HtmlPage, OrgHtml};
use crate::partials::{title_section, up_next};
use crate::store::get_posts_list;
use crate::utils::{html_response, redirect_404_response};

pub async fn post(slug: &str) -> Response {
    let posts = get_posts_list().await;

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

        let html = HtmlPage {
            title: &post.title,
            main: html! {
                ( title_section(&post.title, Some(&subtitle)) )
                article { (OrgHtml(&post.content)) }
                ( up_next(post.prev, post.next) )
            },
        };

        html_response(&html.render().into_string())
    } else {
        redirect_404_response()
    }
}
