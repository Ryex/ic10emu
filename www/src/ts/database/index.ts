import { FrozenObject, ICInfo, LogicType, ObjectID, ObjectTemplate } from "ic10emu_wasm";

import prefabDatabase from "./prefabDatabase";
import { isSome } from "utils";

export type PrefabName = keyof typeof prefabDatabase.prefabs;
export type Prefab<K extends PrefabName> = typeof prefabDatabase.prefabs[K]
export type ReagentName = keyof typeof prefabDatabase.reagents;
export type ReagentHash = (typeof prefabDatabase.reagents)[ReagentName]["hash"]
export type NetworkChannels = [number, number, number, number, number, number, number, number]

export const validCircuitPrefabsNames = ["ItemIntegratedCircuit10"] as const;

type ReadonlyTupleToUnion<T extends readonly unknown[]> = T[number];

export type CircuitPrefabName = ReadonlyTupleToUnion<typeof validCircuitPrefabsNames>;

export type LogicTypeOf<K extends PrefabName> = Prefab<K> extends { logic: { logic_types: {} } } ? keyof Prefab<K>["logic"]["logic_types"] : never;

export interface ObjectFromTemplateOptions<K extends PrefabName> {
  id: ObjectID,
  name?: string,
  logic_values?: Prefab<K> extends { logic: {} } ? Record<LogicTypeOf<K>, number> : never,
  parent?: Prefab<K> extends { item: {} } ? {
    obj: ObjectID,
    slot: number,
  } : never,
  slots?: Prefab<K> extends { slots: {} } ? Record<number, {
    quantity: number,
    occupant: ObjectID
  }> : never,
  connections?: Prefab<K> extends { device: {} } ? Record<number, ObjectID> : never,
  device_pins?: Prefab<K> extends { device: {} } ? Record<number, number> : never,
  reagents?: Prefab<K> extends { device: {} } ? Record<ReagentName, number> : never,
  memory?: Prefab<K> extends { memory: {} } ? number[] | Record<number, number> : never,
  damage?: Prefab<K> extends { item: {} } | { human: {} } ? number : never,
  circuit?: (
    K extends CircuitPrefabName
    ? {
      [TKey in keyof ICInfo]?: (
        ICInfo[TKey] extends Map<infer KType extends string | number | symbol, infer VType>
        ? Record<KType, VType>
        : (ICInfo[TKey])
      )
    }
    : never
  ),
  source_code?: K extends CircuitPrefabName ? string : never,
}

export function objectFromTemplate<K extends PrefabName>(
  prefabName: K,
  options?: ObjectFromTemplateOptions<K>,
): FrozenObject {
  if (!(prefabName in prefabDatabase.prefabs)) {
    return null;
  }
  const template: ObjectTemplate = prefabDatabase.prefabs[prefabName] as ObjectTemplate;
  const frozen: FrozenObject = {
    obj_info: {
      name: options?.name ?? template.prefab.name,
      id: options.id,
      prefab: template.prefab.prefab_name,
      prefab_hash: template.prefab.prefab_hash,
    },
    database_template: true
  };

  if ("item" in template && isSome(options?.parent)) {
    frozen.obj_info.parent_slot = [options.parent.obj, options.parent.slot];

    // root_parent_human: undefined,
  }

  if ("logic" in template && isSome(options?.logic_values)) {
    frozen.obj_info.logic_values = new Map(Object.entries(options?.logic_values ?? {}) as [LogicType, number][]);

    // slot_logic_values: undefined,
  }

  if ("slots" in template && isSome(options?.slots)) {
    frozen.obj_info.slots = new Map();
    for (const [indexStr, slotEntry] of Object.entries(options.slots)) {
      const index = parseInt(indexStr);
      frozen.obj_info.slots.set(index, {
        id: slotEntry.occupant,
        quantity: slotEntry.quantity
      });
    }
  }

  if ("device" in template) {
    if (isSome(options?.connections)) {
      frozen.obj_info.connections = new Map(Object.entries(options.connections).map(([indexStr, net]) => {
        return [parseInt(indexStr), net];
      }));
    }
    if (isSome(options?.device_pins)) {
      frozen.obj_info.device_pins = new Map(Object.entries(options.device_pins).map(([indexStr, obj]) => {
        return [parseInt(indexStr), obj];
      }));
    }
    if (isSome(options?.reagents)) {
      frozen.obj_info.reagents = new Map(Object.entries(options.reagents).map(([reagent, value]: [ReagentName, number]) => {
        return [prefabDatabase.reagents[reagent].hash, value]
      }))
    }

    // visible_devices: undefined,
  }

  if ("memory" in template) {
    const memorySize = template.memory.memory_size;
    frozen.obj_info.memory = Array(memorySize).fill(0);
    if (isSome(options.memory)) {
      if (Array.isArray(options.memory)) {
        for (let i = 0; i < options.memory.length && i < memorySize; i++) {
          frozen.obj_info.memory[i] = options.memory[i];
        }
      } else {
        for (const [indexStr, value] of Object.entries(options.memory)) {
          const index = parseInt(indexStr);
          frozen.obj_info.memory[index] = options.memory[index];
        }
      }
    }
  }

  if ("item" in template && isSome(options.damage)) {
    frozen.obj_info.damage = options.damage;
  }
  if ("human" in template && isSome(options.damage)) {
    frozen.obj_info.damage = options.damage;

    // entity: undefined,
  }

  if (isSome(options?.circuit) || validCircuitPrefabsNames.includes(prefabName as any)) {
    frozen.obj_info.circuit = {
      instruction_pointer: isSome(options?.circuit?.instruction_pointer) ? options.circuit.instruction_pointer : 0,
      yield_instruction_count: isSome(options?.circuit?.yield_instruction_count) ? options.circuit.yield_instruction_count : 0,
      state: isSome(options?.circuit?.state) ? options.circuit.state : "Start",
      aliases: isSome(options?.circuit?.aliases) ? new Map(Object.entries(options.circuit.aliases)) : new Map(),
      defines: isSome(options?.circuit?.defines) ? new Map(Object.entries(options.circuit.defines)) : new Map(),
      labels: isSome(options?.circuit?.labels) ? new Map(Object.entries(options.circuit.labels)) : new Map(),
      registers: isSome(options?.circuit?.registers) ? options.circuit.registers : new Array(18).fill(0),
    }
  }

  if (validCircuitPrefabsNames.includes(prefabName as any) && isSome(options?.source_code)) {
    frozen.obj_info.source_code = options.source_code;
  }

  return frozen;
}

export function genNetwork(id: ObjectID, options: {
  devices?: ObjectID[],
  power_only?: ObjectID[],
  channels?: NetworkChannels,
}) {
  const net = {
    id: 1,
    devices: options.devices ?? [],
    power_only: options.power_only ?? [],
    channels: options.channels ?? Array(8).fill(NaN) as NetworkChannels,
  };
  return net;
}

export { prefabDatabase };
