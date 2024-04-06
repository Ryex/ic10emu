import {
  ace,
  EditSession,
  Range,
  AceLanguageClient,
  setupLspWorker,
} from "./ace";

import { LanguageProvider } from "ace-linters/types/language-provider";

import "@shoelace-style/shoelace/dist/components/dialog/dialog.js";
import "@shoelace-style/shoelace/dist/components/button-group/button-group.js";
import "@shoelace-style/shoelace/dist/components/button/button.js";
import "@shoelace-style/shoelace/dist/components/input/input.js";
import "@shoelace-style/shoelace/dist/components/radio-button/radio-button.js";
import "@shoelace-style/shoelace/dist/components/radio-group/radio-group.js";
import '@shoelace-style/shoelace/dist/components/switch/switch.js';
import SlDialog from "@shoelace-style/shoelace/dist/components/dialog/dialog.js";
import SlRadioGroup from "@shoelace-style/shoelace/dist/components/radio-group/radio-group.js";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.js";
import SlSwitch from "@shoelace-style/shoelace/dist/components/switch/switch.js";

declare global {
  interface Window {
    Editor: IC10Editor;
  }
}

import { BaseElement, defaultCss } from "../components";
import { html } from "lit";
import { customElement, property } from "lit/decorators.js";
import { editorStyles } from "./styles";

@customElement("ace-ic10")
export class IC10Editor extends BaseElement {
  mode: string;
  settings: {
    keyboard: string;
    cursor: string;
    fontSize: number;
    relativeLineNumbers: boolean;
  };
  sessions: Map<number, EditSession>;

  @property({ type: Number })
  accessor active_session: number = 0;

  active_line_markers: Map<number, number | null> = new Map();
  languageProvider?: LanguageProvider;
  // ui: IC10EditorUI;

  static styles = [...defaultCss, editorStyles];

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
  hScrollbarObserver: IntersectionObserver;
  stylesObserver: MutationObserver;
  stylesAdded: string[];
  tooltipObserver: MutationObserver;

  get settingDialog() {
    return this.shadowRoot?.querySelector('sl-dialog') as SlDialog
  }

  constructor() {
    super();
    console.log("constructing editor");

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
  }

  protected render() {
    const result = html`
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
      <sl-dialog label="Editor Settings" class="dialog-focus">
        <sl-radio-group id="editorKeyboardRadio" label="Editor Keyboard Bindings" value=${this.settings.keyboard}>
          <sl-radio-button value="ace">Ace</sl-radio-button>
          <sl-radio-button value="vim">Vim</sl-radio-button>
          <sl-radio-button value="emacs">Emacs</sl-radio-button>
          <sl-radio-button value="sublime">Sublime</sl-radio-button>
          <sl-radio-button value="vscode">VS Code</sl-radio-button>
        </sl-radio-group>
        <sl-radio-group id="editorCursorRadio" label="Editor Cursor Style" value=${this.settings.cursor}>
          <sl-radio-button value="ace">Ace</sl-radio-button>
          <sl-radio-button value="slim">Slim</sl-radio-button>
          <sl-radio-button value="smooth">Smooth</sl-radio-button>
          <sl-radio-button value="smooth slim">Smooth And Slim</sl-radio-button>
          <sl-radio-button value="wide">Wide</sl-radio-button>
        </sl-radio-group>
        <sl-input id="editorFontSize" label="Font Size" type="number" value="${this.settings.fontSize}"></sl-input>
        <sl-switch id="editorRelativeLineNumbers" ?checked=${this.settings.relativeLineNumbers}>Relative Line Numbers</sl-switch>
      </sl-dialog>
    `;
    return result;
  }

  connectedCallback(): void {
    super.connectedCallback();
    this.loadEditorSettings();
  }

  async firstUpdated() {
    console.log("editor firstUpdated");
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

    this.editorDiv = this.shadowRoot?.getElementById("editor") as HTMLElement;
    this.editorContainerDiv = this.shadowRoot?.getElementById(
      "editorContainer",
    ) as HTMLElement;
    this.editorStatusbarDiv = this.shadowRoot?.getElementById(
      "editorStatusbar",
    ) as HTMLElement;

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

    this.stylesAdded = [];
    const stylesToMove: string[] = ["vimMode"];
    const stylesToCopy: string[] = ["autocompletion.css"];
    const that = this;

    this.stylesObserver = new MutationObserver((_mutations, _observer) => {
      // ace adds <style></style> nodes, ours should  be <link rel="stylesheet">
      for (const sheet of document.head.querySelectorAll("style")) {
        if (!that.stylesAdded.includes(sheet.id)) {
          if (stylesToMove.includes(sheet.id)) {
            that.shadowRoot?.appendChild(sheet);
            that.stylesAdded.push(sheet.id);
          } else if (stylesToCopy.includes(sheet.id)) {
            let new_sheet = sheet.cloneNode() as HTMLStyleElement;
            new_sheet.id = `${sheet.id}_clone`;
            that.shadowRoot?.appendChild(new_sheet);
            that.stylesAdded.push(sheet.id);
          }
        }
      }
    });

    this.stylesObserver.observe(document.head, {
      attributes: false,
      childList: true,
      subtree: true,
      characterData: false,
    });

    // Fornow this seems uneeded, tooltips seem to work better on the lightdom
    // this.tooltipObserver = new MutationObserver((_mutations, _observer) => {
    //   // we want the toltips on the shadow-dom not the light dom body
    //   for (const node of document.body.querySelectorAll(
    //     ".ace_tooltip, .ace_editor.ace_autocomplete",
    //   )) {
    //     that.shadowRoot?.appendChild(node);
    //   }
    // });
    // this.tooltipObserver.observe(document.body, {
    //   attributes: false,
    //   childList: true,
    //   subtree: true,
    //   characterData: false,
    // });

    this.sessions.set(this.active_session, this.editor.getSession());
    this.bindSession(
      this.active_session,
      this.sessions.get(this.active_session),
    );
    this.active_line_markers.set(this.active_session, null);

    const worker = await setupLspWorker();
    this.setupLsp(worker);

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
    const that = this;

    window.App!.session.onLoad(((e: CustomEvent) => {
      const session = e.detail;
      const updated_ids: number[] = [];
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
    }) as EventListener);
    window.App!.session.loadFromFragment();

    window.App!.session.onActiveLine(((e: CustomEvent) => {
      const session = e.detail;
      for (const id of session.programs.keys()) {
        const active_line = session.getActiveLine(id);
        if (typeof active_line !== "undefined") {
          const marker = that.active_line_markers.get(id);
          if (marker) {
            that.sessions.get(id)?.removeMarker(marker);
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
    }) as EventListener);

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
      this.shadowRoot!.querySelector(".ace_scrollbar-v")!,
    );

    this.hScrollbarObserver = new IntersectionObserver(
      this._hScrollbarHandler.bind(this),
      { root: null },
    );
    this.hScrollbarObserver.observe(
      this.shadowRoot!.querySelector(".ace_scrollbar-h")!,
    );

    editor.commands.addCommands([{
      name: "showSettingsMenu",
      // description: "Show settings menu",
      bindKey: { win: "Ctrl-,", mac: "Command-,"},
      exec: (_editor: ace.Ace.Editor) => {
        that.settingDialog.show();
      },
    }]);

    this.updateEditorSettings();
    const keyboardRadio = this.renderRoot.querySelector("#editorKeyboardRadio")! as SlRadioGroup;
    const cursorRadio = this.renderRoot.querySelector("#editorCursorRadio")! as SlRadioGroup;
    const fontSize = this.renderRoot.querySelector("#editorFontSize")! as SlInput;
    const relativeLineNumbers = this.renderRoot.querySelector("#editorRelativeLineNumbers")! as SlSwitch;

    keyboardRadio.addEventListener("sl-change", _e => {
      that.settings.keyboard = keyboardRadio.value;
      that.updateEditorSettings();
      that.saveEditorSettings();
    });
    cursorRadio?.addEventListener("sl-change", _e => {
      that.settings.cursor = cursorRadio.value;
      that.updateEditorSettings();
      that.saveEditorSettings();
    });
    fontSize?.addEventListener("sl-change", _e => {
      that.settings.fontSize = parseInt(fontSize.value)
      that.updateEditorSettings();
      that.saveEditorSettings();
    });
    relativeLineNumbers?.addEventListener("sl-change", _e => {
      that.settings.relativeLineNumbers = relativeLineNumbers.checked;
      that.updateEditorSettings();
      that.saveEditorSettings();
    });
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
    var vScrollbar = this.shadowRoot?.querySelector(
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
    var hScrollbar = this.shadowRoot?.querySelector(
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
    this.sessions.get(session_id)?.setValue(content);
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
    this.languageProvider.registerEditor(this.editor);
  }

  activateSession(session_id: number) {
    if (!this.sessions.get(session_id)) {
      return false;
    }
    const session = this.sessions.get(session_id);
    this.editor?.setSession(session);
    const mode = ace.require(this.mode);
    const options = mode?.options ?? {};
    this.languageProvider?.setSessionOptions(session, options);
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

  updateEditorSettings() {
    if (this.settings.keyboard === 'ace') {
      this.editor.setOption('keyboardHandler', null);
    } else {
      this.editor.setOption('keyboardHandler', `ace/keyboard/${this.settings.keyboard}`);
    }
    this.editor.setOption('cursorStyle', this.settings.cursor as any);
    this.editor.setOption('fontSize', this.settings.fontSize);
    this.editor.setOption('relativeLineNumbers', this.settings.relativeLineNumbers);
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
    session?.destroy();
    return true;
  }

  bindSession(session_id: number, session?: EditSession) {
    if (session) {
      session.on("change", () => {
        var val = session.getValue();
        window.App?.session.setProgramCode(session_id, val);
      });
    }
  }
}
