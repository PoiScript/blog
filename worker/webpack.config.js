const { resolve } = require("path");

const { NODE_ENV = "development" } = process.env;

module.exports = {
  mode: NODE_ENV,
  entry: "./index.ts",
  target: false,
  context: __dirname,
  output: {
    path: resolve(__dirname, "dist"),
    filename: "worker.js",
  },
  resolve: {
    extensions: [".ts", ".js"],
  },
  module: {
    rules: [
      {
        test: /\.ts$/i,
        use: "ts-loader",
      },
    ],
  },
};
