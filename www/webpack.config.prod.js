const CopyWebpackPlugin = require("copy-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const CssMinimizerPlugin = require("css-minimizer-webpack-plugin");
const miniCssExtractPlugin = require("mini-css-extract-plugin");
const ForkTsCheckerWebpackPlugin = require("fork-ts-checker-webpack-plugin");
const ImageMinimizerPlugin = require("image-minimizer-webpack-plugin");
const { EsbuildPlugin } = require("esbuild-loader");

const path = require("path");

module.exports = {
  entry: "./src/ts/main.ts",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "main.js",
    clean: true,
  },
  mode: "production",
  devtool: "source-map",
  plugins: [
    new ForkTsCheckerWebpackPlugin(),
    new CopyWebpackPlugin({
      patterns: [
        "img/*.png",
        "img/*/*.png",
        { from: "data/database.json", to: "data" },
        // Copy Shoelace assets to dist/shoelace
        {
          from: path.resolve(
            __dirname,
            "node_modules/@shoelace-style/shoelace/dist/assets",
          ),
          to: "shoelace/assets",
        },
      ],
    }),
    new HtmlWebpackPlugin({ template: "./src/index.html" }),
    new miniCssExtractPlugin(),
  ],
  module: {
    rules: [
      {
        test: /\.[jt]sx?$/,
        exclude: /node_modules/,
        loader: "esbuild-loader",
        options: {
          target: "es2021",
          tsconfig: "./tsconfig.json",
        },
      },
      {
        test: /\.(jpg|png|svg|gif)$/,
        type: "asset/resource",
      },
      {
        test: /\.(scss)$/,
        use: [
          {
            // inject CSS to page
            loader: miniCssExtractPlugin.loader,
          },
          {
            // translates CSS into CommonJS modules
            loader: "css-loader",
          },
          {
            loader: "esbuild-loader",
            options: {
              loader: 'css',
              minify: true,
            },
          },
          {
            // Run postcss actions
            loader: "postcss-loader",
            options: {
              // `postcssOptions` is needed for postcss 8.x;
              // if you use postcss 7.x skip the key
              postcssOptions: {
                // postcss plugins, can be exported to postcss.config.js
                plugins: function () {
                  return [require("autoprefixer")];
                },
              },
            },
          },
          {
            // compiles Sass to CSS
            loader: "sass-loader",
          },
        ],
      },
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
    fallback: {
      crypto: require.resolve("crypto-browserify"),
      buffer: require.resolve("buffer"),
      stream: require.resolve("stream-browserify"),
      vm: require.resolve("vm-browserify"),
    },
  },
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true,
  },
  optimization: {
    minimizer: [
      new EsbuildPlugin({
        target: "es2021", // Syntax to transpile to (see options below for possible values)
        css: true,
      }),
      new ImageMinimizerPlugin({
        minimizer: {
          implementation: ImageMinimizerPlugin.imageminMinify,
          options: {
            // Lossless optimization with custom option
            // Feel free to experiment with options for better result for you
            plugins: [
              ["gifsicle", { interlaced: true }],
              ["jpegtran", { progressive: true }],
              ["optipng", { optimizationLevel: 5 }],
              // Svgo configuration here https://github.com/svg/svgo#configuration
              [
                "svgo",
                {
                  plugins: [
                    {
                      name: "preset-default",
                      params: {
                        overrides: {
                          removeViewBox: false,
                          addAttributesToSVGElement: {
                            params: {
                              attributes: [
                                { xmlns: "http://www.w3.org/2000/svg" },
                              ],
                            },
                          },
                        },
                      },
                    },
                  ],
                },
              ],
            ],
          },
        },
      }),
    ],
  },
};
