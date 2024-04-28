import { html, css, HTMLTemplateResult } from "lit";
import { customElement, property, query, state } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMDeviceDBMixin, VMDeviceMixin } from "virtual_machine/base_device";
import SlSelect from "@shoelace-style/shoelace/dist/components/select/select.component.js";
import { parseIntWithHexOrBinary, parseNumber } from "utils";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.component.js";
import SlDialog from "@shoelace-style/shoelace/dist/components/dialog/dialog.component.js";
import "./slot";
import "./fields";
import "./pins";
import { until } from "lit/directives/until.js";
import { repeat } from "lit/directives/repeat.js";

export type CardTab = "fields" | "slots" | "reagents" | "networks" | "pins";

@customElement("vm-device-card")
export class VMDeviceCard extends VMDeviceDBMixin(VMDeviceMixin(BaseElement)) {
  image_err: boolean;

  @property({ type: Boolean }) open: boolean;

  constructor() {
    super();
    this.open = false;
    this.subscribe(
      "prefabName",
      "name",
      "nameHash",
      "reagents",
      "slots-count",
      "reagents",
      "connections",
      "active-ic",
    );
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
        max-height: 30rem;
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

  _handleDeviceDBLoad(e: CustomEvent<any>): void {
    super._handleDeviceDBLoad(e);
    this.updateDevice();
  }

  onImageErr(e: Event) {
    this.image_err = true;
    console.log("Image load error", e);
  }

  renderHeader(): HTMLTemplateResult {
    const thisIsActiveIc = this.activeICId === this.deviceID;
    const badges: HTMLTemplateResult[] = [];
    if (thisIsActiveIc) {
      badges.push(html`<sl-badge variant="primary" pill pulse>db</sl-badge>`);
    }
    const activeIc = window.VM.vm.activeIC;
    activeIc?.pins?.forEach((id, index) => {
      if (this.deviceID == id) {
        badges.push(
          html`<sl-badge variant="success" pill>d${index}</sl-badge>`,
        );
      }
    }, this);
    return html`
      <sl-tooltip content="${this.prefabName}">
        <img class="image me-2" src="img/stationpedia/${this.prefabName}.png"
          onerror="this.src = '${VMDeviceCard.transparentImg}'" />
      </sl-tooltip>
      <div class="header-name">
        <sl-input id="vmDeviceCard${this.deviceID}Id" class="device-id me-1" size="small" pill value=${this.deviceID}
          @sl-change=${this._handleChangeID}>
          <span slot="prefix">Id</span>
          <sl-copy-button slot="suffix" .value=${this.deviceID}></sl-copy-button>
        </sl-input>
        <sl-input id="vmDeviceCard${this.deviceID}Name" class="device-name me-1" size="small" pill
          placeholder=${this.prefabName} value=${this.name} @sl-change=${this._handleChangeName}>
          <span slot="prefix">Name</span>
          <sl-copy-button slot="suffix" from="vmDeviceCard${this.deviceID}Name.value"></sl-copy-button>
        </sl-input>
        <sl-input id="vmDeviceCard${this.deviceID}NameHash" size="small" pill class="device-name-hash me-1"
          value="${this.nameHash}" readonly>
          <span slot="prefix">Hash</span>
          <sl-copy-button slot="suffix" from="vmDeviceCard${this.deviceID}NameHash.value"></sl-copy-button>
        </sl-input>
        ${badges.map((badge) => badge)}
      </div>
      <div class="ms-auto mt-auto mb-auto me-2">
        <sl-tooltip content=${thisIsActiveIc ? "Removing the selected Active IC is disabled" : "Remove Device" }>
          <sl-icon-button class="remove-button" name="trash" label="Remove Device" ?disabled=${thisIsActiveIc}
            @click=${this._handleDeviceRemoveButton}></sl-icon-button>
        </sl-tooltip>
      </div>
    `;
  }

  renderFields() {
    return this.delayRenderTab(
      "fields",
      html`<vm-device-fields .deviceID=${this.deviceID}></vm-device-fields>`,
    );
  }

  _onSlotImageErr(e: Event) {
    console.log("image_err", e);
  }

  static transparentImg =
    "data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7" as const;

  async renderSlots() {
    return this.delayRenderTab(
      "slots",
      html`
        <div class="flex flex-row flex-wrap">
          ${repeat(this.slots,
            (slot, index) => slot.typ + index.toString(),
            (_slot, index) => html`
              <vm-device-slot .deviceID=${this.deviceID} .slotIndex=${index} class-"flex flex-row max-w-lg mr-2 mb-2">
              </vm-device-slot>
            `,
          )}
        </div>
      `,
    );
  }

  renderReagents() {
    return this.delayRenderTab("reagents", html``);
  }

  renderNetworks() {
    const vmNetworks = window.VM.vm.networks;
    const networks = this.connections.map((connection, index, _conns) => {
      const conn =
        typeof connection === "object" ? connection.CableNetwork : null;
      return html`
        <sl-select hoist placement="top" clearable key=${index} value=${conn?.net} ?disabled=${conn===null}
          @sl-change=${this._handleChangeConnection}>
          <span slot="prefix">Connection:${index} </span>
          ${vmNetworks.map(
          (net) =>
          html`<sl-option value=${net.toString()}>Network ${net}</sl-option>`,
          )}
          <span slot="prefix"> ${conn?.typ} </span>
        </sl-select>
      `;
    });
    return this.delayRenderTab(
      "networks",
      html`<div class="networks">${networks}</div>`,
    );
  }

  renderPins() {
    return this.delayRenderTab(
      "pins",
      html`<div class="pins">
        <vm-device-pins .deviceID=${this.deviceID}></vm-device-pins>
      </div>`
    );
  }

  private tabsShown: CardTab[] = ["fields"];
  private tabResolves: {
    [key in CardTab]: {
      result?: HTMLTemplateResult;
      resolver?: (result: HTMLTemplateResult) => void;
    };
  } = {
      fields: {},
      slots: {},
      reagents: {},
      networks: {},
      pins: {},
    };

  delayRenderTab(
    name: CardTab,
    result: HTMLTemplateResult,
  ): Promise<HTMLTemplateResult> {
    this.tabResolves[name].result = result;
    return new Promise((resolve) => {
      if (this.tabsShown.includes(name)) {
        this.tabResolves[name].resolver = undefined;
        resolve(result);
      } else {
        this.tabResolves[name].resolver = resolve;
      }
    });
  }

  resolveTab(name: CardTab) {
    if (
      typeof this.tabResolves[name].resolver !== "undefined" &&
      typeof this.tabResolves[name].result !== "undefined"
    ) {
      this.tabResolves[name].resolver(this.tabResolves[name].result);
      this.tabsShown.push(name);
    }
  }

  render(): HTMLTemplateResult {
    return html`
      <ic10-details class="device-card" ?open=${this.open}>
        <div class="header" slot="summary">${this.renderHeader()}</div>
        <sl-tab-group @sl-tab-show=${this._handleTabChange}>
          <sl-tab slot="nav" panel="fields" active>Fields</sl-tab>
          <sl-tab slot="nav" panel="slots">Slots</sl-tab>
          <sl-tab slot="nav" panel="reagents" disabled>Reagents</sl-tab>
          <sl-tab slot="nav" panel="networks">Networks</sl-tab>
          <sl-tab slot="nav" panel="pins" ?disabled=${!this.device.pins}>Pins</sl-tab>

          <sl-tab-panel name="fields" active>
            ${until(this.renderFields(), html`<sl-spinner></sl-spinner>`)}
          </sl-tab-panel>
          <sl-tab-panel name="slots">
            ${until(this.renderSlots(), html`<sl-spinner></sl-spinner>`)}
          </sl-tab-panel>
          <sl-tab-panel name="reagents">
            ${until(this.renderReagents(), html`<sl-spinner></sl-spinner>`)}
          </sl-tab-panel>
          <sl-tab-panel name="networks">
            ${until(this.renderNetworks(), html`<sl-spinner></sl-spinner>`)}
          </sl-tab-panel>
          <sl-tab-panel name="pins">${until(this.renderPins(), html`<sl-spinner></sl-spinner>`)} </sl-tab-panel>
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

  _handleTabChange(e: CustomEvent<{ name: string }>) {
    setTimeout(() => this.resolveTab(e.detail.name as CardTab), 100);
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
      window.VM.get().then((vm) => {
        if (!vm.changeDeviceID(this.deviceID, val)) {
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
    window.VM.get().then((vm) => {
      if (!vm.setDeviceName(this.deviceID, name)) {
        input.value = this.name;
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
    window.VM.get().then((vm) =>
      vm.setDeviceConnection(this.deviceID, conn, val),
    );
    this.updateDevice();
  }

}
