const path = require("path");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "../dist");
const web = path.resolve(__dirname, "../web");

module.exports = {
  mode: "development",
  entry: "./index.js",
  context: web,
  output: {
    path: dist,
    filename: "[name].js",
    publicPath: "/assets",
  },
  devServer: {
    contentBase: dist,
  },
  module: {
    rules: [
      {
        test: /\.less$/i,
        use: [MiniCssExtractPlugin.loader, "css-loader", "less-loader"],
      },
    ],
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname,
      outName: "solomon",
      extraArgs: "--target nodejs -- --no-default-features --features=nodejs",
    }),
    new MiniCssExtractPlugin({}),
  ],
};
