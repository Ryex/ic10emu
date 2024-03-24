import { ace } from "./ace.js"
import { Mode as IC10Mode } from "./ic10_mode.js";
import * as one_dark from "ace-code/src/theme/one_dark";
import { AceLanguageClient } from "ace-linters/build/ace-language-client";
import { IC10EditorUI } from './ui.js';

// to make sure language tools are loaded
import _ace_ext_langue_tools from "ace-code/src/ext/language_tools";


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
  this.editor.session.setValue(demoCode)

  this.languageProvider = null;

  this.ui = new IC10EditorUI(this);
}

IC10Editor.prototype.newSession = function(session_id) {
  if (this.sessions.hasOwnProperty(session_id)) {
    return false;
  }
  this.sessions[session_id] = ace.createEditSession("", this.mode);
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

IC10Editor.prototype.activateSession = function (session_id) {
  if (!this.sessions.hasOwnProperty(session_id)) {
    return false;
  }
  this.editor.setSession(this.sessions[session_id]);
  let options = this.mode.options ?? {};
  this.languageProvider.setSessionOptions(this.sessions[session_id], options);
  return true;
}

IC10Editor.prototype.loadEditorSettings = function () {
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

IC10Editor.prototype.saveEditorSettings = function () {
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
  this.sessions[session_id].destroy();
  delete this.sessions[session_id];
  return true;
}

export { IC10Editor, setupLspWorker };
