import { VMRef, init } from "ic10emu_wasm";
import type {
  TemplateDatabase,
} from "ic10emu_wasm";

import * as Comlink from "comlink";

import prefabDatabase from "./prefabDatabase";
import { parseNumber } from "utils";


console.info("Processing Json prefab Database ", prefabDatabase);

const vm: VMRef = init();


const template_database = Object.fromEntries(
  Object.entries(prefabDatabase.prefabsByHash).map(([hash, prefabName]) => [
    parseInt(hash),
    prefabDatabase.prefabs[prefabName],
  ]),
) as TemplateDatabase;

try {
  console.info("Loading Prefab Template Database into VM", template_database);
  const start_time = performance.now();
  // vm.importTemplateDatabase(template_database);
  vm.importTemplateDatabase(template_database);
  const now = performance.now();
  const time_elapsed = (now - start_time) / 1000;
  console.info(`Prefab Template Database loaded in ${time_elapsed} seconds`);
} catch (e) {
  if ("stack" in e) {
    console.error("Error importing template database:", e.toString(), e.stack);
  } else {
    console.error("Error importing template database:", e.toString());
  }
  console.info(JSON.stringify(template_database));
}
postMessage("ready");

Comlink.expose(vm);
