use web_sys::*;

use crate::store::get_assets;

fn get_content_type(name: &str) -> Headers {
    match name.rsplit('.').next() {
        Some("js") => headers! {
            "content-type" => "text/javascript; charset=utf-8"
        },
        Some("gpg") => headers! {
            "content-type" => "text/plain; charset=utf-8"
        },
        Some("css") => headers! {
            "content-type" => "text/css; charset=utf-8"
        },
        Some("jpg") | Some("jpeg") => headers! {
            "content-type" => "image/jpeg"
        },
        Some("png") => headers! {
            "content-type" => "image/png"
        },
        Some("ico") => headers! {
            "content-type" => "image/vnd.microsoft.icon"
        },
        Some("svg") => headers! {
            "content-type" => "image/svg+xml"
        },
        _ => headers! {
            "content-type" => "application/octet-stream"
        },
    }
}

pub async fn assets(name: &str) -> Response {
    let assets = get_assets(name).await;

    Response::new_with_opt_buffer_source_and_init(
        Some(&assets),
        ResponseInit::new()
            .status(200)
            .headers(&get_content_type(name).into()),
    )
    .unwrap()
}
