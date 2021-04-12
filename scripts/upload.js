const fs = require("fs");
const path = require("path");
const { promisify } = require("util");
const { walk } = require("@nodelib/fs.walk");
const { writeKVPairs } = require("./cloudflare");

const walkAsync = promisify(walk);

const assets = path.resolve(__dirname, "../assets");
const content = path.resolve(__dirname, "../content");

const uploadContent = async () => {
  const entries = await walkAsync(content, {
    entryFilter: (entry) => entry.name.endsWith(".org"),
  });

  const body = entries.map(
    (entry) => (
      console.log(path.relative(content, entry.path)),
      {
        key: path.relative(content, entry.path),
        value: fs.readFileSync(entry.path, "utf-8"),
      }
    )
  );

  await writeKVPairs(JSON.stringify(body));
};

const uploadAssets = async () => {
  const entries = await walkAsync(assets, {
    stats: true,
    entryFilter: (entry) => !entry.stats.isDirectory(),
  });

  const body = entries.map((entry) => ({
    key: path.relative(assets, entry.path),
    base64: true,
    value: fs.readFileSync(entry.path).toString("base64"),
  }));

  await writeKVPairs(JSON.stringify(body));
};

uploadContent();
uploadAssets();
