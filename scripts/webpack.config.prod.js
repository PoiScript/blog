const path = require("path");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

const dist = path.resolve(__dirname, "../dist");
const web = path.resolve(__dirname, "../web");

module.exports = {
  mode: "production",
  entry: "./index.js",
  context: web,
  output: {
    path: dist,
    filename: "[name].[contenthash].js",
    publicPath: "/assets",
  },
  module: {
    rules: [
      {
        test: /\.less$/i,
        use: [MiniCssExtractPlugin.loader, "css-loader", "less-loader"],
      },
      {
        test: /\.css$/i,
        use: [MiniCssExtractPlugin.loader, "css-loader"],
      },
    ],
  },
  plugins: [
    new MiniCssExtractPlugin({
      filename: "[name].[contenthash].css",
    }),
  ],
};
