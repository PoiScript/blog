const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const AssetsPlugin = require("assets-webpack-plugin");
const CssMinimizerPlugin = require("css-minimizer-webpack-plugin");
const { resolve } = require("path");

const { NODE_ENV = "development" } = process.env;

module.exports = {
  mode: NODE_ENV,
  entry: "./index.js",
  context: __dirname,
  output: {
    path: resolve(__dirname, "dist"),
    filename: "[name].[contenthash].js",
    publicPath: "/assets",
  },
  optimization: {
    minimize: true,
    minimizer: [`...`, new CssMinimizerPlugin()],
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
  plugins:
    NODE_ENV === "production"
      ? [
          new AssetsPlugin({
            path: resolve(__dirname, "dist"),
          }),
          new MiniCssExtractPlugin({
            filename: "[name].[contenthash].css",
          }),
        ]
      : [],
};
