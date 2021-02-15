const fetch = require("node-fetch");
const fs = require("fs").promises;
const path = require("path");

const config = require("./config");

const { KV_NAMESPACE_ID, ACCOUNT_ID, CF_TOKEN } = {
  ...config,
  ...process.env,
};

if (![KV_NAMESPACE_ID, ACCOUNT_ID, CF_TOKEN].every(Boolean)) {
  console.error(
    "One of `KV_NAMESPACE_ID`, `ACCOUNT_ID`, `CF_TOKEN` is not set."
  );
  process.exit(1);
}

const assets = path.resolve(__dirname, "../assets");
const content = path.resolve(__dirname, "../content");
const dist = path.resolve(__dirname, "../dist");

async function walk(dir) {
  let files = await fs.readdir(dir);
  files = await Promise.all(
    files.map(async (file) => {
      const filePath = path.join(dir, file);
      const stats = await fs.stat(filePath);
      if (stats.isDirectory()) return walk(filePath);
      else if (stats.isFile()) return filePath;
    })
  );

  return files.reduce((all, folderContents) => all.concat(folderContents), []);
}

const uploadContent = async () => {
  const files = await walk(content);
  const body = await Promise.all(
    files
      .filter((filePath) => filePath.endsWith(".org"))
      .map(async (filePath) => {
        return {
          key: `_org_${path.relative(content, filePath).slice(0, -4)}`,
          value: await fs.readFile(filePath, "utf-8"),
        };
      })
  );

  const res = await fetch(
    `https://api.cloudflare.com/client/v4/accounts/${ACCOUNT_ID}/storage/kv/namespaces/${KV_NAMESPACE_ID}/bulk`,
    {
      method: "PUT",
      body: JSON.stringify(body),
      headers: {
        authorization: `Bearer ${CF_TOKEN}`,
        "content-type": "application/json",
      },
    }
  );

  if (res.ok) {
    console.log("Published content");
  } else {
    const json = await res.json();
    throw new Error(JSON.stringify(json));
  }
};

const uploadStatic = async (dir) => {
  const files = await walk(dir);

  const body = await Promise.all(
    files.map(async (filePath) => {
      return {
        key: path.relative(dir, filePath),
        base64: true,
        value: (await fs.readFile(filePath)).toString("base64"),
      };
    })
  );

  const res = await fetch(
    `https://api.cloudflare.com/client/v4/accounts/${ACCOUNT_ID}/storage/kv/namespaces/${KV_NAMESPACE_ID}/bulk`,
    {
      method: "PUT",
      body: JSON.stringify(body),
      headers: {
        authorization: `Bearer ${CF_TOKEN}`,
        "content-type": "application/json",
      },
    }
  );

  if (res.ok) {
    console.log("Published");
  } else {
    const json = await res.json();
    throw new Error(JSON.stringify(json));
  }
};

uploadContent();
uploadStatic(assets);
uploadStatic(dist);
