import {
  Connection,
  ConnectionInfo,
  CableConnectionType,
} from "ic10emu_wasm";
export function connectionFromConnectionInfo(conn: ConnectionInfo): Connection {
  let connection: Connection = "None";
  if (
    conn.typ === "Power" ||
    conn.typ === "Data" ||
    conn.typ === "PowerAndData"
  ) {
    connection = {
      CableNetwork: {
        net: window.VM.vm.defaultNetwork,
        typ: conn.typ as CableConnectionType,
        role: conn.role,
      },
    };
  } else if (conn.typ === "Pipe") {
    connection = {
      Pipe: {
        role: conn.role,
      },
    };
  } else if (conn.typ === "PipeLiquid") {
    connection = {
      PipeLiquid: {
        role: conn.role,
      },
    };
  } else if (conn.typ === "Chute") {
    connection = {
      Chute: {
        role: conn.role,
      },
    };
  } else if (conn.typ === "Elevator") {
    connection = {
      Elevator: {
        role: conn.role,
      },
    };
  } else if (conn.typ === "LaunchPad") {
    connection = {
      LaunchPad: {
        role: conn.role,
      },
    };
  } else if (conn.typ === "LandingPad") {
    connection = {
      LandingPad: {
        role: conn.role,
      },
    };
  }
  return connection;
}
