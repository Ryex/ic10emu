export type DeviceDBEntry = {
  name: string;
  hash: number;
  desc: string;
  logic?: { [key: string]: string };
  slots?: { name: string; typ: string }[];
  modes?: { [key: string]: string };
  conn?: { [key: string]: string[] };
};

export type DeviceDB = {
  logic_enabled: string[];
  slot_logic_enabled: string[];
  devices: string[];
  items: string[];
  strutures: string[];
  db: {
    [key: string]: DeviceDBEntry;
  };
};
