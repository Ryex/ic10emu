// import {init, VM} from "ic10emu_wasm";
import * as ace from "ace-code";
import {Mode as IC10Mode} from "./ic10_mode";
import * as theme from "ace-code/src/theme/one_dark";
import {AceLanguageClient} from "ace-linters/build/ace-language-client";

// init();
//
// const vm = VM.new();
// console.log(vm);

const demoCode = `# This is a comment
define a_def 10
define a_hash HASH("This is a String")
alias a_var r0
alias a_device d0
s d0 12 0 
move r2 LogicType.Temperature
move r3 pinf
main:

l r1 dr15 RatioWater
move r2 100000.001
move r0 HASH("AccessCardBlack")
beqz r1 test
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
const mode = new IC10Mode()
var editor = ace.edit("editor", {
  mode: mode,
  value: demoCode,
  // enableBasicAutocompletion: true,
  // enableLiveAutocompletion: true,
  // enableSnippets: true,
  theme: theme,
  customScrollbar: true,
});

const loaded = w =>
  new Promise(r => w.addEventListener("message", r, { once: true }));

async function setupLsp() {
  // Create a web worker
  let worker = new Worker(new URL('./lspWorker.js', import.meta.url));
  await Promise.all([loaded(worker)]);

  const serverData = {
      module: () => import("ace-linters/build/language-client"),
      modes: "ic10",
      type: "webworker",
      worker: worker,
  }
  // Create a language provider for web worker
  let languageProvider = AceLanguageClient.for(serverData);
  // Register the editor with the language provider
  languageProvider.registerEditor(editor);

  let options = mode.options ?? {};
  languageProvider.setSessionOptions(editor.session, options);

}
setupLsp();



editor.session.setValue(demoCode)
