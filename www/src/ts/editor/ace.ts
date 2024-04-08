import ace from "ace-builds";
import "ace-builds/esm-resolver";

import { AceLanguageClient } from "ace-linters/build/ace-language-client";

// to make sure language tools are loaded
ace.config.loadModule("ace/ext/language_tools");

import { Mode as TextMode } from "ace-builds/src-noconflict/mode-text";

export async function setupLspWorker() {
  // Create a web worker
  let worker = new Worker(new URL("./lspWorker.ts", import.meta.url));

  const loaded = (w: Worker) =>
    new Promise((r) => w.addEventListener("message", r, { once: true }));
  await  Promise.all([loaded(worker)]);

  // Register the editor with the language provider
  return worker;
}

export import Ace = ace.Ace;
export import EditSession = ace.Ace.EditSession;
export import Editor = ace.Ace.Editor;
import { Range } from "ace-builds";

export { ace, TextMode, Range, AceLanguageClient }
