import { ace } from "./ace.js"
import { Mode as IC10Mode } from "./ic10_mode.js";
import * as one_dark from "ace-code/src/theme/one_dark";
import { AceLanguageClient } from "ace-linters/build/ace-language-client";
import { IC10EditorUI } from './ui.js';

// to make sure language tools are loaded
import _ace_ext_langue_tools from "ace-code/src/ext/language_tools";


async function setupLspWorker() {
  // Create a web worker
  let worker = new Worker(new URL('./lspWorker.js', import.meta.url));

  const loaded = w =>
    new Promise(r => w.addEventListener("message", r, { once: true }));
  await Promise.all([loaded(worker)]);

  // Register the editor with the language provider
  return worker;
}


function IC10Editor(session_id) {
  this.mode = new IC10Mode()

  this.settings = {
    keyboard: "ace",
    cursor: "ace",
    fontSize: 16,
    relativeLineNumbers: false,
  };

  this.editor = ace.edit('editor', {
    mode: this.mode,
    enableBasicAutocompletion: true,
    enableLiveAutocompletion: true,
    enableSnippets: true,
    theme: "ace/theme/one_dark",
    fontSize: "16px",
    customScrollbar: true,
    firstLineNumber: 0,
    printMarginColumn: 52,
  });

  this.sessions = {};
  this.sessions[session_id] = this.editor.getSession();
  this.active_session = session_id;
  this.bindSession(session_id, this.sessions[session_id]);

  this.languageProvider = null;

  this.ui = new IC10EditorUI(this);

  const self = this;

  App.session.onLoad((session) => {
    const updated_ids = [];
    for (const id in session.programs) {
      updated_ids.push(id);
      self.createOrSetSession(id, session.programs[id]);
    }
    for (const id in self.sessions) {
      if (!updated_ids.includes(id)) {
        self.destroySession(id);
      }
    }
  })
  App.session.loadFromFragment();

}

IC10Editor.prototype.createOrSetSession = function(session_id, content) {
  if (!this.sessions.hasOwnProperty(session_id)) {
    this.newSession(session_id);
  }
  this.sessions[session_id].setValue(content);
}

IC10Editor.prototype.newSession = function(session_id) {
  if (this.sessions.hasOwnProperty(session_id)) {
    return false;
  }
  this.sessions[session_id] = ace.createEditSession("", this.mode);
  this.bindSession(session_id, this.sessions[session_id]);
}

IC10Editor.prototype.setupLsp = function(lsp_worker) {
  const serverData = {
    module: () => import("ace-linters/build/language-client"),
    modes: "ic10",
    type: "webworker",
    worker: lsp_worker,
  };
  // Create a language provider for web worker
  this.languageProvider = AceLanguageClient.for(serverData);
  this.languageProvider.registerEditor(this.editor);

  for (const session_id in this.sessions) {
    let options = this.mode.options ?? {};
    this.languageProvider.setSessionOptions(this.sessions[session_id], options);
  }

}

IC10Editor.prototype.activateSession = function(session_id) {
  if (!this.sessions.hasOwnProperty(session_id)) {
    return false;
  }
  this.editor.setSession(this.sessions[session_id]);
  let options = this.mode.options ?? {};
  if (this.languageProvider !== null) {
    this.languageProvider.setSessionOptions(this.sessions[session_id], options);
  }
  return true;
}

IC10Editor.prototype.loadEditorSettings = function() {
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

IC10Editor.prototype.saveEditorSettings = function() {
  const toSave = JSON.stringify(this.settings);
  window.localStorage.setItem("editorSettings", toSave);
}

IC10Editor.prototype.destroySession = function(session_id) {
  if (!this.sessions.hasOwnProperty(session_id)) {
    return false;
  }
  if (!(Object.keys(this.sessions).length > 1)) {
    return false;
  }
  const session = this.sessions[session_id];
  delete this.sessions[session_id];
  if (this.active_session = session_id) {
    this.activateSession(Object.keys(this.sessions)[0]);
  }
  session.destroy();
  return true;
}

IC10Editor.prototype.bindSession = function(session_id, session) {
  session.on('change', () => {
    var val = session.getValue();
    window.App.session.setProgramCode(session_id, val);
  });
}

export { IC10Editor, setupLspWorker };
