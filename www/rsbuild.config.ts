import { defineConfig } from "@rsbuild/core";
import { pluginTypeCheck } from "@rsbuild/plugin-type-check";
import { pluginImageCompress } from "@rsbuild/plugin-image-compress";

const rspack = require("@rspack/core");
const { CssExtractRspackPlugin } = require("@rspack/core");

const path = require("path");
const commitHash = require("child_process")
  .execSync("git rev-parse --short HEAD")
  .toString()
  .trim();

export default defineConfig({
  output: {
    targets: ["web"],
  },
  source: {
    entry: {
      index: "./src/ts/index.ts",
    },
  },
  tools: {
    rspack: {
      plugins: [
        new rspack.CopyRspackPlugin({
          patterns: [
            // "src/index.html",
            "img/*.png",
            "img/*/*.png",
            // { from: "data/database.json", to: "data" },
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
        new CssExtractRspackPlugin(),
        new rspack.DefinePlugin({
          __COMMIT_HASH__: JSON.stringify(commitHash),
          __BUILD_DATE__: JSON.stringify(new Date()),
        }),
      ],
    },
    swc: {
      jsc: {
        parser: {
          syntax: "typescript",
          // dynamicImport: true,
          decorators: true,
        },
        transform: {
          legacyDecorator: true,
          decoratorMetadata: true,
          // decoratorVersion: "2022-03",
        },
        // target: "es2021",
      },
    },
    htmlPlugin: {
      template: "./src/index.html",
    },
  },
  plugins: [pluginTypeCheck(), pluginImageCompress()],
});
