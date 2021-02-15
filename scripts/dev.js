// fetch polyfill
const fetch = require("node-fetch");

global.Request = fetch.Request;
global.Response = fetch.Response;
global.Headers = fetch.Headers;
// global.Headers = fetch.Blob;
// fetch polyfill ends here

const express = require("express");
const webpack = require("webpack");
const middleware = require("webpack-dev-middleware");

const config = require("./webpack.config.dev");

const app = express();

app.use(middleware(webpack(config)));

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
