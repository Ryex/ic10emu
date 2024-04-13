export type FieldType = "Read" | "Write" | "ReadWrite";

export interface LogicField {
  field_type: FieldType;
  value: number;
}
export type Fields = Map<string, LogicField>;

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
  readonly fields: Fields;
}
export interface Slot {
  readonly typ: SlotType;
  readonly occupant: SlotOccupant | undefined;
  readonly fields: Fields;
}

export type Reagents = Map<string, Map<number, number>>;

export type Connection = { CableNetwork: number } | "Other";

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
export type LogicType = { readonly LogicType: string };
export type SlotLogicType = { readonly SlotLogicType: string };
export type BatchMode = { readonly BatchMode: string };
export type ReagentMode = { readonly ReagentMode: string };
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
  | LogicType
  | SlotLogicType
  | BatchMode
  | ReagentMode
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
  readonly fields: Fields;
  readonly slots: Slot[];
  readonly reagents: Reagents;
  readonly connections: Connection[];
  readonly aliases?: Aliases | undefined;
  readonly defines?: Defines | undefined;
  readonly pins?: Pins;
  readonly program?: Program;
  getSlotFields(slot: number): Fields;
}
