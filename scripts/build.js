const webpack = require("webpack");
const which = require("which");
const { spawn } = require("child_process");

// run webpack and expose emitted assets
const runWebpack = () =>
  new Promise((resolve, reject) => {
    const config = require("./webpack.config.prod");

    webpack(config).run((err, stats) => {
      if (err) {
        reject(err);
      } else {
        resolve(Object.keys(stats.compilation.assets));
      }
    });
  });

const runWasmPack = (env) =>
  new Promise((resolve, reject) => {
    const bin = which.sync("wasm-pack");

    const p = spawn(
      bin,
      [
        "build",
        "--target",
        "no-modules",
        "--",
        "--no-default-features",
        "--features=workers",
      ],
      {
        stdio: "inherit",
        env: { ...process.env, ...env },
      }
    );

    p.on("close", (code) => {
      if (code === 0) {
        resolve();
      } else {
        reject(new Error("compilation error"));
      }
    });

    p.on("error", reject);
  });

const main = async () => {
  const assets = await runWebpack();
  await runWasmPack({
    JS_URL: assets
      .filter((s) => s.endsWith(".js"))
      .map((s) => `/assets/${s}`)
      .join(","),
    CSS_URL: assets
      .filter((s) => s.endsWith(".css"))
      .map((s) => `/assets/${s}`)
      .join(","),
  });
};

main();
