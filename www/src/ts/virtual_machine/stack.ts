import { html, css } from "lit";
import { customElement } from "lit/decorators.js";
import { defaultCss } from "../components";
import { VMActiveIC } from "./base_device";

import "@shoelace-style/shoelace/dist/components/card/card.js";
import "@shoelace-style/shoelace/dist/components/icon/icon.js";
import "@shoelace-style/shoelace/dist/components/tooltip/tooltip.js";
import "@shoelace-style/shoelace/dist/components/input/input.js";
import { RegisterSpec } from "ic10emu_wasm";

@customElement("vm-ic-stack")
export class VMICStack extends VMActiveIC {
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
    const displayVal = (val: number) => {
      if (Number.POSITIVE_INFINITY === val) {
        return "∞";
      } else if (Number.NEGATIVE_INFINITY === val) {
        return "-∞";
      } else {
        return val.toString();
      }
    };
    const validation =
      "[-+]?(([0-9]+(\\.[0-9]+)?([eE][+-]?[0-9]+)?)|((\\.[0-9]+)([eE][+-]?[0-9]+)?)|([iI][nN][fF][iI][nN][iI][tT][yY]))";
    const sp = this.registers![16];


    return html`
      <sl-card class="card">
        <div class="card-body">
          ${this.stack?.map((val, index) => {
            return html` <sl-input
              type="text"
              value="${displayVal(val)}"
              pattern="${validation}"
              size="small"
              class="stack-input ${sp === index ? "stack-pointer" : "" }"
            >
              <span
                slot="prefix"
              >
                ${index}
              </span>
            </sl-input>`;
          })}
        </div>
      </sl-card>
    `;
  }
}
