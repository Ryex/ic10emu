import * as log from "log";

log.info("Loading Ace Editor ...");

import ace from "ace-code";
import "ace-code/esm-resolver";

import { Range, Editor, EditSession } from "ace-code";

import { HoverTooltip } from "ace-code/src/tooltip";

export import Ace = ace.Ace;

import { AceLanguageClient } from "ace-linters/build/ace-language-client";
import { Marker } from "ace-code/src/layer/marker";
import { MarkerGroup } from "ace-code/src/marker_group";

ace.config.setModuleLoader("ace/mode/ic10", () => import("ic10mode"));

// to make sure language tools are loaded
ace.config.loadModule("ace/ext/language_tools", () =>
  log.trace("loaded: ace/ext/language_tools"),
);

export async function setupLspWorker(url: string) {
  log.trace(`loading lsp worker from '${url}'`);
  // Create a web worker
  let worker = new Worker(url, {
    name: "ic10lsp-Worker",
  });

  const loaded = (w: Worker) =>
    new Promise((r) => w.addEventListener("message", r, { once: true }));
  await Promise.all([loaded(worker)]);

  log.trace(`lsp worker from '${url}' loaded`);
  // Register the editor with the language provider
  return worker;
}


export {
  ace,
  Range,
  Editor,
  EditSession,
  AceLanguageClient,
  Marker,
  MarkerGroup,
  HoverTooltip,
};
