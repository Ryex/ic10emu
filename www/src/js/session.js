
import { BSON } from 'bson';

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

# same with constants
#       vvvv
move r3 pinf

# Labels are known
main:
l r1 dr15 RatioWater
move r2 100000.001

# Hover Hash Strings of Known prefab names
# to get their documentation
#             vvvvvvvvvvvvvvv
move r0 HASH("AccessCardBlack")
beqz r1 test

# -2045627372 is the crc32 hash of a SolarPanel, 
# hover it to see the documentation!
#        vvvvvvvvvv  
move r1 -2045627372
jal test
move r1 $FF
beqz 0 test
move r1 %1000
yield
j main

test:
add r15 r15 1
j ra

`

class Session {
  constructor() {
    this._programs = {};
    this._save_timeout = 0;
    this._onLoadCallbacks = [];
    this.loadFromFragment();

    const self = this;
    window.addEventListener('hashchange', (_event) => {
      self.loadFromFragment();
    });
  }

  get programs() {
    return this._programs;
  }

  set programs(programs) {
    Object.assign(this._programs, programs);
  }

  setProgramCode(id, code) {
    this._programs[id] = code;
    this.save();
  }

  onLoad(callback) {
    this._onLoadCallbacks.push(callback);
  }

  _fireOnLoad() {
    for (const i in this._onLoadCallbacks) {
      const callback = this._onLoadCallbacks[i];
      callback(this);
    }
  }

  save() {
    if (this._save_timeout) clearTimeout(this._save_timeout);
    this._save_timeout = setTimeout(() => {
      this.saveToFragment();
    }, 1000);
  }

  async saveToFragment() {
    const toSave = { programs: this._programs };
    const bytes = BSON.serialize(toSave);
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
      this._programs = { 0: demoCode };
      this._fireOnLoad();
      return;
    }
    if (fragment.length > 0) {
      const c_bytes = base64url_decode(fragment);
      const bytes = await decompressFragment(c_bytes);
      if (bytes !== null) {
        const data = BSON.deserialize(bytes);
        try {
          this._programs = Object.assign({}, data.programs);
          this._fireOnLoad();
          return;
        } catch (e) {
          console.log("Bad session data:", e);
        }
      }
    }
  }

}
 async function decompressFragment(c_bytes) {
  try {
    const bytes = await decompress(c_bytes);
    return bytes;
  } catch (e) {
    console.log("Error decompressing content fragment:", e);
    return null;
  }
}

async function* streamAsyncIterator(stream) {
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

function base64url_encode(buffer) {
  return btoa(Array.from(new Uint8Array(buffer), b => String.fromCharCode(b)).join(''))
    .replace(/\+/g, '-')
    .replace(/\//g, '_')
    .replace(/=+$/, '');
}

function base64url_decode(value) {
  const m = value.length % 4;
  return Uint8Array.from(atob(
    value.replace(/-/g, '+')
      .replace(/_/g, '/')
      .padEnd(value.length + (m === 0 ? 0 : 4 - m), '=')
  ), c => c.charCodeAt(0)).buffer
}

async function concatUintArrays(arrays) {
  const blob = new Blob(arrays);
  const buffer = await blob.arrayBuffer();
  return new Uint8Array(buffer);
}

async function compress(bytes) {
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

async function decompress(bytes) {
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

export { Session };
