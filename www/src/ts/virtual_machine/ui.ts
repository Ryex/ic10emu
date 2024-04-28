import { html, css } from "lit";
import { customElement } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";

import "./controls";
import "./registers";
import "./stack";
import "./device";
import { ToastMessage } from ".";

@customElement("vm-ui")
export class VMUI extends BaseElement {
  static styles = [
    ...defaultCss,
    css`
      sl-tab-group {
        margin-left: 1rem;
        margin-right: 1rem;
        --indicator-color: var(--sl-color-purple-600);
        --sl-color-primary-600: var(--sl-color-purple-600);
      }
      sl-details::part(header) {
        padding: 0.3rem;
      }
      sl-details::part(content) {
        padding: 0.5rem;
      }
      vm-ic-controls {
        margin-left: 1rem;
        margin-right: 1rem;
        margin-top: 0.5rem;
        flex: 0 0 auto;
      }
      .side-container {
        height: calc(100vh - 3.8rem);
        display: flex;
        flex-direction: column;
      }
      vm-device-list {
        display: flex;
        flex-direction: column;
        flex: 1 1 auto;
        overflow-y: auto;
      }
      .tab-panel {
        height: calc(100vh - 19rem);
        overflow-y: auto;
      }
      .tab-group {
        flex: 1 1 auto;
      }
      sl-tab::part(base) {
        padding: var(--sl-spacing-small) var(--sl-spacing-medium);
      }
    `,
  ];

  constructor() {
    super();
  }

  connectedCallback(): void {
    super.connectedCallback();
    window.VM.get().then(vm => vm.addEventListener("vm-message", this._handleVMMessage.bind(this)));
  }

  _handleVMMessage(e: CustomEvent) {
    const msg: ToastMessage = e.detail;
    const alert = Object.assign(document.createElement("sl-alert"), {
      variant: msg.variant,
      closable: true,
      // duration: 5000,
      innerHTML: `
        <sl-icon slot="icon" name="${msg.icon}"></sl-icon>
        <strong>${msg.title}</strong><br />
        ${msg.msg}
      `,
    });

    document.body.append(alert);
    alert.toast();
  }

  protected render() {
    return html`
      <div class="side-container">
        <vm-ic-controls></vm-ic-controls>
        <sl-tab-group class="tab-group">
          <sl-tab slot="nav" panel="active-ic">Active IC</sl-tab>
          <sl-tab slot="nav" panel="devices">Devices</sl-tab>
          <sl-tab-panel name="active-ic" class="tab-panel">
            <sl-details summary="Registers" open>
              <vm-ic-registers></vm-ic-registers>
            </sl-details>
            <sl-details summary="Stack">
              <vm-ic-stack></vm-ic-stack>
            </sl-details>
          </sl-tab-panel>
          <sl-tab-panel name="devices" class="tab-panel">
            <vm-device-list></vm-device-list>
          </sl-tab-panel>
        </sl-tab-group>
      </div>
    `;
  }
}
