import {
  ace,
  Ace,
  Range,
  AceLanguageClient,
  setupLspWorker,
  HoverTooltip,
  AceHidden,
  MarkerGroup,
} from "./ace";

import { LanguageProvider } from "ace-linters/types/language-provider";

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
import { customElement, state, query } from "lit/decorators.js";
import { editorStyles } from "./styles";
import "./shortcuts_ui";
import { AceKeyboardShortcuts } from "./shortcuts_ui";
import {
  LanguageClientConfig,
  ProviderOptions,
} from "ace-linters/types/types/language-service";
import { LineError, ObjectID } from "ic10emu_wasm";
import { marked } from "marked";
import { effect, signal, Signal } from "@lit-labs/preact-signals";
import { App } from "app";
import { VirtualMachine } from "virtualMachine";
import { isSome } from "utils";
import { Session } from "session";

interface SessionStateExtension {
  state?: {
    errorMarkers?: Ace.MarkerGroup;
    activeLineMarker?: ReturnType<Ace.EditSession["addMarker"]>;
    saveTimeout?: ReturnType<typeof setTimeout>;
  }
}

interface MarkerGroupItemExtension {
  tooltipText?: string;
}

type ExtendedEditSession = Ace.EditSession & SessionStateExtension;
type ExtendedMarkerGroupItem = Ace.MarkerGroupItem & MarkerGroupItemExtension

@customElement("ace-ic10")
export class IC10Editor extends BaseElement {
  static styles = [...defaultCss, editorStyles];

  mode: string = "ace/mode/ic10";

  settings: {
    keyboard: string;
    cursor: string;
    fontSize: number;
    relativeLineNumbers: boolean;
  } = {
      keyboard: "ace",
      cursor: "ace",
      fontSize: 16,
      relativeLineNumbers: false,
    };

  sessions: Map<number, ExtendedEditSession> = new Map();

  activeLineMarkers: Map<number, number | null> = new Map();
  languageProvider?: LanguageProvider;

  initialInit: boolean = false;
  aceReady: boolean = false;

  editorDiv: HTMLElement;
  editorContainerDiv: HTMLElement;
  editorStatusbarDiv: HTMLElement;
  editor: Ace.Editor;
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

  @query(".e-kb-shortcuts") kbShortcuts: AceKeyboardShortcuts;

  @query(".e-settings-dialog") settingDialog: SlDialog;

  errorTooltip: AceHidden.HoverTooltip = new HoverTooltip();
  activeLineTooltip: AceHidden.HoverTooltip = new HoverTooltip();

  app: Signal<App> = signal(null);
  vm: Signal<VirtualMachine> = signal(null);

  constructor() {
    super();
    console.log("constructing editor");

    window.Editor = this;
  }

  private async setupApp() {
    this.vm.value = await window.VM.get();
    this.app.value = await window.App.get();
  }

  render() {
    const result = html`
      <div id="editorContainer" style="height: 100%; width: 100%; position: relative; z-index: auto;">
        <div id="editor" style="position: absolute; top: 0; right: 0; bottom: 0; left: 0; z-index: 0; isolation: isolate;">
        </div>
        <div id="editorStatusbar"></div>
      </div>
      <sl-dialog label="Editor Settings" class="dialog-focus e-settings-dialog">
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
        <sl-switch id="editorRelativeLineNumbers" ?checked=${this.settings.relativeLineNumbers}>
          Relative Line Numbers
        </sl-switch>
      </sl-dialog>
      <ace-kb-menu class="e-kb-shortcuts"></ace-kb-menu>
    `;
    return result;
  }

  connectedCallback(): void {
    super.connectedCallback();
    this.loadEditorSettings();
  }

  async firstUpdated() {
    await this.setupApp();

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

    this.stylesObserver = new MutationObserver((_mutations, _observer) => {
      // ace adds <style></style> nodes, ours should  be <link rel="stylesheet">
      for (const sheet of document.head.querySelectorAll("style")) {
        if (!this.stylesAdded.includes(sheet.id)) {
          if (stylesToMove.includes(sheet.id)) {
            this.shadowRoot?.appendChild(sheet);
            this.stylesAdded.push(sheet.id);
          } else if (stylesToCopy.includes(sheet.id)) {
            let new_sheet = sheet.cloneNode() as HTMLStyleElement;
            new_sheet.id = `${sheet.id}_clone`;
            this.shadowRoot?.appendChild(new_sheet);
            this.stylesAdded.push(sheet.id);
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

    const worker = await setupLspWorker();
    this.setupLsp(worker);

    // when the CSS resize Property is added (to a container-div or ace-ic10 )
    // the correct sizing is maintained (after user resize)
    document.addEventListener("mouseup", (e) => {
      this.resizeEditor();
    });

    this.observer = new ResizeObserver((entries) => {
      for (const _entry of entries) {
        this.resizeEditor();
      }
    });

    this.observer.observe(this.editorContainerDiv);
    this.kbShortcuts.editor = this.editor;
    this.kbShortcuts.requestUpdate();

    this.initializeEditor();
  }

  async initializeEditor() {
    let editor = this.editor;

    effect(() => {
      const vm = this.vm.value;
      const vmState = vm?.state
      const circuitHolders = vmState?.circuitHolderIds.value ?? [];
      const seenIds: ObjectID[] = [];
      for (const id of circuitHolders) {
        seenIds.push(id);
        const prog = vmState?.getObjectProgramSource(id);
        this.createOrSetSession(id, prog?.value ?? "");
      }
      const activeSession = this.app.value.session.activeEditorSession.value ?? circuitHolders[0];

      if (isSome(activeSession)) this.activateSession(activeSession);
      for (const [id, _] of this.sessions) {
        if (!seenIds.includes(id)) {
          this.destroySession(id);
        }
      }
    });

    // this.app.value.session.loadFromFragment();

    this.errorTooltip.setDataProvider((e, editor) => {
      const docPos = e.getDocumentPosition();
      const editorSession: ExtendedEditSession = editor.session;
      const errorMarker: ExtendedMarkerGroupItem = editorSession.state?.errorMarkers?.getMarkerAtPosition(docPos);
      if (!errorMarker) return;
      const range: Ace.Range = errorMarker.range;
      if (!range) return;
      if (
        docPos.row < range.start.row ||
        docPos.row > range.end.row ||
        docPos.column < range.start.column ||
        docPos.column > range.end.column) {
        return;
      }
      const domNode = document.createElement("div")
      const tooltipHtml = marked.parseInline(errorMarker.tooltipText?.trim() ?? "", { async: false });
      domNode.innerHTML = tooltipHtml;

      this.errorTooltip.showForRange(editor, range, domNode, e)
    });

    this.errorTooltip.addToEditor(editor);

    this.activeLineTooltip.setDataProvider((e, editor) => {
      const docPos = e.getDocumentPosition();
      const editorSession: ExtendedEditSession = editor.session;
      const activeLineMarker: Ace.MarkerLike = editorSession.getMarkers(true)[editorSession.state?.activeLineMarker];
      if (!activeLineMarker || activeLineMarker.clazz !== "vm_ic_active_line") return;
      const range: Ace.Range = activeLineMarker.range;
      if (!range) return;
      if (docPos.row !== range.start.row) return;

      const domNode = document.createElement("div")
      const activeLine = activeLineMarker.range.start.row;
      const tooltipHtml = marked.parseInline(`Instruction Pointer: Line ${activeLine}`, { async: false });
      domNode.innerHTML = tooltipHtml;

      this.activeLineTooltip.showForRange(editor, range, domNode, e)
    })

    /// not sure a tooltip is needed
    // this.activeLineTooltip.addToEditor(editor);

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

    editor.commands.addCommands([
      {
        name: "showSettingsMenu",
        // description: "Show settings menu",
        bindKey: { win: "Ctrl-,", mac: "Command-," },
        exec: (_editor: Ace.Editor) => {
          this.settingDialog.show();
        },
      },
      {
        name: "showKeyboardShortcuts",
        bindKey: {
          win: "Ctrl-Alt-h",
          mac: "Command-Alt-h",
        },
        exec: (_editor: Ace.Editor) => {
          this.kbShortcuts.show();
        },
      },
    ]);

    this.updateEditorSettings();
    const keyboardRadio = this.renderRoot.querySelector(
      "#editorKeyboardRadio",
    )! as SlRadioGroup;
    const cursorRadio = this.renderRoot.querySelector(
      "#editorCursorRadio",
    )! as SlRadioGroup;
    const fontSize = this.renderRoot.querySelector(
      "#editorFontSize",
    )! as SlInput;
    const relativeLineNumbers = this.renderRoot.querySelector(
      "#editorRelativeLineNumbers",
    )! as SlSwitch;

    keyboardRadio.addEventListener("sl-change", (_e) => {
      this.settings.keyboard = keyboardRadio.value;
      this.updateEditorSettings();
      this.saveEditorSettings();
    });
    cursorRadio?.addEventListener("sl-change", (_e) => {
      this.settings.cursor = cursorRadio.value;
      this.updateEditorSettings();
      this.saveEditorSettings();
    });
    fontSize?.addEventListener("sl-change", (_e) => {
      this.settings.fontSize = parseInt(fontSize.value);
      this.updateEditorSettings();
      this.saveEditorSettings();
    });
    relativeLineNumbers?.addEventListener("sl-change", (_e) => {
      this.settings.relativeLineNumbers = relativeLineNumbers.checked;
      this.updateEditorSettings();
      this.saveEditorSettings();
    });


    this.dispatchEvent(new CustomEvent("editor-ready", { bubbles: true }))
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

  private _resizeEditor() {
    this.editor.resize();
  }

  private _vScrollbarHandler() {
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

  private _hScrollbarHandler() {
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

  createOrSetSession(id: ObjectID, content: string) {
    if (!this.sessions.has(id)) {
      this.newSession(id, content);
    } else {
      const session = this.sessions.get(id);
      if (session.getValue() == content) return;
      session.setValue(content);
    }
  }

  newSession(id: ObjectID, content?: string) {
    if (this.sessions.has(id)) {
      return false;
    }
    const session: ExtendedEditSession = ace.createEditSession(content ?? "", this.mode as any);
    if (!session.state) session.state = {};
    if (!session.state.errorMarkers) {
      session.state.errorMarkers = new MarkerGroup(session);
    }

    effect(() => {
      const sessionErrors = this.vm.value?.state.getProgramErrors(id).value ?? [];

      session.state.errorMarkers.setMarkers(sessionErrors.map((err: LineError): ExtendedMarkerGroupItem => {
        const icError = err.error;
        const lineLength = session.doc.getLine(err.line).length
        if (icError.typ === "ParseError") {
          return {
            range: new Range(icError.line, icError.start, icError.line, icError.end),
            className: "ic10_editor_error_parse",
            tooltipText: `Parse Error: ${icError.msg}`
          };
        } else if (icError.typ === "DuplicateLabel") {
          return {
            range: new Range(icError.line, "label".length + 2, icError.line, "label".length + 2 + icError.label.length),
            className: "ic10_editor_error_duplicate_label",
            tooltipText: `Duplicate Label ${icError.label}: first seen on line ${icError.source_line}`
          };
        } else {
          return {
            range: new Range(err.line, 0, err.line, lineLength),
            className: "ic10_editor_error_runtime",
            tooltipText: `Runtime Error: ${err.msg}`
          };
        }
      }))
    });

    effect(() => {
      const activeLine = this.vm.value?.state.getCircuitInstructionPointer(id).value ?? 0;
      if (session.state.activeLineMarker) {
        session.removeMarker(session.state.activeLineMarker);
      }
      session.state.activeLineMarker = session.addMarker(
        new Range(activeLine, 0, activeLine, 999),
        "vm_ic_active_line",
        "fullLine",
        true,
      );
    })

    session.setOptions({
      firstLineNumber: 0,
    });
    this.sessions.set(id, session);
    this.bindSession(id, session);
  }

  setupLsp(lsp_worker: Worker) {
    const serverData: LanguageClientConfig = {
      module: () => import("ace-linters/build/language-client"),
      modes: "ic10",
      type: "webworker",
      worker: lsp_worker,
    };
    const options: ProviderOptions = {
      functionality: {
        semanticTokens: true,
      },
    };
    // Create a language provider for web worker
    this.languageProvider = AceLanguageClient.for(serverData, options);
    this.languageProvider.registerEditor(this.editor);
    /* TODO: setup a tooltip and marker group for runtime errors
     * https://github.com/ajaxorg/ace/pull/5113/files
     */
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
    if (this.settings.keyboard === "ace") {
      this.editor.setOption("keyboardHandler", null);
    } else {
      this.editor.setOption(
        "keyboardHandler",
        `ace/keyboard/${this.settings.keyboard}`,
      );
    }
    this.editor.setOption("cursorStyle", this.settings.cursor as any);
    this.editor.setOption("fontSize", this.settings.fontSize);
    this.editor.setOption(
      "relativeLineNumbers",
      this.settings.relativeLineNumbers,
    );
  }

  destroySession(session_id: number) {
    if (!this.sessions.has(session_id)) {
      return false;
    }
    if (!(Object.keys(this.sessions).length > 1)) {
      return false;
    }
    const session = this.sessions.get(session_id);
    this.sessions.delete(session_id);
    session?.destroy();
    return true;
  }

  bindSession(session_id: number, session?: ExtendedEditSession) {
    if (session) {
      session.on("change", () => {
        if (session.state?.saveTimeout) {
          clearTimeout(session.state.saveTimeout);
        }
        session.state.saveTimeout = setTimeout(() => {
          var val = session.getValue();
          this.vm.value?.setCode(session_id, val);
        }, 500)
      });
    }
  }
}
