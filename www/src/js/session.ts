
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

`

interface SessionCbFn {
  (param: Session): void;
}

class Session extends EventTarget {
  _programs: Map<number, string>;
  _onLoadCallbacks: SessionCbFn[];
  _activeSession: number;
  _activeLines: Map<number, number>;
  _onActiveLineCallbacks: SessionCbFn[];
  _activeLine: number;
  private _save_timeout: ReturnType<typeof setTimeout>;
  constructor() {
    super();
    this._programs = new Map();
    this._save_timeout = null;
    this._onLoadCallbacks = [];
    this._activeSession = 0;
    this._activeLines = new Map();
    this._onActiveLineCallbacks = [];
    this.loadFromFragment();

    const that = this;
    window.addEventListener('hashchange', (_event) => {
      that.loadFromFragment();
    });
  }

  get programs() {
    return this._programs;
  }

  set programs(programs) {
    this._programs = new Map([...programs]);
  }

  get activeSession() {
    return this._activeSession;
  }

  getActiveLine(id: number) {
    return this._activeLines.get(id);
  }

  setActiveLine(id: number, line: number) {
    this._activeLines.set(id, line);
    this._fireOnActiveLine();
  }

  set activeLine(line: number) {
    this._activeLine = line;
  }

  setProgramCode(id: number, code: string) {
    this._programs.set(id, code);
    this.save();
  }

  onLoad(callback: SessionCbFn) {
    this._onLoadCallbacks.push(callback);
  }

  _fireOnLoad() {
    for (const callback of this._onLoadCallbacks) {
      callback(this);
    }
  }

  onActiveLine(callback: SessionCbFn) {
    this._onActiveLineCallbacks.push(callback);
  }

  _fireOnActiveLine() {
    for (const callback of this._onActiveLineCallbacks) {
      callback(this);
    }
  }

  save() {
    if (this._save_timeout) clearTimeout(this._save_timeout);
    this._save_timeout = setTimeout(() => {
      this.saveToFragment();
      if (window.App.vm) {
        window.App.vm.updateCode();
      }
      this._save_timeout = null;
    }, 1000);
  }

  async saveToFragment() {
    const toSave = { programs: Array.from(this._programs) };
    const bytes = new TextEncoder().encode(JSON.stringify(toSave));
    try {
      const c_bytes = await compress(bytes);
      const fragment = base64url_encode(c_bytes);
      window.history.replaceState(null, "", `#${fragment}`);
    } catch (e) {
      console.log("Error compressing content fragment:", e);
      return;
    }

  }

  async loadFromFragment() {
    const fragment = window.location.hash.slice(1);
    if (fragment === "demo") {
      this._programs = new Map([[0, demoCode]]);
      this._fireOnLoad();
      return;
    }
    if (fragment.length > 0) {
      const c_bytes = base64url_decode(fragment);
      const bytes = await decompressFragment(c_bytes);
      if (bytes !== null) {
        const txt = new TextDecoder().decode(bytes);
        const data = getJson(txt);
        if (data === null) { // backwards compatible
          this._programs = new Map([[0, txt]]);
          this, this._fireOnLoad();
          return;
        }
        try {
          this._programs = new Map(data.programs);
          this._fireOnLoad();
          return;
        } catch (e) {
          console.log("Bad session data:", e);
        }
      }
    }
  }

}
async function decompressFragment(c_bytes: ArrayBuffer) {
  try {
    const bytes = await decompress(c_bytes);
    return bytes;
  } catch (e) {
    console.log("Error decompressing content fragment:", e);
    return null;
  }
}

function getJson(value: any) {
  try {
    return JSON.parse(value);
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
  }
  finally {
    reader.releaseLock();
  }
}

function base64url_encode(buffer: ArrayBuffer) {
  return btoa(Array.from(new Uint8Array(buffer), b => String.fromCharCode(b)).join(''))
    .replace(/\+/g, '-')
    .replace(/\//g, '_')
    .replace(/=+$/, '');
}

function base64url_decode(value: string): ArrayBuffer {
  const m = value.length % 4;
  return Uint8Array.from(atob(
    value.replace(/-/g, '+')
      .replace(/_/g, '/')
      .padEnd(value.length + (m === 0 ? 0 : 4 - m), '=')
  ), c => c.charCodeAt(0)).buffer
}

async function concatUintArrays(arrays: Uint8Array[]) {
  const blob = new Blob(arrays);
  const buffer = await blob.arrayBuffer();
  return new Uint8Array(buffer);
}

async function compress(bytes: ArrayBuffer) {
  const s = new Blob([bytes]).stream();
  const cs = s.pipeThrough(
    new CompressionStream('deflate-raw')
  );
  const chunks = [];
  for await (const chunk of streamAsyncIterator(cs)) {
    chunks.push(chunk);
  }
  return await concatUintArrays(chunks);
}

async function decompress(bytes: ArrayBuffer) {
  const s = new Blob([bytes]).stream();
  const ds = s.pipeThrough(
    new DecompressionStream('deflate-raw')
  );
  const chunks = [];
  for await (const chunk of streamAsyncIterator(ds)) {
    chunks.push(chunk);
  }
  return await concatUintArrays(chunks);
}

export { Session, SessionCbFn };
