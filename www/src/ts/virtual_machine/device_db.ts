import { LogicType, SlotLogicType } from "ic10emu_wasm";

export type SortingClass =
  | "Default"
  | "Kits"
  | "Tools"
  | "Resources"
  | "Food"
  | "Clothing"
  | "Appliances"
  | "Atmospherics"
  | "Storage"
  | "Ores"
  | "Ices";
export type SlotClass =
  | "None"
  | "Helmet"
  | "Suit"
  | "Back"
  | "GasFilter"
  | "GasCanister"
  | "Motherboard"
  | "Circuitboard"
  | "DataDisk"
  | "Organ"
  | "Ore"
  | "Plant"
  | "Uniform"
  | "Entity"
  | "Battery"
  | "Egg"
  | "Belt"
  | "Tool"
  | "Appliance"
  | "Ingot"
  | "Torpedo"
  | "Cartridge"
  | "AccessCard"
  | "Magazine"
  | "Circuit"
  | "Bottle"
  | "ProgrammableChip"
  | "Glasses"
  | "CreditCard"
  | "DirtCanister"
  | "SensorProcessingUnit"
  | "LiquidCanister"
  | "LiquidBottle"
  | "Wreckage"
  | "SoundCartridge"
  | "DrillHead"
  | "ScanningHead"
  | "Flare"
  | "Blocked";
export type NetworkType =
  | "None"
  | "Pipe"
  | "Power"
  | "Data"
  | "Chute"
  | "Elevator"
  | "PipeLiquid"
  | "LandingPad"
  | "LaunchPad"
  | "PowerAndData"
  | "All";
export type ConnectionRole =
  | "None"
  | "Input"
  | "Input2"
  | "Output"
  | "Output2"
  | "Waste";

export type FieldType = "Read" | "Write" | "ReadWrite";

export type ReagentMode = "Contents" | "Recipe" | "Required" | "TotalContents";

export type BatchMode = "Average" | "Maximum" | "Minimum" | "Sum";

export interface DeviceDBItem {
  slotclass: SlotClass;
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
  typ: NetworkType;
  role: ConnectionRole;
  name: string;
}

export interface DeviceDBEntry {
  name: string;
  hash: number;
  title: string;
  desc: string;
  slots?: { name: string; typ: SlotClass }[];
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

