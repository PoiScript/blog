declare const SOLOMON_KV: KVNamespace;
declare const CSS_ASSET: string;

import { Store } from "./pkg/solomon";

export const getStore = async (event: FetchEvent): Promise<Store> => {
  const bin = await SOLOMON_KV.get("_store", "arrayBuffer");

  if (bin) {
    return Store.fromBin(new Uint8Array(bin));
  } else {
    return createStore(event);
  }
};

const getPosts = async (): Promise<string[]> => {
  const { keys } = await SOLOMON_KV.list({
    prefix: "post/",
  });

  const posts = await Promise.all(keys.map((key) => SOLOMON_KV.get(key.name)));

  return posts as string[];
};

const getImageHeader = async (key: string): Promise<ArrayBuffer> => {
  const stream = await SOLOMON_KV.get(key, "stream");

  const reader = stream!.getReader({ mode: "byob" });

  let buffer: ArrayBuffer = new Uint8Array(12);

  let offset = 0;

  while (offset < buffer.byteLength) {
    const { value: view, done } = await reader.read(
      new Uint8Array(buffer, offset, buffer.byteLength - offset)
    );

    if (view) {
      buffer = view.buffer;
      offset += view.byteLength;
    }
    if (done) {
      reader.cancel();
      break;
    }
  }

  return buffer;
};

const getImages = async (): Promise<{ key: string; buf: ArrayBuffer }[]> => {
  const { keys } = await SOLOMON_KV.list({
    prefix: "images/",
  });

  const images = await Promise.all(
    keys.map((key) =>
      getImageHeader(key.name).then((buf) => ({
        key: key.name,
        buf: buf,
      }))
    )
  );

  return images;
};

const getAbout = async () => SOLOMON_KV.get("about.org");

const getCustomCss = async () =>
  SOLOMON_KV.get(
    CSS_ASSET.startsWith("/assets/") ? CSS_ASSET.slice(8) : CSS_ASSET
  );

export const createStore = async (
  event: FetchEvent | ScheduledEvent
): Promise<Store> => {
  const [posts, images, about, customCss] = await Promise.all([
    getPosts(),
    getImages(),
    getAbout(),
    getCustomCss(),
  ]);

  const store = new Store(posts.length, images.length);

  // posts
  for (const post of posts) {
    if (post) store.pushPost(post);
  }

  store.sortPosts();

  // images

  for (const image of images) {
    if (image.buf) store.pushImage(image.key, new Uint8Array(image.buf));
  }

  // about
  if (about) store.addAbout(about);

  // amp custom css
  if (customCss) store.amp_custom_css = customCss;

  event.waitUntil(SOLOMON_KV.put("_store", store.toBin()));

  return store;
};
