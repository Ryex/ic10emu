import { html, css } from "lit";
import { customElement, property } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMTemplateDBMixin, VMObjectMixin } from "virtualMachine/baseDevice";
import SlSelect from "@shoelace-style/shoelace/dist/components/select/select.component.js";
import { ObjectID } from "ic10emu_wasm";
import { effect, watch } from "@lit-labs/preact-signals";
import { SlOption } from "@shoelace-style/shoelace";

@customElement("vm-device-pins")
export class VMDevicePins extends VMObjectMixin(VMTemplateDBMixin(BaseElement)) {
  constructor() {
    super();
    // this.subscribe("visible-devices");
  }

  render() {
    const pins = new Array(this.objectSignals.numPins.value ?? 0)
      .fill(true)
      .map((_, index) => this.objectSignals.pins.value.get(index));
    const visibleDevices = (this.objectSignals.visibleDevices.value ?? []);
    const forceSelectUpdate = () => {
      const slSelect = this.renderRoot.querySelector("sl-select") as SlSelect;
      if (slSelect != null) {
        slSelect.handleValueChange();
      }
    };
    const pinsHtml = pins?.map(
      (pin, index) => {
        return html` <sl-select
          hoist
          placement="top"
          clearable
          key=${index}
          value=${pin}
          @sl-change=${this._handleChangePin}
        >
          <span slot="prefix">d${index}</span>
          ${visibleDevices.map(
            (device, _index) => {
              device.id.subscribe((id: ObjectID) => {
                forceSelectUpdate();
              });
              device.displayName.subscribe((_: string) => {
                forceSelectUpdate();
              });
              return html`
                <sl-option value=${watch(device.id)}>
                  Device ${watch(device.id)} :
                  ${watch(device.displayName)}
                </sl-option>
              `
            }

          )}
        </sl-select>`;
      }
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
