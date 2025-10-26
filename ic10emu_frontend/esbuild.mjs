import * as esbuild from "esbuild";
import { wasmLoader } from "esbuild-plugin-wasm";

import fs from "fs";
import path from "path";


function clearDir(directory) {
  fs.readdir(directory, (err, files) => {
    if (err) throw err;

    for (const file of files) {
      fs.unlink(path.join(directory, file), (err) => {
        if (err) throw err;
      });
    }
  });
}

clearDir("assets/js");

await esbuild.build({
  entryPoints: ["js/ace.ts", "js/lspWorker.ts"],
  bundle: true,
  splitting: true,
  format: "esm",
  outdir: "assets/js",
  plugins: [
    wasmLoader({
      mode: "deferred",
    }),
  ],
});
