
import { html, css, HTMLTemplateResult } from "lit";
import { customElement, property, query, state } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";

import SlInput from "@shoelace-style/shoelace/dist/components/input/input.js";
import {
  structuralEqual,
} from "../../utils";

import SlDrawer from "@shoelace-style/shoelace/dist/components/drawer/drawer.js";
import type { DeviceDB, DeviceDBEntry } from "virtual_machine/device_db";
import { repeat } from "lit/directives/repeat.js";
import { cache } from "lit/directives/cache.js";
import { default as uFuzzy } from "@leeoniya/ufuzzy";

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
    const root = super.connectedCallback();
    window.VM.get().then((vm) =>
      vm.addEventListener(
        "vm-devices-update",
        this._handleDevicesUpdate.bind(this),
      ),
    );
    return root;
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
    `;

    return result;
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

@customElement("vm-add-device-button")
export class VMAddDeviceButton extends BaseElement {
  static styles = [
    ...defaultCss,
    css`
      .add-device-drawer {
        --size: 32rem;
      }

      .search-results {
        display: flex;
        flex-direction: row;
        overflow-x: auto;
      }

      .card {
        margin-top: var(--sl-spacing-small);
        margin-right: var(--sl-spacing-small);
      }

      .card + .card {
      }
    `,
  ];

  @query("sl-drawer") drawer: SlDrawer;
  @query(".device-search-input") searchInput: SlInput;

  private _deviceDB: DeviceDB;
  private _strutures: Map<string, DeviceDBEntry> = new Map();
  private _datapoints: [string, string][] = [];
  private _haystack: string[] = [];
  get deviceDB() {
    return this._deviceDB;
  }

  @state()
  set deviceDB(val: DeviceDB) {
    this._deviceDB = val;
    this._strutures = new Map(
      Object.values(this.deviceDB.db)
        .filter((entry) => this.deviceDB.structures.includes(entry.name), this)
        .filter(
          (entry) => this.deviceDB.logic_enabled.includes(entry.name),
          this,
        )
        .map((entry) => [entry.name, entry]),
    );

    const datapoints: [string, string][] = [];
    for (const entry of this._strutures.values()) {
      datapoints.push(
        [entry.title, entry.name],
        [entry.name, entry.name],
        [entry.desc, entry.name],
      );
    }
    const haystack: string[] = datapoints.map((data) => data[0]);
    this._datapoints = datapoints;
    this._haystack = haystack;
    this.performSearch();
  }

  private _filter: string = "";

  get filter() {
    return this._filter;
  }

  @state()
  set filter(val: string) {
    this._filter = val;
    this.performSearch();
  }

  private _searchResults: DeviceDBEntry[];

  private filterTimeout: number | undefined;

  performSearch() {
    if (this._filter) {
      const uf = new uFuzzy({});
      const [_idxs, info, order] = uf.search(
        this._haystack,
        this._filter,
        0,
        1e3,
      );

      const filtered = order?.map(
        (infoIdx) => this._datapoints[info.idx[infoIdx]],
      );
      const names =
        filtered
          ?.map((data) => data[1])
          ?.filter((val, index, arr) => arr.indexOf(val) === index) ?? [];

      this._searchResults = names.map((name) => this._strutures.get(name)!);
    } else {
      // clear our results and prefilter if the filter is empty
      this._searchResults = [];
    }
  }

  connectedCallback(): void {
    const root = super.connectedCallback();
    window.VM.get().then((vm) =>
      vm.addEventListener(
        "vm-device-db-loaded",
        this._handleDeviceDBLoad.bind(this),
      ),
    );
    return root;
  }

  _handleDeviceDBLoad(e: CustomEvent) {
    this.deviceDB = e.detail;
  }

  renderSearchResults() {
    return repeat(
      this._searchResults ?? [],
      (result) => result.name,
      (result) => cache(html`
        <vm-device-template prefab_name=${result.name} class="card" @add-device-template=${this._handleDeviceAdd}>
        </vm-device-template>
      `)
    );

  }

  _handleDeviceAdd() {
    this.drawer.hide();
  }

  render() {
    return html`
      <sl-button variant="neutral" outline pill @click=${this._handleAddButtonClick}>
        Add Device
      </sl-button>
      <sl-drawer class="add-device-drawer" placement="bottom" no-header>
        <sl-input class="device-search-input" autofocus placeholder="Search For Device" clearable
          @sl-input=${this._handleSearchInput}>
          <span slot="prefix">Search Structures</span>
          <sl-icon slot="suffix" name="search"></sl-icon>
        </sl-input>
        <div class="search-results">${this.renderSearchResults()}</div>
        <sl-button slot="footer" variant="primary" @click=${()=> {
          this.drawer.hide();
          }}
          >
          Close
        </sl-button>
      </sl-drawer>
    `;
  }

  _handleSearchInput(e: CustomEvent) {
    if (this.filterTimeout) {
      clearTimeout(this.filterTimeout);
    }
    const that = this;
    this.filterTimeout = setTimeout(() => {
      that.filter = that.searchInput.value;
      that.filterTimeout = undefined;
    }, 200);
  }

  _handleAddButtonClick() {
    this.drawer.show();
    (this.drawer.querySelector(".device-search-input") as SlInput).select();
  }
}

