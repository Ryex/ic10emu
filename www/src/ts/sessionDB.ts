import type {
  ICError,
  FrozenVM,
  RegisterSpec,
  DeviceSpec,
  LogicType,
  LogicSlotType,
  LogicField,
  Class as SlotType,
  FrozenCableNetwork,
  FrozenObject,
  ObjectInfo,
  ICState,
  ObjectID,
} from "ic10emu_wasm";
import { DBSchema } from "idb";
import { crc32 } from "utils";

export namespace SessionDB {
  export namespace V1 {
    export interface VMState {
      activeIC: number;
      vm: FrozenVM;
    }

    export interface FrozenVM {
      ics: FrozenIC[];
      devices: DeviceTemplate[];
      networks: FrozenNetwork[];
      default_network: number;
    }

    export interface FrozenNetwork {
      id: number;
      devices: number[];
      power_only: number[];
      channels: number[];
    }
    export type RegisterSpec = {
      readonly RegisterSpec: {
        readonly indirection: number;
        readonly target: number;
      };
    };
    export type DeviceSpec = {
      readonly DeviceSpec: {
        readonly device:
          | "Db"
          | { readonly Numbered: number }
          | {
              readonly Indirect: {
                readonly indirection: number;
                readonly target: number;
              };
            };
        readonly connection: number | undefined;
      };
    };
    export type Alias = RegisterSpec | DeviceSpec;

    export type Aliases = Map<string, Alias>;

    export type Defines = Map<string, number>;

    export type Pins = (number | undefined)[];
    export interface SlotOccupantTemplate {
      id?: number;
      fields: { [key in LogicSlotType]?: LogicField };
    }
    export interface ConnectionCableNetwork {
      CableNetwork: {
        net: number | undefined;
        typ: string;
      };
    }
    export type Connection = ConnectionCableNetwork | "Other";

    export interface SlotTemplate {
      typ: SlotType;
      occupant?: SlotOccupantTemplate;
    }

    export interface DeviceTemplate {
      id?: number;
      name?: string;
      prefab_name?: string;
      slots: SlotTemplate[];
      // reagents: { [key: string]: float}
      connections: Connection[];
      fields: { [key in LogicType]?: LogicField };
    }
    export interface FrozenIC {
      device: number;
      id: number;
      registers: number[];
      ip: number;
      ic: number;
      stack: number[];
      aliases: Aliases;
      defines: Defines;
      pins: Pins;
      state: string;
      code: string;
    }
  }

  export namespace V2 {
    export interface VMState {
      activeIC: number;
      vm: FrozenVM;
    }

    function objectFromIC(ic: SessionDB.V1.FrozenIC): FrozenObject {
      return {
        obj_info: {
          name: undefined,
          id: ic.id,
          prefab: "ItemIntegratedCircuit10",
          prefab_hash: crc32("ItemIntegratedCircuit10"),
          memory: ic.stack,
          source_code: ic.code,
          compile_errors: undefined,
          circuit: {
            instruction_pointer: ic.ip,
            yield_instruction_count: ic.ic,
            state: ic.state as ICState,
            aliases: ic.aliases,
            defines: ic.defines,
            labels: new Map(),
            registers: ic.registers,
          },

          // unused
          slots: undefined,
          parent_slot: undefined,
          root_parent_human: undefined,
          damage: undefined,
          device_pins: undefined,
          connections: undefined,
          reagents: undefined,
          logic_values: undefined,
          slot_logic_values: undefined,
          entity: undefined,
          socketed_ic: undefined,
          visible_devices: undefined,
        },
        database_template: true,
        template: undefined,
      };
    }
    function objectsFromV1Template(
      template: SessionDB.V1.DeviceTemplate,
      idFn: () => number,
      socketedIcFn: (id: number) => number | undefined,
    ): FrozenObject[] {
      const slotOccupantsPairs = new Map(
        template.slots.flatMap((slot, index) => {
          if (typeof slot.occupant !== "undefined") {
            return [
              [
                index,
                [
                  {
                    obj_info: {
                      name: undefined,
                      id: slot.occupant.id ?? idFn(),
                      prefab: undefined,
                      prefab_hash: slot.occupant.fields.PrefabHash?.value,
                      damage: slot.occupant.fields.Damage?.value,

                      socketed_ic: undefined,
                      // unused
                      memory: undefined,
                      source_code: undefined,
                      compile_errors: undefined,
                      circuit: undefined,
                      slots: undefined,
                      device_pins: undefined,
                      connections: undefined,
                      reagents: undefined,
                      logic_values: undefined,
                      slot_logic_values: undefined,
                      entity: undefined,
                      visible_devices: undefined,
                    },
                    database_template: true,
                    template: undefined,
                  },
                  slot.occupant.fields.Quantity ?? 1,
                ],
              ],
            ] as [number, [FrozenObject, number]][];
          } else {
            return [] as [number, [FrozenObject, number]][];
          }
        }),
      );
      const frozen: FrozenObject = {
        obj_info: {
          name: template.name,
          id: template.id,
          prefab: template.prefab_name,
          prefab_hash: undefined,
          slots: new Map(
            Array.from(slotOccupantsPairs.entries()).map(
              ([index, [obj, quantity]]) => [
                index,
                {
                  quantity,
                  id: obj.obj_info.id,
                },
              ],
            ),
          ),
          socketed_ic: socketedIcFn(template.id),

          logic_values: new Map(
            Object.entries(template.fields).map(([key, val]) => {
              return [key as LogicType, val.value];
            }),
          ),

          // unused
          memory: undefined,
          source_code: undefined,
          compile_errors: undefined,
          circuit: undefined,
          parent_slot: undefined,
          root_parent_human: undefined,
          damage: undefined,
          device_pins: undefined,
          connections: undefined,
          reagents: undefined,
          slot_logic_values: undefined,
          entity: undefined,
          visible_devices: undefined,
        },
        database_template: true,
        template: undefined,
      };
      return [
        ...Array.from(slotOccupantsPairs.entries()).map(
          ([_index, [obj, _quantity]]) => obj,
        ),
        frozen,
      ];
    }

    export function fromV1State(v1State: SessionDB.V1.VMState): VMState {
      const highestObjetId = Math.max(
        ...v1State.vm.devices
          .map((device) => device.id ?? -1)
          .concat(v1State.vm.ics.map((ic) => ic.id ?? -1)),
      );
      let nextId = highestObjetId + 1;
      const deviceIcs = new Map(
        v1State.vm.ics.map((ic) => [ic.device, objectFromIC(ic)]),
      );
      const objects = v1State.vm.devices.flatMap((device) => {
        return objectsFromV1Template(
          device,
          () => nextId++,
          (id) => deviceIcs.get(id)?.obj_info.id ?? undefined,
        );
      });
      const vm: FrozenVM = {
        objects,
        circuit_holders: objects.flatMap((obj) =>
          "socketed_ic" in obj.obj_info &&
          typeof obj.obj_info.socketed_ic !== "undefined"
            ? [obj.obj_info.id]
            : [],
        ),
        program_holders: objects.flatMap((obj) =>
          "source_code" in obj.obj_info &&
          typeof obj.obj_info.source_code !== "undefined"
            ? [obj.obj_info.id]
            : [],
        ),
        default_network_key: v1State.vm.default_network,
        networks: v1State.vm.networks as FrozenCableNetwork[],
        wireless_receivers: [],
        wireless_transmitters: [],
      };
      const v2State: VMState = {
        activeIC: v1State.activeIC,
        vm,
      };
      return v2State;
    }
  }

  export enum DBVersion {
    V1 = 1,
    V2 = 2,
  }

  export const LOCAL_DB_VERSION = DBVersion.V2 as const;
  export type CurrentDBSchema = AppDBSchemaV2;
  export type CurrentDBVmState = V2.VMState;
  export const LOCAL_DB_SESSION_STORE = "sessionsV2" as const;

  export interface AppDBSchemaV1 extends DBSchema {
    sessions: {
      key: string;
      value: {
        name: string;
        date: Date;
        session: V1.VMState;
      };
      indexes: {
        "by-date": Date;
        "by-name": string;
      };
    };
  }

  export interface AppDBSchemaV2 extends DBSchema {
    sessions: {
      key: string;
      value: {
        name: string;
        date: Date;
        session: V1.VMState;
      };
      indexes: {
        "by-date": Date;
        "by-name": string;
      };
    };
    sessionsV2: {
      key: string;
      value: {
        name: string;
        date: Date;
        version: DBVersion.V2;
        session: V2.VMState;
      };
      indexes: {
        "by-date": Date;
        "by-name": string;
      };
    };
  }
}
