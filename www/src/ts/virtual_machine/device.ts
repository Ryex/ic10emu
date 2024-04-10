import { Slot } from "ic10emu_wasm";
import { html, css, HTMLTemplateResult, PropertyValueMap } from "lit";
import { customElement, state } from "lit/decorators.js";
import { BaseElement, defaultCss, IC10Details } from "../components";
import { VMDeviceMixin } from "./base_device";

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
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.js";
import { parseNumber, structuralEqual } from "../utils";
import SlSelect from "@shoelace-style/shoelace/dist/components/select/select.js";
import SlDetails from "@shoelace-style/shoelace/dist/components/details/details.js";

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
          id="vmDeviceCard${this.deviceID}Name"
          class="device-name"
          size="small"
          pill
          placeholder="${this.prefabName}"
          @sl-change=${this._handleChangeName}
        >
          <span slot="prefix">Device ${this.deviceID}</span>
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
    window.VM?.setDeviceName(this.deviceID, input.value)
    this.updateDevice();
  }

  _handleChangeField(e: CustomEvent) {
    const input = e.target as SlInput;
    const field = input.getAttribute("key")!;
    const val = parseNumber(input.value);
    window.VM?.setDeviceField(this.deviceID, field, val)
    this.updateDevice();
  }

  _handleChangeSlotField(e: CustomEvent) {
    const input = e.target as SlInput;
    const slot = parseInt(input.getAttribute("slotIndex")!);
    const field = input.getAttribute("key")!;
    const val = parseNumber(input.value);
    window.VM?.setDeviceSlotField(this.deviceID, slot, field, val)
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
