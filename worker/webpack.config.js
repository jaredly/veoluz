const path = require("path");
const webpack = require('webpack')
const HtmlWebpackPlugin = require("html-webpack-plugin");

const dist = path.resolve(__dirname, "dist");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: "./js/index.js",
  target: 'webworker',
  mode: 'production',
  output: {
    path: dist,
    filename: "bundle.js"
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new webpack.optimize.LimitChunkCountPlugin({
      maxChunks: 2,
    }),

    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "crate"),
      forceMode: 'production'
    }),
  ]
};
