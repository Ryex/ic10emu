export type FieldType = "Read" | "Write" | "ReadWrite";

export  interface LogicField {
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

export interface Slot {
  typ: SlotType;
  fields: Fields;
}

export type Reagents = Map<string, Map<number, number>>;

export type Connection = { CableNetwork: number } | "Other";

export type Alias =
  | { RegisterSpec: { indirection: number; target: number } }
  | {
      DeviceSpec: {
        device:
          | "Db"
          | { Numbered: number }
          | { Indirect: { indirection: number; target: number } };
      };
      connection: number | undefined;
    };

export type Aliases = Map<string, Alias>;

export type Defines = Map<string, number>;

export type Pins = (number | undefined)[];

export type Operand =
  | { RegisterSpec: { indirection: number; target: number } }
  | {
      DeviceSpec: {
        device:
          | "Db"
          | { Numbered: number }
          | { Indirect: { indirection: number; target: number } };
      };
      connection: number | undefined;
    }
  | {
      Number:
        | { Float: number }
        | { Binary: number }
        | { Hexadecimal: number }
        | { Constant: number }
        | { String: string }
        | { Enum: number };
    }
  | { LogicType: string }
  | { SlotLogicType: string }
  | { BatchMode: string }
  | { ReagentMode: string }
  | { Identifier: { name: string } };

export interface Instruction {
  instruction: string;
  operands: Operand[];
}

export type ICError = {
  ParseError: { line: number; start: number; end: number; msg: string };
};

export interface Program {
  instructions: Instruction[];
  errors: ICError[];
  labels: Map<string, number>;
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
}
