// import { ace } from "./ace.js"
import ace from "ace-builds";
import "ace-builds/esm-resolver";

// patch prompt ext
ace.config.setModuleLoader('ace/ext/prompt', () => import('./prompt_patch'));
ace.config.setDefaultValue("session", "theme", "ace/theme/one_dark");


import "ace-builds/src-noconflict/ext-language_tools";
ace.require("ace/ext/language_tools");

import "./ic10_mode";
import { AceLanguageClient } from "ace-linters/build/ace-language-client";
import { IC10EditorUI } from './ui';
import { Range } from 'ace-builds';

import { App } from "../index";
import { Session } from "../session";

// import { Mode as TextMode } from 'ace-code/src/mode/text';
// to make sure language tools are loaded
ace.config.loadModule("ace/ext/language_tools");

import { Mode as TextMode } from "ace-builds/src-noconflict/mode-text";


async function setupLspWorker() {
  // Create a web worker
  let worker = new Worker(new URL('./lspWorker.ts', import.meta.url));

  const loaded = (w: Worker) =>
    new Promise(r => w.addEventListener("message", r, { once: true }));
  await Promise.all([loaded(worker)]);

  // Register the editor with the language provider
  return worker;
}

declare global {
  interface Window { Editor: IC10Editor }
}

class IC10Editor {
  mode: string;
  settings: { keyboard: string; cursor: string; fontSize: number; relativeLineNumbers: boolean; };
  aceEditor: ace.Ace.Editor;
  sessions: Map<number, ace.Ace.EditSession>;
  active_session: number;
  active_line_markers: Map<number, number | null>;
  languageProvider: null | AceLanguageClient;
  ui: IC10EditorUI;
  constructor(session_id: number) {
    window.Editor = this;
    this.mode = "ace/mode/ic10";

    this.settings = {
      keyboard: "ace",
      cursor: "ace",
      fontSize: 16,
      relativeLineNumbers: false,
    };

    this.aceEditor = ace.edit('editor', {
      mode: this.mode,
      enableBasicAutocompletion: true,
      enableLiveAutocompletion: true,
      enableSnippets: true,
      theme: "ace/theme/one_dark",
      fontSize: 16,
      customScrollbar: false,
      firstLineNumber: 0,
      printMarginColumn: 52,
      placeholder: "Your code goes here ...",
    });

    this.sessions = new Map();
    this.sessions.set(session_id, this.aceEditor.getSession());
    this.active_session = session_id;
    this.bindSession(session_id, this.sessions.get(session_id));
    this.active_line_markers = new Map();
    this.active_line_markers.set(session_id, null);

    this.languageProvider = null;

    this.ui = new IC10EditorUI(this);

    const that = this;

    App.session.onLoad((session: Session ) => {
      const updated_ids = [];
      for (const [id, _] of session.programs) {
        updated_ids.push(id);
        that.createOrSetSession(id, session.programs.get(id));
      }
      that.activateSession(that.active_session);
      for (const [id, _] of that.sessions) {
        if (!updated_ids.includes(id)) {
          that.destroySession(id);
        }
      }

    });
    App.session.loadFromFragment();

    App.session.onActiveLine((session: Session) => {
      for (const id of session.programs.keys()) {
        const active_line = session.getActiveLine(id);
        if (typeof active_line !== "undefined") {
          const marker = that.active_line_markers.get(id);
          if (marker) {
            that.sessions.get(id).removeMarker(marker);
            that.active_line_markers.set(id, null);
          }
          const session = that.sessions.get(id);
          if (session) {
            that.active_line_markers.set(id, session.addMarker(new Range(active_line, 0, active_line, 1), "vm_ic_active_line", "fullLine", true));
            if (that.active_session == id) {
              // editor.resize(true);
              that.aceEditor.scrollToLine(active_line, true, true, ()=>{})
            }
          }
        }
      }
    })

  }

  createOrSetSession(session_id: number, content: any) {
    if (!this.sessions.hasOwnProperty(session_id)) {
      this.newSession(session_id);
    }
    this.sessions.get(session_id).setValue(content);
  }

  newSession(session_id: number) {
    if (this.sessions.hasOwnProperty(session_id)) {
      return false;
    }
    const session = ace.createEditSession("", this.mode as any);
    session.setOptions({
      firstLineNumber: 0,
    })
    this.sessions.set(session_id, session);
    this.bindSession(session_id, session);
  }

  setupLsp(lsp_worker: Worker) {
    const serverData = {
      module: () => import("ace-linters/build/language-client"),
      modes: "ic10",
      type: "webworker",
      worker: lsp_worker,
    };
    // Create a language provider for web worker
    this.languageProvider = AceLanguageClient.for(serverData as any);
    (this.languageProvider as any).registerEditor(this.aceEditor);

    // for (const session_id of this.sessions.keys()) {
    //   let options = {};
    //   (this.languageProvider as any).setSessionOptions(this.sessions.get(session_id), options);
    // }

  }

  activateSession(session_id: number) {
    if (!this.sessions.get(session_id)) {
      return false;
    }
    this.aceEditor.setSession(this.sessions.get(session_id));
    this.active_session = session_id;
    return true;
  }

  loadEditorSettings() {
    const saved_settings = window.localStorage.getItem("editorSettings");
    if (saved_settings !== null && saved_settings.length > 0) {
      try {
        const saved = JSON.parse(saved_settings);
        const temp = Object.assign({}, this.settings, saved);
        Object.assign(this.settings, temp);
      } catch (e) {
        console.log("error loading editor settings", e);
      }
    }
  }

  saveEditorSettings() {
    const toSave = JSON.stringify(this.settings);
    window.localStorage.setItem("editorSettings", toSave);
  }

  destroySession(session_id: number) {
    if (!this.sessions.hasOwnProperty(session_id)) {
      return false;
    }
    if (!(Object.keys(this.sessions).length > 1)) {
      return false;
    }
    const session = this.sessions.get(session_id);
    this.sessions.delete(session_id);
    if (this.active_session = session_id) {
      this.activateSession(this.sessions.entries().next().value);
    }
    session.destroy();
    return true;
  }

  bindSession(session_id: number, session: ace.Ace.EditSession) {
    session.on('change', () => {
      var val = session.getValue();
      window.App.session.setProgramCode(session_id, val);
    });
  }
}









export { IC10Editor, setupLspWorker };
