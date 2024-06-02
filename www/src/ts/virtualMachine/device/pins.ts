import { html, css } from "lit";
import { customElement, property } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMTemplateDBMixin, VMObjectMixin } from "virtualMachine/baseDevice";
import SlSelect from "@shoelace-style/shoelace/dist/components/select/select.component.js";
import { ObjectID } from "ic10emu_wasm";

@customElement("vm-device-pins")
export class VMDevicePins extends VMObjectMixin(VMTemplateDBMixin(BaseElement)) {
  constructor() {
    super();
    this.subscribe("ic", "visible-devices");
  }

  render() {
    const pins = new Array(this.numPins ?? 0)
      .fill(true)
      .map((_, index) => this.pins.get(index));
    const visibleDevices = (this.visibleDevices ?? []).map((id) => window.VM.vm.objects.get(id));
    const pinsHtml = pins?.map(
      (pin, index) =>
        html` <sl-select
          hoist
          placement="top"
          clearable
          key=${index}
          value=${pin}
          @sl-change=${this._handleChangePin}
        >
          <span slot="prefix">d${index}</span>
          ${visibleDevices.map(
            (device, _index) => html`
              <sl-option value=${device.obj_info.id.toString()}>
                Device ${device.obj_info.id} :
                ${device.obj_info.name ?? device.obj_info.prefab}
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
    window.VM.get().then((vm) => vm.setDevicePin(this.objectID, pin, val));
    this.updateDevice();
  }
}
