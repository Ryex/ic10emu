import {init} from "ic10emu_wasm";
import * as ace from "ace-code";
import {Mode as IC10Mode} from "./ic10_mode";
import * as theme from "ace-code/src/theme/one_dark";
import {AceLanguageClient} from "ace-linters/build/ace-language-client";
import ace_ext_statusbar from "ace-code/src/ext/statusbar";
import ace_ext_keybinding_menu from "ace-code/src/ext/keybinding_menu";
import _ace_ext_langue_tools from "ace-code/src/ext/language_tools";

// make sure Ace can load through webpack
// trimmed down version of https://github.com/ajaxorg/ace-builds/blob/master/esm-resolver.js but for ace-code
ace.config.setModuleLoader("ace/theme/one_dark", () => import("ace-code/src/theme/one_dark"));
ace.config.setModuleLoader('ace/ext/beautify', () => import('ace-code/src/ext/beautify.js'));
ace.config.setModuleLoader('ace/ext/code_lens', () => import('ace-code/src/ext/code_lens.js'));
ace.config.setModuleLoader('ace/ext/command_bar', () => import('ace-code/src/ext/command_bar.js'));
ace.config.setModuleLoader('ace/ext/elastic_tabstops_lite', () => import('ace-code/src/ext/elastic_tabstops_lite.js'));
ace.config.setModuleLoader('ace/ext/emmet', () => import('ace-code/src/ext/emmet.js'));
ace.config.setModuleLoader('ace/ext/error_marker', () => import('ace-code/src/ext/error_marker.js'));
ace.config.setModuleLoader('ace/ext/hardwrap', () => import('ace-code/src/ext/hardwrap.js'));
ace.config.setModuleLoader('ace/ext/inline_autocomplete', () => import('ace-code/src/ext/inline_autocomplete.js'));
ace.config.setModuleLoader('ace/ext/keyboard_menu', () => import('ace-code/src/ext/keybinding_menu.js'));
ace.config.setModuleLoader('ace/ext/language_tools', () => import('ace-code/src/ext/language_tools.js'));
ace.config.setModuleLoader('ace/ext/linking', () => import('ace-code/src/ext/linking.js'));
ace.config.setModuleLoader('ace/ext/modelist', () => import('ace-code/src/ext/modelist.js'));
ace.config.setModuleLoader('ace/ext/options', () => import('ace-code/src/ext/options.js'));
ace.config.setModuleLoader('ace/ext/prompt', () => import('ace-code/src/ext/prompt.js'));
ace.config.setModuleLoader('ace/ext/rtl', () => import('ace-code/src/ext/rtl.js'));
ace.config.setModuleLoader('ace/ext/searchbox', () => import('ace-code/src/ext/searchbox.js'));
// ace.config.setModuleLoader('ace/ext/settings_menu', () => import('ace-code/src/ext/settings_menu.js'));
ace.config.setModuleLoader('ace/ext/simple_tokenizer', () => import('ace-code/src/ext/simple_tokenizer.js'));
ace.config.setModuleLoader('ace/ext/spellcheck', () => import('ace-code/src/ext/spellcheck.js'));
ace.config.setModuleLoader('ace/ext/split', () => import('ace-code/src/ext/split.js'));
ace.config.setModuleLoader('ace/ext/static_highlight', () => import('ace-code/src/ext/static_highlight.js'));
ace.config.setModuleLoader('ace/ext/statusbar', () => import('ace-code/src/ext/statusbar.js'));
ace.config.setModuleLoader('ace/ext/textarea', () => import('ace-code/src/ext/textarea.js'));
ace.config.setModuleLoader('ace/ext/themelist', () => import('ace-code/src/ext/themelist.js'));
ace.config.setModuleLoader('ace/ext/whitespace', () => import('ace-code/src/ext/whitespace.js'));
ace.config.setModuleLoader('ace/keyboard/emacs', () => import('ace-code/src/keyboard/emacs.js'));
ace.config.setModuleLoader('ace/keyboard/sublime', () => import('ace-code/src/keyboard/sublime.js'));
ace.config.setModuleLoader('ace/keyboard/vim', () => import('ace-code/src/keyboard/vim.js'));
ace.config.setModuleLoader('ace/keyboard/vscode', () => import('ace-code/src/keyboard/vscode.js'));

function docReady(fn) {
  // see if DOM is already available
  if (document.readyState === "complete" || document.readyState === "interactive") {
    setTimeout(fn, 1);
  } else {
    document.addEventListener("DOMContentLoaded", fn);
  }
}

init();

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

const loaded = w =>
  new Promise(r => w.addEventListener("message", r, { once: true }));

async function setupLsp(editor, mode) {
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

docReady(() => {
  const mode = new IC10Mode()
  var editor = ace.edit("editor", {
    mode: mode,
    value: demoCode,
    enableBasicAutocompletion: true,
    enableLiveAutocompletion: true,
    enableSnippets: true,
    // theme: theme,
    fontSize: "16px",
    customScrollbar: false,
    firstLineNumber: 0,
    printMarginColumn: 52,
  });
  editor.setTheme("ace/theme/one_dark");
  var statusBar =  new ace_ext_statusbar.StatusBar(editor, document.getElementById("statusBar"));
  statusBar.updateStatus(editor);
  ace_ext_keybinding_menu.init(editor);
  // editor.setOption("keyboardHandler", "ace/keyboard/vim");


  editor.session.setValue(demoCode)
  setupLsp(editor, mode);
});





