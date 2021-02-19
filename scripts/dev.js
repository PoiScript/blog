// fetch polyfill
const fetch = require("node-fetch");

global.Request = fetch.Request;
global.Response = fetch.Response;
global.Headers = fetch.Headers;
// fetch polyfill ends here

const express = require("express");
const webpack = require("webpack");
const middleware = require("webpack-dev-middleware");
const { watchWasmPack } = require("./wasm-pack");

const config = require("./webpack.config.dev");

const app = express();

const compiler = webpack(config);

watchWasmPack({
  target: "nodejs",
  features: "nodejs",
  onSuccess: () => {
    const module = require.cache[require.resolve("../pkg/solomon")];
    if (module) {
      delete module.parent.children;
      delete require.cache[require.resolve("../pkg/solomon")];
      console.log("Reloaded wasm");
    }
  },
});

app.use(middleware(compiler, { writeToDisk: true }));

app.use(async (req, res) => {
  const { handleRequest } = require("../pkg/solomon");
  const fetchRes = await handleRequest(req.url);

  // converts a fetch response to node.js one
  res.statusCode = fetchRes.status;
  res.statusMessage = fetchRes.statusText;
  for (const [key, value] of fetchRes.headers.entries()) {
    res.setHeader(key, value);
  }
  res.write(fetchRes.body);
  res.end();
});

app.listen(4200);
