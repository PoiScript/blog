const path = require("path");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

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
      {
        test: /\.css$/i,
        use: [MiniCssExtractPlugin.loader, "css-loader"],
      },
    ],
  },
  plugins: [new MiniCssExtractPlugin({})],
};
