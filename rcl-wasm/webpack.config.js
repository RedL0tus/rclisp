const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true
  },
  devServer: {
    static: {
      directory: path.join(__dirname, 'static'),
    },
    compress: true,
    port: 9000,
  },
  module : {
    rules : [
      {
        test : /\.css$/i,
        use : [ 'style-loader', 'css-loader' ],
      },
    ]
  },
  performance: {
    maxEntrypointSize: 512000,
    maxAssetSize: 512000
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        { from: "static/CNAME", to: "" },
      ]
    }),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ]
};
