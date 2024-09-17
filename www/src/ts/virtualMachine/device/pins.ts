import { html } from "lit";
import { customElement } from "lit/decorators.js";
import { BaseElement } from "components";
import { VMObjectMixin } from "virtualMachine/baseDevice";
import SlSelect from "@shoelace-style/shoelace/dist/components/select/select.component.js";
import { ObjectID, ObjectTemplate } from "ic10emu_wasm";
import { computed, watch } from "@lit-labs/preact-signals";
import { createRef, ref, Ref } from "lit/directives/ref.js";
import { isSome, range } from "utils";

@customElement("vm-device-pins")
export class VMDevicePins extends VMObjectMixin(BaseElement) {

  forceSelectUpdate(...slSelects: Ref<SlSelect>[]) {
    for (const slSelect of slSelects) {
      if (slSelect.value != null && "handleValueChange" in slSelect.value) {
        slSelect.value.handleValueChange();
      }
    }
  }

  private _pinSelectRefMap: Map<number, Ref<SlSelect>> = new Map();

  getPinSelectRef(index: number): Ref<SlSelect> {
    if (!this._pinSelectRefMap.has(index)) {
      this._pinSelectRefMap.set(index, createRef());
    }
    return this._pinSelectRefMap.get(index);
  }

  visibleDeviceIds = computed(() => {
    const vm = this.vm.value;
    const obj = vm?.state.getObject(this.objectIDSignal.value).value
    return obj?.obj_info.visible_devices ?? [];
  });

  numPins = computed(() => {
    const vm = this.vm.value;
    return vm?.state.getDeviceNumPins(this.objectIDSignal.value).value;
  })

  deviceOptions = computed(() => {
    return this.visibleDeviceIds.value.map(id => {
      const deviceDisplayName = this.vm.value?.state.getObjectDisplayName(id);
      deviceDisplayName.subscribe(() => {
        this.forceSelectUpdate(...this._pinSelectRefMap.values());
      });
      return html`
        <sl-option value=${id}}>
          Device ${id} :
          ${watch(deviceDisplayName)}
        </sl-option>
      `
    });
  });

  render() {
    const pinsHtml = computed(() => {
      return range(this.numPins.value).map(
        index => {
          const selectRef = this.getPinSelectRef(index);
          const pin = computed(() => {
            const vm = this.vm.value;
            return vm?.state.getDevicePin(this.objectIDSignal.value, index).value;
          });

          return html`
            <sl-select
              hoist
              placement="top"
              clearable
              key=${index}
              value=${watch(pin)}
              @sl-change=${this._handleChangePin}
              ${ref(selectRef)}
            >
              <span slot="prefix">d${index}</span>
              ${watch(this.deviceOptions)}
            </sl-select>
          `;
        }
      );
    });
    return html`${watch(pinsHtml)}`;
  }

  _handleChangePin(e: CustomEvent) {
    const select = e.target as SlSelect;
    const pin = parseInt(select.getAttribute("key")!);
    const val = select.value ? parseInt(select.value as string) : undefined;
    window.VM.get().then((vm) => vm.setDevicePin(this.objectID, pin, val));
  }
}
