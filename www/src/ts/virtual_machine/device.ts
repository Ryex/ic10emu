import { Slot } from "ic10emu_wasm";
import { html, css, HTMLTemplateResult } from "lit";
import { customElement, property, query, state } from "lit/decorators.js";
import { BaseElement, defaultCss } from "../components";
import { VMDeviceMixin } from "./base_device";

import { default as uFuzzy } from "@leeoniya/ufuzzy";

import "@shoelace-style/shoelace/dist/components/card/card.js";
import "@shoelace-style/shoelace/dist/components/icon/icon.js";
import "@shoelace-style/shoelace/dist/components/tooltip/tooltip.js";
import "@shoelace-style/shoelace/dist/components/input/input.js";
import "@shoelace-style/shoelace/dist/components/details/details.js";
import "@shoelace-style/shoelace/dist/components/tab/tab.js";
import "@shoelace-style/shoelace/dist/components/tab-panel/tab-panel.js";
import "@shoelace-style/shoelace/dist/components/tab-group/tab-group.js";
import "@shoelace-style/shoelace/dist/components/copy-button/copy-button.js";
import "@shoelace-style/shoelace/dist/components/select/select.js";
import "@shoelace-style/shoelace/dist/components/badge/badge.js";
import "@shoelace-style/shoelace/dist/components/option/option.js";
import "@shoelace-style/shoelace/dist/components/drawer/drawer.js";
import "@shoelace-style/shoelace/dist/components/icon/icon.js";

import SlInput from "@shoelace-style/shoelace/dist/components/input/input.js";
import { parseNumber, structuralEqual } from "../utils";
import SlSelect from "@shoelace-style/shoelace/dist/components/select/select.js";
import SlDetails from "@shoelace-style/shoelace/dist/components/details/details.js";
import SlDrawer from "@shoelace-style/shoelace/dist/components/drawer/drawer.js";
import { DeviceDB, DeviceDBEntry } from "./device_db";

@customElement("vm-device-card")
export class VMDeviceCard extends VMDeviceMixin(BaseElement) {
  image_err: boolean;

  static styles = [
    ...defaultCss,
    css`
      :host {
        display: block;
        box-sizing: border-box;
      }
      .card {
        width: 100%;
        box-sizing: border-box;
      }
      .image {
        width: 4rem;
        height: 4rem;
      }
      .header {
        display: flex;
        flex-direction: row;
      }
      .header-name {
        display: flex;
        flex-direction: row;
        width: 100%;
        flex-grow: 1;
        align-items: center;
        flex-wrap: wrap;
      }
      .device-name::part(input) {
        width: 10rem;
      }
      .device-id::part(input) {
        width: 2rem;
      }
      .device-name-hash::part(input) {
        width: 7rem;
      }
      sl-divider {
        --spacing: 0.25rem;
      }
      sl-button[variant="success"] {
        /* Changes the success theme color to purple using primitives */
        --sl-color-success-600: var(--sl-color-purple-700);
      }
      sl-button[variant="primary"] {
        /* Changes the success theme color to purple using primitives */
        --sl-color-primary-600: var(--sl-color-cyan-600);
      }
      sl-button[variant="warning"] {
        /* Changes the success theme color to purple using primitives */
        --sl-color-warning-600: var(--sl-color-amber-600);
      }
      sl-tab-group {
        margin-left: 1rem;
        margin-right: 1rem;
        --indicator-color: var(--sl-color-purple-600);
        --sl-color-primary-600: var(--sl-color-purple-600);
      }
      sl-tab::part(base) {
        padding: var(--sl-spacing-small) var(--sl-spacing-medium)
      }
      sl-tab-group::part(base) {
        height: 16rem;
        overflow-y: auto;
      }
    `,
  ];

  onImageErr(e: Event) {
    this.image_err = true;
    console.log("Image load error", e);
  }

  renderHeader(): HTMLTemplateResult {
    const activeIc = window.VM?.activeIC;
    const badges: HTMLTemplateResult[] = [];
    if (this.deviceID == activeIc?.id) {
      badges.push(html`<sl-badge variant="primary" pill pulse>db</sl-badge>`);
    }
    activeIc?.pins?.forEach((id, _index) => {
      if (this.deviceID == id) {
        badges.push(html`<sl-badge variant="success" pill></sl-badge>`);
      }
    }, this);
    return html`
      <sl-tooltip content="${this.prefabName}">
        <img
          class="image"
          src="img/stationpedia/${this.prefabName}.png"
          @onerr=${this.onImageErr}
        />
      </sl-tooltip>
      <div class="header-name">
        <sl-input
          id="vmDeviceCard${this.deviceID}Id"
          class="device-id"
          size="small"
          pill
          value=${this.deviceID}
          disabled
        >
          <span slot="prefix">Id</span>
          <sl-copy-button slot="suffix" value=${this.deviceID}></sl-copy-button>
        </sl-input>
        <sl-input
          id="vmDeviceCard${this.deviceID}Name"
          class="device-name"
          size="small"
          pill
          placeholder="${this.prefabName}"
          @sl-change=${this._handleChangeName}
        >
          <span slot="prefix">Name</span>
          <sl-copy-button
            slot="suffix"
            from="vmDeviceCard${this.deviceID}Name.value"
          ></sl-copy-button>
        </sl-input>
        <sl-input
          id="vmDeviceCard${this.deviceID}NameHash"
          size="small"
          pill
          class="device-name-hash"
          value="${this.nameHash}"
          disabled
        >
          <span slot="prefix">Hash</span>
          <sl-copy-button
            slot="suffix"
            from="vmDeviceCard${this.deviceID}NameHash.value"
          ></sl-copy-button>
        </sl-input>
        ${badges.map((badge) => badge)}
      </div>
    `;
  }

  renderFields(): HTMLTemplateResult {
    const fields = Array.from(this.fields);
    const inputIdBase = `vmDeviceCard${this.deviceID}Field`;
    return html`
      ${fields.map(([name, field], _index, _fields) => {
        return html` <sl-input
          id="${inputIdBase}${name}"
          key="${name}"
          value="${field.value}"
          ?disabled=${field.field_type === "Read"}
          @sl-change=${this._handleChangeField}
        >
          <span slot="prefix">${name}</span>
          <sl-copy-button
            slot="suffix"
            from="${inputIdBase}${name}.value"
          ></sl-copy-button>
          <span slot="suffix">${field.field_type}</span>
        </sl-input>`;
      })}
    `;
  }

  renderSlot(slot: Slot, slotIndex: number): HTMLTemplateResult {
    const fields = Array.from(slot.fields);
    const inputIdBase = `vmDeviceCard${this.deviceID}Slot${slotIndex}Field`;
    return html`
      <sl-card class="slot-card">
        <span slot="header" class="slot-header"
          >${slotIndex} : ${slot.typ}</span
        >
        <div class="slot-fields">
          ${fields.map(
            ([name, field], _index, _fields) => html`
              <sl-input
                id="${inputIdBase}${name}"
                slotIndex=${slotIndex}
                key="${name}"
                value="${field.value}"
                ?disabled=${field.field_type === "Read"}
                @sl-change=${this._handleChangeSlotField}
              >
                <span slot="prefix">${name}</span>
                <sl-copy-button
                  slot="suffix"
                  from="${inputIdBase}${name}.value"
                ></sl-copy-button>
                <span slot="suffix">${field.field_type}</span>
              </sl-input>
            `,
          )}
        </div>
      </sl-card>
    `;
  }

  renderSlots(): HTMLTemplateResult {
    return html`
      <div clas="slots">
        ${this.slots.map((slot, index, _slots) => this.renderSlot(slot, index))}
      </div>
    `;
  }

  renderReagents(): HTMLTemplateResult {
    return html``;
  }

  renderNetworks(): HTMLTemplateResult {
    const vmNetworks = window.VM!.networks;
    return html`
      <div class="networks">
        ${this.connections.map((connection, index, _conns) => {
          const conn =
            typeof connection === "object" ? connection.CableNetwork : null;
          return html`
            <sl-select
              hoist
              placement="top"
              clearable
              key=${index}
              value=${conn}
              ?disabled=${conn === null}
              @sl-change=${this._handleChangeConnection}
            >
              <span slot="prefix">Connection:${index}</span>
              ${vmNetworks.map(
                (net) =>
                  html`<sl-option value=${net}>Network ${net}</sl-option>`,
              )}
            </sl-select>
          `;
        })}
      </div>
    `;
  }
  renderPins(): HTMLTemplateResult {
    const pins = this.pins;
    const visibleDevices = window.VM!.visibleDevices(this.deviceID);
    return html`
      <div class="pins">
        ${pins?.map(
          (pin, index) =>
            html`<sl-select
              hoist
              placement="top"
              clearable
              key=${index}
              value=${pin}
              @sl-change=${this._handleChangePin}
            >
              <span slot="prefix">d${index}</span>
              ${visibleDevices.map(
                (device, _index) =>
                  html`<sl-option value=${device.id}>
                    Device ${device.id} : ${device.name ?? device.prefabName}
                  </sl-option>`,
              )}
            </sl-select>`,
        )}
      </div>
    `;
  }

  render(): HTMLTemplateResult {
    return html`
      <ic10-details class="device-card" open>
        <div class="header" slot="summary">${this.renderHeader()}</div>
        <sl-tab-group>
          <sl-tab slot="nav" panel="fields">Fields</sl-tab>
          <sl-tab slot="nav" panel="slots">Slots</sl-tab>
          <sl-tab slot="nav" panel="reagents" disabled>Reagents</sl-tab>
          <sl-tab slot="nav" panel="networks">Networks</sl-tab>
          <sl-tab slot="nav" panel="pins" ?disabled=${!this.pins}>Pins</sl-tab>

          <sl-tab-panel name="fields">${this.renderFields()}</sl-tab-panel>
          <sl-tab-panel name="slots">${this.renderSlots()}</sl-tab-panel>
          <sl-tab-panel name="reagents">${this.renderReagents()}</sl-tab-panel>
          <sl-tab-panel name="networks">${this.renderNetworks()}</sl-tab-panel>
          <sl-tab-panel name="pins">${this.renderPins()}</sl-tab-panel>
        </sl-tab-group>
      </ic10-details>
    `;
  }

  _handleChangeName(e: CustomEvent) {
    const input = e.target as SlInput;
    window.VM?.setDeviceName(this.deviceID, input.value);
    this.updateDevice();
  }

  _handleChangeField(e: CustomEvent) {
    const input = e.target as SlInput;
    const field = input.getAttribute("key")!;
    const val = parseNumber(input.value);
    window.VM?.setDeviceField(this.deviceID, field, val);
    this.updateDevice();
  }

  _handleChangeSlotField(e: CustomEvent) {
    const input = e.target as SlInput;
    const slot = parseInt(input.getAttribute("slotIndex")!);
    const field = input.getAttribute("key")!;
    const val = parseNumber(input.value);
    window.VM?.setDeviceSlotField(this.deviceID, slot, field, val);
    this.updateDevice();
  }

  _handleChangeConnection(e: CustomEvent) {
    const select = e.target as SlSelect;
    const conn = parseInt(select.getAttribute("key")!);
    const last = this.device.connections[conn];
    const val = select.value ? parseInt(select.value as string) : undefined;
    if (typeof last === "object" && typeof last.CableNetwork === "number") {
      // is there no other connection to the previous network?
      if (
        !this.device.connections.some((other_conn, index) => {
          structuralEqual(last, other_conn) && index !== conn;
        })
      ) {
        this.device.removeDeviceFromNetwork(last.CableNetwork);
      }
    }
    if (typeof val !== "undefined") {
      this.device.addDeviceToNetwork(conn, val);
    } else {
      this.device.setConnection(conn, val);
    }

    this.updateDevice();
  }

  _handleChangePin(e: CustomEvent) {
    const select = e.target as SlSelect;
    const pin = parseInt(select.getAttribute("key")!);
    const val = select.value ? parseInt(select.value as string) : undefined;
    this.device.setPin(pin, val);
    this.updateDevice();
  }
}

@customElement("vm-device-list")
export class VMDeviceList extends BaseElement {
  @state() accessor devices: number[];

  static styles = [
    ...defaultCss,
    css`
      .header {
        margin-botton: 1rem;
        margin-right: 2rem;
        padding: 0.25rem 0.25rem;
        align-items: center;
        display: flex;
        flex-direction: row;
        width: 100%;
        box-sizing: border-box;
      }
      .device-list {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
      }
      .device-list-card {
      }
    `,
  ];

  constructor() {
    super();
    this.devices = window.VM!.deviceIds;
  }

  connectedCallback(): void {
    const root = super.connectedCallback();
    window.VM?.addEventListener(
      "vm-devices-update",
      this._handleDevicesUpdate.bind(this),
    );
    return root;
  }

  _handleDevicesUpdate(e: CustomEvent) {
    const ids = e.detail;
    if (!structuralEqual(this.devices, ids)) {
      this.devices = ids;
    }
  }

  protected render(): HTMLTemplateResult {
    return html`
      <div class="header">
        <span>
          Devices:
          <sl-badge variant="neutral" pill>${this.devices.length}</sl-badge>
        </span>
        <vm-add-device-button class="ms-auto"></vm-add-device-button>
      </div>
      <div class="device-list">
        ${this.devices.map(
          (id, _index, _ids) =>
            html`<vm-device-card
              .deviceID=${id}
              class="device-list-card"
            ></vm-device-card>`,
        )}
      </div>
    `;
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

  @query("sl-drawer") accessor drawer: SlDrawer;
  @query(".device-search-input") accessor searchInput: SlInput;

  private _deviceDB: DeviceDB;
  private _strutures: Map<string, DeviceDBEntry>;

  get deviceDB() {
    return this._deviceDB;
  }

  @state()
  set deviceDB(val: DeviceDB) {
    this._deviceDB = val;
    this._strutures = new Map(
      Object.values(this.deviceDB.db)
        .filter((entry) => this.deviceDB.strutures.includes(entry.name), this)
        .filter(
          (entry) => this.deviceDB.logic_enabled.includes(entry.name),
          this,
        )
        .map((entry) => [entry.name, entry]),
    );
    this.performSearch();
  }

  _filter: string = "";

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
    if (this.filter) {
      const datapoints: [string, string][] = [];
      for (const entry of this._strutures.values()) {
        datapoints.push([entry.name, entry.name], [entry.desc, entry.name]);
      }
      const haystack: string[] = datapoints.map((data) => data[0]);
      const uf = new uFuzzy({});
      const [_idxs, info, order] = uf.search(haystack, this._filter, 0, 1e3);

      const filtered = order?.map((infoIdx) => datapoints[info.idx[infoIdx]]);
      const names =
        filtered
          ?.map((data) => data[1])
          ?.filter((val, index, arr) => arr.indexOf(val) === index) ?? [];

      this._searchResults = names.map((name) => this._strutures.get(name)!);
    } else {
      this._searchResults =
        [] ?? this._strutures ? [...this._strutures.values()] : [];
    }
  }

  connectedCallback(): void {
    const root = super.connectedCallback();
    window.VM!.addEventListener(
      "vm-device-db-loaded",
      this._handleDeviceDBLoad.bind(this),
    );
    return root;
  }

  _handleDeviceDBLoad(e: CustomEvent) {
    this.deviceDB = e.detail;
  }

  renderSearchResults(): HTMLTemplateResult {
    const renderedResults: HTMLTemplateResult[] = this._searchResults?.map(
      (result) =>
        html`<vm-device-template
          name=${result.name}
          class="card"
        ></vm-device-template>`,
    );
    return html`${renderedResults}`;
  }

  render() {
    return html`
      <sl-button
        variant="neutral"
        outline
        pill
        @click=${this._handleAddButtonClick}
      >
        Add Device
      </sl-button>
      <sl-drawer class="add-device-drawer" placement="bottom" no-header>
        <sl-input
          class="device-search-input"
          autofocus
          placeholder="Search For Device"
          clearable
          @sl-input=${this._handleSearchInput}
        >
          <span slot="prefix">Search Structures</span>
          <sl-icon slot="suffix" name="search"></sl-icon>"
        </sl-input>
        <div class="search-results">${this.renderSearchResults()}</div>
        <sl-button
          slot="footer"
          variant="primary"
          @click=${() => {
            this.drawer.hide();
          }}
        >
          Close
        </sl-button>
      </sl-drawer>
    `;
  }

  _handleSearchInput(e: CustomEvent) {
    console.log("search-input", e);
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
  }
}

@customElement("vm-device-template")
export class VmDeviceTemplate extends BaseElement {
  @property({ type: String }) accessor name: string;

  private _deviceDB: DeviceDB;
  private image_err: boolean = false;

  static styles = [
    ...defaultCss,
    css`
      .template-card {
        --padding: var(--sl-spacing-small)
      }
      .image {
        width: 3rem;
        height: 3rem;
      }
      .header {
        display: flex;
        flex-direction: row;
      }
      .card-body {
        // height: 18rem;
        overflow-y: auto;
      }
      sl-tab::part(base) {
        padding: var(--sl-spacing-small) var(--sl-spacing-medium)
      }
      sl-tab-group::part(base) {
        height: 14rem;
        overflow-y: auto;
      }
    `,
  ];

  constructor() {
    super();
    this.deviceDB = window.VM!.db;
  }

  get deviceDB() {
    return this._deviceDB;
  }

  @state()
  set deviceDB(val: DeviceDB) {
    this._deviceDB = val;
  }

  connectedCallback(): void {
    const root = super.connectedCallback();
    window.VM!.addEventListener(
      "vm-device-db-loaded",
      this._handleDeviceDBLoad.bind(this),
    );
    return root;
  }

  _handleDeviceDBLoad(e: CustomEvent) {
    this.deviceDB = e.detail;
  }

  onImageErr(e: Event) {
    this.image_err = true;
    console.log("Image load error", e);
  }

  renderFields(): HTMLTemplateResult {
    const device = this.deviceDB.db[this.name];
    const fields = device.logic ? Object.entries(device.logic) : [];
    return html`
      ${fields.map(([name, field_type], _index, _fields) => {
        return html` <sl-input
          key="${name}"
          value="0"
        >
          <span slot="prefix">${name}</span>
          <span slot="suffix">${field_type}</span>
        </sl-input>`;
      })}
    `;
  }

  renderSlot(slot: Slot, slotIndex: number): HTMLTemplateResult {
    const fields = Array.from(slot.fields);
    return html` <sl-card class="slot-card"> </sl-card> `;
  }

  renderSlots(): HTMLTemplateResult {
    return html` <div clas="slots"></div> `;
  }

  renderReagents(): HTMLTemplateResult {
    return html``;
  }

  renderNetworks(): HTMLTemplateResult {
    const vmNetworks = window.VM!.networks;
    return html` <div class="networks"></div> `;
  }

  renderPins(): HTMLTemplateResult {
    const device = this.deviceDB.db[this.name];
    return html` <div class="pins"></div> `;
  }

  render() {
    const device = this.deviceDB.db[this.name];
    return html`
      <sl-card class="template-card">
        <div class="header" slot="header">
          <sl-tooltip content="${device.name}">
            <img
              class="image"
              src="img/stationpedia/${device.name}.png"
              @onerr=${this.onImageErr}
            />
          </sl-tooltip>
          <div class=vstack>
            <span class="prefab-name">${device.name}</span>
            <span class="prefab-hash">${device.hash}</span>
          </div>
          <sl-button class="ms-auto" pill variant="success">Add <sl-icon slot="prefix" name="plus-lg"></sl-icon></sl-button>
        </div>
        <div class=card-body>

        <sl-tab-group>
          <sl-tab slot="nav" panel="fields">Fields</sl-tab>
          <sl-tab slot="nav" panel="slots">Slots</sl-tab>
          <sl-tab slot="nav" panel="reagents">Reagents</sl-tab>
          <sl-tab slot="nav" panel="networks">Networks</sl-tab>
          <sl-tab slot="nav" panel="pins">Pins</sl-tab>

          <sl-tab-panel name="fields">${this.renderFields()}</sl-tab-panel>
          <sl-tab-panel name="slots">${this.renderSlots()}</sl-tab-panel>
          <sl-tab-panel name="reagents">${this.renderReagents()}</sl-tab-panel>
          <sl-tab-panel name="networks">${this.renderNetworks()}</sl-tab-panel>
          <sl-tab-panel name="pins">${this.renderPins()}</sl-tab-panel>
        </sl-tab-group>

        </div>
      </sl-card>
    `;
  }
}
