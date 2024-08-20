import { html, css, nothing } from "lit";
import { customElement, query } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { ComputedObjectSignals, globalObjectSignalMap, VMActiveICMixin } from "virtualMachine/baseDevice";

import SlSelect from "@shoelace-style/shoelace/dist/components/select/select.js";
import { computed, Signal, watch } from "@lit-labs/preact-signals";
import { FrozenObjectFull } from "ic10emu_wasm";

@customElement("vm-ic-controls")
export class VMICControls extends VMActiveICMixin(BaseElement) {

  circuitHolders: Signal<ComputedObjectSignals[]>;

  constructor() {
    super();
    this.subscribe("active-ic")
    this.circuitHolders = computed(() => {
      const ids = window.VM.vm.circuitHolderIds.value;
      const circuitHolders = [];
      for (const id of ids) {
        circuitHolders.push(globalObjectSignalMap.get(id));
      }
      return circuitHolders;
    });
  }

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
        flex-grow: 1;
      }
      .button-group-toolbar sl-button-group:not(:last-of-type) {
        margin-right: var(--sl-spacing-x-small);
      }
      .active-ic-select {
        width: 100%;
      }
      sl-divider {
        --spacing: 0.25rem;
      }

      sl-button[variant="success"] {
        /* Changes the success theme color to purple using primitives */
        --sl-color-success-600: var(--sl-color-purple-700);
        --sl-color-success-500: var(--sl-color-purple-600);
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

  @query(".active-ic-select") activeICSelect: SlSelect;

  forceSelectUpdate() {
    if (this.activeICSelect != null) {
      this.activeICSelect.handleValueChange();
    }
  }

  protected render() {
    const icsOptions = computed(() => {
      return this.circuitHolders.value.map((circuitHolder) => {

        circuitHolder.prefabName.subscribe((_) => {this.forceSelectUpdate()});
        circuitHolder.id.subscribe((_) => {this.forceSelectUpdate()});
        circuitHolder.displayName.subscribe((_) => {this.forceSelectUpdate()});

        const span = circuitHolder.name ? html`<span slot="suffix">${watch(circuitHolder.prefabName)}</span>` : nothing ;
        return html`
          <sl-option
            prefabName=${watch(circuitHolder.prefabName)}
            value=${watch(circuitHolder.id)}
          >
            ${span}
            Device:${watch(circuitHolder.id)} ${watch(circuitHolder.displayName)}
          </sl-option>`
      });
    });
    icsOptions.subscribe((_) => {this.forceSelectUpdate()});

    const icErrors = computed(() => {
      return this.objectSignals?.errors.value?.map(
        (err) =>
          typeof err === "object"
            && "ParseError" in err
            ? html`<div class="hstack">
                <span>
                  Line: ${err.ParseError.line} -
                  ${"ParseError" in err ? err.ParseError.start : "N/A"}:${err.ParseError.end}
                </span>
                <span class="ms-auto">${err.ParseError.msg}</span>
              </div>`
            : html`${JSON.stringify(err)}`,
      ) ?? nothing;
    });

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
              size="small"
              placement="bottom"
              value="${watch(this.objectID)}"
              @sl-change=${this._handleChangeActiveIC}
              class="active-ic-select"
            >
              ${watch(icsOptions)}
            </sl-select>
          </div>
        </div>
        <div class="stats">
          <div class="hstack">
            <span>Instruction Pointer</span>
            <span class="ms-auto">${this.objectSignals ? watch(this.objectSignals.icIP) : nothing}</span>
          </div>
          <sl-divider></sl-divider>
          <div class="hstack">
            <span>Last Run Operations Count</span>
            <span class="ms-auto">${this.objectSignals ? watch(this.objectSignals.icOpCount) : nothing}</span>
          </div>
          <sl-divider></sl-divider>
          <div class="hstack">
            <span>Last State</span>
            <span class="ms-auto">${this.objectSignals ? watch(this.objectSignals.icState) : nothing}</span>
          </div>
          <sl-divider></sl-divider>
          <div class="vstack">
            <span>Errors</span>
            ${watch(icErrors)}
          </div>
        </div>
      </sl-card>
    `;
  }

  _handleRunClick() {
    window.VM.get().then((vm) => vm.run());
  }
  _handleStepClick() {
    window.VM.get().then((vm) => vm.step());
  }
  _handleResetClick() {
    window.VM.get().then((vm) => vm.reset());
  }

  _handleChangeActiveIC(e: CustomEvent) {
    const select = e.target as SlSelect;
    const icId = parseInt(select.value as string);
    window.App.app.session.activeIC = icId;
  }
}
