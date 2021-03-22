const webpack = require("webpack");
const Watchpack = require("watchpack");
const which = require("which");
const path = require("path");
const { spawn } = require("child_process");

const src = path.resolve(__dirname, "../src");

const watchWasmPack = (opts) => {
  const wp = new Watchpack({
    aggregateTimeout: 1000,
    poll: true,
  });

  runWasmPack(opts);

  wp.watch([], [src]);

  wp.on("aggregated", () => runWasmPack(opts));
};

const runWasmPack = (opts) =>
  new Promise((resolve, reject) => {
    const bin = which.sync("wasm-pack");

    const p = spawn(
      bin,
      [
        "build",
        "--target",
        opts.target,
        ...(opts.dev ? ["--dev"] : []),
        "--",
        "--no-default-features",
        `--features=${opts.features}`,
      ],
      { stdio: "inherit" }
    );

    p.on("close", (code) => {
      if (code === 0) {
        if (opts.onSuccess) {
          opts.onSuccess();
        }
        resolve();
      } else {
        reject(new Error("compilation error"));
      }
    });

    p.on("error", reject);
  });

module.exports = { watchWasmPack, runWasmPack };
