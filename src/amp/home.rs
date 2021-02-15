use maud::{html, Render};
use web_sys::*;

use super::AmpPage;
use crate::store::get_posts_list;

pub async fn home() -> Response {
    let posts = get_posts_list().await;

    let amp = AmpPage {
        title: "Link",
        canonical: "/",
        main: html! {
            @for post in posts {
                ."post-item" {
                    a.title href={ "/post/"(post.slug) } { (post.title) }
                    .subtitle { "2018/08/14 · #emacs #org-mode" }
                }
            }
        },
    };

    return Response::new_with_opt_str_and_init(
        Some(&amp.render().into_string()),
        ResponseInit::new().status(200).headers(
            &headers!(
                "content-type" => "text/html; charset=utf-8"
            )
            .into(),
        ),
    )
    .unwrap();
}
