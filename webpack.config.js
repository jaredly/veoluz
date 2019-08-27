const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
// const { GenerateSW } = require("workbox-webpack-plugin");
const { InjectManifest } = require("workbox-webpack-plugin");

const mainJs = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "docs", "app"),
    filename: "index.js"
  },
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: ["style-loader", "css-loader"]
      }
    ]
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: "index.html"
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "."),
      forceMode: "production"
    }),
    // Have this example work in Edge which doesn't ship `TextEncoder` or
    // `TextDecoder` at this time.
    new webpack.ProvidePlugin({
      TextDecoder: ["text-encoding", "TextDecoder"],
      TextEncoder: ["text-encoding", "TextEncoder"]
    }),
    // new GenerateSW(),
    new InjectManifest({
      swSrc: "./blank-sw.js",
      swDest: "service-worker.js",
      globDirectory: "./docs/app",
      globPatterns: ["./examples/*.json", "./examples/*.png"]
    })
  ],
  mode: "development"
};

const worker = require("./worker/webpack.config");
worker.output.path = path.resolve(__dirname, "docs", "app");
// worker.plugins.push(new GenerateSW({ swDest: "worker-worker.js" }));
worker.plugins.push(
  new InjectManifest({
    swSrc: "./blank-sw.js",
    swDest: "worker-worker.js",
    globDirectory: path.resolve(__dirname, "./docs", "app"),
    globPatterns: ["./examples/*/*.json", "./examples/*/*.png"]
  })
);
module.exports = [mainJs, worker];
