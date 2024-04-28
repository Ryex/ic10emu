
import { html, css } from "lit";
import { customElement, property } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMDeviceDBMixin, VMDeviceMixin } from "virtual_machine/base_device";
import SlSelect from "@shoelace-style/shoelace/dist/components/select/select.component.js";

@customElement("vm-device-pins")
export class VMDevicePins extends VMDeviceMixin(VMDeviceDBMixin(BaseElement)) {
  constructor() {
    super();
    this.subscribe("ic", "visible-devices");
  }

  render() {
    const pins = this.pins;
    const visibleDevices = window.VM.vm.visibleDevices(this.deviceID);
    const pinsHtml = pins?.map(
      (pin, index) =>
        html`
          <sl-select hoist placement="top" clearable key=${index} value=${pin} @sl-change=${this._handleChangePin}>
            <span slot="prefix">d${index}</span>
            ${visibleDevices.map(
            (device, _index) =>
              html`
                <sl-option value=${device.id}>
                  Device ${device.id} : ${device.name ?? device.prefabName}
                </sl-option>
              `,
          )}
          </sl-select>`,
    );
    return pinsHtml;
  }

  _handleChangePin(e: CustomEvent) {
    const select = e.target as SlSelect;
    const pin = parseInt(select.getAttribute("key")!);
    const val = select.value ? parseInt(select.value as string) : undefined;
    window.VM.get().then((vm) => vm.setDevicePin(this.deviceID, pin, val));
    this.updateDevice();
  }

}
