import { html, css } from "lit";
import { customElement, property } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMObjectMixin } from "virtualMachine/baseDevice";
import { displayNumber, parseNumber } from "utils";
import type { LogicType } from "ic10emu_wasm";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.component.js";
import { computed, Signal, watch } from "@lit-labs/preact-signals";

@customElement("vm-device-fields")
export class VMDeviceSlot extends VMObjectMixin(BaseElement) {
  constructor() {
    super();
  }

  logicFieldNames = computed(() => {
    return this.vm.value?.state.getObjectFieldNames(this.objectIDSignal.value).value;
  });

  render() {
    const inputIdBase = `vmDeviceCard${this.objectID}Field`;
    const fieldsHtml = computed(() => {
      return this.logicFieldNames.value.map((name) => {
        const field = computed(() => {
          return this.vm.value?.state.getObjectField(this.objectIDSignal.value, name).value ?? null;
        });
        const typ = computed(() => {
          return field.value?.field_type ?? null;
        });
        const value = computed(() => {
          return displayNumber(field.value?.value ?? null);
        });
        return html` <sl-input id="${inputIdBase}${name}" key="${name}" value="${watch(value)}" size="small"
        @sl-change=${this._handleChangeField}>
        <span slot="prefix">${name}</span>
        <sl-copy-button slot="suffix" from="${inputIdBase}${name}.value"></sl-copy-button>
        <span slot="suffix">${watch(typ)}</span>
      </sl-input>`;
      })
    });
    return html`
      ${watch(fieldsHtml)}
    `;
  }

  _handleChangeField(e: CustomEvent) {
    const input = e.target as SlInput;
    const field = input.getAttribute("key")! as LogicType;
    const val = parseNumber(input.value);
    window.VM.get().then((vm) => {
      if (!vm.setObjectField(this.objectID, field, val, true)) {
        input.value = displayNumber(this.vm.value?.state.getObjectField(this.objectIDSignal.value, field).value?.value ?? null);
      }
    });
  }
}
