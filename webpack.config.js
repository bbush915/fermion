const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  devServer: {
    compress: true,
    headers: {
      "Cross-Origin-Embedder-Policy": "require-corp",
      "Cross-Origin-Opener-Policy": "same-origin",
    },
    hot: true,
    port: 8080,
    static: {
      directory: path.join(__dirname, "public"),
    },
  },
  ignoreWarnings: [
    /Circular dependency between chunks with runtime/,
    /exceed the recommended size limit/,
    /combined asset size exceeds the recommended limit/,
  ],
  mode: "production",
  module: {
    rules: [
      {
        test: /\.(html)$/,
        use: ["html-loader"],
      },
      {
        test: /\.ts$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
      {
        test: /\.css$/i,
        use: ["style-loader", "css-loader"],
      },
      {
        test: /\.(png|svg|jpg|jpeg|gif)$/i,
        type: "asset/resource",
      },
    ],
  },
  output: {
    path: path.join(__dirname, "public"),
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: "src/index.html",
      favicon: "src/assets/favicon.ico",
    }),
    new WasmPackPlugin({
      crateDirectory: path.join(__dirname, "crate"),
      extraArgs: "--target web",
      outDir: path.join(__dirname, "crate", "pkg"),
      outName: "fermion-wasm",
    }),
  ],
  resolve: {
    extensions: [".ts", ".js"],
  },
};
