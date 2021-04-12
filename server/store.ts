import fs from "fs";
import { resolve, join, relative } from "path";
import fetch from "node-fetch";

import { Store } from "./pkg/solomon";

const { readFile, readdir, stat } = fs.promises;

async function walk(dir: string): Promise<string[]> {
  let files: string[] = await readdir(dir);

  const ret: string[] = [];

  for (const file of files) {
    const filePath = join(dir, file);
    const stats = await stat(filePath);

    if (stats.isDirectory()) {
      ret.push(...(await walk(filePath)));
    } else if (stats.isFile()) {
      ret.push(filePath);
    }
  }

  return ret;
}

export const getStore = async () => {
  const files = await readdir(resolve(__dirname, "../content/post"));

  const posts = await Promise.all(
    files
      .filter((file) => file.endsWith(".org"))
      .map((file) => resolve(__dirname, "../content/post/", file))
      .map((path) => readFile(path, "utf8"))
  );

  const root = resolve(__dirname, "../");
  const assets = resolve(__dirname, "../assets");

  const files_ = await walk(assets);

  const images = files_.filter(
    (file) =>
      file.endsWith(".jpg") || file.endsWith(".png") || file.endsWith(".jpeg")
  );

  const store = new Store(posts.length, images.length);

  posts.forEach((post) => store.pushPost(post));

  store.sortPosts();

  store.addAbout(
    await readFile(resolve(__dirname, "../content/about.org"), "utf8")
  );

  for (const image of images) {
    const key = relative(root, image);
    const buffer = await readFile(image);
    store.pushImage(key, buffer);
  }

  store.amp_custom_css = await fetch(
    "http://localhost:4200/assets/main.css"
  ).then((res) => res.text());

  return store;
};
