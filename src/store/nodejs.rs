use js_sys::{try_iter, Array, ArrayBuffer, JsString, Promise, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::post::{Post, UpNext};

#[wasm_bindgen(module = fs)]
extern "C" {
    #[wasm_bindgen(js_namespace = promises, js_name = "readdir")]
    fn read_dir(path: &str) -> Promise;

    #[wasm_bindgen(js_namespace = promises, js_name = "readFile")]
    fn read_file_1(path: JsString) -> Promise;

    #[wasm_bindgen(js_namespace = promises, js_name = "readFile")]
    fn read_file_2(path: JsString, opts: &str) -> Promise;
}

#[wasm_bindgen]
extern "C" {
    type Global;

    #[wasm_bindgen(getter, static_method_of = Global, js_class = globalThis, js_name = CSS_ASSET)]
    fn css_asset() -> JsValue;
}

pub async fn get_about() -> Post {
    unsafe {
        let s = JsFuture::from(read_file_2(JsString::from("content/about.org"), "utf-8"))
            .await
            .unwrap()
            .as_string()
            .unwrap();

        Post::from(&s).unwrap()
    }
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

        let mut posts = try_iter(&posts)
            .unwrap()
            .unwrap()
            .filter_map(|value| Post::from(&value.unwrap().as_string().unwrap()))
            .collect::<Vec<_>>();

        posts.sort_by(|a, b| b.published.cmp(&a.published));

        for index in 0..posts.len() - 1 {
            posts[index].next = Some(UpNext {
                title: posts[index + 1].title.clone(),
                slug: posts[index + 1].slug.clone(),
            });
            posts[index + 1].prev = Some(UpNext {
                title: posts[index].title.clone(),
                slug: posts[index].slug.clone(),
            });
        }

        posts
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

pub async fn get_css() -> String {
    unsafe {
        JsFuture::from(read_file_2(
            JsString::from("dist/").concat(&Global::css_asset()),
            "utf-8",
        ))
        .await
        .unwrap()
        .as_string()
        .unwrap()
    }
}
