import { HTMLTemplateResult, html, css, CSSResultGroup } from "lit";
import { customElement, property, query } from "lit/decorators.js";
import { BaseElement, defaultCss } from "../components";
import "./nav.ts";

import "@shoelace-style/shoelace/dist/components/split-panel/split-panel.js";

import "../editor";
import { IC10Editor } from "../editor";
import { Session } from "../session";
import { VirtualMachine } from "../virtual_machine";



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
      .z-fix {
        z-index: 900;
      }
      sl-split-panel {
        height: 100%;
      }
    `,
  ];

  editorSettings: { fontSize: number; relativeLineNumbers: boolean };

  get editor() {
    return this.renderRoot.querySelector('ace-ic10') as IC10Editor;
  }

  vm!: VirtualMachine;
  session!: Session;

  constructor() {
    super();
    window.App = this;
    this.session = new Session();
    this.vm = new VirtualMachine();

  }

  protected render(): HTMLTemplateResult {
    return html`
      <div class="app-container">
        <app-nav class=z-fix></app-nav>
        <div class="app-body">
          <sl-split-panel
            style="--min: 20em; --max: calc(100% - 20em);"
            primary="start"
            snap="512px 50%"
            snap-threshold="15"
          >
            <ace-ic10 slot="start"></ace-ic10>
            <div slot="end">Controls</div>
          </sl-split-panel>
        </div>
      </div>
    `;
  }
}

declare global {
  interface Window {
    App?: App;
  }
}

