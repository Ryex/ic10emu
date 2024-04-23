import type {
  LogicType,
  Slot, SlotOccupant, SlotLogicType, SlotType
} from "ic10emu_wasm";
import { html, css, HTMLTemplateResult } from "lit";
import { customElement, property, query, state } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMDeviceMixin } from "virtual_machine/base_device";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.js";
import {
  displayNumber,
  parseIntWithHexOrBinary,
  parseNumber
} from "../../utils";
import SlSelect from "@shoelace-style/shoelace/dist/components/select/select.js";
import type { DeviceDB } from "virtual_machine/device_db";
import { SlDialog } from "@shoelace-style/shoelace";


@customElement("vm-device-card")
export class VMDeviceCard extends VMDeviceMixin(BaseElement) {
  image_err: boolean;

  @property({ type: Boolean }) open: boolean;

  constructor() {
    super();
    this.open = false;
  }

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
        flex-grow: 1;
      }
      .header-name {
        display: flex;
        flex-direction: row;
        width: 100%;
        flex-grow: 1;
        align-items: center;
        flex-wrap: wrap;
      }
      .device-card {
        --padding: var(--sl-spacing-small);
      }
      .device-name::part(input) {
        width: 10rem;
      }
      .device-id::part(input) {
        width: 7rem;
      }
      .device-name-hash::part(input) {
        width: 7rem;
      }
      .slot-header.image {
        width: 1.5rem;
        height: 1.5rem;
        border: var(--sl-panel-border-width) solid var(--sl-panel-border-color);
        border-radius: var(--sl-border-radius-medium);
        background-color: var(--sl-color-neutral-0);
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
        padding: var(--sl-spacing-small) var(--sl-spacing-medium);
      }
      sl-tab-group::part(base) {
        max-height: 20rem;
        overflow-y: auto;
      }
      sl-icon-button.remove-button::part(base) {
        color: var(--sl-color-danger-600);
      }
      sl-icon-button.remove-button::part(base):hover,
      sl-icon-button.remove-button::part(base):focus {
        color: var(--sl-color-danger-500);
      }
      sl-icon-button.remove-button::part(base):active {
        color: var(--sl-color-danger-600);
      }
      .remove-dialog-body {
        display: flex;
        flex-direction: row;
      }
      .dialog-image {
        width: 3rem;
        height: 3rem;
      }
    `,
  ];

  private _deviceDB: DeviceDB;

  get deviceDB(): DeviceDB {
    return this._deviceDB;
  }

  @state()
  set deviceDB(val: DeviceDB) {
    this._deviceDB = val;
    this.updateDevice();
    this.requestUpdate();
  }

  connectedCallback(): void {
    super.connectedCallback();
    window.VM.vm.addEventListener(
      "vm-device-db-loaded",
      this._handleDeviceDBLoad.bind(this)
    );
  }

  _handleDeviceDBLoad(e: CustomEvent) {
    this.deviceDB = e.detail;
  }

  onImageErr(e: Event) {
    this.image_err = true;
    console.log("Image load error", e);
  }

  renderHeader(): HTMLTemplateResult {
    const activeIc = window.VM.vm.activeIC;
    const thisIsActiveIc = activeIc.id === this.deviceID;
    const badges: HTMLTemplateResult[] = [];
    if (this.deviceID == activeIc?.id) {
      badges.push(html`<sl-badge variant="primary" pill pulse>db</sl-badge>`);
    }
    activeIc?.pins?.forEach((id, index) => {
      if (this.deviceID == id) {
        badges.push(
          html`<sl-badge variant="success" pill>d${index}</sl-badge>`
        );
      }
    }, this);
    return html`
      <sl-tooltip content="${this.prefabName}">
        <img class="image" src="img/stationpedia/${this.prefabName}.png"
          onerror="this.src = '${VMDeviceCard.transparentImg}'" />
      </sl-tooltip>
      <div class="header-name">
        <sl-input id="vmDeviceCard${this.deviceID}Id" class="device-id" size="small" pill value=${this.deviceID}
          @sl-change=${this._handleChangeID}>
          <span slot="prefix">Id</span>
          <sl-copy-button slot="suffix" value=${this.deviceID}></sl-copy-button>
        </sl-input>
        <sl-input id="vmDeviceCard${this.deviceID}Name" class="device-name" size="small" pill placeholder=${this.prefabName}
          value=${this.name} @sl-change=${this._handleChangeName}>
          <span slot="prefix">Name</span>
          <sl-copy-button slot="suffix" from="vmDeviceCard${this.deviceID}Name.value"></sl-copy-button>
        </sl-input>
        <sl-input id="vmDeviceCard${this.deviceID}NameHash" size="small" pill class="device-name-hash"
          value="${this.nameHash}" disabled>
          <span slot="prefix">Hash</span>
          <sl-copy-button slot="suffix" from="vmDeviceCard${this.deviceID}NameHash.value"></sl-copy-button>
        </sl-input>
        ${badges.map((badge) => badge)}
      </div>
      <div class="ms-auto mt-auto mb-auto me-2">
        <sl-tooltip content=${thisIsActiveIc ? "Removing the selected Active IC is disabled" : "Remove Device"}>
          <sl-icon-button class="remove-button" name="trash" label="Remove Device" ?disabled=${thisIsActiveIc}
            @click=${this._handleDeviceRemoveButton}></sl-icon-button>
        </sl-tooltip>
      </div>
    `;
  }

  renderFields(): HTMLTemplateResult {
    const fields = Array.from(this.fields.entries());
    const inputIdBase = `vmDeviceCard${this.deviceID}Field`;
    return html`
      ${fields.map(([name, field], _index, _fields) => {
      return html` <sl-input id="${inputIdBase}${name}" key="${name}" value="${displayNumber(field.value)}" size="small"
        @sl-change=${this._handleChangeField}>
        <span slot="prefix">${name}</span>
        <sl-copy-button slot="suffix" from="${inputIdBase}${name}.value"></sl-copy-button>
        <span slot="suffix">${field.field_type}</span>
      </sl-input>`;
    })}
    `;
  }

  lookupSlotOccupantImg(
    occupant: SlotOccupant | undefined,
    typ: SlotType
  ): string {
    if (typeof occupant !== "undefined") {
      const hashLookup = (this.deviceDB ?? {}).names_by_hash ?? {};
      const prefabName = hashLookup[occupant.prefab_hash] ?? "UnknownHash";
      return `img/stationpedia/${prefabName}.png`;
    } else {
      return `img/stationpedia/SlotIcon_${typ}.png`;
    }
  }

  _onSlotImageErr(e: Event) {
    console.log("image_err", e);
  }

  static transparentImg = "data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7" as const;

  renderSlot(slot: Slot, slotIndex: number): HTMLTemplateResult {
    const _fields = this.device.getSlotFields(slotIndex);
    const fields = Array.from(_fields.entries());
    const inputIdBase = `vmDeviceCard${this.deviceID}Slot${slotIndex}Field`;
    const slotImg = this.lookupSlotOccupantImg(slot.occupant, slot.typ);
    return html`
      <sl-card class="slot-card">
        <img slot="header" class="slot-header image" src="${slotImg}" onerror="this.src = '${VMDeviceCard.transparentImg}'" />
        <span slot="header" class="slot-header">${slotIndex} : ${slot.typ}</span>
        ${typeof slot.occupant !== "undefined"
        ? html`
                <span slot="header" class="slot-header">
                  Occupant: ${slot.occupant.id} : ${slot.occupant.prefab_hash}
                </span>
                <span slot="header" class="slot-header">
                  Quantity: ${slot.occupant.quantity}/
                  ${slot.occupant.max_quantity}
                </span>
              `
        : ""}
        <div class="slot-fields">
          ${fields.map(
          ([name, field], _index, _fields) => html`
          <sl-input
            id="${inputIdBase}${name}"
            slotIndex=${slotIndex}
            key="${name}"
            value="${displayNumber(field.value)}"
            size="small"
            @sl-change=${this._handleChangeSlotField}
          >
            <span slot="prefix">${name}</span>
            <sl-copy-button slot="suffix" from="${inputIdBase}${name}.value"></sl-copy-button>
            <span slot="suffix">${field.field_type}</span>
          </sl-input>
          `
        )}
        </div>
      </sl-card>
    `;
  }

  renderSlots(): HTMLTemplateResult {
    return html`
      <div class="slots">
        ${this.slots.map((slot, index, _slots) => this.renderSlot(slot, index))}
      </div>
    `;
  }

  renderReagents(): HTMLTemplateResult {
    return html``;
  }

  renderNetworks(): HTMLTemplateResult {
    const vmNetworks = window.VM.vm.networks;
    const networks = this.connections.map((connection, index, _conns) => {
      const conn = typeof connection === "object" ? connection.CableNetwork : null;
      return html`
        <sl-select hoist placement="top" clearable key=${index} value=${conn?.net} ?disabled=${conn === null}
          @sl-change=${this._handleChangeConnection}>
          <span slot="prefix">Connection:${index} </span>
          ${vmNetworks.map(
        (net) => html`<sl-option value=${net}>Network ${net}</sl-option>`
      )}
          <span slot="prefix"> ${conn?.typ} </span>
        </sl-select>
      `;
    });
    return html`
      <div class="networks">
        ${networks}
      </div>
    `;
  }
  renderPins(): HTMLTemplateResult {
    const pins = this.pins;
    const visibleDevices = window.VM.vm.visibleDevices(this.deviceID);
    const pinsHtml = pins?.map(
      (pin, index) => html`
          <sl-select hoist placement="top" clearable key=${index} value=${pin} @sl-change=${this._handleChangePin}>
            <span slot="prefix">d${index}</span>
            ${visibleDevices.map(
        (device, _index) => html`
                <sl-option value=${device.id}>
                  Device ${device.id} : ${device.name ?? device.prefabName}
                </sl-option>
              `
      )}
          </sl-select>`
    );
    return html`
      <div class="pins" >
        ${pinsHtml}
      </div>
    `;
  }

  render(): HTMLTemplateResult {
    return html`
      <ic10-details class="device-card" ?open=${this.open}>
        <div class="header" slot="summary">${this.renderHeader()}</div>
        <sl-tab-group>
          <sl-tab slot="nav" panel="fields" active>Fields</sl-tab>
          <sl-tab slot="nav" panel="slots">Slots</sl-tab>
          <sl-tab slot="nav" panel="reagents" disabled>Reagents</sl-tab>
          <sl-tab slot="nav" panel="networks">Networks</sl-tab>
          <sl-tab slot="nav" panel="pins" ?disabled=${!this.pins}>Pins</sl-tab>

          <sl-tab-panel name="fields" active>${this.renderFields()}</sl-tab-panel>
          <sl-tab-panel name="slots">${this.renderSlots()}</sl-tab-panel>
          <sl-tab-panel name="reagents">${this.renderReagents()}</sl-tab-panel>
          <sl-tab-panel name="networks">${this.renderNetworks()}</sl-tab-panel>
          <sl-tab-panel name="pins">${this.renderPins()}</sl-tab-panel>
        </sl-tab-group>
      </ic10-details>
      <sl-dialog class="remove-device-dialog" no-header @sl-request-close=${this._preventOverlayClose}>
        <div class="remove-dialog-body">
          <img class="dialog-image mt-auto mb-auto me-2" src="img/stationpedia/${this.prefabName}.png"
            onerror="this.src = '${VMDeviceCard.transparentImg}'" />
          <div class="flex-g">
            <p><strong>Are you sure you want to remove this device?</strong></p>
            <span>Id ${this.deviceID} : ${this.name ?? this.prefabName}</span>
          </div>
        </div>
        <div slot="footer">
          <sl-button variant="primary" autofocus @click=${this._closeRemoveDialog}>Close</sl-button>
          <sl-button variant="danger" @click=${this._removeDialogRemove}>Remove</sl-button>
        </div>
      </sl-dialog>
    `;
  }

  @query(".remove-device-dialog") removeDialog: SlDialog;

  _preventOverlayClose(event: CustomEvent) {
    if (event.detail.source === "overlay") {
      event.preventDefault();
    }
  }

  _closeRemoveDialog() {
    this.removeDialog.hide();
  }

  _handleChangeID(e: CustomEvent) {
    const input = e.target as SlInput;
    const val = parseIntWithHexOrBinary(input.value);
    if (!isNaN(val)) {
      window.VM.get().then(vm => {
        if (!vm.changeDeviceId(this.deviceID, val)) {
          input.value = this.deviceID.toString();
        }
      });
    } else {
      input.value = this.deviceID.toString();
    }
  }

  _handleChangeName(e: CustomEvent) {
    const input = e.target as SlInput;
    const name = input.value.length === 0 ? undefined : input.value;
    window.VM.get().then(vm => {
      if (!vm.setDeviceName(this.deviceID, name)) {
        input.value = this.name;
      }
      this.updateDevice();
    });
  }

  _handleChangeField(e: CustomEvent) {
    const input = e.target as SlInput;
    const field = input.getAttribute("key")! as LogicType;
    const val = parseNumber(input.value);
    window.VM.get().then((vm) => {
      if (!vm.setDeviceField(this.deviceID, field, val, true)) {
        input.value = this.fields.get(field).value.toString();
      }
      this.updateDevice();
    });
  }

  _handleChangeSlotField(e: CustomEvent) {
    const input = e.target as SlInput;
    const slot = parseInt(input.getAttribute("slotIndex")!);
    const field = input.getAttribute("key")! as SlotLogicType;
    const val = parseNumber(input.value);
    window.VM.get().then((vm) => {
      if (!vm.setDeviceSlotField(this.deviceID, slot, field, val, true)) {
        input.value = this.device.getSlotField(slot, field).toString();
      }
      this.updateDevice();
    });
  }

  _handleDeviceRemoveButton(_e: Event) {
    this.removeDialog.show();
  }

  _removeDialogRemove() {
    this.removeDialog.hide();
    window.VM.get().then((vm) => vm.removeDevice(this.deviceID));
  }

  _handleChangeConnection(e: CustomEvent) {
    const select = e.target as SlSelect;
    const conn = parseInt(select.getAttribute("key")!);
    const val = select.value ? parseInt(select.value as string) : undefined;
    window.VM.get().then((vm) => vm.setDeviceConnection(this.deviceID, conn, val)
    );
    this.updateDevice();
  }

  _handleChangePin(e: CustomEvent) {
    const select = e.target as SlSelect;
    const pin = parseInt(select.getAttribute("key")!);
    const val = select.value ? parseInt(select.value as string) : undefined;
    window.VM.get().then((vm) => vm.setDevicePin(this.deviceID, pin, val));
    this.updateDevice();
  }
}

