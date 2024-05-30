import type {
  Connection,
  ObjectTemplate,
  LogicField,
  LogicType,
  Slot,
} from "ic10emu_wasm";
import { html, css, HTMLTemplateResult } from "lit";
import { customElement, property, query, state } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";

import { connectionFromDeviceDBConnection } from "./dbutils";
import { crc32, displayNumber, parseNumber } from "utils";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.component.js";
import SlSelect from "@shoelace-style/shoelace/dist/components/select/select.component.js";
import { VMDeviceCard } from "./card";
import { VMTemplateDBMixin } from "virtual_machine/base_device";

@customElement("vm-device-template")
export class VmObjectTemplate extends VMTemplateDBMixin(BaseElement) {

  static styles = [
    ...defaultCss,
    css`
      .template-card {
        --padding: var(--sl-spacing-small);
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
      sl-tab-group {
        --indicator-color: var(--sl-color-purple-600);
        --sl-color-primary-600: var(--sl-color-purple-600);
      }
      sl-tab::part(base) {
        padding: var(--sl-spacing-small) var(--sl-spacing-medium);
      }
      sl-tab-group::part(base) {
        height: 18rem;
        overflow-y: auto;
      }
    `,
  ];

  @state() fields: { [key in LogicType]?: LogicField };
  @state() slots: SlotTemplate[];
  @state() template: ObjectTemplate;
  @state() device_id: number | undefined;
  @state() device_name: string | undefined;
  @state() connections: Connection[];

  constructor() {
    super();
    this.templateDB = window.VM.vm.templateDB;
  }

  private _prefab_name: string;

  get prefab_name(): string {
    return this._prefab_name;
  }

  @property({ type: String })
  set prefab_name(val: string) {
    this._prefab_name = val;
    this.setupState();
  }

  get dbTemplate(): ObjectTemplate {
    return this.templateDB.get( crc32(this.prefab_name));
  }

  setupState() {

    this.fields = Object.fromEntries(
      Object.entries(this.dbTemplate?.logic ?? {}).map(([lt, ft]) => {
        const value = lt === "PrefabHash" ? this.dbTemplate.prefab.prefab_hash : 0.0;
        return [lt, { field_type: ft, value } as LogicField];
      }),
    );

    this.slots = (this.dbTemplate?.slots ?? []).map(
      (slot, _index) =>
        ({
          typ: slot.typ,
        }) as SlotTemplate,
    );

    const connections = Object.entries(this.dbTemplate?.conn ?? {}).map(
      ([index, conn]) =>
        [index, connectionFromDeviceDBConnection(conn)] as const,
    );
    connections.sort((a, b) => {
      if (a[0] < b[0]) {
        return -1;
      } else if (a[0] > b[0]) {
        return 1;
      } else {
        return 0;
      }
    });

    this.connections = connections.map((conn) => conn[1]);
  }
  renderFields(): HTMLTemplateResult {
    const fields = Object.entries(this.fields);
    return html`
      ${fields.map(([name, field], _index, _fields) => {
        return html`
          <sl-input
            key="${name}"
            value="${displayNumber(field.value)}"
            size="small"
            @sl-change=${this._handleChangeField}
            ?disabled=${name === "PrefabHash"}
          >
            <span slot="prefix">${name}</span>
            <span slot="suffix">${field.field_type}</span>
          </sl-input>
        `;
      })}
    `;
  }

  _handleChangeField(e: CustomEvent) {
    const input = e.target as SlInput;
    const field = input.getAttribute("key")! as LogicType;
    const val = parseNumber(input.value);
    this.fields[field].value = val;
    if (field === "ReferenceId" && val !== 0) {
      this.device_id = val;
    }
    this.requestUpdate();
  }

  renderSlot(slot: Slot, slotIndex: number): HTMLTemplateResult {
    return html`<sl-card class="slot-card"> </sl-card>`;
  }

  renderSlots(): HTMLTemplateResult {
    return html`<div clas="slots"></div>`;
  }

  renderReagents(): HTMLTemplateResult {
    return html``;
  }

  renderNetworks() {
    const vm = window.VM.vm;
    const vmNetworks = vm.networks;
    const connections = this.connections;
    return html`
      <div class="networks">
        ${connections.map((connection, index, _conns) => {
          const conn =
            typeof connection === "object" ? connection.CableNetwork : null;
          return html`
            <sl-select
              hoist
              placement="top"
              clearable
              key=${index}
              value=${conn?.net}
              ?disabled=${conn === null}
              @sl-change=${this._handleChangeConnection}
            >
              <span slot="prefix">Connection:${index} </span>
              ${vmNetworks.map(
                (net) =>
                  html`<sl-option value=${net}>Network ${net}</sl-option>`,
              )}
              <span slot="prefix"> ${conn?.typ} </span>
            </sl-select>
          `;
        })}
      </div>
    `;
  }

  _handleChangeConnection(e: CustomEvent) {
    const select = e.target as SlSelect;
    const conn = parseInt(select.getAttribute("key")!);
    const val = select.value ? parseInt(select.value as string) : undefined;
    (this.connections[conn] as ConnectionCableNetwork).CableNetwork.net = val;
    this.requestUpdate();
  }

  renderPins(): HTMLTemplateResult {
    const device = this.templateDB.db[this.prefab_name];
    return html`<div class="pins"></div>`;
  }

  render() {
    const device = this.dbTemplate;
    return html`
      <sl-card class="template-card">
        <div class="header h-20 w-96" slot="header">
          <sl-tooltip content="${device?.name}">
            <img
              class="image me-2"
              src="img/stationpedia/${device?.name}.png"
              onerror="this.src = '${VMDeviceCard.transparentImg}'"
            />
          </sl-tooltip>
          <div class="vstack">
            <span class="prefab-title">${device.title}</span>
            <span class="prefab-name"><small>${device?.name}</small></span>
            <span class="prefab-hash"><small>${device?.hash}</small></span>
          </div>
          <sl-button
            class="ms-auto mt-auto mb-auto"
            pill
            variant="success"
            @click=${this._handleAddButtonClick}
            >Add <sl-icon slot="prefix" name="plus-lg"></sl-icon>
          </sl-button>
        </div>
        <div class="card-body">
          <sl-tab-group>
            <sl-tab slot="nav" panel="fields">Fields</sl-tab>
            <sl-tab slot="nav" panel="slots">Slots</sl-tab>
            <!-- <sl-tab slot="nav" panel="reagents">Reagents</sl-tab> -->
            <sl-tab slot="nav" panel="networks">Networks</sl-tab>
            <!-- <sl-tab slot="nav" panel="pins">Pins</sl-tab> -->

            <sl-tab-panel name="fields">${this.renderFields()}</sl-tab-panel>
            <sl-tab-panel name="slots">${this.renderSlots()}</sl-tab-panel>
            <!-- <sl-tab-panel name="reagents">${this.renderReagents()}</sl-tab-panel> -->
            <sl-tab-panel name="networks"
              >${this.renderNetworks()}</sl-tab-panel
            >
            <!-- <sl-tab-panel name="pins">${this.renderPins()}</sl-tab-panel> -->
          </sl-tab-group>
        </div>
      </sl-card>
    `;
  }
  _handleAddButtonClick() {
    this.dispatchEvent(
      new CustomEvent("add-device-template", { bubbles: true }),
    );
    const template: DeviceTemplate = {
      id: this.device_id,
      name: this.device_name,
      prefab_name: this.prefab_name,
      slots: this.slots,
      connections: this.connections,
      fields: this.fields,
    };
    window.VM.vm.addDeviceFromTemplate(template);

    // reset state for new device
    this.setupState();
  }
}
