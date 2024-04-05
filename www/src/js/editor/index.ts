import ace from "ace-builds";
import "ace-builds/esm-resolver";

import { AceLanguageClient } from "ace-linters/build/ace-language-client";

import { IC10EditorUI } from "./ui";
import { Range } from "ace-builds";

import { App } from "../index";
import { Session } from "../session";

// import { Mode as TextMode } from 'ace-code/src/mode/text';
// to make sure language tools are loaded
ace.config.loadModule("ace/ext/language_tools");

import { Mode as TextMode } from "ace-builds/src-noconflict/mode-text";

async function setupLspWorker() {
  // Create a web worker
  let worker = new Worker(new URL("./lspWorker.ts", import.meta.url));

  const loaded = (w: Worker) =>
    new Promise((r) => w.addEventListener("message", r, { once: true }));
  await Promise.all([loaded(worker)]);

  // Register the editor with the language provider
  return worker;
}

declare global {
  interface Window {
    Editor: IC10Editor;
  }
}

import { BaseElement } from "../components";
import { html, css } from "lit";
import { customElement, property } from "lit/decorators.js";

@customElement("ace-ic10")
class IC10Editor extends BaseElement {
  mode: string;
  settings: {
    keyboard: string;
    cursor: string;
    fontSize: number;
    relativeLineNumbers: boolean;
  };
  sessions: Map<number, ace.Ace.EditSession>;

  @property({ type: Number })
  accessor active_session: number = 0;

  active_line_markers: Map<number, number | null> = new Map();
  languageProvider?: AceLanguageClient;
  ui: IC10EditorUI;

  static styles = css`
    :host {
      display: block;
      width: 100%;
      height: 100%;
    }
    #editor {
      border: var(--lae-border, 1px solid var(--lumo-contrast-20pct));
      border-radius: var(--lae-border-radius, var(--lumo-border-radius));
      @apply --ace-widget-editor;
    }
    #editorStatusbar {
      z-index: 9 !important;
      position: absolute !important;
      right: 4px;
      bottom: 4px;
    }
    .ace_status-indicator {
      background-color: var(--las-background-color, #777);
      color: var(--las-color, white);
      text-align: center;
      border: none;
      border-radius: var(--las-border-radius, 7px);
      padding-right: 3px;
      padding-left: 3px;
      padding-bottom: 1px;
      font-size: small;
      opacity: 0.9;
    }
    .hide_statusbar {
      display: none;
    }
    .ace_marker-layer .green {
      background-color: var(--lumo-success-color);
      color: var(--lumo-primary-contrast-color);
      position: absolute;
    }
    .ace_marker-layer .darkGrey {
      background-color: var(--lumo-shade-50pct);
      color: var(--lumo-primary-contrast-color);
      position: absolute;
    }
    .ace_marker-layer .red {
      background-color: var(--lumo-error-color);
      color: var(--lumo-primary-contrast-color);
      position: absolute;
    }
    .ace_marker-layer .blue {
      background-color: var(--lumo-primary-color);
      color: var(--lumo-primary-contrast-color);
      position: absolute;
    }
    .ace_marker-layer .orange {
      background-color: #ff9900;
      color: #555;
      position: absolute;
    }
    .ace_placeholder {
      color: #808080 !important;
      font-family: var(--lumo-font-family) !important;
      transform: scale(1) !important;
      opacity: 1 !important;
      font-style: italic !important;
    }
  `;
  initialInit: boolean;
  editorDiv: HTMLElement;
  editorContainerDiv: HTMLElement;
  editorStatusbarDiv: HTMLElement;
  editor: ace.Ace.Editor;
  statusBar: any;
  snippetManager: any;
  observer: ResizeObserver;
  private _statusbarIndex: number;
  private _statusbar: any;
  vScrollbarObserver: IntersectionObserver;
  hScrollbarObserver: any;

  constructor() {
    super();
    console.log('constructing editor')

    window.Editor = this;
    this.mode = "ace/mode/ic10";

    this.settings = {
      keyboard: "ace",
      cursor: "ace",
      fontSize: 16,
      relativeLineNumbers: false,
    };

    this.sessions = new Map();
    this.active_line_markers = new Map();

    // this.ui = new IC10EditorUI(this);

    const that = this;

    App.session.onLoad((session: Session) => {
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
            that.active_line_markers.set(
              id,
              session.addMarker(
                new Range(active_line, 0, active_line, 1),
                "vm_ic_active_line",
                "fullLine",
                true,
              ),
            );
            if (that.active_session == id) {
              // editor.resize(true);
              // TODO: Scroll to line if vm was stepped
              //that.editor.scrollToLine(active_line, true, true, ()=>{})
            }
          }
        }
      }
    });
  }

  render() {
    return html`
      <div
        id="editorContainer"
        style="height: 100%; width: 100%; position: relative;"
      >
        <div
          id="editor"
          style="position: absolute; top: 0; right: 0; bottom: 0; left: 0;"
        ></div>
        <div id="editorStatusbar"></div>
      </div>
    `;
  }

  async firstUpdated() {
    console.log('editor firstUpdated')
    if (!ace.require("ace/ext/language_tools")) {
      await import("ace-builds/src-noconflict/ext-language_tools");
    }
    if (!ace.require("ace/ext/statusbar")) {
      await import("ace-builds/src-noconflict/ext-statusbar");
    }
    if (!ace.require("ace/mode/ic10")) {
      await import("./ic10_mode");
    }
    // patch prompt ext
    ace.config.setModuleLoader(
      "ace/ext/prompt",
      () => import("./prompt_patch"),
    );
    ace.config.setDefaultValue("session", "theme", "ace/theme/one_dark");

    this.initialInit = true;

    this.editorDiv = this.shadowRoot.getElementById("editor");
    this.editorContainerDiv = this.shadowRoot.getElementById("editorContainer");
    this.editorStatusbarDiv = this.shadowRoot.getElementById("editorStatusbar");

    this.editor = ace.edit(this.editorDiv, {
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
    this.editor.renderer.attachToShadowRoot();
    this.statusBar = ace.require("ace/ext/statusbar").StatusBar;
    this.snippetManager = ace.require("ace/snippets").snippetManager;

    this.sessions.set(this.active_session, this.editor.getSession());
    this.bindSession(
      this.active_session,
      this.sessions.get(this.active_session),
    );
    this.active_line_markers.set(this.active_session, null);

    const worker = await setupLspWorker();
    this.setupLsp(worker);

    const that = this;

    // when the CSS resize Property is added (to a container-div or ace-ic10 )
    // the correct sizing is maintained (after user resize)
    document.addEventListener("mouseup", function (e) {
      that.resizeEditor();
    });

    this.observer = new ResizeObserver(function (entries) {
      entries.forEach(function (entry) {
        that.resizeEditor();
      });
    });

    this.observer.observe(this.editorContainerDiv);

    this.initializeEditor();
  }

  initializeEditor() {
    let editor = this.editor;

    // change -> possibility to allow saving the value without having to wait for blur
    editor.on("change", () => this.editorChangeAction());

    this._statusbarIndex = 1;
    this._statusbar = new this.statusBar(
      this.editor,
      this.editorStatusbarDiv,
      this._statusbarIndex,
    );
    this._statusbar.updateStatus(this.editor);

    this.vScrollbarObserver = new IntersectionObserver(
      this._vScrollbarHandler.bind(this),
      { root: null },
    );
    this.vScrollbarObserver.observe(
      this.shadowRoot.querySelector(".ace_scrollbar-v"),
    );

    this.hScrollbarObserver = new IntersectionObserver(
      this._hScrollbarHandler.bind(this),
      { root: null },
    );
    this.hScrollbarObserver.observe(
      this.shadowRoot.querySelector(".ace_scrollbar-h"),
    );
  }

  resizeEditor() {
    if (this.editor == undefined) {
      this.addEventListener("editor-ready", () => this._resizeEditor(), {
        once: true,
      });
    } else {
      this._resizeEditor();
    }
  }

  /** @private */
  _resizeEditor() {
    this.editor.resize();
  }

  /** @private */
  _vScrollbarHandler() {
    var vScrollbar = this.shadowRoot.querySelector(
      ".ace_scrollbar-v",
    ) as HTMLDivElement;
    if (vScrollbar.style.display === "none") {
      this.editorStatusbarDiv.style.right = "4px";
    } else {
      let width = vScrollbar.offsetWidth - vScrollbar.clientWidth;
      if (width === undefined || width === null) {
        width = 20;
      }
      this.editorStatusbarDiv.style.right = width + 4 + "px";
    }
  }

  /** @private */
  _hScrollbarHandler() {
    var hScrollbar = this.shadowRoot.querySelector(
      ".ace_scrollbar-h",
    ) as HTMLDivElement;
    if (hScrollbar.style.display === "none") {
      this.editorStatusbarDiv.style.bottom = "4px";
    } else {
      let height = hScrollbar.offsetHeight - hScrollbar.clientHeight;
      if (height === undefined || height === null) {
        height = 20;
      }
      this.editorStatusbarDiv.style.bottom = height + 4 + "px";
    }
  }

  editorChangeAction() {
    this.dispatchEvent(
      new CustomEvent("editor-change", {
        detail: {
          value: this.editorValue,
        },
      }),
    );
  }

  get editorValue() {
    if (this.editor == undefined) {
      return "";
    }
    return this.editor.getValue();
  }

  set editorValue(value) {
    if (this.editor == undefined || value === undefined) {
      return;
    }
    this.editor.setValue(value, 1);
  }

  focusEditor() {
    if (this.editor == undefined) {
      this.addEventListener("editor-ready", (e) => this.editor.focus(), {
        once: true,
      });
    } else {
      this.editor.focus();
    }
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
    });
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
    (this.languageProvider as any).registerEditor(this.editor);

  }

  activateSession(session_id: number) {
    if (!this.sessions.get(session_id)) {
      return false;
    }
    this.editor.setSession(this.sessions.get(session_id));
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
    if ((this.active_session = session_id)) {
      this.activateSession(this.sessions.entries().next().value);
    }
    session.destroy();
    return true;
  }

  bindSession(session_id: number, session: ace.Ace.EditSession) {
    session.on("change", () => {
      var val = session.getValue();
      window.App.session.setProgramCode(session_id, val);
    });
  }
}

export { IC10Editor };
