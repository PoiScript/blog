const FormData = require("form-data");
const fs = require("fs");
const path = require("path");
const { promisify } = require("util");
const { walk } = require("@nodelib/fs.walk");
const { resolve } = require("path");
const { uploadWorker, writeKVPairs } = require("./cloudflare");
const { KV_NAMESPACE_ID } = require("./config");

const walkAsync = promisify(walk);

const assets = require("../web/dist/webpack-assets.json");
const dist = resolve(__dirname, "../web/dist");

const fsStream = (seg) => fs.createReadStream(path.resolve(__dirname, seg));

const main = async () => {
  const form = new FormData();

  form.append(
    "metadata",
    JSON.stringify({
      bindings: [
        {
          name: "wasm",
          part: "solomon_bg",
          type: "wasm_module",
        },
        {
          type: "kv_namespace",
          name: "SOLOMON_KV",
          namespace_id: KV_NAMESPACE_ID,
        },
        {
          name: "JS_ASSET",
          text: assets.main.js,
          type: "plain_text",
        },
        {
          name: "CSS_ASSET",
          text: assets.main.css,
          type: "plain_text",
        },
      ],
      body_part: "script",
    }),
    {
      contentType: "application/json",
    }
  );

  form.append("script", fsStream("../worker/dist/worker.js"), {
    contentType: "application/javascript",
  });

  form.append("solomon_bg", fsStream("../worker/pkg/solomon_bg.wasm"), {
    contentType: "application/wasm",
  });

  await uploadWorker(form);

  const files = await walkAsync(dist, {
    entryFilter: (entry) => !entry.name.endsWith(".json"),
  });

  const body = files.map((entry) => ({
    key: entry.name,
    base64: true,
    value: fs.readFileSync(entry.path).toString("base64"),
  }));

  writeKVPairs(JSON.stringify(body));
};

main();
