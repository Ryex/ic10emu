import { HTMLTemplateResult, html, css, CSSResultGroup } from "lit";
import { customElement, property, query } from "lit/decorators.js";
import { BaseElement, defaultCss } from "../components";
import "./nav";
import "./share";
import { ShareSessionDialog } from "./share";

import { setBasePath } from "@shoelace-style/shoelace/dist/utilities/base-path.js";

// Set the base path to the folder you copied Shoelace's assets to
setBasePath("shoelace");

import "@shoelace-style/shoelace/dist/components/split-panel/split-panel.js";

import "../editor";
import { IC10Editor } from "../editor";
import { Session } from "../session";
import { VirtualMachine } from "../virtual_machine";
import { openFile, saveFile } from "../utils";

import "../virtual_machine/ui";
import "./save";
import { SaveDialog } from "./save";

declare global {
  const __COMMIT_HASH__: string;
  const __BUILD_DATE__: string;
}

import packageJson from "../../../package.json"

@customElement("ic10emu-app")
export class App extends BaseElement {
  static styles = [
    ...defaultCss,
    css`
      :host {
        height: 100vh;
        display: block;
      }
      .app-container {
        display: flex;
        flex-direction: column;
        height: 100%;
      }
      .app-body {
        flex-grow: 1;
      }
      sl-split-panel {
        height: 100%;
      }
    `,
  ];

  appVersion = packageJson.version;
  gitVer = __COMMIT_HASH__;
  buildDate = __BUILD_DATE__;

  editorSettings: { fontSize: number; relativeLineNumbers: boolean };

  @query("ace-ic10") editor: IC10Editor;
  @query("session-share-dialog") shareDialog: ShareSessionDialog;
  @query("save-dialog") saveDialog: SaveDialog;

  // get editor() {
  //   return this.renderRoot.querySelector("ace-ic10") as IC10Editor;
  // }

  vm: VirtualMachine;
  session: Session;

  constructor() {
    super();
    this.session = new Session(this);
    this.vm = new VirtualMachine(this);
    window.App.set(this);
  }

  protected createRenderRoot(): HTMLElement | DocumentFragment {
    const root = super.createRenderRoot();
    root.addEventListener("app-share-session", this._handleShare.bind(this));
    root.addEventListener("app-open-file", this._handleOpenFile.bind(this));
    root.addEventListener("app-export", this._handleExport.bind(this));
    root.addEventListener("app-save", this._handleSave.bind(this));
    return root;
  }

  protected render(): HTMLTemplateResult {
    return html`
      <div class="app-container">
        <app-nav appVer=${this.appVersion} gitVer=${this.gitVer} buildDate=${this.buildDate} ></app-nav>
        <div class="app-body">
          <sl-split-panel
            style="--min: 20em; --max: calc(100% - 20em);"
            primary="start"
            snap="512px 50%"
            snap-threshold="15"
          >
            <ace-ic10 slot="start"></ace-ic10>
            <div slot="end"><vm-ui></vm-ui></div>
          </sl-split-panel>
        </div>
        <session-share-dialog></session-share-dialog>
        <save-dialog></save-dialog>
      </div>
    `;
  }

  firstUpdated(): void {}

  _handleShare(_e: Event) {
    // TODO:
    this.shareDialog.link = window.location.href;
    this.shareDialog.show();
  }

  _handleExport(_e: Event) {
    saveFile(window.Editor.editorValue);
  }

  _handleSave(_e: Event) {
    this.saveDialog.show("save");
  }
  _handleOpenFile(_e: Event) {
    openFile(window.Editor.editor);
  }
}


