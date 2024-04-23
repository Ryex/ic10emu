import { html, css } from "lit";
import { customElement } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMActiveICMixin } from "virtual_machine/base_device";

import "@shoelace-style/shoelace/dist/components/card/card.js";
import "@shoelace-style/shoelace/dist/components/icon/icon.js";
import "@shoelace-style/shoelace/dist/components/tooltip/tooltip.js";
import "@shoelace-style/shoelace/dist/components/input/input.js";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.js";
import { displayNumber, parseNumber } from "utils";

@customElement("vm-ic-stack")
export class VMICStack extends VMActiveICMixin(BaseElement) {
  static styles = [
    ...defaultCss,
    css`
      :host {
      }
      .card {
        --padding: 0.5rem;
        --sl-input-font-size-small: 0.75em;
      }
      .card-body {
        display: flex;
        flex-flow: row wrap;
        max-height: 15rem;
        overflow-y: auto;
      }
      .stack-input {
        width: 8rem;
      }
      .stack-pointer::part(prefix) {
        background: rgb(121, 82, 179);
      }
      sl-input::part(prefix) {
        padding-right: 0.25rem;
      }
    `,
  ];

  constructor() {
    super();
  }

  protected render() {
    const sp = this.registers![16];

    return html`
      <sl-card class="card">
        <div class="card-body">
          ${this.stack?.map((val, index) => {
            return html`
              <sl-tooltip placement="left">
                <div slot="content">
                  ${sp === index ? html`<strong>Stack Pointer</strong>` : ""}
                  Address ${index}
                </div>
                <sl-input
                  type="text"
                  value="${displayNumber(val)}"
                  size="small"
                  class="stack-input ${sp === index ? "stack-pointer" : ""}"
                  @sl-change=${this._handleCellChange}
                  key=${index}
                >
                  <span slot="prefix"> ${index} </span>
                </sl-input>
              </sl-tooltip>
            `;
          })}
        </div>
      </sl-card>
    `;
  }

  _handleCellChange(e: Event) {
    const input = e.target as SlInput;
    const index = parseInt(input.getAttribute("key")!);
    const val = parseNumber(input.value);
    window.VM.get().then(vm => vm.setStack(index, val));
  }
}
