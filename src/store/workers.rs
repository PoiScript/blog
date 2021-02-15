use js_sys::{try_iter, Array, ArrayBuffer, Object, Promise, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::post::{Post, UpNext};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = SOLOMON_KV, js_name = "get")]
    fn kv_get_1(key: &str) -> Promise;

    #[wasm_bindgen(js_namespace = SOLOMON_KV, js_name = "get")]
    fn kv_get_2(key: &str, ty: &str) -> Promise;

    #[wasm_bindgen(js_namespace = SOLOMON_KV, js_name = "list")]
    fn kv_list(opt: &JsValue) -> Promise;
}

pub async fn list_keys(prefix: &str) -> JsValue {
    let opt = Object::new();

    Reflect::set(
        &opt,
        &JsValue::from_str("prefix"),
        &JsValue::from_str(prefix),
    )
    .unwrap();
    let value = JsFuture::from(kv_list(&opt)).await.unwrap();

    Reflect::get(&value, &JsValue::from_str("keys")).unwrap()
}

pub async fn get_posts_list() -> Vec<Post> {
    unsafe {
        let keys = list_keys("_org_post").await;

        let promises = Array::new();

        let iter = try_iter(&keys).unwrap().unwrap();

        for key in iter {
            let key = key.unwrap();
            let key = Reflect::get(&key, &JsValue::from_str("name")).unwrap();
            promises.push(&kv_get_1(&key.as_string().unwrap()));
        }

        let posts = JsFuture::from(Promise::all(&promises)).await.unwrap();

        let iter = try_iter(&posts).unwrap().unwrap();

        let mut posts = iter
            .filter_map(|post| {
                let post = post.unwrap().as_string().unwrap();
                Post::from(&post)
            })
            .collect::<Vec<_>>();

        if posts.len() < 2 {
            return posts;
        }

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
        let value = JsFuture::from(kv_get_2(name, "arrayBuffer")).await.unwrap();

        value.into()
    }
}
