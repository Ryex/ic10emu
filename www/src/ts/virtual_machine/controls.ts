import { html, css } from "lit";
import { customElement } from "lit/decorators.js";
import { BaseElement, defaultCss } from "../components";
import { VMActiveICMixin } from "./base_device";
import { structuralEqual } from "../utils";

import "@shoelace-style/shoelace/dist/components/card/card.js";
import "@shoelace-style/shoelace/dist/components/button-group/button-group.js";
import "@shoelace-style/shoelace/dist/components/button/button.js";
import "@shoelace-style/shoelace/dist/components/icon/icon.js";
import "@shoelace-style/shoelace/dist/components/tooltip/tooltip.js";
import "@shoelace-style/shoelace/dist/components/divider/divider.js";
import "@shoelace-style/shoelace/dist/components/select/select.js";
import "@shoelace-style/shoelace/dist/components/badge/badge.js";
import "@shoelace-style/shoelace/dist/components/option/option.js";
import SlSelect from "@shoelace-style/shoelace/dist/components/select/select.js";

@customElement("vm-ic-controls")
export class VMICControls extends VMActiveICMixin(BaseElement) {
  static styles = [
    ...defaultCss,
    css`
      :host {
        display: block;
        box-sizing: border-box;
      }
      .card {
        width: 100%;
        box-sizing: border-box;
      }
      .controls {
        display: flex;
        flex-direction: row;
        font-size: var(--sl-font-size-small);
      }
      .stats {
        font-size: var(--sl-font-size-x-small);
      }
      .device-id {
        margin-left: 2rem;
      }
      .button-group-toolbar sl-button-group:not(:last-of-type) {
        margin-right: var(--sl-spacing-x-small);
      }
      sl-divider {
        --spacing: 0.25rem;
      }

      sl-button[variant="success"] {
        /* Changes the success theme color to purple using primitives */
        --sl-color-success-600: var(--sl-color-purple-700);
      }
      sl-button[variant="primary"] {
        /* Changes the success theme color to purple using primitives */
        --sl-color-primary-600: var(--sl-color-cyan-600);
      }
      sl-button[variant="warning"] {
        /* Changes the success theme color to purple using primitives */
        --sl-color-warning-600: var(--sl-color-amber-600);
      }
    `,
  ];

  constructor() {
    super();
    this.deviceID = window.App!.session.activeIC;
  }

  protected render() {
    const ics = Array.from(window.VM!.ics);
    return html`
      <sl-card class="card">
        <div class="controls" slot="header">
          <sl-button-group>
            <sl-tooltip
              content="Run the active IC through one tick (128 operations)"
            >
              <sl-button
                size="small"
                variant="primary"
                @click=${this._handleRunClick}
              >
                <span>Run</span>
                <sl-icon name="play" label="Run" slot="prefix"></sl-icon>
              </sl-button>
            </sl-tooltip>
            <sl-tooltip content="Run the active IC through a single operations">
              <sl-button
                size="small"
                variant="success"
                @click=${this._handleStepClick}
              >
                <span>Step</span>
                <sl-icon
                  name="chevron-bar-right"
                  label="Step"
                  slot="prefix"
                ></sl-icon>
              </sl-button>
            </sl-tooltip>
            <sl-tooltip content="Reset the active IC">
              <sl-button
                size="small"
                variant="warning"
                @click=${this._handleResetClick}
              >
                <span>Reset</span>
                <sl-icon
                  name="arrow-clockwise"
                  label="Reset"
                  slot="prefix"
                ></sl-icon>
              </sl-button>
            </sl-tooltip>
          </sl-button-group>
          <div class="device-id">
            <sl-select
              hoist
              placement="bottom"
              value="${this.deviceID}"
              @sl-change=${this._handleChangeActiveIC}
            >
              ${ics.map(
                ([id, device], _index) =>
                  html`<sl-option value=${id}>
                    Device:${id} ${device.name ?? device.prefabName}
                  </sl-option>`,
              )}
            </sl-select>
          </div>
        </div>
        <div class="stats">
          <div class="hstack">
            <span>Instruction Pointer</span>
            <span class="ms-auto">${this.icIP}</span>
          </div>
          <sl-divider></sl-divider>
          <div class="hstack">
            <span>Last Run Operations Count</span>
            <span class="ms-auto">${this.icOpCount}</span>
          </div>
          <sl-divider></sl-divider>
          <div class="hstack">
            <span>Last State</span>
            <span class="ms-auto">${this.icState}</span>
          </div>
          <sl-divider></sl-divider>
          <div class="vstack">
            <span>Errors</span>
            ${this.errors.map(
              (err) =>
                html`<div class="hstack">
                  <span>
                    Line: ${err.ParseError.line} -
                    ${err.ParseError.start}:${err.ParseError.end}
                  </span>
                  <span class="ms-auto">${err.ParseError.msg}</span>
                </div>`,
            )}
          </div>
        </div>
      </sl-card>
    `;
  }

  _handleRunClick() {
    window.VM?.run();
  }
  _handleStepClick() {
    window.VM?.step();
  }
  _handleResetClick() {
    window.VM?.reset();
  }

  _handleChangeActiveIC(e: CustomEvent) {
    const select = e.target as SlSelect;
    const icId = parseInt(select.value as string);
    window.App!.session.activeIC = icId;
  }
}
