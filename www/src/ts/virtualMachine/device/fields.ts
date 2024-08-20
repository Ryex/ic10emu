import { html, css } from "lit";
import { customElement, property } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMTemplateDBMixin, VMObjectMixin } from "virtualMachine/baseDevice";
import { displayNumber, parseNumber } from "utils";
import type { LogicType } from "ic10emu_wasm";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.component.js";
import { computed, Signal, watch } from "@lit-labs/preact-signals";

@customElement("vm-device-fields")
export class VMDeviceSlot extends VMObjectMixin(VMTemplateDBMixin(BaseElement)) {
  constructor() {
    super();
    this.setupSignals();
  }

  setupSignals() {
    this.logicFieldNames = computed(() => {
      return Array.from(this.objectSignals.logicFields.value.keys());
    });
  }

  logicFieldNames: Signal<LogicType[]>;

  render() {
    const inputIdBase = `vmDeviceCard${this.objectID}Field`;
    const fieldsHtml = computed(() => {
      return this.logicFieldNames.value.map((name) => {
        const field = computed(() => {
          return this.objectSignals.logicFields.value.get(name);
        });
        const typ = computed(() => {
          return field.value.field_type;
        });
        const value = computed(() => {
          return displayNumber(field.value.value);
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
      if (!vm.setObjectField(this.objectID.peek(), field, val, true)) {
        input.value = this.objectSignals.logicFields.value.get(field).value.toString();
      }
      this.updateObject();
    });
  }
}
