import { VMRef, init } from "ic10emu_wasm";
import type { StationpediaPrefab, ObjectTemplate } from "ic10emu_wasm";

import * as Comlink from "comlink";

import * as json_database from "../../../data/database.json" with { type: "json" };

export interface PrefabDatabase {
  prefabs: { [key in StationpediaPrefab]: ObjectTemplate};
  reagents: {
    [key: string]: {
      Hash: number;
      Unit: string;
      Sources?: {
        [key in StationpediaPrefab]: number;
      };
    };
  };
  prefabsByHash: {
    [key: number]: StationpediaPrefab;
  };
  structures: StationpediaPrefab[];
  devices: StationpediaPrefab[];
  items: StationpediaPrefab[];
  logicableItems: StationpediaPrefab[];
  suits: StationpediaPrefab[];
  circuitHolders: StationpediaPrefab[];
}

const prefab_database = json_database as unknown as PrefabDatabase;

const vm: VMRef = init();

const template_database = new Map(
  Object.entries(prefab_database.prefabsByHash).map(([hash, name]) => {
    return [parseInt(hash), prefab_database.prefabs[name]];
  }),
);

console.info("Loading Prefab Template Database into VM", template_database);
const start_time = performance.now();
vm.importTemplateDatabase(template_database);
const now = performance.now();
const time_elapsed = (now - start_time) / 1000;
console.log(`Prefab Templat Database loaded in ${time_elapsed} seconds`);

Comlink.expose(vm);
