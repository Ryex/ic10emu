import { VMState } from "../session";

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

export const demoVMState: VMState = {
  vm: {
    ics: [
      {
        device: 1,
        id: 2,
        registers: Array(18).fill(0),
        ip: 0,
        ic: 0,
        stack: Array(512).fill(0),
        aliases: new Map(),
        defines: new Map(),
        pins: Array(6).fill(undefined),
        state: "Start",
        code: demoCode,
      },
    ],
    devices: [
      {
        id: 1,
        prefab_name: "StructureCircuitHousing",
        slots: [
          {
            typ: "ProgrammableChip",
            occupant: {
              id: 2,
              fields: {
                "Quantity":{
                  field_type: "Read",
                  value: 1
                },
                "MaxQuantity": {
                  field_type: "Read",
                  value: 1,
                },
                "SortingClass": {
                  field_type: "Read",
                  value: 0,
                }
              },
            },
          },
        ],
        connections: [
          {
            CableNetwork: {
              net: 1,
              typ: "Data",
            },
          },
          {
            CableNetwork: {
              net: undefined,
              typ: "Power",
            },
          },
        ],
        fields: {},
      },
    ],
    networks: [
      {
        id: 1,
        devices: [1],
        power_only: [],
        channels: Array(8).fill(NaN),
      },
    ],
    default_network: 1,
  },
  activeIC: 1,
};
