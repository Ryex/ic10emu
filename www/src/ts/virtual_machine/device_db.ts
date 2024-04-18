import { LogicType, SlotLogicType, SortingClass, SlotType, FieldType, ReagentMode, BatchMode, ConnectionType, ConnectionRole } from "ic10emu_wasm";
export interface DeviceDBItem {
  slotclass: SlotType;
  sorting: SortingClass;
  maxquantity?: number;
  filtertype?: string;
  consumable?: boolean;
  ingredient?: boolean;
  reagents?: { [key: string]: number};
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

export interface DeviceDBEntry {
  name: string;
  hash: number;
  title: string;
  desc: string;
  slots?: { name: string; typ: SlotType }[];
  logic?: { [key in LogicType]?: FieldType };
  slotlogic?: { [key in SlotLogicType]?: number[] };
  modes?: { [key: number]: string };
  conn?: { [key: number]: DeviceDBConnection };
  item?: DeviceDBItem;
  device?: DeviceDBDevice;
};

export interface DBStates {
  activate: boolean;
  color: boolean;
  lock: boolean;
  mode: boolean;
  onoff: boolean;
  open: boolean;
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
  reagent_hashes: { [key: string]: number}
};

