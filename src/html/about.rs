use web_sys::*;

use crate::html::post;

pub async fn about() -> Response {
    post("about").await
}
