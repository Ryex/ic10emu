import { HTMLTemplateResult, html, css, CSSResultGroup } from "lit";
import { customElement, property, query } from "lit/decorators.js";
import { BaseElement, defaultCss } from "../components";
import "./nav";
import "./share";
import { ShareSessionDialog } from "./share";
import "../editor";
import { IC10Editor } from "../editor";
import { Session } from "../session";
import { VirtualMachine } from "../virtualMachine";
import { openFile, saveFile } from "../utils";

import "../virtualMachine/ui";
import "./save";
import { SaveDialog } from "./save";
import "./welcome";
import { AppWelcome } from "./welcome";

declare global {
  const __COMMIT_HASH__: string;
  const __BUILD_DATE__: string;
}

import packageJson from "../../../package.json"
import { until } from "lit/directives/until.js";

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
  @query("app-welcome") appWelcome: AppWelcome;

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

  createRenderRoot(): HTMLElement | DocumentFragment {
    const root = super.createRenderRoot();
    root.addEventListener("app-share-session", this._handleShare.bind(this));
    root.addEventListener("app-open-file", this._handleOpenFile.bind(this));
    root.addEventListener("app-export", this._handleExport.bind(this));
    root.addEventListener("app-save", this._handleSave.bind(this));
    root.addEventListener("app-load", this._handleLoad.bind(this));
    root.addEventListener("app-changelog", this._handleChangelog.bind(this));
    return root;
  }

  protected render(): HTMLTemplateResult {
    const mainBody = window.VM.get().then(vm => {
      return html`
        <sl-split-panel
          style="--min: 20em; --max: calc(100% - 20em);"

          primary="start"
          snap="512px 50%"
          snap-threshold="15"
        >
          <ace-ic10 slot="start"></ace-ic10>
          <div slot="end"><vm-ui></vm-ui></div>
        </sl-split-panel>
      `;
    });
    return html`
      <div class="app-container">
        <app-nav appVer=${this.appVersion} gitVer=${this.gitVer} buildDate=${this.buildDate} ></app-nav>
        <div class="app-body">
          ${until(
            mainBody,
            html`
              <div class="w-full h-full place-content-center">
                <div class="w-full h-fit justify-center">
                  <p class="mt-auto mr-auto ml-auto w-fit text-2xl">
                    Loading Ic10 Virtual Machine
                    <sl-spinner class="self-center" style="font-size: 1.5rem; --track-width: 5px;">
                    </sl-spinner>
                  </p>
                </div>
              </div>
            `
          )}
        </div>
        <session-share-dialog></session-share-dialog>
        <save-dialog></save-dialog>
        <app-welcome @sl-after-hide=${this.afterWelcomeHide}></app-welcome>
      </div>
    `;
  }

  firstUpdated(): void {
    setTimeout(() => {
      this.checkSeenVersion();
    }, 2000);
  }

  checkSeenVersion() {
    const seenVersionsStr = window.localStorage.getItem("seenVersions");
    let seenVersions: string[] = [];
    if (seenVersionsStr !== null && seenVersionsStr.length > 0) {
      try {
        const saved = JSON.parse(seenVersionsStr);
        seenVersions = saved;
      } catch (e) {
        console.log("error pulling seen versions", e);
      }
    }
    const ourVer = `${this.appVersion}_${this.gitVer}_${this.buildDate}`;
    if (!seenVersions.includes(ourVer)) {
      this.appWelcome.show();
    }
  }

  afterWelcomeHide() {
    const seenVersionsStr = window.localStorage.getItem("seenVersions");
    const seenVersions: string[] = [];
    if (seenVersionsStr !== null && seenVersionsStr.length > 0) {
      try {
        const saved = JSON.parse(seenVersionsStr);
        seenVersions.concat(saved);
      } catch (e) {
        console.log("error pulling seen versions", e);
      }
    }
    const unique = new Set(seenVersions);
    const ourVer = `${this.appVersion}_${this.gitVer}_${this.buildDate}`;
    if (this.appWelcome.dontShowAgain) {
      unique.add(ourVer)
    } else {
      unique.delete(ourVer)
    }
    window.localStorage.setItem("seenVersions", JSON.stringify(Array.from(unique)));
  }

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

  _handleLoad(_e: Event) {
    this.saveDialog.show("load");
  }
  _handleOpenFile(_e: Event) {
    openFile(window.Editor.editor);
  }

  _handleChangelog(_e: Event) {
    this.appWelcome.show();
  }
}


