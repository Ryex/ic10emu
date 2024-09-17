import { html, css, HTMLTemplateResult, nothing } from "lit";
import { customElement, property, query } from "lit/decorators.js";
import { watch, computed } from '@lit-labs/preact-signals';
import { BaseElement, defaultCss } from "components";
import { VMObjectMixin } from "virtualMachine/baseDevice";
import SlSelect from "@shoelace-style/shoelace/dist/components/select/select.component.js";
import { crc32, isSome, parseIntWithHexOrBinary, range } from "utils";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.component.js";
import SlDialog from "@shoelace-style/shoelace/dist/components/dialog/dialog.component.js";
import "./slot";
import "./fields";
import "./pins";
import { until } from "lit/directives/until.js";
import { repeat } from "lit/directives/repeat.js";
import { Connection } from "ic10emu_wasm";
import { createRef, ref, Ref } from "lit/directives/ref.js";

import * as log from "log";

export type CardTab = "fields" | "slots" | "reagents" | "networks" | "pins";

@customElement("vm-device-card")
export class VMDeviceCard extends VMObjectMixin(BaseElement) {
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

  onImageErr(e: Event) {
    this.image_err = true;
    log.error("Image load error", e);
  }

  thisIsActiveIc = computed(() => {
    return this.vm.value?.activeIC.value === this.objectIDSignal.value;
  });

  activeIcPins = computed(() => {
    return this.vm.value?.state.getDevicePins(this.vm.value?.activeIC.value).value ?? [];
  });

  prefabName = computed(() => {
    return this.vm.value?.state.getObject(this.objectIDSignal.value).value?.obj_info.prefab ?? "unknown";
  });

  objectName = computed(() => {
    return this.vm.value?.state.getObject(this.objectIDSignal.value).value?.obj_info.name ?? "";
  });

  objectNameHash = computed(() => {
    return crc32(this.vm.value?.state.getObject(this.objectIDSignal.value).value?.obj_info.name ?? "");
  });


  renderHeader(): HTMLTemplateResult {
    const badgesHtml = computed(() => {
      const badges: HTMLTemplateResult[] = [];
      if (this.thisIsActiveIc.value) {
        badges.push(html`<sl-badge variant="primary" pill pulse>db</sl-badge>`);
      }
      this.activeIcPins.value.forEach(([pin, id]) => {
        if (this.objectIDSignal.value == id) {
          badges.push(
            html`<sl-badge variant="success" pill>d${pin}</sl-badge>`,
          );
        }
      }, this);
      return badges
    });

    const removeText = computed(() => {
      return this.thisIsActiveIc.value
        ? "Removing the selected Active IC is disabled"
        : "Remove Device";
    });

    return html`
      <sl-tooltip content="${watch(this.prefabName)}">
        <img
          class="image me-2"
          src="img/stationpedia/${watch(this.prefabName)}.png"
          onerror="this.src = '${VMDeviceCard.transparentImg}'"
        />
      </sl-tooltip>
      <div class="header-name">
        <sl-input
          id="vmDeviceCard${watch(this.objectIDSignal)}Id"
          class="device-id me-1"
          size="small"
          pill
          value=${watch(this.objectIDSignal)}
          @sl-change=${this._handleChangeID}
        >
          <span slot="prefix">Id</span>
          <sl-copy-button
            slot="suffix"
            .value=${watch(this.objectIDSignal)}
          ></sl-copy-button>
        </sl-input>
        <sl-input
          id="vmDeviceCard${watch(this.objectIDSignal)}Name"
          class="device-name me-1"
          size="small"
          pill
          placeholder=${watch(this.prefabName)}
          value=${watch(this.objectName)}
          @sl-change=${this._handleChangeName}
        >
          <span slot="prefix">Name</span>
          <sl-copy-button
            slot="suffix"
            from="vmDeviceCard${watch(this.objectIDSignal)}Name.value"
          ></sl-copy-button>
        </sl-input>
        <sl-input
          id="vmDeviceCard${watch(this.objectIDSignal)}NameHash"
          size="small"
          pill
          class="device-name-hash me-1"
          value="${watch(this.objectNameHash)}"
          readonly
        >
          <span slot="prefix">Hash</span>
          <sl-copy-button
            slot="suffix"
            from="vmDeviceCard${watch(this.objectIDSignal)}NameHash.value"
          ></sl-copy-button>
        </sl-input>
        ${watch(badgesHtml)}
      </div>
      <div class="ms-auto mt-auto mb-auto me-2">
        <sl-tooltip
          content=${watch(removeText)}
        >
          <sl-icon-button
            class="remove-button"
            name="trash"
            label="Remove Device"
            ?disabled=${watch(this.thisIsActiveIc)}
            @click=${this._handleDeviceRemoveButton}
          ></sl-icon-button>
        </sl-tooltip>
      </div>
    `;
  }

  renderFields() {
    return this.delayRenderTab(
      "fields",
      html`<vm-device-fields .objectID=${watch(this.objectIDSignal)}></vm-device-fields>`,
    );
  }

  _onSlotImageErr(e: Event) {
    console.log("image_err", e);
  }

  static transparentImg =
    "data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7" as const;

  objectSlotCount = computed(() => {
    return this.vm.value?.state.getObjectSlotCount(this.objectIDSignal.value).value;
  });

  async renderSlots() {
    const slotsHtml = computed(() => {
      return repeat(range(this.objectSlotCount.value),
        (_slot, index) => html`
          <vm-object-slot .objectID=${watch(this.objectIDSignal)} .slotIndex=${index} class-"flex flex-row max-w-lg mr-2 mb-2">
          </vm-object-slot>
        `,
      );
    });
    return this.delayRenderTab(
      "slots",
      html`
        <div class="flex flex-row flex-wrap">
          ${watch(slotsHtml)}
        </div>
      `,
    );
  }

  renderReagents() {
    return this.delayRenderTab("reagents", html``);
  }

  networkIds = computed(() => {
    return this.vm.value?.state.networkIds.value ?? [];
  });

  numConnections = computed(() => {
    return this.vm.value?.state.getObjectConnectionCount(this.objectIDSignal.value).value;
  })

  private _connectionsSelectRefMap: Map<number, Ref<SlSelect>> = new Map();

  getConnectionSelectRef(index: number): Ref<SlSelect> {
    if (!this._connectionsSelectRefMap.has(index)) {
      this._connectionsSelectRefMap.set(index, createRef());
    }
    return this._connectionsSelectRefMap.get(index);
  }

  forceSelectUpdate(...slSelects: Ref<SlSelect>[]) {
    for (const slSelect of slSelects) {
      if (slSelect.value != null && "handleValueChange" in slSelect.value) {
        slSelect.value.handleValueChange();
      }
    }
  }

  renderConnections() {
    const connectionsHtml = computed(() => range(this.numConnections.value).map(index => {
      const conn = computed(() => {
        return this.vm.value?.state.getObjectConnection(this.objectIDSignal.value, index).value;
      });
      const connNet = computed(() => {
        const connection: Connection = conn.value ?? "None";
        if (typeof connection === "object" && "CableNetwork" in connection) {
          return connection.CableNetwork.net;
        }
        return null;
      });
      const selectDisabled = computed(() => !isSome(connNet.value));
      const selectOptions = computed(() => {
        return this.networkIds.value.map(id => html`
          <sl-option value=${id}>
            Network ${id}
          </sl-option>
        `);
      });
      const connTyp = computed(() => {
        const connection: Connection = conn.value ?? "None";
        return typeof connection === "object" ? Object.keys(connection)[0] : connection;
      });

      const connectionSelectRef = this.getConnectionSelectRef(index);
      selectOptions.subscribe(() => { this.forceSelectUpdate(connectionSelectRef) })

      connNet.subscribe((net) => {
        if (isSome(connectionSelectRef.value)) {
          connectionSelectRef.value.value = net.toString(0)
          connectionSelectRef.value.handleValueChange();
        }
      })

      return html`
        <sl-select
          hoist
          placement="top"
          clearable
          key=${index}
          value=${watch(connNet)}
          ?disabled=${watch(selectDisabled)}
          @sl-change=${this._handleChangeConnection}
          ${ref(connectionSelectRef)}
        >
          <span slot="prefix">Connection:${index} </span>
            ${watch(selectOptions)}
          <span slot="prefix"> ${watch(connTyp)} </span>
        </sl-select>
      `;
    }));
    return this.delayRenderTab(
      "networks",
      html`<div class="networks">${watch(connectionsHtml)}</div>`,
    );
  }

  renderPins() {
    return this.delayRenderTab(
      "pins",
      html`<div class="pins">
        <vm-device-pins .objectID=${watch(this.objectIDSignal)}></vm-device-pins>
      </div>`,
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

  numPins = computed(() => {
    return this.vm.value?.state.getDeviceNumPins(this.objectIDSignal.value)
  });

  displayName = computed(() => {
    const obj = this.vm.value?.state.getObject(this.objectIDSignal.value).value;
    return obj?.obj_info.name ?? obj?.obj_info.prefab ?? null;
  });

  imageName = computed(() => {
    const obj = this.vm.value?.state.getObject(this.objectIDSignal.value).value;
    return obj?.obj_info.prefab ?? "error";
  });

  render(): HTMLTemplateResult {
    const disablePins = computed(() => { return !this.numPins.value; });
    return html`
      <ic10-details class="device-card" ?open=${this.open}>
        <div class="header" slot="summary">${this.renderHeader()}</div>
        <sl-tab-group @sl-tab-show=${this._handleTabChange}>
          <sl-tab slot="nav" panel="fields" active>Fields</sl-tab>
          <sl-tab slot="nav" panel="slots">Slots</sl-tab>
          <sl-tab slot="nav" panel="reagents" disabled>Reagents</sl-tab>
          <sl-tab slot="nav" panel="networks">Networks</sl-tab>
          <sl-tab slot="nav" panel="pins" ?disabled=${watch(disablePins)}
            >Pins</sl-tab
          >

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
            ${until(this.renderConnections(), html`<sl-spinner></sl-spinner>`)}
          </sl-tab-panel>
          <sl-tab-panel name="pins"
            >${until(this.renderPins(), html`<sl-spinner></sl-spinner>`)}
          </sl-tab-panel>
        </sl-tab-group>
      </ic10-details>
      <sl-dialog
        class="remove-device-dialog"
        no-header
        @sl-request-close=${this._preventOverlayClose}
      >
        <div class="remove-dialog-body">
          <img
            class="dialog-image mt-auto mb-auto me-2"
            src="img/stationpedia/${watch(this.imageName)}.png"
            onerror="this.src = '${VMDeviceCard.transparentImg}'"
          />
          <div class="flex-g">
            <p><strong>Are you sure you want to remove this device?</strong></p>
            <span>Id ${watch(this.objectIDSignal)} : ${watch(this.displayName)}</span>
          </div>
        </div>
        <div slot="footer">
          <sl-button
            variant="primary"
            autofocus
            @click=${this._closeRemoveDialog}
            >Close</sl-button
          >
          <sl-button variant="danger" @click=${this._removeDialogRemove}
            >Remove</sl-button
          >
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
        if (!vm.changeObjectID(this.objectID, val)) {
          input.value = this.objectID.toString();
        }
      });
    } else {
      input.value = this.objectID.toString();
    }
  }

  _handleChangeName(e: CustomEvent) {
    const input = e.target as SlInput;
    const name = input.value.length === 0 ? undefined : input.value;
    window.VM.get().then((vm) => {
      if (!vm.setObjectName(this.objectID, name)) {
        input.value = this.objectName.peek();
      }
    });
  }
  _handleDeviceRemoveButton(_e: Event) {
    this.removeDialog.show();
  }

  _removeDialogRemove() {
    this.removeDialog.hide();
    window.VM.get().then((vm) => vm.removeDevice(this.objectID));
  }

  _handleChangeConnection(e: CustomEvent) {
    const select = e.target as SlSelect;
    const conn = parseInt(select.getAttribute("key")!);
    const val = select.value ? parseInt(select.value as string) : undefined;
    window.VM.get().then((vm) =>
      vm.setDeviceConnection(this.objectID, conn, val),
    );
  }
}
