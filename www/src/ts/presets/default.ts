
import { SessionDB } from "../sessionDB";
import { genNetwork, objectFromTemplate } from "../database";

export const defaultVMState: SessionDB.CurrentDBVmState = {
  vm: {
    objects: [
      objectFromTemplate("StructureCircuitHousing", {
        id: 1,
        slots: {
          0: {
            quantity: 1,
            occupant: 2
          }
        },
        connections: {
          0: 1
        }
      }),
      objectFromTemplate("ItemIntegratedCircuit10", {
        id: 2,
      }),
    ],
    networks: [
      genNetwork(1, {
        devices: [1],
      })
    ],
    program_holders: [2],
    circuit_holders: [1],
    default_network_key: 1,
    wireless_receivers: [],
    wireless_transmitters: [],
  },
  activeIC: 1,
};
