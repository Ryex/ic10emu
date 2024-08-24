import { html, css, nothing } from "lit";
import { customElement, query } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";

import SlSelect from "@shoelace-style/shoelace/dist/components/select/select.js";
import { computed, Signal, SignalWatcher, watch } from "@lit-labs/preact-signals";
import { FrozenObjectFull } from "ic10emu_wasm";
import { VMObjectMixin } from "./baseDevice";
import { createRef, Ref, ref } from "lit/directives/ref.js";

@customElement("vm-ic-controls")
export class VMICControls extends VMObjectMixin(SignalWatcher(BaseElement)) {

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

  constructor() {
    super();
    this.activeIC.subscribe(() => this.forceSelectUpdate());
    this.icOptions.subscribe(() => this.forceSelectUpdate());
  }

  activeICSelect: Ref<SlSelect> = createRef();

  selectUpdateTimeout: ReturnType<typeof setTimeout> = null;

  forceSelectUpdate() {
    if (this.selectUpdateTimeout) {
      clearTimeout(this.selectUpdateTimeout);
    }
    this.selectUpdateTimeout = setTimeout(() => {
      if (this.activeICSelect.value != null) {
        this.activeICSelect.value.value = this.activeIC.value.toString();
        this.activeICSelect.value.handleValueChange();
      }
    }, 100);
  }

  activeIC = computed(() => {
    return this.vm.value?.activeIC.value
  })

  circuitHolderIds = computed(() => {
    return this.vm.value?.state.circuitHolderIds.value ?? [];
  });

  errors = computed(() => {
    const obj = this.vm.value?.state.getObject(this.activeIC.value).value;
    return obj?.obj_info.compile_errors ?? [];
  });

  icIP = computed(() => {
    const circuit = this.vm.value?.state.getCircuitInfo(this.activeIC.value).value;
    return circuit?.instruction_pointer ?? null;
  });

  icOpCount = computed(() => {
    const circuit = this.vm.value?.state.getCircuitInfo(this.activeIC.value).value;
    return circuit?.yield_instruction_count ?? 0;
  });

  icState = computed(() => {
    const circuit = this.vm.value?.state.getCircuitInfo(this.activeIC.value).value;
    return circuit?.state ?? null;
  });


  icOptions = computed(() => {
    return this.circuitHolderIds.value.map(id => {
      const circuitHolder = computed(() => {
        return this.vm.value?.state.getObject(id).value;
      });

      const prefabName = computed(() => {
        return circuitHolder.value?.obj_info.prefab ?? "";
      });
      const displayName = computed(() => {
        return circuitHolder.value?.obj_info.name ?? circuitHolder.value?.obj_info.prefab ?? "";
      });

      prefabName.subscribe(() => this.forceSelectUpdate());
      displayName.subscribe(() => this.forceSelectUpdate());

      const span = html`<span slot="suffix">${watch(displayName)}</span>`;
      return html`
        <sl-option
          prefabName=${watch(prefabName)}
          .value=${id}
        >
          ${span}
          Device:${id} ${watch(displayName)}
        </sl-option>`
    });
  });

  render() {

    const icErrors = computed(() => {
      return this.errors.value.map(
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
              value="${this.activeIC.value}"
              @sl-change=${this._handleChangeActiveIC}
              class="active-ic-select"
              ${ref(this.activeICSelect)}
            >
              ${watch(this.icOptions)}
            </sl-select>
          </div>
        </div>
        <div class="stats">
          <div class="hstack">
            <span>Instruction Pointer</span>
            <span class="ms-auto">${watch(this.icIP)}</span>
          </div>
          <sl-divider></sl-divider>
          <div class="hstack">
            <span>Last Run Operations Count</span>
            <span class="ms-auto">${watch(this.icOpCount)}</span>
          </div>
          <sl-divider></sl-divider>
          <div class="hstack">
            <span>Last State</span>
            <span class="ms-auto">${watch(this.icState)}</span>
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
