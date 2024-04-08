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


function replacer(key: any, value: any) {
  if(value instanceof Map) {
    return {
      dataType: 'Map',
      value: Array.from(value.entries()), // or with spread: value: [...value]
    };
  } else {
    return value;
  }
}

function reviver(_key: any, value: any) {
  if(typeof value === 'object' && value !== null) {
    if (value.dataType === 'Map') {
      return new Map(value.value);
    }
  }
  return value;
}

export function toJson(value: any): string {
  return JSON.stringify(value, replacer);
}

export function fromJson(value: string): any {
  return JSON.parse(value, reviver)
}

export function structuralEqual(a: any, b: any): boolean {
  const _a = JSON.stringify(a, replacer);
  const _b = JSON.stringify(b, replacer);
  return _a === _b;

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
    a.download = "code.ic10";
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
