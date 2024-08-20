import { html, css, nothing } from "lit";
import { customElement } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMActiveICMixin } from "virtualMachine/baseDevice";

import { RegisterSpec } from "ic10emu_wasm";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.js";
import { displayNumber, parseNumber } from "utils";
import { computed, Signal, watch } from "@lit-labs/preact-signals";

@customElement("vm-ic-registers")
export class VMICRegisters extends VMActiveICMixin(BaseElement) {
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
        max-height: 8rem;
        overflow-y: auto;
      }
      .reg-input {
        width: 10rem;
      }
      .tooltip {
        --max-width: 6rem;
      }
    `,
  ];

  static defaultAliases: [string, number][] = [
    ["sp", 16],
    ["ra", 17],
  ];

  constructor() {
    super();
    this.subscribe("active-ic")
  }

  protected render() {
    const registerAliases: Signal<[string, number][]> = computed(() => {
      return [...(Array.from(this.objectSignals.aliases.value?.entries() ?? []))].flatMap(
        ([alias, target]) => {
          if ("RegisterSpec" in target && target.RegisterSpec.indirection === 0) {
            return [[alias, target.RegisterSpec.target]] as [string, number][];
          } else {
            return [] as [string, number][];
          }
        }
      ).concat(VMICRegisters.defaultAliases);
    });

    const registerHtml = this.objectSignals?.registers.peek().map((val, index) => {
      const aliases = computed(() => {
        return registerAliases.value
          .filter(([_alias, target]) => index === target)
          .map(([alias, _target]) => alias);
      });
      const aliasesList = computed(() => {
        return aliases.value.join(", ");
      });
      const aliasesText = computed(() => {
        return aliasesList.value || "None";
      });
      const valDisplay = computed(() => {
        const val = this.objectSignals.registers.value[index];
        return displayNumber(val);
      });
      return html`
        <sl-tooltip placement="left" class="tooltip">
          <div slot="content">
            <strong>Register r${index}</strong> Aliases:
            <em>${watch(aliasesText)}</em>
          </div>
          <sl-input
            type="text"
            value="${watch(valDisplay)}"
            size="small"
            class="reg-input"
            @sl-change=${this._handleCellChange}
            key=${index}
          >
            <span slot="prefix">r${index}</span>
            <span slot="suffix">${watch(aliasesList)}</span>
          </sl-input>
        </sl-tooltip>
      `;
    }) ?? nothing;

    return html`
      <sl-card class="card">
        <div class="card-body">
          ${registerHtml}
        </div>
      </sl-card>
    `;
  }

  _handleCellChange(e: Event) {
    const input = e.target as SlInput;
    const index = parseInt(input.getAttribute("key")!);
    const val = parseNumber(input.value);
    window.VM.vm.setRegister(index, val);
  }
}
