import { Ace } from "ace-builds";

export function docReady(fn: () => void) {
  // see if DOM is already available
  if (
    document.readyState === "complete" ||
    document.readyState === "interactive"
  ) {
    setTimeout(fn, 1);
  } else {
    document.addEventListener("DOMContentLoaded", fn);
  }
}

function isZeroNegative(zero: number) {
  return Object.is(zero, -0);
}

function makeCRCTable() {
  let c;
  const crcTable = [];
  for (let n = 0; n < 256; n++) {
    c = n;
    for (let k = 0; k < 8; k++) {
      c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
    }
    crcTable[n] = c;
  }
  return crcTable;
}

const crcTable = makeCRCTable();

// Signed crc32 for string
export function crc32(str: string): number {
  let crc = 0 ^ -1;
  for (let i = 0, len = str.length; i < len; i++) {
    crc = (crc >>> 8) ^ crcTable[(crc ^ str.charCodeAt(i)) & 0xff];
  }
  return crc ^ -1;
}

export function numberToString(n: number): string {
  if (isZeroNegative(n)) return "-0";
  return n.toString();
}
export function displayNumber(n: number): string {
  return numberToString(n).replace("Infinity", "∞");
}

function replacer(_key: any, value: any) {
  if (value instanceof Map) {
    return {
      dataType: "Map",
      value: Array.from(value.entries()), // or with spread: value: [...value]
    };
  } else if (
    typeof value === "number" &&
    (!Number.isFinite(value) || Number.isNaN(value) || isZeroNegative(value))
  ) {
    return {
      dataType: "Number",
      value: numberToString(value),
    };
  } else if (typeof value === "undefined") {
    return {
      dataType: "undefined",
    };
  } else {
    return value;
  }
}

function reviver(_key: any, value: any) {
  if (typeof value === "object" && value !== null) {
    if (value.dataType === "Map") {
      return new Map(value.value);
    } else if (value.dataType === "Number") {
      return parseFloat(value.value);
    } else if (value.dataType === "undefined") {
      return undefined;
    }
  }
  return value;
}

export function toJson(value: any): string {
  return JSON.stringify(value, replacer);
}

export function fromJson(value: string): any {
  return JSON.parse(value, reviver);
}


export function compareMaps(map1: Map<any, any>, map2: Map<any, any>): boolean {
  let testVal;
  if (map1.size !== map2.size) {
    return false;
  }
  for (let [key, val] of map1) {
    testVal = map2.get(key);
    if((testVal === undefined && !map2.has(key)) || !structuralEqual(val, testVal)) {
      return false;
    }
  }
  return true;
}

export function compareArrays( array1: any[], array2: any[]): boolean {
  if (array1.length !== array2.length) {
    return false;
  }
  for ( let i = 0, len = array1.length; i < len; i++) {
    if (!structuralEqual(array1[i], array2[i])) {
      return false;
    }
  }
  return true;
}

export function compareStructuralObjects(a: object, b: object): boolean {
  const aProps = new Map(Object.entries(a));
  const bProps = new Map(Object.entries(b));
  return compareMaps(aProps, bProps);
}
export function structuralEqual(a: any, b: any): boolean {
  const aType = typeof a;
  const bType = typeof b;
  if ( aType !== typeof bType) {
    return false;
  }
  if (a instanceof Map && b instanceof Map) {
    return compareMaps(a, b);
  }
  if (Array.isArray(a) && Array.isArray(b)) {
    return compareArrays(a, b);
  }
  if (aType === "object" && bType === "object") {
    return compareStructuralObjects(a, b);
  }
  return a !== b;
}

// probably not needed, fetch() exists now
export function makeRequest(opts: {
  method: string;
  url: string;
  headers: { [key: string]: string };
  params: any;
}) {
  return new Promise(function (resolve, reject) {
    var xhr = new XMLHttpRequest();
    xhr.open(opts.method, opts.url);
    xhr.onload = function () {
      if (xhr.status >= 200 && xhr.status < 300) {
        resolve(xhr.response);
      } else {
        reject({
          status: xhr.status,
          statusText: xhr.statusText,
        });
      }
    };
    xhr.onerror = function () {
      reject({
        status: xhr.status,
        statusText: xhr.statusText,
      });
    };
    if (opts.headers) {
      Object.keys(opts.headers).forEach(function (key) {
        xhr.setRequestHeader(key, opts.headers[key]);
      });
    }
    var params = opts.params;
    if (params && typeof params === "object") {
      params = Object.keys(params)
        .map(function (key) {
          return (
            encodeURIComponent(key) + "=" + encodeURIComponent(params[key])
          );
        })
        .join("&");
    }
    xhr.send(params);
  });
}

export async function saveFile(content: BlobPart) {
  const blob = new Blob([content], { type: "text/plain" });
  if (typeof window.showSaveFilePicker !== "undefined") {
    console.log("Saving via FileSystem API");
    try {
      const saveHandle = await window.showSaveFilePicker({
        types: [
          {
            // suggestedName: "code.ic10",
            description: "Text Files",
            accept: {
              "text/plain": [".txt", ".ic10"],
            },
          },
        ],
      });
      const ws = await saveHandle.createWritable();
      await ws.write(blob);
      await ws.close();
    } catch (e) {
      console.log(e);
    }
  } else {
    console.log("saving file via hidden link event");
    var a = document.createElement("a");
    const date = new Date().valueOf().toString(16);
    a.download = `code_${date}.ic10`;
    a.href = window.URL.createObjectURL(blob);
    a.click();
  }
}

export async function openFile(editor: Ace.Editor) {
  if (typeof window.showOpenFilePicker !== "undefined") {
    console.log("opening file via FileSystem Api");
    try {
      const [fileHandle] = await window.showOpenFilePicker();
      const file = await fileHandle.getFile();
      const contents = await file.text();
      const session = editor.getSession();
      session.setValue(contents);
    } catch (e) {
      console.log(e);
    }
  } else {
    console.log("opening file via hidden input event");
    let input = document.createElement("input");
    input.type = "file";
    input.accept = ".txt,.ic10,.mips,text/*";
    input.onchange = (_) => {
      const files = Array.from(input.files!);
      console.log(files);
      const file = files[0];
      var reader = new FileReader();
      reader.onload = (e) => {
        const contents = e.target!.result as string;
        const session = editor.getSession();
        // session.id = file.name;
        session.setValue(contents);
      };
      reader.readAsText(file);
    };
    input.click();
  }
}

export function parseNumber(s: string): number {
  switch (s.toLowerCase()) {
    case "nan":
      return Number.NaN;
    case "pinf":
      return Number.POSITIVE_INFINITY;
    case "ninf":
      return Number.NEGATIVE_INFINITY;
    case "pi":
      return 3.141592653589793;
    case "deg2rad":
      return 0.0174532923847437;
    case "rad2deg":
      return 57.2957801818848;
    case "epsilon":
      return Number.EPSILON;
  }
  if (/^%[01]+$/.test(s)) {
    return parseInt(s.slice(1), 2);
  }
  if (/^\$[0-9A-Fa-f]+$/.test(s)) {
    return parseInt(s.slice(1), 16);
  }
  if (/[a-fA-F]/.test(s)) {
    const hex = parseHex(s);
    if (!isNaN(hex)) {
      return hex;
    }
  }
  s.replace("∞", "Infinity");
  return parseFloat(s);
}

export function parseHex(h: string): number {
  var val = parseInt(h, 16);
  if (val.toString(16) === h.toLowerCase()) {
    return val;
  } else {
    return NaN;
  }
}

export function parseIntWithHexOrBinary(s: string): number {
  if (/^%[01]+$/.test(s)) {
    return parseInt(s.slice(1), 2);
  }
  if (/^\$[0-9A-Fa-f]+$/.test(s)) {
    return parseInt(s.slice(1), 16);
  }
  if (/[a-fA-F]/.test(s)) {
    const hex = parseHex(s);
    if (!isNaN(hex)) {
      return hex;
    }
  }
  return parseInt(s);
}

export function clamp(val: number, min: number, max: number) {
  return Math.min(Math.max(val, min), max);
}

export type TypedEventTarget<EventMap extends object> = {
  new (): TypedEventTargetInterface<EventMap>;
};

interface TypedEventTargetInterface<EventMap> extends EventTarget {
  addEventListener<K extends keyof EventMap>(
    type: K,
    callback: (
      event: EventMap[K] extends Event ? EventMap[K] : never,
    ) => EventMap[K] extends Event ? void : never,
    options?: boolean | AddEventListenerOptions,
  ): void;

  addEventListener(
    type: string,
    callback: EventListenerOrEventListenerObject | null,
    options?: EventListenerOptions | boolean,
  ): void;
}
