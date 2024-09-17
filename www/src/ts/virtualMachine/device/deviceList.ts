import { html, css, HTMLTemplateResult, PropertyValueMap } from "lit";
import { customElement, query, state } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";

import SlInput from "@shoelace-style/shoelace/dist/components/input/input.js";
import { isSome, structuralEqual } from "utils";

import { repeat } from "lit/directives/repeat.js";
import { default as uFuzzy } from "@leeoniya/ufuzzy";
import { VMSlotAddDialog } from "./slotAddDialog";
import "./addDevice"
import { SlotModifyEvent } from "./slot";
import { computed, ReadonlySignal, Signal, signal, SignalWatcher, watch } from "@lit-labs/preact-signals";
import { ObjectID } from "ic10emu_wasm";
import { VMObjectMixin } from "virtualMachine/baseDevice";

@customElement("vm-device-list")
export class VMDeviceList extends VMObjectMixin(BaseElement) {
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
  }

  devices: ReadonlySignal<ObjectID[]> = (() => {
    let last: ObjectID[] = null;
    return computed(() => {
      const vm = this.vm.value;
      const next: ObjectID[] = vm?.state.vm.value?.objects.flatMap((obj): ObjectID[] => {
        if (!isSome(obj.obj_info.parent_slot) && !isSome(obj.obj_info.root_parent_human)) {
          return [obj.obj_info.id]
        }
        return [];
      })
      if (structuralEqual(last, next)) {
        return last;
      }
      last = next;
      return next;
    });
  })();

  private _filter: Signal<string> = signal("");
  private _filteredDeviceIds: ReadonlySignal<ObjectID[]> = (() => {
    let last: ObjectID[] = null;
    return computed(() => {
      const vm = this.vm.value;
      let next = this.devices.value;
      if (this._filter.value) {
        const datapoints: [string, number][] = [];
        for (const device_id of this.devices.value) {
          const name = vm?.state.getObjectName(device_id).value;
          const prefab = vm?.state.getObjectPrefabName(device_id).value;
          if (name != null) {
            datapoints.push([name, device_id]);
          }
          if (prefab != null) {
            datapoints.push([prefab, device_id]);
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
        next = deviceIds;
      }
      if (structuralEqual(last, next)) {
        return last;
      }
      last = next;
      return next;
    });
  })();

  protected firstUpdated(_changedProperties: PropertyValueMap<any> | Map<PropertyKey, unknown>): void {
    this.renderRoot.querySelector(".device-list").addEventListener(
      "device-modify-slot",
      this._showDeviceSlotDialog.bind(this),
    );
  }

  protected render(): HTMLTemplateResult {
    const deviceCards = computed(() => repeat(
      this.filteredDeviceIds.value,
      (id) => id,
      (id) =>
        html`<vm-device-card .objectID=${id} class="device-list-card">
        </vm-device-card>`,
    ));
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
      <div class="device-list">${watch(deviceCards)}</div>
      <vm-slot-add-dialog></vm-slot-add-dialog>
    `;

    return result;
  }

  @query("vm-slot-add-dialog") accessor slotDialog: VMSlotAddDialog;

  _showDeviceSlotDialog(
    e: CustomEvent<SlotModifyEvent>,
  ) {
    this.slotDialog.show(e.detail.objectID, e.detail.slotIndex);
  }

  get filteredDeviceIds() {
    if (typeof this._filteredDeviceIds !== "undefined") {
      return this._filteredDeviceIds;
    } else {
      return this.devices;
    }
  }

  @query(".device-filter-input") accessor filterInput: SlInput;
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

