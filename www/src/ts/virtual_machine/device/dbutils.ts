import { Connection } from "ic10emu_wasm";
import { DeviceDBConnection } from "../device_db";

const CableNetworkTypes: readonly string[] = Object.freeze(["Power", "Data", "PowerAndData"]);
export function connectionFromDeviceDBConnection(conn: DeviceDBConnection): Connection {
  if (CableNetworkTypes.includes(conn.typ)) {
    return {
      CableNetwork: {
        net: window.VM.vm.ic10vm.defaultNetwork,
        typ: conn.typ
      }
    };
  } else {
    return "Other";
  }
}
