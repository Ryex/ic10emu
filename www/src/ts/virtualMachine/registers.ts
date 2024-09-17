import { html, css, nothing } from "lit";
import { customElement } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMObjectMixin } from "virtualMachine/baseDevice";

import SlInput from "@shoelace-style/shoelace/dist/components/input/input.js";
import { displayNumber, parseNumber, range, structuralEqual } from "utils";
import { computed, ReadonlySignal, Signal, watch } from "@lit-labs/preact-signals";

@customElement("vm-ic-registers")
export class VMICRegisters extends VMObjectMixin(BaseElement) {
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


  circuit = computed(() => {
    return this.vm.value?.state.getCircuitInfo(this.vm.value?.activeIC.value).value;
  });

  registerCount = computed(() => {
    return this.vm.value?.state.getCircuitRegistersCount(this.vm.value.activeIC.value).value;
  })

  registerAliases = (() => {
    let last: [string, number][] = null;
    return computed(() => {
      const aliases = this.vm.value?.state.getCircuitAliases(this.vm.value?.activeIC.value).value
      const forRegisters = [...(Object.entries(aliases ?? {}) ?? [])].flatMap(([alias, target]): [string, number][] => {
        if ("RegisterSpec" in target && target.RegisterSpec.indirection === 0) {
          return [[alias, target.RegisterSpec.target]];
        }
        return [];
      }).concat(VMICRegisters.defaultAliases);
      if (structuralEqual(last, forRegisters)) {
        return last;
      }
      last = forRegisters;
      return forRegisters;
    });
  })();

  aliasesFor(index: number): ReadonlySignal<string[]> {
    return computed(() => {
      return this.registerAliases.value?.flatMap(([alias, target]): string[] => target === index ? [alias] : [])
    });
  }

  registerAt(index: number): ReadonlySignal<number> {
    return computed(() => {
      return this.vm.value?.state.getCircuitRegistersAt(this.vm.value?.activeIC.value, index).value
    })
  }

  protected render() {

    const registerHtml = computed(() => range(this.registerCount.value).map(index => {
      const aliasesList = computed(() => {
        return this.aliasesFor(index).value?.join(", ") ?? nothing;
      })
      const aliasesText = computed(() => {
        return this.aliasesFor(index).value?.join(", ") ?? "None";
      });
      const valDisplay = computed(() => {
        const val = this.registerAt(index).value;
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
    }) ?? nothing);

    return html`
      <sl-card class="card">
        <div class="card-body">
          ${watch(registerHtml)}
        </div>
      </sl-card>
    `;
  }

  _handleCellChange(e: Event) {
    const input = e.target as SlInput;
    const index = parseInt(input.getAttribute("key")!);
    const val = parseNumber(input.value);
    window.VM.vm.setRegister(this.vm.value?.activeIC.value, index, val);
  }
}
