import { IC10Editor } from "./editor";
import { Session } from "./session";
import { VirtualMachine } from "./virtual_machine";
import { docReady, openFile, saveFile } from "./utils";
// import { makeRequest } from "./utils";
declare global {
  interface Window {
    App: App;
  }
}

type App = {
  editorSettings: { fontSize: number; relativeLineNumbers: boolean };
  editor: IC10Editor;
  vm: VirtualMachine;
  session: Session;
};

export const App: App = {
  editor: null,
  vm: null,
  session: new Session(),
  editorSettings: { fontSize: 16, relativeLineNumbers: false },
};

window.App = App;

// const dbPromise = makeRequest({ method: "GET", url: "/data/database.json"});
// const dbPromise = fetch("/data/database.json").then(resp => resp.json());
const dbPromise = import("../../data/database.json");

docReady(() => {
  App.vm = new VirtualMachine();

  dbPromise.then((db) => App.vm.setupDeviceDatabase(db));

  const init_session_id = App.vm.devices.get(0).id;

  // App.editor = new IC10Editor(init_session_id);

  // setupLspWorker().then((worker) => {
  //   App.editor.setupLsp(worker);
  // });

  // Menu
  document.getElementById("mainMenuShare").addEventListener(
    "click",
    (_event) => {
      const link = document.getElementById("shareLinkText") as HTMLInputElement;
      link.setAttribute("value", window.location.href);
      link.setSelectionRange(0, 0);
    },
    { capture: true },
  );
  document.getElementById("shareLinkCopyButton").addEventListener(
    "click",
    (event) => {
      event.preventDefault();
      const link = document.getElementById("shareLinkText") as HTMLInputElement;
      link.select();
      link.setSelectionRange(0, 99999);
      navigator.clipboard.writeText(link.value);
    },
    { capture: true },
  );
  document.getElementById("mainMenuOpenFile").addEventListener(
    "click",
    (_event) => {
      openFile(App.editor.editor);
    },
    { capture: true },
  );
  document.getElementById("mainMenuSaveAs").addEventListener(
    "click",
    (_event) => {
      saveFile(App.editor.editor.getSession().getValue());
    },
    { capture: true },
  );
  document.getElementById("mainMenuKeyboardShortcuts").addEventListener(
    "click",
    (_event) => {
      App.editor.editor.execCommand("showKeyboardShortcuts");
    },
    { capture: true },
  );
});
