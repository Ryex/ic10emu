import { html, css, HTMLTemplateResult, PropertyValueMap } from "lit";
import { customElement, query, state } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";

import SlInput from "@shoelace-style/shoelace/dist/components/input/input.js";
import { structuralEqual } from "../../utils";

import { repeat } from "lit/directives/repeat.js";
import { default as uFuzzy } from "@leeoniya/ufuzzy";
import { VMSlotAddDialog } from "./slot_add_dialog";
import "./add_device"
import { SlotModifyEvent } from "./slot";

@customElement("vm-device-list")
export class VMDeviceList extends BaseElement {
  @state() devices: number[];

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
    this.devices = [...window.VM.vm.deviceIds];
  }

  connectedCallback(): void {
    super.connectedCallback();
    window.VM.get().then((vm) =>
      vm.addEventListener(
        "vm-devices-update",
        this._handleDevicesUpdate.bind(this),
      ),
    );
  }

  protected firstUpdated(_changedProperties: PropertyValueMap<any> | Map<PropertyKey, unknown>): void {
    this.renderRoot.querySelector(".device-list").addEventListener(
      "device-modify-slot",
      this._showDeviceSlotDialog.bind(this),
    );
  }

  _handleDevicesUpdate(e: CustomEvent) {
    const ids = e.detail;
    if (!structuralEqual(this.devices, ids)) {
      this.devices = ids;
      this.devices.sort();
    }
  }

  protected render(): HTMLTemplateResult {
    const deviceCards = repeat(
      this.filteredDeviceIds,
      (id) => id,
      (id) =>
        html`<vm-device-card .deviceID=${id} class="device-list-card">
        </vm-device-card>`,
    );
    const result = html`
      <div class="header">
        <span>
          Devices:
          <sl-badge variant="neutral" pill>${this.devices.length}</sl-badge>
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

  private _filteredDeviceIds: number[] | undefined;
  private _filter: string = "";

  @query(".device-filter-input") filterInput: SlInput;
  get filter() {
    return this._filter;
  }

  @state()
  set filter(val: string) {
    this._filter = val;
    this.performSearch();
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

  performSearch() {
    if (this._filter) {
      const datapoints: [string, number][] = [];
      for (const device_id of this.devices) {
        const device = window.VM.vm.devices.get(device_id);
        if (device) {
          if (typeof device.name !== "undefined") {
            datapoints.push([device.name, device.id]);
          }
          if (typeof device.prefabName !== "undefined") {
            datapoints.push([device.prefabName, device.id]);
          }
        }
      }
      const haystack: string[] = datapoints.map((data) => data[0]);
      const uf = new uFuzzy({});
      const [_idxs, info, order] = uf.search(haystack, this._filter, 0, 1e3);

      const filtered = order?.map((infoIdx) => datapoints[info.idx[infoIdx]]);
      const deviceIds: number[] =
        filtered
          ?.map((data) => data[1])
          ?.filter((val, index, arr) => arr.indexOf(val) === index) ?? [];
      this._filteredDeviceIds = deviceIds;
    } else {
      this._filteredDeviceIds = undefined;
    }
  }
}

