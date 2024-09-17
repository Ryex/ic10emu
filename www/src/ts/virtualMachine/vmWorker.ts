import { VMRef, init } from "ic10emu_wasm";
import type {
  Reagent,
  ObjectTemplate,
} from "ic10emu_wasm";

import * as Comlink from "comlink";

import * as log from "log";

import prefabDatabase from "../database/prefabDatabase";
import { comlinkSpecialJsonTransferHandler } from "utils";

Comlink.transferHandlers.set("SpecialJson", comlinkSpecialJsonTransferHandler);


const vm: VMRef = init();


log.info("Processing Json Database", prefabDatabase);
{
  const start_time = performance.now();
  const template_database = new Map(
    Object.entries(prefabDatabase.prefabsByHash).map(([hash, prefabName]) => [
      parseInt(hash),
      prefabDatabase.prefabs[prefabName] as ObjectTemplate,
    ]),
  );
  log.info("Loading Prefab Template Database into VM", template_database);

  try {
    vm.importTemplateDatabase(template_database);
    const now = performance.now();
    const time_elapsed = (now - start_time) / 1000;
    log.info(`Prefab Template Database loaded in ${time_elapsed} seconds`);
  } catch (e) {
    if ("stack" in e) {
      log.error("Error importing template database:", e.toString(), e.stack);
    } else {
      log.error("Error importing template database:", e.toString());
    }
  }
}

{
  const start_time = performance.now();
  const reagent_database = new Map(
    Object.entries(prefabDatabase.reagents).map(([_name, entry]) => [
      entry.id,
      {
        id: entry.id satisfies number,
        name: entry.name satisfies string,
        hash: entry.hash satisfies number,
        unit: entry.unit satisfies string,
        is_organic: entry.is_organic satisfies boolean,
        sources: new Map(Object.entries(entry.sources))
      } satisfies Reagent
    ])
  )
  log.info("Loading Reagent Database into VM", reagent_database);

  try {
    vm.importReagentDatabase(reagent_database);
    const now = performance.now();
    const time_elapsed = (now - start_time) / 1000;
    log.info(`Prefab Reagent Database loaded in ${time_elapsed} seconds`);
  } catch (e) {
    if ("stack" in e) {
      log.error("Error importing reagent database:", e.toString(), e.stack);
    } else {
      log.error("Error importing reagent database:", e.toString());
    }
  }
}

postMessage("ready");

Comlink.expose(vm);
