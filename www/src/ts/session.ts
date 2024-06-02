import type {
  ICError,
  FrozenVM,
  RegisterSpec,
  DeviceSpec,
  LogicType,
  LogicSlotType,
  LogicField,
  Class as SlotType,
  FrozenCableNetwork,
  FrozenObject,
  ObjectInfo,
  ICState,
  ObjectID,
} from "ic10emu_wasm";
import { App } from "./app";

import { openDB, DBSchema, IDBPTransaction, IDBPDatabase } from "idb";
import {
  TypedEventTarget,
  crc32,
  dispatchTypedEvent,
  fromJson,
  toJson,
} from "./utils";

import * as presets from "./presets";
const { demoVMState } = presets;

export interface SessionEventMap {
  "sessions-local-update": CustomEvent;
  "session-active-ic": CustomEvent<ObjectID>;
  "session-id-change": CustomEvent<{ old: ObjectID; new: ObjectID }>;
  "session-errors": CustomEvent<ObjectID[]>;
  "session-load": CustomEvent<Session>;
  "active-line": CustomEvent<ObjectID>;
}

export class Session extends TypedEventTarget<SessionEventMap>() {
  private _programs: Map<number, string>;
  private _errors: Map<number, ICError[]>;
  private _activeIC: number;
  private _activeLines: Map<number, number>;
  private _save_timeout?: ReturnType<typeof setTimeout>;
  private _vm_state: FrozenVM;

  private app: App;

  constructor(app: App) {
    super();
    this.app = app;
    this._programs = new Map();
    this._errors = new Map();
    this._save_timeout = undefined;
    this._activeIC = 1;
    this._activeLines = new Map();
    this._vm_state = undefined;
    this.loadFromFragment();

    const that = this;
    window.addEventListener("hashchange", (_event) => {
      that.loadFromFragment();
    });
  }

  get programs(): Map<number, string> {
    return this._programs;
  }

  set programs(programs: Iterable<[number, string]>) {
    this._programs = new Map([...programs]);
    this._fireOnLoad();
  }

  get activeIC() {
    return this._activeIC;
  }

  set activeIC(val: number) {
    this._activeIC = val;
    this.dispatchCustomEvent("session-active-ic", this.activeIC);
  }

  changeID(oldID: number, newID: number) {
    if (this.programs.has(oldID)) {
      this.programs.set(newID, this.programs.get(oldID));
      this.programs.delete(oldID);
    }
    this.dispatchCustomEvent("session-id-change", { old: oldID, new: newID });
  }

  onIDChange(callback: (e: CustomEvent<{ old: number; new: number }>) => any) {
    this.addEventListener("session-id-change", callback);
  }

  onActiveIc(callback: (e: CustomEvent<number>) => any) {
    this.addEventListener("session-active-ic", callback);
  }

  get errors() {
    return this._errors;
  }

  getActiveLine(id: number) {
    return this._activeLines.get(id);
  }

  setActiveLine(id: number, line: number) {
    const last = this._activeLines.get(id);
    if (last !== line) {
      this._activeLines.set(id, line);
      this._fireOnActiveLine(id);
    }
  }

  setProgramCode(id: number, code: string) {
    this._programs.set(id, code);
    if (this.app.vm) {
      this.app.vm.updateCode();
    }
    this.save();
  }

  setProgramErrors(id: number, errors: ICError[]) {
    this._errors.set(id, errors);
    this._fireOnErrors([id]);
  }

  _fireOnErrors(ids: number[]) {
    this.dispatchCustomEvent("session-errors", ids);
  }

  onErrors(callback: (e: CustomEvent<number[]>) => any) {
    this.addEventListener("session-errors", callback);
  }

  onLoad(callback: (e: CustomEvent<Session>) => any) {
    this.addEventListener("session-load", callback);
  }

  _fireOnLoad() {
    this.dispatchCustomEvent("session-load", this);
  }

  onActiveLine(callback: (e: CustomEvent<number>) => any) {
    this.addEventListener("active-line", callback);
  }

  _fireOnActiveLine(id: number) {
    this.dispatchCustomEvent("active-line", id);
  }

  save() {
    if (this._save_timeout) clearTimeout(this._save_timeout);
    this._save_timeout = setTimeout(() => {
      this.saveToFragment();
      this._save_timeout = undefined;
    }, 1000);
  }

  async saveToFragment() {
    const toSave = { vm: this.app.vm.saveVMState(), activeIC: this.activeIC };
    const bytes = new TextEncoder().encode(toJson(toSave));
    try {
      const c_bytes = await compress(bytes, defaultCompression);
      const fragment = base64url_encode(c_bytes);
      window.history.replaceState(null, "", `#${fragment}`);
    } catch (e) {
      console.log("Error compressing content fragment:", e);
      return;
    }
  }

  async load(data: SessionDB.CurrentDBVmState | OldPrograms | string) {
    if (typeof data === "string") {
      this._activeIC = 1;
      this.app.vm.restoreVMState(demoVMState.vm);
      this._programs = new Map([[1, data]]);
    } else if ("programs" in data) {
      this._activeIC = 1;
      this.app.vm.restoreVMState(demoVMState.vm);
      this._programs = new Map(data.programs);
    } else if ("vm" in data) {
      this._programs = new Map();
      const state = data.vm;
      // assign first so it's present when the
      // vm fires events
      this._activeIC = data.activeIC;
      const vm = await window.VM.get()
      await vm.restoreVMState(state);
      this.programs = vm.getPrograms();
      // assign again to fire event
      this.activeIC = data.activeIC;
    }
    this._fireOnLoad();
  }

  async loadFromFragment() {
    const fragment = window.location.hash.slice(1);
    if (fragment === "demo") {
      this.load(demoVMState);
      return;
    }
    if (fragment.length > 0) {
      const c_bytes = base64url_decode(fragment);
      const bytes = await decompressFragment(c_bytes);
      if (bytes !== null) {
        const txt = new TextDecoder().decode(bytes);
        const data = getJson(txt);
        if (data === null) {
          // backwards compatible
          this.load(txt);
          return;
        } else if ("programs" in data) {
          this.load(data as OldPrograms);
          return;
        } else if ("vm" in data && "activeIC" in data) {
          this.load(data as SessionDB.CurrentDBVmState);
        } else {
          console.log("Bad session data:", data);
        }
      }
    }
  }

  async openIndexDB() {
    return await openDB<SessionDB.CurrentDBSchema>(
      "ic10-vm-sessions",
      SessionDB.LOCAL_DB_VERSION,
      {
        async upgrade(db, oldVersion, newVersion, transaction, event) {
          if (oldVersion < SessionDB.DBVersion.V1) {
            const sessionStore = db.createObjectStore("sessions");
            sessionStore.createIndex("by-date", "date");
            sessionStore.createIndex("by-name", "name");
          }
          if (oldVersion < SessionDB.DBVersion.V2) {
            const v1Transaction =
              transaction as unknown as IDBPTransaction<SessionDB.AppDBSchemaV1>;
            const v1SessionStore = v1Transaction.objectStore("sessions");
            const v1Sessions = await v1SessionStore.getAll();
            const v2SessionStore = db.createObjectStore("sessionsV2");
            v2SessionStore.createIndex("by-date", "date");
            v2SessionStore.createIndex("by-name", "name");
            for (const v1Session of v1Sessions) {
              await v2SessionStore.add({
                name: v1Session.name,
                date: v1Session.date,
                version: SessionDB.DBVersion.V2,
                session: SessionDB.V2.fromV1State(v1Session.session),
              });
            }
          }
        },
      },
    );
  }

  async saveLocal(name: string) {
    const state: SessionDB.CurrentDBVmState = {
      vm: await (await window.VM.get()).ic10vm.saveVMState(),
      activeIC: this.activeIC,
    };
    const db = await this.openIndexDB();
    const transaction = db.transaction(
      [SessionDB.LOCAL_DB_SESSION_STORE],
      "readwrite",
    );
    const sessionStore = transaction.objectStore(
      SessionDB.LOCAL_DB_SESSION_STORE,
    );
    await sessionStore.put(
      {
        name,
        date: new Date(),
        version: SessionDB.LOCAL_DB_VERSION,
        session: state,
      },
      name,
    );
    this.dispatchCustomEvent("sessions-local-update");
  }

  async loadFromLocal(name: string) {
    const db = await this.openIndexDB();
    const save = await db.get(SessionDB.LOCAL_DB_SESSION_STORE, name);
    if (typeof save !== "undefined") {
      const { session } = save;
      this.load(session);
    }
  }

  async deleteLocalSave(name: string) {
    const db = await this.openIndexDB();
    const transaction = db.transaction(
      [SessionDB.LOCAL_DB_SESSION_STORE],
      "readwrite",
    );
    const sessionStore = transaction.objectStore(
      SessionDB.LOCAL_DB_SESSION_STORE,
    );
    await sessionStore.delete(name);
    this.dispatchCustomEvent("sessions-local-update");
  }
  async getLocalSaved() {
    const db = await this.openIndexDB();
    const sessions = await db.getAll(SessionDB.LOCAL_DB_SESSION_STORE);
    return sessions;
  }
}

export namespace SessionDB {
  export namespace V1 {
    export interface VMState {
      activeIC: number;
      vm: FrozenVM;
    }

    export interface FrozenVM {
      ics: FrozenIC[];
      devices: DeviceTemplate[];
      networks: FrozenNetwork[];
      default_network: number;
    }

    export interface FrozenNetwork {
      id: number;
      devices: number[];
      power_only: number[];
      channels: number[];
    }
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
        readonly connection: number | undefined;
      };
    };
    export type Alias = RegisterSpec | DeviceSpec;

    export type Aliases = Map<string, Alias>;

    export type Defines = Map<string, number>;

    export type Pins = (number | undefined)[];
    export interface SlotOccupantTemplate {
      id?: number;
      fields: { [key in LogicSlotType]?: LogicField };
    }
    export interface ConnectionCableNetwork {
      CableNetwork: {
        net: number | undefined;
        typ: string;
      };
    }
    export type Connection = ConnectionCableNetwork | "Other";

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
    export interface FrozenIC {
      device: number;
      id: number;
      registers: number[];
      ip: number;
      ic: number;
      stack: number[];
      aliases: Aliases;
      defines: Defines;
      pins: Pins;
      state: string;
      code: string;
    }
  }

  export namespace V2 {
    export interface VMState {
      activeIC: number;
      vm: FrozenVM;
    }

    function objectFromIC(ic: SessionDB.V1.FrozenIC): FrozenObject {
      return {
        obj_info: {
          name: undefined,
          id: ic.id,
          prefab: "ItemIntegratedCircuit10",
          prefab_hash: crc32("ItemIntegratedCircuit10"),
          memory: ic.stack,
          source_code: ic.code,
          compile_errors: undefined,
          circuit: {
            instruction_pointer: ic.ip,
            yield_instruction_count: ic.ic,
            state: ic.state as ICState,
            aliases: Object.fromEntries(ic.aliases.entries()),
            defines: Object.fromEntries(ic.defines.entries()),
            labels: {},
            registers: ic.registers,
          },

          // unused
          slots: undefined,
          damage: undefined,
          device_pins: undefined,
          connections: undefined,
          reagents: undefined,
          logic_values: undefined,
          slot_logic_values: undefined,
          entity: undefined,
          socketed_ic: undefined,
          visible_devices: undefined,
        },
        database_template: true,
        template: undefined,
      };
    }
    function objectsFromV1Template(
      template: SessionDB.V1.DeviceTemplate,
      idFn: () => number,
      socketedIcFn: (id: number) => number | undefined,
    ): FrozenObject[] {
      const slotOccupantsPairs = new Map(
        template.slots.flatMap((slot, index) => {
          if (typeof slot.occupant !== "undefined") {
            return [
              [
                index,
                [
                  {
                    obj_info: {
                      name: undefined,
                      id: slot.occupant.id ?? idFn(),
                      prefab: undefined,
                      prefab_hash: slot.occupant.fields.PrefabHash?.value,
                      damage: slot.occupant.fields.Damage?.value,

                      socketed_ic: undefined,
                      // unused
                      memory: undefined,
                      source_code: undefined,
                      compile_errors: undefined,
                      circuit: undefined,
                      slots: undefined,
                      device_pins: undefined,
                      connections: undefined,
                      reagents: undefined,
                      logic_values: undefined,
                      slot_logic_values: undefined,
                      entity: undefined,
                      visible_devices: undefined,
                    },
                    database_template: true,
                    template: undefined,
                  },
                  slot.occupant.fields.Quantity ?? 1,
                ],
              ],
            ] as [number, [FrozenObject, number]][];
          } else {
            return [] as [number, [FrozenObject, number]][];
          }
        }),
      );
      const frozen: FrozenObject = {
        obj_info: {
          name: template.name,
          id: template.id,
          prefab: template.prefab_name,
          prefab_hash: undefined,
          slots: Object.fromEntries(
            Array.from(slotOccupantsPairs.entries()).map(
              ([index, [obj, quantity]]) => [
                index,
                {
                  quantity,
                  id: obj.obj_info.id,
                },
              ],
            ),
          ),
          socketed_ic: socketedIcFn(template.id),

          logic_values: Object.fromEntries(
            Object.entries(template.fields).map(([key, val]) => {
              return [key, val.value];
            }),
          ) as Record<LogicType, number>,

          // unused
          memory: undefined,
          source_code: undefined,
          compile_errors: undefined,
          circuit: undefined,
          damage: undefined,
          device_pins: undefined,
          connections: undefined,
          reagents: undefined,
          slot_logic_values: undefined,
          entity: undefined,
          visible_devices: undefined,
        },
        database_template: true,
        template: undefined,
      };
      return [
        ...Array.from(slotOccupantsPairs.entries()).map(
          ([_index, [obj, _quantity]]) => obj,
        ),
        frozen,
      ];
    }

    export function fromV1State(v1State: SessionDB.V1.VMState): VMState {
      const highestObjetId = Math.max(
        ...v1State.vm.devices
          .map((device) => device.id ?? -1)
          .concat(v1State.vm.ics.map((ic) => ic.id ?? -1)),
      );
      let nextId = highestObjetId + 1;
      const deviceIcs = new Map(
        v1State.vm.ics.map((ic) => [ic.device, objectFromIC(ic)]),
      );
      const objects = v1State.vm.devices.flatMap((device) => {
        return objectsFromV1Template(
          device,
          () => nextId++,
          (id) => deviceIcs.get(id)?.obj_info.id ?? undefined,
        );
      });
      const vm: FrozenVM = {
        objects,
        circuit_holders: objects.flatMap((obj) =>
          "socketed_ic" in obj.obj_info &&
          typeof obj.obj_info.socketed_ic !== "undefined"
            ? [obj.obj_info.id]
            : [],
        ),
        program_holders: objects.flatMap((obj) =>
          "source_code" in obj.obj_info &&
          typeof obj.obj_info.source_code !== "undefined"
            ? [obj.obj_info.id]
            : [],
        ),
        default_network_key: v1State.vm.default_network,
        networks: v1State.vm.networks as FrozenCableNetwork[],
        wireless_receivers: [],
        wireless_transmitters: [],
      };
      const v2State: VMState = {
        activeIC: v1State.activeIC,
        vm,
      };
      return v2State;
    }
  }

  export enum DBVersion {
    V1 = 1,
    V2 = 2,
  }

  export const LOCAL_DB_VERSION = DBVersion.V2 as const;
  export type CurrentDBSchema = AppDBSchemaV2;
  export type CurrentDBVmState = V2.VMState;
  export const LOCAL_DB_SESSION_STORE = "sessionsV2" as const;

  export interface AppDBSchemaV1 extends DBSchema {
    sessions: {
      key: string;
      value: {
        name: string;
        date: Date;
        session: V1.VMState;
      };
      indexes: {
        "by-date": Date;
        "by-name": string;
      };
    };
  }

  export interface AppDBSchemaV2 extends DBSchema {
    sessions: {
      key: string;
      value: {
        name: string;
        date: Date;
        session: V1.VMState;
      };
      indexes: {
        "by-date": Date;
        "by-name": string;
      };
    };
    sessionsV2: {
      key: string;
      value: {
        name: string;
        date: Date;
        version: DBVersion.V2;
        session: V2.VMState;
      };
      indexes: {
        "by-date": Date;
        "by-name": string;
      };
    };
  }
}

export interface OldPrograms {
  programs: [number, string][];
}

const byteToHex: string[] = [];

for (let n = 0; n <= 0xff; ++n) {
  const hexOctet = n.toString(16).padStart(2, "0");
  byteToHex.push(hexOctet);
}

function bufToHex(arrayBuffer: ArrayBuffer): string {
  const buff = new Uint8Array(arrayBuffer);
  const hexOctets = new Array(buff.length);

  for (let i = 0; i < buff.length; ++i) hexOctets[i] = byteToHex[buff[i]];

  return hexOctets.join("");
}

export type CompressionFormat = "gzip" | "deflate" | "deflate-raw";
const defaultCompression = "gzip";

function guessFormat(bytes: ArrayBuffer): CompressionFormat {
  const header = bufToHex(bytes.slice(0, 8));
  if (
    header.startsWith("789c") ||
    header.startsWith("7801") ||
    header.startsWith("78DA")
  ) {
    return "deflate";
  } else if (header.startsWith("1f8b08")) {
    return "gzip";
  } else {
    return "deflate-raw";
  }
}

async function decompressFragment(c_bytes: ArrayBuffer) {
  try {
    const format = guessFormat(c_bytes);
    console.log("Decompressing fragment with:", format);
    const bytes = await decompress(c_bytes, format);
    return bytes;
  } catch (e) {
    console.log("Error decompressing content fragment:", e);
    return null;
  }
}

function getJson(value: any) {
  try {
    return fromJson(value);
  } catch (_) {
    return null;
  }
}

async function* streamAsyncIterator(stream: ReadableStream) {
  // Get a lock on the stream
  const reader = stream.getReader();

  try {
    while (true) {
      // Read from the stream
      const { done, value } = await reader.read();
      if (done) return;
      yield value;
    }
  } finally {
    reader.releaseLock();
  }
}

function base64url_encode(buffer: ArrayBuffer) {
  return btoa(
    Array.from(new Uint8Array(buffer), (b) => String.fromCharCode(b)).join(""),
  )
    .replace(/\+/g, "-")
    .replace(/\//g, "_")
    .replace(/=+$/, "");
}

function base64url_decode(value: string): ArrayBuffer {
  const m = value.length % 4;
  return Uint8Array.from(
    atob(
      value
        .replace(/-/g, "+")
        .replace(/_/g, "/")
        .padEnd(value.length + (m === 0 ? 0 : 4 - m), "="),
    ),
    (c) => c.charCodeAt(0),
  ).buffer;
}

async function concatUintArrays(arrays: Uint8Array[]) {
  const blob = new Blob(arrays);
  const buffer = await blob.arrayBuffer();
  return new Uint8Array(buffer);
}

async function compress(
  bytes: ArrayBuffer,
  format: CompressionFormat = defaultCompression,
) {
  const s = new Blob([bytes]).stream();
  const cs = s.pipeThrough(new CompressionStream(format));
  const chunks: Uint8Array[] = [];
  for await (const chunk of streamAsyncIterator(cs)) {
    chunks.push(chunk);
  }
  return await concatUintArrays(chunks);
}

async function decompress(
  bytes: ArrayBuffer,
  format: CompressionFormat = defaultCompression,
) {
  const s = new Blob([bytes]).stream();
  const ds = s.pipeThrough(new DecompressionStream(format));
  const chunks: Uint8Array[] = [];
  for await (const chunk of streamAsyncIterator(ds)) {
    chunks.push(chunk);
  }
  return await concatUintArrays(chunks);
}
