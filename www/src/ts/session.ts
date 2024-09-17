import type {
  ObjectID,
} from "ic10emu_wasm";
import { App } from "./app";

import { openDB, IDBPTransaction } from "idb";
import {
  TypedEventTarget,
  fromJson,
  toJson,
} from "./utils";

import * as presets from "./presets";
import { computed, signal, Signal } from "@lit-labs/preact-signals";
import { SessionDB } from "sessionDB";
import { VMState } from "virtualMachine/state";
const { demoVMState } = presets;

export interface SessionEventMap {
  "sessions-local-update": CustomEvent;
  "session-active-ic": CustomEvent<ObjectID>;
  "session-id-change": CustomEvent<{ old: ObjectID; new: ObjectID }>;
  "session-errors": CustomEvent<ObjectID[]>;
  "session-load": CustomEvent<Session>;
  "active-line": CustomEvent<ObjectID>;
}

export class Session extends TypedEventTarget<SessionEventMap, typeof EventTarget>(EventTarget) {
  private _activeIC: Signal<ObjectID> = signal(null);
  private _activeEditorSession: Signal<ObjectID> = signal(null);
  private _save_timeout?: ReturnType<typeof setTimeout> = undefined;
  private app: App;

  vmState: Signal<VMState> = signal(null);

  constructor(app: App) {
    super();
    this.app = app;
    this.loadFromFragment();

    window.addEventListener("hashchange", (_event) => {
      this.loadFromFragment();
    });
  }

  get activeIC(): Signal<ObjectID> {
    return this._activeIC;
  }

  set activeIC(val: ObjectID) {
    this._activeIC.value = val;
    this.dispatchCustomEvent("session-active-ic", this.activeIC.peek());
  }

  get activeEditorSession(): Signal<ObjectID> {
    return this._activeEditorSession;
  }

  set activeEditorSession(val: ObjectID) {
    this._activeEditorSession.value = val;
  }

  onActiveIc(callback: (e: CustomEvent<ObjectID>) => any) {
    this.addEventListener("session-active-ic", callback);
  }

  onLoad(callback: (e: CustomEvent<Session>) => any) {
    this.addEventListener("session-load", callback);
  }

  _fireOnLoad() {
    this.dispatchCustomEvent("session-load", this);
  }

  onActiveLine(callback: (e: CustomEvent<ObjectID>) => any) {
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
    const vm = await window.VM.get()
    const toSave = { vm: vm.state.vm.value, activeIC: this.activeIC };
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
    const vm = await window.VM.get()
    if (typeof data === "string") {
      this.activeIC = 1;
      await vm.restoreVMState(demoVMState.vm);
    } else if ("programs" in data) {
      this.activeIC = 1;
      await vm.restoreVMState(demoVMState.vm);
    } else if ("vm" in data) {
      const state = data.vm;
      // assign first so it's present when the
      // vm fires events
      this._activeIC.value = data.activeIC;
      await vm.restoreVMState(state);
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
    } else {
      this.load(presets.defaultVMState);
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
      activeIC: this.activeIC.peek(),
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
