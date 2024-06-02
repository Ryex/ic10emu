import { ObjectInfo } from "ic10emu_wasm";
import { SessionDB } from "../session";

export const demoCode = `# Highlighting Demo

# This is a comment

# Hover a define id anywhere to see it's definition
define a_def 10

# Hover HASH("String")'s to see computed crc32
#     hover here    vvvvvvvvvvvvvvvv
define a_hash HASH("This is a String")

# hover over an alias anywhere in the code
# to see it's definition
alias a_var r0
alias a_device d0

# instructions have Auto Completion,
# numeric logic types are identified on hover
s db 12 0
#    ^^
# hover here

# Enums and their values are Known, Hover them!
#        vvvvvvvvvvvvvvvvvv
move r2 LogicType.Temperature
push r2

# same with constants
#       vvvv
move r3 pinf

# Labels are known
main:
l r1 dr15 RatioWater
move r2 100000.001
push r2

# Hover Hash Strings of Known prefab names
# to get their documentation
#             vvvvvvvvvvvvvvv
move r0 HASH("AccessCardBlack")
push r0
beqzal r1 test

# -2045627372 is the crc32 hash of a SolarPanel,
# hover it to see the documentation!
#        vvvvvvvvvv
move r1 -2045627372
jal test
move r1 $FF
push r1
beqzal 0 test
move r1 %1000
push r1
yield
j main

test:
add r15 r15 1
j ra

`;

export const demoVMState: SessionDB.CurrentDBVmState = {
  vm: {
    objects: [
      {
        obj_info: {
          id: 1,
          prefab: "StructureCircuitHousing",
          socketed_ic: 2,
          slots: {
            0: { id: 2, quantity: 1 },
          },
          connections: {
            0: 1,
          },
          // unused, provided to make compiler happy
          name: undefined,
          prefab_hash: undefined,
          compile_errors: undefined,
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
          source_code: demoCode,
          memory: new Array(512).fill(0),
          circuit: {
            instruction_pointer: 0,
            yield_instruction_count: 0,
            state: "Start",
            aliases: {},
            defines: {},
            labels: {},
            registers: new Array(18).fill(0),
          },

          // unused, provided to make compiler happy
          name: undefined,
          prefab_hash: undefined,
          compile_errors: undefined,
          slots: undefined,
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
