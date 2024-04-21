const demoCode = `# Highlighting Demo
# This is a comment

# Hover a define id anywhere to see it's definition
define a_def 10

# Hover HASH("String")'s to see computed crc32
#     hover here    vvvvvvvvvvvvvvvv
define a_hash HASH("This is a String")

# hover over an alias anywhere in the code
# to see it's definition
alias a_var r0
alias a_device d0

# instructions have Auto Completion,
# numeric logic types are identified on hover
s db 12 0
#    ^^
# hover here

# Enums and their values are Known, Hover them!
#        vvvvvvvvvvvvvvvvvv
move r2 LogicType.Temperature
push r2

# same with constants
#       vvvv
move r3 pinf

# Labels are known
main:
l r1 dr15 RatioWater
move r2 100000.001
push r2

# Hover Hash Strings of Known prefab names
# to get their documentation
#             vvvvvvvvvvvvvvv
move r0 HASH("AccessCardBlack")
push r0
beqzal r1 test

# -2045627372 is the crc32 hash of a SolarPanel,
# hover it to see the documentation!
#        vvvvvvvvvv
move r1 -2045627372
jal test
move r1 $FF
push r1
beqzal 0 test
move r1 %1000
push r1
yield
j main

test:
add r15 r15 1
j ra

`;

import type { ICError, FrozenVM } from "ic10emu_wasm";
import { App } from "./app";

import { openDB, DBSchema } from 'idb';
import { fromJson, toJson } from "./utils";

const LOCAL_DB_VERSION = 1;

export class Session extends EventTarget {
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
    this.dispatchEvent(
      new CustomEvent("session-active-ic", { detail: this.activeIC }),
    );
  }

  onActiveIc(callback: EventListenerOrEventListenerObject) {
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
    this.dispatchEvent(
      new CustomEvent("session-errors", {
        detail: ids,
      }),
    );
  }

  onErrors(callback: EventListenerOrEventListenerObject) {
    this.addEventListener("session-errors", callback);
  }

  onLoad(callback: EventListenerOrEventListenerObject) {
    this.addEventListener("session-load", callback);
  }

  _fireOnLoad() {
    this.dispatchEvent(
      new CustomEvent("session-load", {
        detail: this,
      }),
    );
  }

  onActiveLine(callback: EventListenerOrEventListenerObject) {
    this.addEventListener("active-line", callback);
  }

  _fireOnActiveLine(id: number) {
    this.dispatchEvent(
      new CustomEvent("active-line", {
        detail: id,
      }),
    );
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

  async load(data: VMState | OldPrograms | string) {
    if (typeof data === "string") {
      this._programs = new Map([[1, data]]);
    } else if ( "programs" in data) {
      this._programs = new Map(data.programs);
    } else if ( "vm" in data ) {
      this._programs = new Map();
      const state = data.vm;
      // assign first so it's present when the
      // vm fires events
      this._activeIC = data.activeIC;
      this.app.vm.restoreVMState(state);
      this.programs = this.app.vm.getPrograms();
      // assign again to fire event
      this.activeIC = data.activeIC;
    }
    this._fireOnLoad();
  }

  async loadFromFragment() {
    const fragment = window.location.hash.slice(1);
    if (fragment === "demo") {
      this.load(demoCode);
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
          this.load(data as VMState);
        } else {
          console.log("Bad session data:", data);
        }
      }
    }
  }

  async openIndexDB() {
    return await openDB<AppDBSchemaV1>("ic10-vm-sessions", LOCAL_DB_VERSION, {
      upgrade(db, oldVersion, newVersion, transaction, event) {
        // only db verison currently known is v1
        if (oldVersion < 1) {
          const sessionStore = db.createObjectStore('sessions');
          sessionStore.createIndex('by-date', 'date');
          sessionStore.createIndex('by-name', 'name');
        }
      },
    });
  }

  async saveLocal(name: string) {
    const state: VMState = {
      vm: (await window.VM.get()).ic10vm.saveVMState(),
      activeIC: this.activeIC,
    };
    const db = await this.openIndexDB();
    const transaction = db.transaction(['sessions'], "readwrite");
    const sessionStore = transaction.objectStore("sessions");
    await sessionStore.put({
      name,
      date: new Date(),
      session: state,
    }, name);
    this.dispatchEvent(new CustomEvent("sessions-local-update"))
  }

  async loadFromLocal(name: string) {
    const db = await this.openIndexDB();
    const save = await db.get("sessions", name);
    if (typeof save !== "undefined") {
      const { session } = save;
      this.load(session);
    }
  }

  async deleteLocalSave(name: string) {
    const db = await this.openIndexDB();
    const transaction = db.transaction(['sessions'], "readwrite");
    const sessionStore = transaction.objectStore("sessions");
    await sessionStore.delete(name);
    this.dispatchEvent(new CustomEvent("sessions-local-update"))
  }
  async getLocalSaved() {
    const db = await this.openIndexDB();
    const sessions = await db.getAll('sessions');
    return sessions;
  }
}

export interface VMState {
  activeIC: number;
  vm: FrozenVM;
}

interface AppDBSchemaV1 extends DBSchema {
  sessions: {
    key: string;
    value: {
      name: string;
      date: Date;
      session: VMState;
    }
    indexes: {
      'by-date': Date;
      'by-name': string;
    };
  }
}

export interface OldPrograms {
  programs: [number, string][]
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
