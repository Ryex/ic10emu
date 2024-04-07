import { HTMLTemplateResult, html, css, CSSResultGroup } from "lit";
import { customElement, property, query } from "lit/decorators.js";
import { BaseElement, defaultCss } from "../components";
import "./nav";
import "./share";
import { ShareSessionDialog } from "./share";

import { setBasePath } from '@shoelace-style/shoelace/dist/utilities/base-path.js';

// Set the base path to the folder you copied Shoelace's assets to
setBasePath('/shoelace');

import "@shoelace-style/shoelace/dist/components/split-panel/split-panel.js";

import "../editor";
import { IC10Editor } from "../editor";
import { Session } from "../session";
import { VirtualMachine } from "../virtual_machine";
import { openFile, saveFile } from "../utils";

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
        // z-index: auto;
      }
      // .z-fix {
      //   z-index: 900;
      // }
      sl-split-panel {
        height: 100%;
      }
    `,
  ];

  editorSettings: { fontSize: number; relativeLineNumbers: boolean };

  @query('ace-ic10') accessor editor: IC10Editor;
  @query('session-share-dialog') accessor shareDialog: ShareSessionDialog;

  // get editor() {
  //   return this.renderRoot.querySelector("ace-ic10") as IC10Editor;
  // }

  vm!: VirtualMachine;
  session!: Session;

  constructor() {
    super();
    window.App = this;
    this.session = new Session();
    this.vm = new VirtualMachine();

  }

  protected createRenderRoot(): HTMLElement | DocumentFragment {
    const root = super.createRenderRoot();
    root.addEventListener('app-share-session', this._handleShare.bind(this));
    root.addEventListener('app-open-file', this._handleOpenFile.bind(this));
    root.addEventListener('app-save-as', this._handleSaveAs.bind(this));
    return root;
  }

  protected render(): HTMLTemplateResult {
    return html`
      <div class="app-container">
        <app-nav></app-nav>
        <div class="app-body">
          <sl-split-panel
            style="--min: 20em; --max: calc(100% - 20em);"
            primary="start"
            snap="512px 50%"
            snap-threshold="15"
          >
            <ace-ic10 slot="start" style=""></ace-ic10>
            <div slot="end">Controls</div>
          </sl-split-panel>
        </div>
        <session-share-dialog></session-share-dialog>
      </div>
    `;
  }

  firstUpdated(): void {
  }

  _handleShare(_e: Event) {
    // TODO:
    this.shareDialog.link = window.location.href;
    this.shareDialog.show();
  }

  _handleSaveAs(_e: Event) {
    saveFile(window.Editor.editorValue);
  }

  _handleOpenFile(_e: Event) {
    openFile(window.Editor.editor);
  }

}

declare global {
  interface Window {
    App?: App;
  }
}
