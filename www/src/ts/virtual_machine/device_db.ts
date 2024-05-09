import {
  LogicType,
  LogicSlotType,
  SortingClass,
  SlotType,
  MemoryAccess,
  ReagentMode,
  BatchMode,
  ConnectionType,
  ConnectionRole,
} from "ic10emu_wasm";
export interface DeviceDBItem {
  slotclass: SlotType;
  sorting: SortingClass;
  maxquantity?: number;
  filtertype?: string;
  consumable?: boolean;
  ingredient?: boolean;
  reagents?: { [key: string]: number };
}

export interface DeviceDBDevice {
  states: DBStates;
  reagents: boolean;
  atmosphere: boolean;
  pins?: number;
}

export interface DeviceDBConnection {
  typ: ConnectionType;
  role: ConnectionRole;
  name: string;
}

export interface DeviceDBInstruction {
  typ: string;
  value: number;
  desc: string;
}

export interface DeviceDBMemory {
  size: number;
  sizeDisplay: string;
  access: MemoryAccess
  instructions?: { [key: string]: DeviceDBInstruction };
}

export type MemoryAccess = "Read" | "Write" | "ReadWrite" | "None";


export interface DeviceDBEntry {
  name: string;
  hash: number;
  title: string;
  desc: string;
  slots?: { name: string; typ: SlotType }[];
  logic?: { [key in LogicType]?: MemoryAccess };
  slotlogic?: { [key: number]: {[key in LogicSlotType]?: MemoryAccess }  };
  modes?: { [key: number]: string };
  conn?: { [key: number]: DeviceDBConnection }
  item?: DeviceDBItem;
  device?: DeviceDBDevice;
  transmitter: boolean;
  receiver: boolean;
  memory?: DeviceDBMemory;
}

export interface DBStates {
  activate: boolean;
  color: boolean;
  lock: boolean;
  mode: boolean;
  onoff: boolean;
  open: boolean;
}

export interface DeviceDBReagent {
  Hash: number;
  Unit: string;
  Sources?: { [key: string]: number };
}

export interface DeviceDB {
  logic_enabled: string[];
  slot_logic_enabled: string[];
  devices: string[];
  items: string[];
  structures: string[];
  db: {
    [key: string]: DeviceDBEntry;
  };
  names_by_hash: { [key: number]: string };
  reagents: { [key: string]: DeviceDBReagent };
  enums: { [key: string]: { [key: string]: number } };
}
