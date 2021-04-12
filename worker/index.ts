declare const wasm: WebAssembly.Module;

import init, * as pkg from "./pkg/solomon";
import { getAssets } from "./assets";
import { getStore, createStore } from "./store";
import * as response from "./responses";

addEventListener("scheduled", (event) => {
  createStore(event);
});

addEventListener("fetch", (event) => {
  try {
    event.respondWith(handleRequest(event));
  } catch (e) {
    if (
      e instanceof Error &&
      (e.message === "POST_NOT_FOUND" || e.message === "ABOUT_NOT_FOUND")
    ) {
      event.respondWith(response.redirect("/404"));
    } else {
      event.respondWith(new Response(e.stack, { status: 500 }));
    }
  }
});

async function handleRequest(event: FetchEvent): Promise<Response> {
  const url = new URL(event.request.url).pathname;

  if (url.length > 1 && url.endsWith("/")) {
    return response.redirect(url.slice(-1));
  }

  if (url.startsWith("/assets/")) {
    const key = url.slice("/assets/".length);
    return getAssets(key);
  }

  await init(wasm);

  const store = await getStore(event);

  if (url === "/") {
    return response.html(pkg.htmlHome(store));
  }

  if (url === "/about") {
    return response.html(pkg.htmlAbout(store));
  }

  if (url === "/link") {
    return response.html(pkg.htmlLink());
  }

  if (url.startsWith("/post/")) {
    const slug = url.slice("/post/".length);
    return response.html(pkg.htmlPost(store, slug));
  }

  if (url === "/amp") {
    return response.html(pkg.ampHome(store));
  }

  if (url === "/amp/about") {
    return response.html(pkg.ampAbout(store));
  }

  if (url === "/amp/link") {
    return response.html(pkg.ampLink(store));
  }

  if (url.startsWith("/amp/post/")) {
    const slug = url.slice("/amp/post/".length);
    return response.html(pkg.ampPost(store, slug));
  }

  if (url === "/rss" || url === "/atom.xml") {
    return response.rss(pkg.rss(store));
  }

  if (url === "/_store") {
    return response.text(store.debugFormat());
  }

  throw new Error("ABOUT_NOT_FOUND");
}
