import { defineConfig } from "@rspack/cli";
import { rspack } from "@rspack/core";
import { TsCheckerRspackPlugin } from 'ts-checker-rspack-plugin';

// Target browsers, see: https://github.com/browserslist/browserslist
const targets = ["last 2 versions", "> 0.2%",  "not dead",  "Firefox ESR"];

export default defineConfig({
	entry: {
		ace: "./js/ace.ts"
	},
	resolve: {
    extensions: ['.ts', '.tsx', '.js'],
  },
	module: {
		rules: [
			{
				test: /\.svg$/,
				type: "asset"
			},
			{
				test: /\.js$/,
				use: [
					{
						loader: "builtin:swc-loader",
						options: {
							jsc: {
								parser: {
									syntax: "ecmascript"
								}
							},
							env: { targets }
						}
					}
				]
			},
			{
				test: /\.tsx?$/,
				use: [
					{
						loader: "builtin:swc-loader",
						exclude: [/node_modules/],
						options: {
							jsc: {
								experimental: {
             			keepImportAttributes: true,
            		},
								parser: {
									syntax: "typescript"
								}
							},
							env: { targets }
						},
						type: 'javascript/auto',
					}
				]
			}
		]
	},
	plugins: [new TsCheckerRspackPlugin()],
	optimization: {
		minimizer: [
			new rspack.SwcJsMinimizerRspackPlugin(),

		]
	},
	experiments: {
		css: true
	}
});
