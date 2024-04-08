import { html, css } from "lit";
import { customElement } from "lit/decorators.js";
import { defaultCss } from "../components";
import { VMActiveIC } from "./base_device";

import "@shoelace-style/shoelace/dist/components/card/card.js";
import "@shoelace-style/shoelace/dist/components/icon/icon.js";
import "@shoelace-style/shoelace/dist/components/tooltip/tooltip.js";
import "@shoelace-style/shoelace/dist/components/input/input.js";
import { RegisterSpec } from "ic10emu_wasm";

@customElement("vm-ic-registers")
export class VMICRegisters extends VMActiveIC {
  static styles = [
    ...defaultCss,
    css`
      :host {
      }
      .card {
        --padding: 0.5rem;
        --sl-input-font-size-small: 0.75em
      }
      .card-body {
        display: flex;
        flex-flow: row wrap;
        max-height: 8rem;
        overflow-y: auto;
      }
      .reg-input {
        width: 10rem;
      }
    `,
  ];

  static defaultAliases: [string, number][] = [
    ["sp", 16],
    ["ra", 17],
  ];

  constructor() {
    super();
  }

  protected render() {
    // const inputTypeFromVal = (val: number) => { if (val === Number.NEGATIVE_INFINITY || val === Number.POSITIVE_INFINITY || Number.isNaN(val)) { return "text"; } else { return "number"; } };
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
    const registerAliases: [string, number][] = (
      (
        [...(this.aliases ?? [])].filter(
          ([_alias, target]) =>
            "RegisterSpec" in target && target.RegisterSpec.indirection === 0,
        ) as [string, RegisterSpec][]
      ).map(([alias, target]) => [alias, target.RegisterSpec.target]) as [
        string,
        number,
      ][]
    ).concat(VMICRegisters.defaultAliases);
    return html`
      <sl-card class="card">
        <div class="card-body">
          ${this.registers?.map((val, index) => {
            const aliases = registerAliases
              .filter(([_alias, target]) => index === target)
              .map(([alias, _target]) => alias);
            return html` <sl-input
              type="text"
              value="${displayVal(val)}"
              pattern="${validation}"
              size="small"
              class="reg-input"
            >
              <span slot="prefix">r${index}</span>
              <span slot="suffix">${aliases.join(", ")}</span>
            </sl-input>`;
          })}
        </div>
      </sl-card>
    `;
  }
}
