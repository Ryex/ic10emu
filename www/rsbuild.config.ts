import { defineConfig } from "@rsbuild/core";
import { pluginTypeCheck } from "@rsbuild/plugin-type-check";
import { pluginImageCompress } from "@rsbuild/plugin-image-compress";
import { pluginSass } from "@rsbuild/plugin-sass";

const rspack = require("@rspack/core");
const { CssExtractRspackPlugin } = require("@rspack/core");

const path = require("path");
const commitHash = require("child_process")
  .execSync("git rev-parse --short HEAD")
  .toString()
  .trim();

export default defineConfig({
  environments: {
    web: {
      output: {
        target: 'web',
      }
    }
  },
  source: {
    entry: {
      index: path.resolve(__dirname, "./src/ts/index.ts"),
    },
  },
  html: {
    appIcon: {
      icons: [],
    }
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
            {
              from: path.resolve(
                __dirname,
                "../CHANGELOG.md"
              ),
              to: "static/",
            }
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
          decorators: true,
        },
        transform: {
          decoratorMetadata: true,
        },
      },
    },
    htmlPlugin: {
      template: "./src/index.html",
    },
  },
  plugins: [pluginSass(), pluginTypeCheck(), pluginImageCompress()],
});
