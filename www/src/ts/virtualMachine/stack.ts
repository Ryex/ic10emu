import { html, css, nothing } from "lit";
import { customElement } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMActiveICMixin } from "virtualMachine/baseDevice";

import SlInput from "@shoelace-style/shoelace/dist/components/input/input.js";
import { displayNumber, parseNumber } from "utils";
import { computed, watch } from "@lit-labs/preact-signals";

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
    this.subscribe("active-ic")
  }

  protected render() {
    const sp = computed(() => {
      return this.objectSignals.registers.value != null ? this.objectSignals.registers.value[16] : 0;
    });

    const memoryHtml = this.objectSignals?.memory.peek()?.map((val, index) => {
      const content = computed(() => {
        return sp.value === index ? html`<strong>Stack Pointer</strong>` : nothing;
      });
      const pointerClass = computed(() => {
        return sp.value === index ? "stack-pointer" : nothing;
      });
      const displayVal = computed(() => {
        return displayNumber(this.objectSignals.memory.value[index]);
      });

      return html`
        <sl-tooltip placement="left">
          <div slot="content">
            ${watch(content)}
            Address ${index}
          </div>
          <sl-input
            type="text"
            value="${watch(displayVal)}"
            size="small"
            class="stack-input ${watch(pointerClass)}"
            @sl-change=${this._handleCellChange}
            key=${index}
          >
            <span slot="prefix"> ${index} </span>
          </sl-input>
        </sl-tooltip>
      `;
    }) ?? nothing;

    return html`
      <sl-card class="card">
        <div class="card-body">
          ${memoryHtml}
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
