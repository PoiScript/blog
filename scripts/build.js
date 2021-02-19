const webpack = require("webpack");

const { runWasmPack } = require("./wasm-pack");

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

const main = async () => {
  const assets = await runWebpack();
  await runWasmPack({
    target: "no-modules",
    features: "workers",
    env: {
      JS_URL: assets.filter((s) => s.endsWith(".js")).join(","),
      CSS_URL: assets.filter((s) => s.endsWith(".css")).join(","),
    },
  });
};

main();
