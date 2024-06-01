import { VMRef, init } from "ic10emu_wasm";
import type {
  StationpediaPrefab,
  ObjectTemplate,
  InternalAtmoInfo,
  ThermalInfo,
  LogicSlotType,
  MemoryAccess,
  LogicType,
  MachineTier,
  RecipeRange,
  Instruction,
  Recipe,
  InstructionPart,
  InstructionPartType,
  GasType,
  TemplateDatabase,
} from "ic10emu_wasm";

import * as Comlink from "comlink";

import prefabDatabase from "./prefabDatabase";
import { parseNumber } from "utils";

export interface PrefabDatabase {
  prefabs: Map<StationpediaPrefab, ObjectTemplate>;
  reagents: Map<
    string,
    {
      Hash: number;
      Unit: string;
      Sources?: Map<StationpediaPrefab, number>;
    }
  >;
  prefabsByHash: Map<number, StationpediaPrefab>;
  structures: StationpediaPrefab[];
  devices: StationpediaPrefab[];
  items: StationpediaPrefab[];
  logicableItems: StationpediaPrefab[];
  suits: StationpediaPrefab[];
  circuitHolders: StationpediaPrefab[];
}

type JsonDBPrefabs = typeof prefabDatabase.prefabs;

// function buildObjectTemplate<K extends keyof JsonDBPrefabs>(
//   template: JsonDBPrefabs[K],
// ): ObjectTemplate {
//   switch (template.templateType) {
//     case "Structure":
//       return {
//         templateType: "Structure",
//         prefab: template.prefab,
//         structure: template.structure,
//         thermal_info:
//           "thermal_info" in template ? template.thermal_info : undefined,
//         internal_atmo_info:
//           "internal_atmo_info" in template
//             ? (template.internal_atmo_info as InternalAtmoInfo)
//             : undefined,
//       };
//     case "StructureSlots":
//       return {
//         templateType: "StructureSlots",
//         prefab: template.prefab,
//         structure: template.structure,
//         thermal_info:
//           "thermal_info" in template ? template.thermal_info : undefined,
//         internal_atmo_info:
//           "internal_atmo_info" in template
//             ? (template.internal_atmo_info as InternalAtmoInfo)
//             : undefined,
//         slots: template.slots.map((slot) => slot),
//       };
//     case "StructureLogic":
//       return {
//         templateType: "StructureLogic",
//         prefab: template.prefab,
//         structure: template.structure,
//         thermal_info:
//           "thermal_info" in template
//             ? (template.thermal_info as ThermalInfo)
//             : undefined,
//         internal_atmo_info:
//           "internal_atmo_info" in template
//             ? (template.internal_atmo_info as InternalAtmoInfo)
//             : undefined,
//         slots: [...template.slots],
//         logic: {
//           logic_slot_types: new Map(
//             Object.entries(template.logic.logic_slot_types).map(
//               ([key, values]) => [
//                 parseInt(key),
//                 new Map(
//                   Object.entries(values).map(([key, val]) => [
//                     key as LogicSlotType,
//                     val as MemoryAccess,
//                   ]),
//                 ),
//               ],
//             ),
//           ),
//           logic_types: new Map(
//             Object.entries(template.logic.logic_types).map(([key, val]) => [
//               key as LogicType,
//               val as MemoryAccess,
//             ]),
//           ),
//           modes:
//             "modes" in template.logic
//               ? new Map(
//                 Object.entries(template.logic.modes).map(([key, val]) => [
//                   parseInt(key),
//                   val,
//                 ]),
//               )
//               : undefined,
//           transmission_receiver: template.logic.transmission_receiver,
//           wireless_logic: template.logic.wireless_logic,
//           circuit_holder: template.logic.circuit_holder,
//         },
//       };
//     case "StructureLogicDevice":
//       return {
//         templateType: "StructureLogicDevice",
//         prefab: template.prefab,
//         structure: template.structure,
//         thermal_info:
//           "thermal_info" in template
//             ? (template.thermal_info as ThermalInfo)
//             : undefined,
//         internal_atmo_info:
//           "internal_atmo_info" in template
//             ? (template.internal_atmo_info as InternalAtmoInfo)
//             : undefined,
//         slots: [...template.slots],
//         logic: {
//           logic_slot_types: new Map(
//             Object.entries(template.logic.logic_slot_types).map(
//               ([key, values]) => [
//                 parseInt(key),
//                 new Map(
//                   Object.entries(values).map(([key, val]) => [
//                     key as LogicSlotType,
//                     val as MemoryAccess,
//                   ]),
//                 ),
//               ],
//             ),
//           ),
//           logic_types: new Map(
//             Object.entries(template.logic.logic_types).map(([key, val]) => [
//               key as LogicType,
//               val as MemoryAccess,
//             ]),
//           ),
//           modes:
//             "modes" in template.logic
//               ? new Map(
//                 Object.entries(template.logic.modes).map(([key, val]) => [
//                   parseInt(key),
//                   val,
//                 ]),
//               )
//               : undefined,
//           transmission_receiver: template.logic.transmission_receiver,
//           wireless_logic: template.logic.wireless_logic,
//           circuit_holder: template.logic.circuit_holder,
//         },
//         device: {
//           connection_list: [...template.device.connection_list],
//           device_pins_length:
//             "device_pins_length" in template.device
//               ? (template.device.device_pins_length as number)
//               : undefined,
//           has_activate_state: template.device.has_activate_state,
//           has_atmosphere: template.device.has_atmosphere,
//           has_color_state: template.device.has_color_state,
//           has_lock_state: template.device.has_lock_state,
//           has_mode_state: template.device.has_mode_state,
//           has_on_off_state: template.device.has_on_off_state,
//           has_open_state: template.device.has_open_state,
//           has_reagents: template.device.has_reagents,
//         },
//       };
//     case "StructureLogicDeviceConsumer":
//       return {
//         templateType: "StructureLogicDeviceConsumer",
//         prefab: template.prefab,
//         structure: template.structure,
//         thermal_info:
//           "thermal_info" in template
//             ? (template.thermal_info as ThermalInfo)
//             : undefined,
//         internal_atmo_info:
//           "internal_atmo_info" in template
//             ? (template.internal_atmo_info as InternalAtmoInfo)
//             : undefined,
//         slots: [...template.slots],
//         logic: {
//           logic_slot_types: new Map(
//             Object.entries(template.logic.logic_slot_types).map(
//               ([key, values]) => [
//                 parseInt(key),
//                 new Map(
//                   Object.entries(values).map(([key, val]) => [
//                     key as LogicSlotType,
//                     val as MemoryAccess,
//                   ]),
//                 ),
//               ],
//             ),
//           ),
//           logic_types: new Map(
//             Object.entries(template.logic.logic_types).map(([key, val]) => [
//               key as LogicType,
//               val as MemoryAccess,
//             ]),
//           ),
//           modes:
//             "modes" in template.logic
//               ? new Map(
//                 Object.entries(template.logic.modes).map(([key, val]) => [
//                   parseInt(key),
//                   val,
//                 ]),
//               )
//               : undefined,
//           transmission_receiver: template.logic.transmission_receiver,
//           wireless_logic: template.logic.wireless_logic,
//           circuit_holder: template.logic.circuit_holder,
//         },
//         device: {
//           connection_list: [...template.device.connection_list],
//           device_pins_length:
//             "device_pins_length" in template.device
//               ? (template.device.device_pins_length as number)
//               : undefined,
//           has_activate_state: template.device.has_activate_state,
//           has_atmosphere: template.device.has_atmosphere,
//           has_color_state: template.device.has_color_state,
//           has_lock_state: template.device.has_lock_state,
//           has_mode_state: template.device.has_mode_state,
//           has_on_off_state: template.device.has_on_off_state,
//           has_open_state: template.device.has_open_state,
//           has_reagents: template.device.has_reagents,
//         },
//         consumer_info: {
//           consumed_resouces: [...template.consumer_info.consumed_resouces],
//           processed_reagents: [...template.consumer_info.processed_reagents],
//         },
//         fabricator_info: undefined,
//       };
//     case "StructureLogicDeviceConsumerMemory":
//       return {
//         templateType: "StructureLogicDeviceConsumerMemory",
//         prefab: template.prefab,
//         structure: template.structure,
//         thermal_info:
//           "thermal_info" in template
//             ? (template.thermal_info as ThermalInfo)
//             : undefined,
//         internal_atmo_info:
//           "internal_atmo_info" in template
//             ? (template.internal_atmo_info as InternalAtmoInfo)
//             : undefined,
//         slots: [...template.slots],
//         logic: {
//           logic_slot_types: new Map(
//             Object.entries(template.logic.logic_slot_types).map(
//               ([key, values]) => [
//                 parseInt(key),
//                 new Map(
//                   Object.entries(values).map(([key, val]) => [
//                     key as LogicSlotType,
//                     val as MemoryAccess,
//                   ]),
//                 ),
//               ],
//             ),
//           ),
//           logic_types: new Map(
//             Object.entries(template.logic.logic_types).map(([key, val]) => [
//               key as LogicType,
//               val as MemoryAccess,
//             ]),
//           ),
//           modes:
//             "modes" in template.logic
//               ? new Map(
//                 Object.entries(template.logic.modes).map(([key, val]) => [
//                   parseInt(key),
//                   val,
//                 ]),
//               )
//               : undefined,
//           transmission_receiver: template.logic.transmission_receiver,
//           wireless_logic: template.logic.wireless_logic,
//           circuit_holder: template.logic.circuit_holder,
//         },
//         device: {
//           connection_list: [...template.device.connection_list],
//           device_pins_length:
//             "device_pins_length" in template.device
//               ? (template.device.device_pins_length as number)
//               : undefined,
//           has_activate_state: template.device.has_activate_state,
//           has_atmosphere: template.device.has_atmosphere,
//           has_color_state: template.device.has_color_state,
//           has_lock_state: template.device.has_lock_state,
//           has_mode_state: template.device.has_mode_state,
//           has_on_off_state: template.device.has_on_off_state,
//           has_open_state: template.device.has_open_state,
//           has_reagents: template.device.has_reagents,
//         },
//         consumer_info: {
//           consumed_resouces: [...template.consumer_info.consumed_resouces],
//           processed_reagents: [...template.consumer_info.processed_reagents],
//         },
//         fabricator_info:
//           "fabricator_info" in template
//             ? {
//               tier: template.fabricator_info.tier as MachineTier,
//               recipes: new Map(
//                 Object.entries(template.fabricator_info.recipes).map(
//                   ([key, val]) => {
//                     const recipe: Recipe = {
//                       tier: val.tier as MachineTier,
//                       time: val.time as number,
//                       energy: val.energy as number,
//                       temperature: val.temperature as RecipeRange,
//                       pressure: val.pressure as RecipeRange,
//                       required_mix: {
//                         rule: val.required_mix.rule as number,
//                         is_any: val.required_mix.is_any as boolean,
//                         is_any_to_remove: val.required_mix
//                           .is_any_to_remove as boolean,
//                         reagents: new Map(
//                           Object.entries(val.required_mix.reagents),
//                         ) as Map<string, number>,
//                       },
//                       count_types: val.count_types,
//                       reagents: new Map(Object.entries(val.reagents)) as Map<
//                         string,
//                         number
//                       >,
//                     };
//
//                     return [key, recipe];
//                   },
//                 ),
//               ),
//             }
//             : undefined,
//         memory: {
//           memory_access: template.memory.memory_access as MemoryAccess,
//           memory_size: template.memory.memory_size,
//           instructions:
//             "instructions" in template.memory
//               ? new Map(
//                 Object.entries(template.memory.instructions).map(
//                   ([key, val]) => {
//                     const instruction: Instruction = {
//                       description: val.description,
//                       description_stripped: val.description_stripped,
//                       typ: val.typ,
//                       value: val.value,
//                       valid: [
//                         val.valid[0],
//                         typeof val.valid[1] === "number"
//                           ? val.valid[1]
//                           : undefined,
//                       ],
//                       parts: val.parts.map((part) => {
//                         const instPart: InstructionPart = {
//                           range: [...part.range],
//                           name: part.name,
//                           typ: part.typ as InstructionPartType,
//                         };
//                         return instPart;
//                       }),
//                     };
//                     return [key, instruction];
//                   },
//                 ),
//               )
//               : undefined,
//         },
//       };
//     case "StructureLogicDeviceMemory":
//       return {
//         templateType: "StructureLogicDeviceMemory",
//         prefab: template.prefab,
//         structure: template.structure,
//         thermal_info:
//           "thermal_info" in template
//             ? (template.thermal_info as ThermalInfo)
//             : undefined,
//         internal_atmo_info:
//           "internal_atmo_info" in template
//             ? (template.internal_atmo_info as InternalAtmoInfo)
//             : undefined,
//         slots: [...template.slots],
//         logic: {
//           logic_slot_types: new Map(
//             Object.entries(template.logic.logic_slot_types).map(
//               ([key, values]) => [
//                 parseInt(key),
//                 new Map(
//                   Object.entries(values).map(([key, val]) => [
//                     key as LogicSlotType,
//                     val as MemoryAccess,
//                   ]),
//                 ),
//               ],
//             ),
//           ),
//           logic_types: new Map(
//             Object.entries(template.logic.logic_types).map(([key, val]) => [
//               key as LogicType,
//               val as MemoryAccess,
//             ]),
//           ),
//           modes:
//             "modes" in template.logic
//               ? new Map(
//                 Object.entries(template.logic.modes).map(([key, val]) => [
//                   parseInt(key),
//                   val,
//                 ]),
//               )
//               : undefined,
//           transmission_receiver: template.logic.transmission_receiver,
//           wireless_logic: template.logic.wireless_logic,
//           circuit_holder: template.logic.circuit_holder,
//         },
//         device: {
//           connection_list: [...template.device.connection_list],
//           device_pins_length:
//             "device_pins_length" in template.device
//               ? (template.device.device_pins_length as number)
//               : undefined,
//           has_activate_state: template.device.has_activate_state,
//           has_atmosphere: template.device.has_atmosphere,
//           has_color_state: template.device.has_color_state,
//           has_lock_state: template.device.has_lock_state,
//           has_mode_state: template.device.has_mode_state,
//           has_on_off_state: template.device.has_on_off_state,
//           has_open_state: template.device.has_open_state,
//           has_reagents: template.device.has_reagents,
//         },
//         memory: {
//           memory_access: template.memory.memory_access as MemoryAccess,
//           memory_size: template.memory.memory_size,
//           instructions:
//             "instructions" in template.memory
//               ? new Map(
//                 Object.entries(template.memory.instructions).map(
//                   ([key, val]) => {
//                     const instruction: Instruction = {
//                       description: val.description,
//                       description_stripped: val.description_stripped,
//                       typ: val.typ,
//                       value: val.value,
//                       valid: [
//                         val.valid[0],
//                         typeof val.valid[1] === "number"
//                           ? val.valid[1]
//                           : undefined,
//                       ],
//                       parts: val.parts.map(
//                         (part: {
//                           range: readonly [number, number];
//                           name: string;
//                           typ: string;
//                         }) => {
//                           const instPart: InstructionPart = {
//                             range: [...part.range],
//                             name: part.name,
//                             typ: part.typ as InstructionPartType,
//                           };
//                           return instPart;
//                         },
//                       ),
//                     };
//                     return [key, instruction];
//                   },
//                 ),
//               )
//               : undefined,
//         },
//       };
//     case "StructureCircuitHolder":
//       return {
//         templateType: "StructureCircuitHolder",
//         prefab: template.prefab,
//         structure: template.structure,
//         thermal_info:
//           "thermal_info" in template
//             ? (template.thermal_info as ThermalInfo)
//             : undefined,
//         internal_atmo_info:
//           "internal_atmo_info" in template
//             ? (template.internal_atmo_info as InternalAtmoInfo)
//             : undefined,
//         slots: [...template.slots],
//         logic: {
//           logic_slot_types: new Map(
//             Object.entries(template.logic.logic_slot_types).map(
//               ([key, values]) => [
//                 parseInt(key),
//                 new Map(
//                   Object.entries(values).map(([key, val]) => [
//                     key as LogicSlotType,
//                     val as MemoryAccess,
//                   ]),
//                 ),
//               ],
//             ),
//           ),
//           logic_types: new Map(
//             Object.entries(template.logic.logic_types).map(([key, val]) => [
//               key as LogicType,
//               val as MemoryAccess,
//             ]),
//           ),
//           modes:
//             "modes" in template.logic
//               ? new Map(
//                 Object.entries(template.logic.modes).map(([key, val]) => [
//                   parseInt(key),
//                   val,
//                 ]),
//               )
//               : undefined,
//           transmission_receiver: template.logic.transmission_receiver,
//           wireless_logic: template.logic.wireless_logic,
//           circuit_holder: template.logic.circuit_holder,
//         },
//         device: {
//           connection_list: [...template.device.connection_list],
//           device_pins_length:
//             "device_pins_length" in template.device
//               ? (template.device.device_pins_length as number)
//               : undefined,
//           has_activate_state: template.device.has_activate_state,
//           has_atmosphere: template.device.has_atmosphere,
//           has_color_state: template.device.has_color_state,
//           has_lock_state: template.device.has_lock_state,
//           has_mode_state: template.device.has_mode_state,
//           has_on_off_state: template.device.has_on_off_state,
//           has_open_state: template.device.has_open_state,
//           has_reagents: template.device.has_reagents,
//         },
//       };
//     case "Item":
//       return {
//         templateType: "Item",
//         prefab: template.prefab,
//         thermal_info:
//           "thermal_info" in template
//             ? (template.thermal_info as ThermalInfo)
//             : undefined,
//         internal_atmo_info:
//           "internal_atmo_info" in template
//             ? (template.internal_atmo_info as InternalAtmoInfo)
//             : undefined,
//         item: {
//           consumable: template.item.consumable,
//           ingredient: template.item.ingredient,
//           max_quantity: template.item.max_quantity,
//           slot_class: template.item.slot_class,
//           sorting_class: template.item.sorting_class,
//           filter_type:
//             "filter_type" in template.item
//               ? template.item.filter_type
//               : undefined,
//           reagents:
//             "reagents" in template.item
//               ? (new Map(Object.entries(template.item.reagents)) as Map<
//                 string,
//                 number
//               >)
//               : undefined,
//         },
//       };
//     case "ItemSlots":
//       return {
//         templateType: "ItemSlots",
//         prefab: template.prefab,
//         thermal_info:
//           "thermal_info" in template
//             ? (template.thermal_info as ThermalInfo)
//             : undefined,
//         internal_atmo_info:
//           "internal_atmo_info" in template
//             ? (template.internal_atmo_info as InternalAtmoInfo)
//             : undefined,
//         item: {
//           consumable: template.item.consumable,
//           ingredient: template.item.ingredient,
//           max_quantity: template.item.max_quantity,
//           slot_class: template.item.slot_class,
//           sorting_class: template.item.sorting_class,
//           filter_type:
//             "filter_type" in template.item
//               ? (template.item.filter_type as GasType)
//               : undefined,
//           reagents:
//             "reagents" in template.item
//               ? (new Map(Object.entries(template.item.reagents)) as Map<
//                 string,
//                 number
//               >)
//               : undefined,
//         },
//         slots: [...template.slots],
//       };
//     case "ItemConsumer":
//       return {
//         templateType: "ItemConsumer",
//         prefab: template.prefab,
//         thermal_info:
//           "thermal_info" in template
//             ? (template.thermal_info as ThermalInfo)
//             : undefined,
//         internal_atmo_info:
//           "internal_atmo_info" in template
//             ? (template.internal_atmo_info as InternalAtmoInfo)
//             : undefined,
//         item: {
//           consumable: template.item.consumable,
//           ingredient: template.item.ingredient,
//           max_quantity: template.item.max_quantity,
//           slot_class: template.item.slot_class,
//           sorting_class: template.item.sorting_class,
//           filter_type:
//             "filter_type" in template.item
//               ? (template.item.filter_type as GasType)
//               : undefined,
//           reagents:
//             "reagents" in template.item
//               ? (new Map(Object.entries(template.item.reagents)) as Map<
//                 string,
//                 number
//               >)
//               : undefined,
//         },
//         slots: [...template.slots],
//         consumer_info: {
//           consumed_resouces: [...template.consumer_info.consumed_resouces],
//           processed_reagents: [...template.consumer_info.processed_reagents],
//         },
//       };
//     case "ItemLogic":
//       return {
//         templateType: "ItemLogic",
//         prefab: template.prefab,
//         thermal_info:
//           "thermal_info" in template
//             ? (template.thermal_info as ThermalInfo)
//             : undefined,
//         internal_atmo_info:
//           "internal_atmo_info" in template
//             ? (template.internal_atmo_info as InternalAtmoInfo)
//             : undefined,
//         item: {
//           consumable: template.item.consumable,
//           ingredient: template.item.ingredient,
//           max_quantity: template.item.max_quantity,
//           slot_class: template.item.slot_class,
//           sorting_class: template.item.sorting_class,
//           filter_type:
//             "filter_type" in template.item
//               ? (template.item.filter_type as GasType)
//               : undefined,
//           reagents:
//             "reagents" in template.item
//               ? (new Map(Object.entries(template.item.reagents)) as Map<
//                 string,
//                 number
//               >)
//               : undefined,
//         },
//         slots: [...template.slots],
//         logic: {
//           logic_slot_types: new Map(
//             Object.entries(template.logic.logic_slot_types).map(
//               ([key, values]) => [
//                 parseInt(key),
//                 new Map(
//                   Object.entries(values).map(([key, val]) => [
//                     key as LogicSlotType,
//                     val as MemoryAccess,
//                   ]),
//                 ),
//               ],
//             ),
//           ),
//           logic_types: new Map(
//             Object.entries(template.logic.logic_types).map(([key, val]) => [
//               key as LogicType,
//               val as MemoryAccess,
//             ]),
//           ),
//           modes:
//             "modes" in template.logic
//               ? new Map(
//                 Object.entries(template.logic.modes).map(([key, val]) => [
//                   parseInt(key),
//                   val,
//                 ]),
//               )
//               : undefined,
//           transmission_receiver: template.logic.transmission_receiver,
//           wireless_logic: template.logic.wireless_logic,
//           circuit_holder: template.logic.circuit_holder,
//         },
//       };
//     case "ItemLogicMemory":
//       return {
//         templateType: "ItemLogicMemory",
//         prefab: template.prefab,
//         thermal_info:
//           "thermal_info" in template
//             ? (template.thermal_info as ThermalInfo)
//             : undefined,
//         internal_atmo_info:
//           "internal_atmo_info" in template
//             ? (template.internal_atmo_info as InternalAtmoInfo)
//             : undefined,
//         item: {
//           consumable: template.item.consumable,
//           ingredient: template.item.ingredient,
//           max_quantity: template.item.max_quantity,
//           slot_class: template.item.slot_class,
//           sorting_class: template.item.sorting_class,
//           filter_type:
//             "filter_type" in template.item
//               ? (template.item.filter_type as GasType)
//               : undefined,
//           reagents:
//             "reagents" in template.item
//               ? (new Map(Object.entries(template.item.reagents)) as Map<
//                 string,
//                 number
//               >)
//               : undefined,
//         },
//         slots: [...template.slots],
//         logic: {
//           logic_slot_types: new Map(
//             Object.entries(template.logic.logic_slot_types).map(
//               ([key, values]) => [
//                 parseInt(key),
//                 new Map(
//                   Object.entries(values).map(([key, val]) => [
//                     key as LogicSlotType,
//                     val as MemoryAccess,
//                   ]),
//                 ),
//               ],
//             ),
//           ),
//           logic_types: new Map(
//             Object.entries(template.logic.logic_types).map(([key, val]) => [
//               key as LogicType,
//               val as MemoryAccess,
//             ]),
//           ),
//           modes:
//             "modes" in template.logic
//               ? new Map(
//                 Object.entries(template.logic.modes).map(([key, val]) => [
//                   parseInt(key),
//                   val,
//                 ]),
//               )
//               : undefined,
//           transmission_receiver: template.logic.transmission_receiver,
//           wireless_logic: template.logic.wireless_logic,
//           circuit_holder: template.logic.circuit_holder,
//         },
//         memory: {
//           memory_access: template.memory.memory_access as MemoryAccess,
//           memory_size: template.memory.memory_size,
//           instructions:
//             "instructions" in template.memory
//               ? new Map(
//                 Object.entries(template.memory.instructions).map(
//                   ([key, val]) => {
//                     const instruction: Instruction = {
//                       description: val.description,
//                       description_stripped: val.description_stripped,
//                       typ: val.typ,
//                       value: val.value,
//                       valid: [
//                         val.valid[0],
//                         typeof val.valid[1] === "number"
//                           ? val.valid[1]
//                           : undefined,
//                       ],
//                       parts: val.parts.map(
//                         (part: {
//                           range: readonly [number, number];
//                           name: string;
//                           typ: string;
//                         }) => {
//                           const instPart: InstructionPart = {
//                             range: [...part.range],
//                             name: part.name,
//                             typ: part.typ as InstructionPartType,
//                           };
//                           return instPart;
//                         },
//                       ),
//                     };
//                     return [key, instruction];
//                   },
//                 ),
//               )
//               : undefined,
//         },
//       };
//     case "ItemCircuitHolder":
//       return {
//         templateType: "ItemCircuitHolder",
//         prefab: template.prefab,
//         thermal_info:
//           "thermal_info" in template
//             ? (template.thermal_info as ThermalInfo)
//             : undefined,
//         internal_atmo_info:
//           "internal_atmo_info" in template
//             ? (template.internal_atmo_info as InternalAtmoInfo)
//             : undefined,
//         item: {
//           consumable: template.item.consumable,
//           ingredient: template.item.ingredient,
//           max_quantity: template.item.max_quantity,
//           slot_class: template.item.slot_class,
//           sorting_class: template.item.sorting_class,
//           filter_type:
//             "filter_type" in template.item
//               ? (template.item.filter_type as GasType)
//               : undefined,
//           reagents:
//             "reagents" in template.item
//               ? (new Map(Object.entries(template.item.reagents)) as Map<
//                 string,
//                 number
//               >)
//               : undefined,
//         },
//         slots: [...template.slots],
//         logic: {
//           logic_slot_types: new Map(
//             Object.entries(template.logic.logic_slot_types).map(
//               ([key, values]) => [
//                 parseInt(key),
//                 new Map(
//                   Object.entries(values).map(([key, val]) => [
//                     key as LogicSlotType,
//                     val as MemoryAccess,
//                   ]),
//                 ),
//               ],
//             ),
//           ),
//           logic_types: new Map(
//             Object.entries(template.logic.logic_types).map(([key, val]) => [
//               key as LogicType,
//               val as MemoryAccess,
//             ]),
//           ),
//           modes:
//             "modes" in template.logic
//               ? new Map(
//                 Object.entries(template.logic.modes).map(([key, val]) => [
//                   parseInt(key),
//                   val,
//                 ]),
//               )
//               : undefined,
//           transmission_receiver: template.logic.transmission_receiver,
//           wireless_logic: template.logic.wireless_logic,
//           circuit_holder: template.logic.circuit_holder,
//         },
//       };
//     case "ItemSuit":
//       return {
//         templateType: "ItemSuit",
//         prefab: template.prefab,
//         thermal_info:
//           "thermal_info" in template
//             ? (template.thermal_info as ThermalInfo)
//             : undefined,
//         internal_atmo_info:
//           "internal_atmo_info" in template
//             ? (template.internal_atmo_info as InternalAtmoInfo)
//             : undefined,
//         item: {
//           consumable: template.item.consumable,
//           ingredient: template.item.ingredient,
//           max_quantity: template.item.max_quantity,
//           slot_class: template.item.slot_class,
//           sorting_class: template.item.sorting_class,
//           filter_type:
//             "filter_type" in template.item
//               ? (template.item.filter_type as GasType)
//               : undefined,
//           reagents:
//             "reagents" in template.item
//               ? (new Map(Object.entries(template.item.reagents)) as Map<
//                 string,
//                 number
//               >)
//               : undefined,
//         },
//         slots: [...template.slots],
//         suit_info: template.suit_info,
//       };
//     // case "ItemSuitLogic":
//     //   return {
//     //     templateType: "ItemSuitLogic",
//     //     prefab: template.prefab,
//     //     thermal_info: "thermal_info" in template ? template.thermal_info as ThermalInfo : undefined,
//     //     internal_atmo_info: "internal_atmo_info" in template ? template.internal_atmo_info as InternalAtmoInfo : undefined,
//     //     item: {
//     //       consumable: template.item.consumable,
//     //       ingredient: template.item.ingredient,
//     //       max_quantity: template.item.max_quantity,
//     //       slot_class: template.item.slot_class,
//     //       sorting_class: template.item.sorting_class,
//     //       filter_type: "filter_type" in template.item ? template.item.filter_type as GasType : undefined,
//     //       reagents: "reagents" in template.item ? new Map(Object.entries(template.item.reagents)) as Map<string, number> : undefined,
//     //     },
//     //     slots: [...template.slots],
//     //     suit_info: template.suit_info,
//     //     logic: {
//     //       logic_slot_types: new Map(
//     //         Object.entries(template.logic.logic_slot_types)
//     //           .map(([key, values]) => [
//     //             parseInt(key),
//     //             new Map(
//     //               Object.entries(values)
//     //                 .map(([key, val]) => [key as LogicSlotType, val as MemoryAccess])
//     //             )
//     //           ])
//     //       ),
//     //       logic_types: new Map(
//     //         Object.entries(template.logic.logic_types)
//     //           .map(([key, val]) => [key as LogicType, val as MemoryAccess])
//     //       ),
//     //       modes: "modes" in template.logic
//     //         ? new Map(
//     //           Object.entries(template.logic.modes).map(([key, val]) => [parseInt(key), val])
//     //         )
//     //         : undefined,
//     //       transmission_receiver: template.logic.transmission_receiver,
//     //       wireless_logic: template.logic.wireless_logic,
//     //       circuit_holder: template.logic.circuit_holder
//     //     },
//     //   }
//     case "ItemSuitCircuitHolder":
//       return {
//         templateType: "ItemSuitCircuitHolder",
//         prefab: template.prefab,
//         thermal_info:
//           "thermal_info" in template
//             ? (template.thermal_info as ThermalInfo)
//             : undefined,
//         internal_atmo_info:
//           "internal_atmo_info" in template
//             ? (template.internal_atmo_info as InternalAtmoInfo)
//             : undefined,
//         item: {
//           consumable: template.item.consumable,
//           ingredient: template.item.ingredient,
//           max_quantity: template.item.max_quantity,
//           slot_class: template.item.slot_class,
//           sorting_class: template.item.sorting_class,
//           filter_type:
//             "filter_type" in template.item
//               ? (template.item.filter_type as GasType)
//               : undefined,
//           reagents:
//             "reagents" in template.item
//               ? (new Map(Object.entries(template.item.reagents)) as Map<
//                 string,
//                 number
//               >)
//               : undefined,
//         },
//         slots: [...template.slots],
//         suit_info: template.suit_info,
//         logic: {
//           logic_slot_types: new Map(
//             Object.entries(template.logic.logic_slot_types).map(
//               ([key, values]) => [
//                 parseInt(key),
//                 new Map(
//                   Object.entries(values).map(([key, val]) => [
//                     key as LogicSlotType,
//                     val as MemoryAccess,
//                   ]),
//                 ),
//               ],
//             ),
//           ),
//           logic_types: new Map(
//             Object.entries(template.logic.logic_types).map(([key, val]) => [
//               key as LogicType,
//               val as MemoryAccess,
//             ]),
//           ),
//           modes:
//             "modes" in template.logic
//               ? new Map(
//                 Object.entries(template.logic.modes).map(([key, val]) => [
//                   parseInt(key),
//                   val,
//                 ]),
//               )
//               : undefined,
//           transmission_receiver: template.logic.transmission_receiver,
//           wireless_logic: template.logic.wireless_logic,
//           circuit_holder: template.logic.circuit_holder,
//         },
//         memory: {
//           memory_access: template.memory.memory_access as MemoryAccess,
//           memory_size: template.memory.memory_size,
//           instructions:
//             "instructions" in template.memory
//               ? new Map(
//                 Object.entries(template.memory.instructions).map(
//                   ([key, val]) => {
//                     const instruction: Instruction = {
//                       description: val.description,
//                       description_stripped: val.description_stripped,
//                       typ: val.typ,
//                       value: val.value,
//                       valid: [
//                         val.valid[0],
//                         typeof val.valid[1] === "number"
//                           ? val.valid[1]
//                           : undefined,
//                       ],
//                       parts: val.parts.map(
//                         (part: {
//                           range: readonly [number, number];
//                           name: string;
//                           typ: string;
//                         }) => {
//                           const instPart: InstructionPart = {
//                             range: [...part.range],
//                             name: part.name,
//                             typ: part.typ as InstructionPartType,
//                           };
//                           return instPart;
//                         },
//                       ),
//                     };
//                     return [key, instruction];
//                   },
//                 ),
//               )
//               : undefined,
//         },
//       };
//     default:
//       return undefined;
//   }
// }
//
// function buildPrefabDatabase(): PrefabDatabase {
//   return {
//     prefabs: new Map(
//       Object.entries(prefabDatabase.prefabs).flatMap(([key, val]) => {
//         const template = buildObjectTemplate(val);
//         if (typeof template !== "undefined") {
//           return [[key as StationpediaPrefab, template]];
//         } else {
//           return [];
//         }
//       }),
//     ),
//     prefabsByHash: new Map(
//       Object.entries(prefabDatabase.prefabsByHash).map(([key, val]) => [
//         parseInt(key),
//         val as StationpediaPrefab,
//       ]),
//     ),
//     structures: [...prefabDatabase.structures] as StationpediaPrefab[],
//     devices: [...prefabDatabase.devices] as StationpediaPrefab[],
//     items: [...prefabDatabase.items] as StationpediaPrefab[],
//     logicableItems: [...prefabDatabase.logicableItems] as StationpediaPrefab[],
//     circuitHolders: [...prefabDatabase.circuitHolders] as StationpediaPrefab[],
//     suits: [...prefabDatabase.suits] as StationpediaPrefab[],
//     reagents: new Map(
//       Object.entries(prefabDatabase.reagents).map(([key, val]) => {
//         return [
//           key,
//           {
//             Hash: val.Hash,
//             Unit: val.Unit,
//             Sources:
//               "Sources" in val
//                 ? (new Map(Object.entries(val.Sources)) as Map<
//                   StationpediaPrefab,
//                   number
//                 >)
//                 : undefined,
//           },
//         ];
//       }),
//     ),
//   };
// }
//
console.info("Processing Json prefab Database ", prefabDatabase);
//
// const prefab_database = buildPrefabDatabase();
//
// console.info("Prcessed prefab Database ", prefab_database);

const vm: VMRef = init();

// const template_database = new Map(
//   Array.from(prefab_database.prefabsByHash.entries()).map(([hash, name]) => {
//     return [hash, prefab_database.prefabs.get(name)];
//   }),
// );

// console.info("Loading Prefab Template Database into VM", template_database);
try {
  const start_time = performance.now();
  // vm.importTemplateDatabase(template_database);
  vm.importTemplateDatabase(
    Object.fromEntries(
      Object.entries(prefabDatabase.prefabsByHash)
        .map(([hash, prefabName]) => [parseInt(hash), prefabDatabase.prefabs[prefabName]])
    ) as TemplateDatabase
  );
  const now = performance.now();
  const time_elapsed = (now - start_time) / 1000;
  console.info(`Prefab Template Database loaded in ${time_elapsed} seconds`);
} catch (e) {
  if ("stack" in e) {
    console.error("Error importing template database:", e.toString(), e.stack);
  } else {
    console.error("Error importing template database:", e.toString());
  }
}

postMessage("ready");

Comlink.expose(vm);
