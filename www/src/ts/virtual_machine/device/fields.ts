import { html, css } from "lit";
import { customElement, property } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMTemplateDBMixin, VMObjectMixin } from "virtual_machine/baseDevice";
import { displayNumber, parseNumber } from "utils";
import type { LogicType } from "ic10emu_wasm";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.component.js";

@customElement("vm-device-fields")
export class VMDeviceSlot extends VMObjectMixin(VMTemplateDBMixin(BaseElement)) {
  constructor() {
    super();
    this.subscribe("fields");
  }

  render() {
    const fields = Array.from(this.logicFields.entries());
    const inputIdBase = `vmDeviceCard${this.objectID}Field`;
    return html`
      ${fields.map(([name, field], _index, _fields) => {
      return html` <sl-input id="${inputIdBase}${name}" key="${name}" value="${displayNumber(field.value)}" size="small"
        @sl-change=${this._handleChangeField}>
        <span slot="prefix">${name}</span>
        <sl-copy-button slot="suffix" from="${inputIdBase}${name}.value"></sl-copy-button>
        <span slot="suffix">${field.field_type}</span>
      </sl-input>`;
      })}
    `;
  }

  _handleChangeField(e: CustomEvent) {
    const input = e.target as SlInput;
    const field = input.getAttribute("key")! as LogicType;
    const val = parseNumber(input.value);
    window.VM.get().then((vm) => {
      if (!vm.setObjectField(this.objectID, field, val, true)) {
        input.value = this.logicFields.get(field).value.toString();
      }
      this.updateDevice();
    });
  }
}
