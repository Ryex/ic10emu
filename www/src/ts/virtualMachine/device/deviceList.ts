import { html, css, HTMLTemplateResult, PropertyValueMap } from "lit";
import { customElement, query, state } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";

import SlInput from "@shoelace-style/shoelace/dist/components/input/input.js";
import { structuralEqual } from "utils";

import { repeat } from "lit/directives/repeat.js";
import { default as uFuzzy } from "@leeoniya/ufuzzy";
import { VMSlotAddDialog } from "./slotAddDialog";
import "./addDevice"
import { SlotModifyEvent } from "./slot";
import { computed, Signal, signal, SignalWatcher, watch } from "@lit-labs/preact-signals";
import { globalObjectSignalMap } from "virtualMachine/baseDevice";
import { ObjectID } from "ic10emu_wasm";

@customElement("vm-device-list")
export class VMDeviceList extends SignalWatcher(BaseElement) {
  devices: Signal<ObjectID[]>;
  private _filter: Signal<string> = signal("");
  private _filteredDeviceIds: Signal<number[] | undefined>;

  static styles = [
    ...defaultCss,
    css`
      .header {
        margin-bottom: 1rem;
        padding: 0.25rem 0.25rem;
        align-items: center;
        display: flex;
        flex-direction: row;
        width: 100%;
        box-sizing: border-box;
      }
      .device-list {
        display: flex;
        flex-direction: column;
        box-sizing: border-box;
      }
      .device-list-card {
        width: 100%;
      }
      .device-filter-input {
        margin-left: auto;
      }
    `,
  ];

  constructor() {
    super();
    this.devices = computed(() => {
      const objIds = window.VM.vm.objectIds.value;
      const deviceIds = [];
      for (const id of objIds) {
        const obj = window.VM.vm.objects.get(id);
        const info = obj.value.obj_info;
        if (!(info.parent_slot != null || info.root_parent_human != null)) {
          deviceIds.push(id)
        }
      }
      deviceIds.sort();
      return deviceIds;
    });
    this._filteredDeviceIds = computed(() => {
      if (this._filter.value) {
        const datapoints: [string, number][] = [];
        for (const device_id of this.devices.value) {
          const device = globalObjectSignalMap.get(device_id);
          if (device) {
            const name = device.name.peek();
            const id = device.id.peek();
            const prefab = device.prefabName.peek();
            if (name != null) {
              datapoints.push([name, id]);
            }
            if (prefab != null) {
              datapoints.push([prefab, id]);
            }
          }
        }
        const haystack: string[] = datapoints.map((data) => data[0]);
        const uf = new uFuzzy({});
        const [_idxs, info, order] = uf.search(haystack, this._filter.value, 0, 1e3);

        const filtered = order?.map((infoIdx) => datapoints[info.idx[infoIdx]]);
        const deviceIds: number[] =
          filtered
            ?.map((data) => data[1])
            ?.filter((val, index, arr) => arr.indexOf(val) === index) ?? [];
        return deviceIds;
      } else {
        return Array.from(this.devices.value);
      }
    });
  }

  protected firstUpdated(_changedProperties: PropertyValueMap<any> | Map<PropertyKey, unknown>): void {
    this.renderRoot.querySelector(".device-list").addEventListener(
      "device-modify-slot",
      this._showDeviceSlotDialog.bind(this),
    );
  }

  protected render(): HTMLTemplateResult {
    const deviceCards = repeat(
      this.filteredDeviceIds.value,
      (id) => id,
      (id) =>
        html`<vm-device-card .deviceID=${id} class="device-list-card">
        </vm-device-card>`,
    );
    const numDevices = computed(() => this.devices.value.length);
    const result = html`
      <div class="header">
        <span>
          Devices:
          <sl-badge variant="neutral" pill>${watch(numDevices)}</sl-badge>
        </span>
        <sl-input
          class="device-filter-input"
          placeholder="Filter Devices"
          clearable
          @sl-input=${this._handleFilterInput}
        >
          <sl-icon slot="suffix" name="search"></sl-icon>"
        </sl-input>
        <vm-add-device-button class="ms-auto"></vm-add-device-button>
      </div>
      <div class="device-list">${deviceCards}</div>
      <vm-slot-add-dialog></vm-slot-add-dialog>
    `;

    return result;
  }

  @query("vm-slot-add-dialog") slotDialog: VMSlotAddDialog;

  _showDeviceSlotDialog(
    e: CustomEvent<SlotModifyEvent>,
  ) {
    this.slotDialog.show(e.detail.deviceID, e.detail.slotIndex);
  }

  get filteredDeviceIds() {
    if (typeof this._filteredDeviceIds !== "undefined") {
      return this._filteredDeviceIds;
    } else {
      return this.devices;
    }
  }

  @query(".device-filter-input") filterInput: SlInput;
  get filter() {
    return this._filter.value;
  }

  @state()
  set filter(val: string) {
    this._filter.value = val;
  }

  private filterTimeout: number | undefined;

  _handleFilterInput(_e: CustomEvent) {
    if (this.filterTimeout) {
      clearTimeout(this.filterTimeout);
    }
    const that = this;
    this.filterTimeout = setTimeout(() => {
      that.filter = that.filterInput.value;
      that.filterTimeout = undefined;
    }, 500);
  }
}

