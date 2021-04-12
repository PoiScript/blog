const fetch = require("node-fetch");

const {
  KV_NAMESPACE_ID,
  ACCOUNT_ID,
  CF_TOKEN,
  WORKER_NAME,
} = require("./config");

const writeKVPairs = async (body) => {
  const res = await fetch(
    `https://api.cloudflare.com/client/v4/accounts/${ACCOUNT_ID}/storage/kv/namespaces/${KV_NAMESPACE_ID}/bulk`,
    {
      method: "PUT",
      body,
      headers: {
        authorization: `Bearer ${CF_TOKEN}`,
        "content-type": "application/json",
      },
    }
  );

  if (res.ok) {
    console.log("Wrote KV pairs");
  } else {
    const json = await res.json();
    throw new Error(JSON.stringify(json));
  }
};

const uploadWorker = async (body) => {
  const res = await fetch(
    `https://api.cloudflare.com/client/v4/accounts/${ACCOUNT_ID}/workers/scripts/${WORKER_NAME}`,
    {
      method: "PUT",
      body,
      headers: {
        authorization: `Bearer ${CF_TOKEN}`,
      },
    }
  );

  if (res.ok) {
    console.log("Uploaded worker");
  } else {
    const text = await res.text();
    throw new Error(text);
  }
};

module.exports = { uploadWorker, writeKVPairs };
