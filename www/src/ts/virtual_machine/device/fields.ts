import { html, css } from "lit";
import { customElement, property } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMDeviceDBMixin, VMDeviceMixin } from "virtual_machine/base_device";
import { displayNumber, parseNumber } from "utils";
import type { LogicType } from "ic10emu_wasm";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.component.js";

@customElement("vm-device-fields")
export class VMDeviceSlot extends VMDeviceMixin(VMDeviceDBMixin(BaseElement)) {
  constructor() {
    super();
    this.subscribe("fields");
  }

  render() {
    const fields = Array.from(this.fields.entries());
    const inputIdBase = `vmDeviceCard${this.deviceID}Field`;
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
      if (!vm.setDeviceField(this.deviceID, field, val, true)) {
        input.value = this.fields.get(field).value.toString();
      }
      this.updateDevice();
    });
  }
}
