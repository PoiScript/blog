(globalThis as any).JS_ASSET = "/assets/main.js";
(globalThis as any).CSS_ASSET = "/assets/main.css";

import express from "express";
import webpack from "webpack";
import middleware from "webpack-dev-middleware";
import { resolve } from "path";

import { getStore } from "./store";

import wasm from "./pkg/solomon";

const config = require("../web/webpack.config");

const app = express();

const compiler = webpack(config);

app.use(middleware(compiler));

app.use("/assets", express.static(resolve(__dirname, "../assets")));

app.get("/", async (req, res) => {
  res.send(wasm.htmlHome(await getStore()));
});

app.get("/about", async (req, res) => {
  res.send(wasm.htmlAbout(await getStore()));
});

app.get("/link", async (req, res) => {
  res.send(wasm.htmlLink());
});

app.get("/post/:slug", async (req, res) => {
  res.send(wasm.htmlPost(await getStore(), req.params.slug));
});

app.get("/amp", async (req, res) => {
  res.send(wasm.ampHome(await getStore()));
});

app.get("/amp/about", async (req, res) => {
  res.send(wasm.ampAbout(await getStore()));
});

app.get("/amp/link", async (req, res) => {
  res.send(wasm.ampLink(await getStore()));
});

app.get("/amp/post/:slug", async (req, res) => {
  res.send(wasm.ampPost(await getStore(), req.params.slug));
});

app.get("/rss", async (req, res) => {
  res.send(wasm.rss(await getStore()));
});

app.get("/atom.xml", async (req, res) => {
  res.set("content-type", "application/xml; charset=utf-8");
  res.send(wasm.rss(await getStore()));
});

app.get("/_store", async (req, res) => {
  res.send((await getStore()).debugFormat());
});

app.get("*", (req, res) => {
  res.send(wasm.htmlNotFound());
});

app.listen(4200);
