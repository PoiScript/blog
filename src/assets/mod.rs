use web_sys::*;

use crate::store::get_assets;

fn get_content_type(name: &str) -> Headers {
    match name.rsplit('.').next() {
        Some("js") => headers! {
            "content-type" => "text/javascript; charset=utf-8",
            "cache-control" => "public, max-age=31536000, immutable"
        },
        Some("gpg") => headers! {
            "content-type" => "text/plain; charset=utf-8",
            "cache-control" => "no-cache"
        },
        Some("css") => headers! {
            "content-type" => "text/css; charset=utf-8",
            "cache-control" => "public, max-age=31536000, immutable"
        },
        Some("jpg") | Some("jpeg") => headers! {
            "content-type" => "image/jpeg",
            "cache-control" => "public, max-age=31536000, immutable"
        },
        Some("png") => headers! {
            "content-type" => "image/png",
            "cache-control" => "public, max-age=31536000, immutable"
        },
        Some("ico") => headers! {
            "content-type" => "image/x-icon",
            "cache-control" => "public, max-age=31536000, immutable"
        },
        Some("svg") => headers! {
            "content-type" => "image/svg+xml",
            "cache-control" => "public, max-age=31536000, immutable"
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
