use maud::{html, Render};
use web_sys::*;

use super::HtmlPage;
use crate::partials::title_section;

pub async fn not_found() -> Response {
    let html = HtmlPage {
        title: "Not Found",
        main: html! {
            (title_section("Not Found", None))
            "Not Found"
        },
    };

    return Response::new_with_opt_str_and_init(
        Some(&html.render().into_string()),
        ResponseInit::new().status(200).headers(
            &headers!(
                "content-type" => "text/html; charset=utf-8"
            )
            .into(),
        ),
    )
    .unwrap();
}
