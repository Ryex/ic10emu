import { HTMLTemplateResult, html, css } from "lit";
import { customElement, property, query } from "lit/decorators.js";
import { BaseElement, defaultCss } from "../components";
import "@shoelace-style/shoelace/dist/components/details/details.js";
import "@shoelace-style/shoelace/dist/components/tab/tab.js";
import "@shoelace-style/shoelace/dist/components/tab-panel/tab-panel.js";
import "@shoelace-style/shoelace/dist/components/tab-group/tab-group.js";

import "./controls";
import "./registers";
import "./stack";
import "./device";

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
      }
      .side-container {
        height: 100%
        overflow-y: auto;
      }
    `,
  ];

  constructor() {
    super();
  }

  protected render() {
    return html`
      <div class="side-container">
        <vm-ic-controls></vm-ic-controls>
        <sl-tab-group>
          <sl-tab slot="nav" panel="active-ic">Active IC</sl-tab>
          <sl-tab slot="nav" panel="devices">Devices</sl-tab>
          <sl-tab-panel name="active-ic">
            <sl-details summary="Registers" open>
              <vm-ic-registers></vm-ic-registers>
            </sl-details>
            <sl-details summary="Stack">
              <vm-ic-stack></vm-ic-stack>
            </sl-details>
          </sl-tab-panel>
          <sl-tab-panel name="devices">
            <vm-device-list></vm-device-list>
          </sl-tab-panel>
        </sl-tab-group>
      </div>
    `;
  }
}
