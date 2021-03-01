const webpack = require("webpack");

const { runWasmPack } = require("./wasm-pack");

// run webpack and expose emitted assets
const runWebpack = () =>
  new Promise((resolve, reject) => {
    const config = require("./webpack.config.prod");

    webpack(config, (err, stats) => {
      if (err) {
        reject(err);
      } else {
        resolve(Object.keys(stats.compilation.assets));
      }
    });
  });

const main = async () => {
  const assets = await runWebpack();
  await runWasmPack({
    target: "no-modules",
    features: "workers",
    env: {
      JS_URL: assets.find((s) => s.endsWith(".js")),
      CSS_URL: assets.find((s) => s.endsWith(".css")),
    },
  });
};

main();
