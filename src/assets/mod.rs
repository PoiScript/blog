use web_sys::*;

use crate::store::get_assets;

fn get_content_type(name: &str) -> Headers {
    let headers = ::web_sys::Headers::new().unwrap();
    headers.set("x-content-type-options", "nosniff").unwrap();

    match name.rsplit('.').next() {
        Some("js") => {
            headers
                .set("content-type", "text/javascript; charset=utf-8")
                .unwrap();
            headers
                .set("cache-control", "public, max-age=31536000, immutable")
                .unwrap();
        }
        Some("gpg") => {
            headers
                .set("content-type", "text/plain; charset=utf-8")
                .unwrap();
            headers.set("cache-control", "no-cache").unwrap();
        }
        Some("css") => {
            headers
                .set("content-type", "text/css; charset=utf-8")
                .unwrap();
            headers
                .set("cache-control", "public, max-age=31536000, immutable")
                .unwrap();
        }
        Some("jpg") | Some("jpeg") => {
            headers.set("content-type", "image/jpeg").unwrap();
            headers
                .set("cache-control", "public, max-age=31536000, immutable")
                .unwrap();
        }
        Some("png") => {
            headers.set("content-type", "image/png").unwrap();
            headers
                .set("cache-control", "public, max-age=31536000, immutable")
                .unwrap();
        }
        Some("ico") => {
            headers.set("content-type", "image/x-icon").unwrap();
            headers
                .set("cache-control", "public, max-age=31536000, immutable")
                .unwrap();
        }
        Some("svg") => {
            headers.set("content-type", "image/svg+xml").unwrap();
            headers
                .set("cache-control", "public, max-age=31536000, immutable")
                .unwrap();
        }
        _ => {
            headers
                .set("content-type", "application/octet-stream")
                .unwrap();
        }
    };

    headers
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
