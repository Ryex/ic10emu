export type FieldType = "Read" | "Write" | "ReadWrite";

export interface LogicField {
  field_type: FieldType;
  value: number;
}
export type LogicFields = Map<LogicType, LogicField>;
export type SlotLogicFields = Map<SlotLogicType, LogicField>;

export type SlotType =
  | "AccessCard"
  | "Appliance"
  | "Back"
  | "Battery"
  | "Blocked"
  | "Bottle"
  | "Cartridge"
  | "Circuitboard"
  | "CreditCard"
  | "DataDisk"
  | "DrillHead"
  | "Egg"
  | "Entity"
  | "Flare"
  | "GasCanister"
  | "GasFilter"
  | "Helmet"
  | "Ingot"
  | "LiquidBottle"
  | "LiquidCanister"
  | "Magazine"
  | "Ore"
  | "Organ"
  | "Plant"
  | "ProgramableChip"
  | "ScanningHead"
  | "SensorProcessingUnit"
  | "SoundCartridge"
  | "Suit"
  | "Tool"
  | "Torpedo"
  | "None";

export interface SlotOccupant {
  readonly id: number;
  readonly prefab_hash: number;
  readonly quantity: number;
  readonly max_quantity: number;
  readonly damage: number;
  readonly fields: SlotLogicFields;
}
export interface Slot {
  readonly typ: SlotType;
  readonly occupant: SlotOccupant | undefined;
  readonly fields: SlotLogicFields;
}

export type Reagents = Map<string, Map<number, number>>;

export interface ConnectionCableNetwork {
  CableNetwork: {
    net: number | undefined;
    typ: string;
  };
}

export type Connection = ConnectionCableNetwork | "Other";

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
  };
  readonly connection: number | undefined;
};
export type OperandLogicType = { readonly LogicType: string };
export type OperandSlotLogicType = { readonly SlotLogicType: string };
export type OperandBatchMode = { readonly BatchMode: string };
export type OperandReagentMode = { readonly ReagentMode: string };
export type Identifier = { readonly Identifier: { name: string } };

export type NumberFloat = { readonly Float: number };
export type NumberBinary = { readonly Binary: number };
export type NumberHexadecimal = { readonly Hexadecimal: number };
export type NumberConstant = { readonly Constant: number };
export type NumberString = { readonly String: string };
export type NumberEnum = { readonly Enum: number };

export type NumberOperand = {
  Number:
    | NumberFloat
    | NumberBinary
    | NumberHexadecimal
    | NumberConstant
    | NumberString
    | NumberEnum;
};
export type Operand =
  | RegisterSpec
  | DeviceSpec
  | NumberOperand
  | OperandLogicType
  | OperandSlotLogicType
  | OperandBatchMode
  | OperandReagentMode
  | Identifier;

export type Alias = RegisterSpec | DeviceSpec;

export type Aliases = Map<string, Alias>;

export type Defines = Map<string, number>;

export type Pins = (number | undefined)[];

export interface Instruction {
  readonly instruction: string;
  readonly operands: Operand[];
}

export type ICError = {
  readonly ParseError: {
    readonly line: number;
    readonly start: number;
    readonly end: number;
    readonly msg: string;
  };
};

export interface Program {
  readonly instructions: Instruction[];
  readonly errors: ICError[];
  readonly labels: Map<string, number>;
}

export interface DeviceRef {
  readonly fields: LogicFields;
  readonly slots: Slot[];
  readonly reagents: Reagents;
  readonly connections: Connection[];
  readonly aliases?: Aliases | undefined;
  readonly defines?: Defines | undefined;
  readonly pins?: Pins;
  readonly program?: Program;
  getSlotFields(slot: number): SlotLogicFields;
}

export interface SlotOccupantTemplate {
  id?: number;
  fields: { [key in SlotLogicType]?: LogicField };
}

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

export interface VM {
  addDeviceFromTemplate(template: DeviceTemplate): number;
}
