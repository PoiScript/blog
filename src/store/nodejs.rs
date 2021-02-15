use js_sys::{try_iter, Array, ArrayBuffer, JsString, Promise, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::post::Post;

#[wasm_bindgen(module = fs)]
extern "C" {
    #[wasm_bindgen(js_namespace = promises, js_name = "readdir")]
    fn read_dir(path: &str) -> Promise;

    #[wasm_bindgen(js_namespace = promises, js_name = "readFile")]
    fn read_file_1(path: JsString) -> Promise;

    #[wasm_bindgen(js_namespace = promises, js_name = "readFile")]
    fn read_file_2(path: JsString, opts: &str) -> Promise;
}

pub async fn get_posts_list() -> Vec<Post> {
    unsafe {
        let files = JsFuture::from(read_dir("content/post")).await.unwrap();

        let posts = JsFuture::from(Promise::all(
            &Array::from(&files)
                .filter(&mut |value: JsValue, _, _| {
                    value
                        .as_string()
                        .map(|s| s.ends_with(".org"))
                        .unwrap_or(false)
                })
                .map(&mut |value: JsValue, _, _| {
                    JsValue::from(&read_file_2(
                        JsString::from("content/post/").concat(&value),
                        "utf-8",
                    ))
                }),
        ))
        .await
        .unwrap();

        let mut res = try_iter(&posts)
            .unwrap()
            .unwrap()
            .filter_map(|value| Post::from(&value.unwrap().as_string().unwrap()))
            .collect::<Vec<_>>();

        res.sort_by(|a, b| b.published.cmp(&a.published));

        res
    }
}

pub async fn get_assets(name: &str) -> ArrayBuffer {
    unsafe {
        let buffer = JsFuture::from(read_file_1(
            JsString::from("assets/").concat(&JsValue::from_str(&name)),
        ))
        .await
        .unwrap();

        Uint8Array::from(buffer).buffer()
    }
}
