
import { SessionDB } from "../sessionDB";

export const defaultVMState: SessionDB.CurrentDBVmState = {
  vm: {
    objects: [
      {
        obj_info: {
          id: 1,
          prefab: "StructureCircuitHousing",
          socketed_ic: 2,
          slots: new Map([
            [0, { id: 2, quantity: 1 }],
          ]),
          connections: new Map([
            [0, 1],
          ]),
          // unused, provided to make compiler happy
          name: undefined,
          prefab_hash: undefined,
          compile_errors: undefined,
          parent_slot: undefined,
          root_parent_human: undefined,
          damage: undefined,
          device_pins: undefined,
          reagents: undefined,
          logic_values: undefined,
          slot_logic_values: undefined,
          entity: undefined,
          visible_devices: undefined,
          memory: undefined,
          source_code: undefined,
          circuit: undefined,
        },
        template: undefined,
        database_template: true,
      },
      {
        obj_info: {
          id: 2,
          prefab: "ItemIntegratedCircuit10",
          source_code: "",
          memory: new Array(512).fill(0),
          circuit: {
            instruction_pointer: 0,
            yield_instruction_count: 0,
            state: "Start",
            aliases: new Map(),
            defines: new Map(),
            labels: new Map(),
            registers: new Array(18).fill(0),
          },

          // unused, provided to make compiler happy
          name: undefined,
          prefab_hash: undefined,
          compile_errors: undefined,
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
        template: undefined,
        database_template: true,
      },
    ],
    networks: [
      {
        id: 1,
        devices: [1],
        power_only: [],
        channels: Array(8).fill(NaN) as [
          number,
          number,
          number,
          number,
          number,
          number,
          number,
          number,
        ],
      },
    ],
    program_holders: [2],
    circuit_holders: [1],
    default_network_key: 1,
    wireless_receivers: [],
    wireless_transmitters: [],
  },
  activeIC: 1,
};
