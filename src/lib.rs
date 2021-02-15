#[cfg(not(any(feature = "nodejs", feature = "workers")))]
compile_error!("Either feature \"nodejs\" or \"workers\" must be enabled.");

extern crate console_error_panic_hook;
extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::prelude::*;
use web_sys::*;

#[macro_use]
mod utils;

mod amp;
mod assets;
mod constants;
mod handlers;
mod html;
mod org;
mod partials;
mod post;
mod route;
mod rss;
mod store;

use std::panic;

use route::{match_route, Matched};

#[wasm_bindgen(js_name = handleRequest)]
pub async fn handle_request(url: String) -> Response {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    match match_route(&url) {
        Matched::Home => html::home().await,
        Matched::About => html::about().await,
        Matched::Link => html::link().await,
        Matched::Post(slug) => html::post(slug).await,
        Matched::NotFound => html::not_found().await,

        Matched::AMPHome => amp::home().await,
        Matched::AMPAbout => amp::about().await,
        Matched::AMPLink => amp::link().await,
        Matched::AMPPost(slug) => amp::post(slug).await,

        Matched::OrgAbout => org::org("about").await,
        Matched::OrgPost(slug) => org::org(slug).await,

        Matched::RSS => rss::rss().await,

        Matched::Assets(name) => assets::assets(name).await,
    }
}
