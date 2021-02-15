use web_sys::*;

use crate::store::get_posts_list;

pub async fn org(slug: &str) -> Response {
    let posts = get_posts_list().await;

    if let Some(post) = posts.into_iter().find(|p| p.slug == slug) {
        Response::new_with_opt_str_and_init(
            Some(&post.content),
            ResponseInit::new().status(200).headers(
                &headers!(
                    "content-type" => "text/plain; charset=utf-8"
                )
                .into(),
            ),
        )
        .unwrap()
    } else {
        Response::new_with_opt_str_and_init(
            Some("Not found"),
            ResponseInit::new().status(404).headers(
                &headers!(
                    "content-type" => "text/plain; charset=utf-8"
                )
                .into(),
            ),
        )
        .unwrap()
    }
}
