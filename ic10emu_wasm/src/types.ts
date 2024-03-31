type FieldType = 'Read' | 'Write' | 'ReadWrite';

interface LogicField {
    field_type: FieldType,
    value: number,
}
type Fields = Map<string, LogicField>;

type SlotType = 'AccessCard' | 'Appliance' | 'Back' | 'Battery' | 'Blocked' | 'Bottle' | 'Cartridge' | 'Circuitboard' | 'CreditCard' | 'DataDisk' | 'DrillHead' | 'Egg' | 'Entity' | 'Flare' | 'GasCanister' | 'GasFilter' | 'Helmet' | 'Ingot' | 'LiquidBottle' | 'LiquidCanister' | 'Magazine' | 'Ore' | 'Organ' | 'Plant' | 'ProgramableChip' | 'ScanningHead' | 'SensorProcessingUnit' | 'SoundCartridge' | 'Suit' | 'Tool' | 'Torpedo' | 'None';
    
interface Slot {
    typ: SlotType,
    fields: Fields,
}

type Reagents = Map<string, Map<number, number>>;

type Connection = { CableNetwork: number } | 'Other' ;

type Alias = { RegisterSpec: {indirection: number, target: number} } | { DeviceSpec: { device: "Db" | { Numbered: number } | { Indirect: { indirection: number, target: number }  } }, connection: number | undefined };

type Aliases = Map<string, Alias>;

type Defines = Map<string, number>;

type Pins = (number | undefined)[]

export interface DeviceRef {
    readonly fields: Fields | undefined;
    readonly slots: Slot[] | undefined;
    readonly reagents: Reagents | undefined;
    readonly connections: Connection[] | undefined;
    readonly aliases: Aliases | undefined;
    readonly defines: Defines | undefined;
    readonly pins: Pins;
}
