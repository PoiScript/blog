use chrono::{DateTime, NaiveDate, Utc};
use imagesize::blob_size;
use js_sys::Error;
use orgize::{Element, Event, Org};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpNext {
    pub(crate) title: String,
    pub(crate) slug: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Post {
    pub(crate) title: String,
    pub(crate) slug: String,
    pub(crate) content: String,
    pub(crate) published: DateTime<Utc>,
    pub(crate) updated: Option<DateTime<Utc>>,
    pub(crate) tags: Vec<String>,
    pub(crate) prev: Option<UpNext>,
    pub(crate) next: Option<UpNext>,
}

fn parse_timestamp(value: &str) -> Option<DateTime<Utc>> {
    NaiveDate::parse_from_str(value, "[%Y-%m-%d %a]")
        .map(|date| DateTime::<Utc>::from_utc(date.and_hms(0, 0, 0), Utc))
        .ok()
}

impl Post {
    pub fn from(content: &str) -> Post {
        let org = Org::parse(content);
        let (mut title, mut published, mut updated, mut slug, mut tags) =
            (None, None, None, None, None);

        for event in org.iter() {
            if let Event::Start(Element::Keyword(keyword)) = event {
                match &*keyword.key {
                    "TITLE" => title = Some(keyword.value.to_string()),
                    "PUBLISHED" => {
                        published = Some(parse_timestamp(&keyword.value).unwrap());
                    }
                    "UPDATED" => {
                        updated = Some(parse_timestamp(&keyword.value).unwrap());
                    }
                    "TAGS" => {
                        tags = Some(
                            keyword
                                .value
                                .split_whitespace()
                                .map(|s| s.to_string())
                                .collect(),
                        )
                    }
                    "SLUG" => slug = Some(keyword.value.to_string()),
                    _ => (),
                }
            }
        }

        Post {
            published: published.expect("Missing keyword PUBLISHED"),
            updated,
            content: content.into(),
            title: title.expect("Missing keyword TITLE"),
            slug: slug.expect("Missing keyword SLUG"),
            tags: tags.expect("Missing keyword TAGS"),
            next: None,
            prev: None,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize)]
pub struct Store {
    pub(crate) amp_custom_css: String,
    pub(crate) posts: Vec<Post>,
    pub(crate) about: Option<Post>,
    pub(crate) image_sizes: Vec<(String, usize, usize)>,
}

#[wasm_bindgen]
impl Store {
    #[wasm_bindgen(constructor)]
    pub fn new(posts_capacity: usize, image_sizes_capacity: usize) -> Store {
        Store {
            amp_custom_css: String::new(),
            about: None,
            posts: Vec::with_capacity(posts_capacity),
            image_sizes: Vec::with_capacity(image_sizes_capacity),
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_amp_custom_css(&mut self, css: String) {
        self.amp_custom_css = css;
    }

    #[wasm_bindgen(js_name = addAbout)]
    pub fn add_about(&mut self, content: String) {
        self.about = Some(Post::from(&content));
    }

    #[wasm_bindgen(js_name = pushPost)]
    pub fn push_post(&mut self, content: String) {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        self.posts.push(Post::from(&content));
    }

    #[wasm_bindgen(js_name = sortPosts)]
    pub fn sort_posts(&mut self) {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        if self.posts.len() < 2 {
            return;
        }

        self.posts.sort_by(|a, b| b.published.cmp(&a.published));

        for index in 0..self.posts.len() - 1 {
            self.posts[index].next = Some(UpNext {
                title: self.posts[index + 1].title.clone(),
                slug: self.posts[index + 1].slug.clone(),
            });
            self.posts[index + 1].prev = Some(UpNext {
                title: self.posts[index].title.clone(),
                slug: self.posts[index].slug.clone(),
            });
        }
    }

    #[wasm_bindgen(js_name = pushImage)]
    pub fn push_image(&mut self, key: String, buffer: Vec<u8>) {
        if let Ok(size) = blob_size(&buffer) {
            self.image_sizes.push((key, size.width, size.height));
        }
    }

    #[wasm_bindgen(js_name = toBin)]
    pub fn to_bin(&self) -> Vec<u8> {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        bincode::serialize(&self).unwrap()
    }

    #[wasm_bindgen(js_name = fromBin)]
    pub fn from_bin(bin: Vec<u8>) -> Self {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        bincode::deserialize(&bin).unwrap()
    }

    #[wasm_bindgen(js_name = debugFormat)]
    pub fn debug_format(&self) -> String {
        format!("{:#?}", &self)
    }
}

impl Store {
    pub fn get_post(&self, slug: &str) -> Result<&Post, JsValue> {
        self.posts
            .iter()
            .find(|post| post.slug == slug)
            .map(|post| Ok(post))
            .unwrap_or_else(|| Err(Error::new("POST_NOT_FOUND").into()))
    }

    pub fn get_size(&self, key: &str) -> Option<(usize, usize)> {
        self.image_sizes
            .iter()
            .find(|item| item.0 == key)
            .map(|item| (item.1, item.2))
    }

    pub fn get_about(&self) -> Result<&Post, JsValue> {
        self.about
            .as_ref()
            .map(|about| Ok(about))
            .unwrap_or_else(|| Err(Error::new("ABOUT_NOT_FOUND").into()))
    }
}
