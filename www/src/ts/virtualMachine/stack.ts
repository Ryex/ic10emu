import { html, css, nothing } from "lit";
import { customElement } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMObjectMixin } from "virtualMachine/baseDevice";

import SlInput from "@shoelace-style/shoelace/dist/components/input/input.js";
import { displayNumber, parseNumber, range } from "utils";
import { computed, watch } from "@lit-labs/preact-signals";

@customElement("vm-ic-stack")
export class VMICStack extends VMObjectMixin(BaseElement) {
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

  circuit = computed(() => {
    return this.vm.value?.state.getCircuitInfo(this.vm.value?.activeIC.value).value;
  });

  sp = computed(() => {
    return this.circuit.value?.registers[16] ?? 0;
  });

  socketedIc = computed(() => {
    return this.vm.value?.state.getObject(this.vm.value?.activeIC.value).value?.obj_info.socketed_ic ?? null;
  })

  memorySize = computed(() => {
    return this.vm.value?.state.getObjectMemorySize(this.socketedIc.value).value;
  });

  memoryAt(index: number) {
    return computed(() => {
      return this.vm.value?.state.getObjectMemoryAt(this.socketedIc.value, index).value
    });
  }

  protected render() {
    const memoryHtml = computed(() => range(this.memorySize.value).map(index => {
      const content = computed(() => {
        return this.sp.value === index ? html`<strong>Stack Pointer</strong>` : nothing;
      });
      const pointerClass = computed(() => {
        return this.sp.value === index ? "stack-pointer" : nothing;
      });
      const displayVal = computed(() => {
        return displayNumber(this.memoryAt(index).value);
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
    }));

    return html`
      <sl-card class="card">
        <div class="card-body">
          ${watch(memoryHtml)}
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
