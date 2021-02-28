use maud::{html, Render};
use web_sys::*;

use super::HtmlPage;
use crate::store::get_posts_list;
use crate::utils::html_response;

pub async fn home() -> Response {
    let posts = get_posts_list().await;

    let html = HtmlPage {
        title: "Home",
        main: html! {
            @for post in posts {
                ."post-item" {
                    a.title href={ "/post/"(post.slug) } { (post.title) }
                    .subtitle {
                        (post.published.format("%F"))
                        " ·"
                        @for tag in post.tags {
                            " #" (tag)
                        }
                    }
                }
            }
        },
    };

    html_response(&html.render().into_string())
}
