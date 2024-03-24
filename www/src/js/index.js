import { init } from "ic10emu_wasm";

import { IC10Editor, setupLspWorker } from "./editor";

const App = {
  editor: null,
  sessions: [],
  languageProvider: null,
  editorSettings: {
  }
};

window.App = App;


function docReady(fn) {
  // see if DOM is already available
  if (document.readyState === "complete" || document.readyState === "interactive") {
    setTimeout(fn, 1);
  } else {
    document.addEventListener("DOMContentLoaded", fn);
  }
}

init();

docReady(() => { 

  App.editor = new IC10Editor();

  setupLspWorker().then((worker) => {
    App.editor.setupLsp(worker);
  })


  // Menu
  document.getElementById("mainMenuShare").addEventListener('click', (_event) => {
    const link = document.getElementById("shareLinkText");
    link.setAttribute('value', window.location);
    link.setSelectionRange(0, 0);
  }, { capture: true });
  document.getElementById("shareLinkCopyButton").addEventListener('click', (event) => {
    event.preventDefault();
    const link = document.getElementById("shareLinkText");
    link.select();
    link.setSelectionRange(0, 99999);
    navigator.clipboard.writeText(link.value);
  }, { capture: true });
  document.getElementById("mainMenuOpenFile").addEventListener('click', (_event) => {
    openFile(editor);
  }, { capture: true });
  document.getElementById("mainMenuSaveAs").addEventListener('click', (_event) => {
    saveFile(editor.getSession().getValue())

  }, { capture: true });
  document.getElementById("mainMenuKeyboardShortcuts").addEventListener('click', (_event) => {
    App.editor.editor.execCommand("showKeyboardShortcuts");
  }, { capture: true });

});

async function saveFile(content) {
  const blob = new Blob([content], { type: "text/plain" });
  if (typeof window.showSaveFilePicker !== "undefined") {
    console.log("Saving via FileSystem API")
    const options = {
      types: [
        {
          suggestedName: "code.ic10",
          description: 'Text Files',
          accept: {
            'text/plain': ['.txt', '.ic10'],
          },
        },
      ],
    };
    const saveHandle = await window.showSaveFilePicker(options);
    const ws = await saveHandle.createWritable();
    await ws.write(blob);
    await ws.close();
  } else {
    console.log("saving file via hidden link event");
    var a = document.createElement('a');
    a.download = "code.ic10";
    a.href = window.URL.createObjectURL(blob);
    a.click();
  }
}

async function openFile(editor) {
  if (typeof window.showOpenFilePicker !== "undefined") {
    console.log("opening file via FileSystem Api");
    const [fileHandle] = await window.showOpenFilePicker();
    const file = await fileHandle.getFile();
    const contents = await file.text();
    const session = editor.getSession();
    session.setValue(contents);
  } else {
    console.log("opening file via hidden input event");
    let input = document.createElement('input');
    input.type = 'file';
    input.accept = ".txt,.ic10,.mips,text/*";
    input.onchange = _ => {
      const files = Array.from(input.files);
      console.log(files);
      const file = files[0];
      var reader = new FileReader();
      reader.onload = (e) => {
        const contents = e.target.result;
        const session = editor.getSession();
        // session.id = file.name;
        session.setValue(contents);
      };
      reader.readAsText(file);
    };
    input.click();
  }
}



