const fetch = require("node-fetch");
const FormData = require("form-data");
const fs = require("fs");
const path = require("path");
const { PassThrough } = require("stream");

const config = require("./config");

const { KV_NAMESPACE_ID, ACCOUNT_ID, WORKER_NAME, CF_TOKEN } = {
  ...config,
  ...process.env,
};

if (![KV_NAMESPACE_ID, ACCOUNT_ID, WORKER_NAME, CF_TOKEN].every(Boolean)) {
  console.error(
    "One of `KV_NAMESPACE_ID`, `ACCOUNT_ID`, `WORKER_NAME`, `CF_TOKEN` is not set."
  );
  process.exit(1);
}

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
      ],
      body_part: "script",
    }),
    {
      contentType: "application/json",
    }
  );

  const workerScript = await fs.promises.readFile(
    path.resolve(__dirname, "../worker/worker.js"),
    "utf-8"
  );
  const wasmBindgenScript = await fs.promises.readFile(
    path.resolve(__dirname, "../pkg/solomon.js"),
    "utf-8"
  );

  form.append(
    "script",
    workerScript.replace("/* WASM_BINDGEN_SCRIPT */", wasmBindgenScript),
    {
      contentType: "application/javascript",
    }
  );
  form.append("solomon_bg", fsStream("../pkg/solomon_bg.wasm"), {
    contentType: "application/wasm",
  });

  const res = await fetch(
    `https://api.cloudflare.com/client/v4/accounts/${ACCOUNT_ID}/workers/scripts/${WORKER_NAME}`,
    {
      method: "PUT",
      body: form,
      headers: {
        authorization: `Bearer ${CF_TOKEN}`,
      },
    }
  );

  if (res.ok) {
    console.log("Published");
  } else {
    const json = await res.json();
    throw new Error(json);
  }
};

main();
