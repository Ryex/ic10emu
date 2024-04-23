import { html, css, HTMLTemplateResult } from "lit";
import { customElement, property, query, state } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMDeviceDBMixin, VMDeviceMixin } from "virtual_machine/base_device";
import type { DeviceDB } from "virtual_machine/device_db";
import SlSelect from "@shoelace-style/shoelace/dist/components/select/select.component.js";
import { displayNumber, parseIntWithHexOrBinary, parseNumber } from "utils";
import {
  LogicType,
  Slot,
  SlotLogicType,
  SlotOccupant,
  SlotType,
} from "ic10emu_wasm";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.component.js";
import SlDialog from "@shoelace-style/shoelace/dist/components/dialog/dialog.component.js";
import { VMDeviceCard } from "./card";
import { when } from "lit/directives/when.js";

@customElement("vm-device-slot")
export class VMDeviceSlot extends VMDeviceMixin(VMDeviceDBMixin(BaseElement)) {
  @property({ type: Number }) slotIndex: number;

  constructor() {
    super();
  }

  static styles = [
    ...defaultCss,
    css`
      .slot-card {
        --padding: var(--sl-spacing-small);
      }
      .slot-card::part(base) {
        background-color: var(--sl-color-neutral-50);
      }
    `,
  ];

  slotOccupantImg(): string {
    const slot = this.slots[this.slotIndex];
    if (typeof slot.occupant !== "undefined") {
      const hashLookup = (this.deviceDB ?? {}).names_by_hash ?? {};
      const prefabName = hashLookup[slot.occupant.prefab_hash] ?? "UnknownHash";
      return `img/stationpedia/${prefabName}.png`;
    } else {
      return `img/stationpedia/SlotIcon_${slot.typ}.png`;
    }
  }

  slotOccupantPrefabName(): string {
    const slot = this.slots[this.slotIndex];
    if (typeof slot.occupant !== "undefined") {
      const hashLookup = (this.deviceDB ?? {}).names_by_hash ?? {};
      const prefabName = hashLookup[slot.occupant.prefab_hash] ?? "UnknownHash";
      return prefabName;
    } else {
      return undefined;
    }
  }

  renderHeader() {
    const inputIdBase = `vmDeviceSlot${this.deviceID}Slot${this.slotIndex}Head`;
    const slot = this.slots[this.slotIndex];
    const slotImg = this.slotOccupantImg();
    const img = html`<img class="w-10 h-10" src="${slotImg}" oanerror="this.src = '${VMDeviceCard.transparentImg}'" />`;

    return html`
      <div class="flex flex-row w-full me-2">
        <div
          class="relative border border-neutral-200/40 rounded-lg p-1
            hover:ring-2 hover:ring-purple-500 hover:ring-offset-1
            hover:ring-offset-purple-500 cursor-pointer me-2"
          @click=${this._handleSlotClick}
        >
          <sl-tooltip content="${this.slotOccupantPrefabName() ?? slot.typ}">
            ${img}
          </sl-tooltip>
          ${when(
            typeof slot.occupant !== "undefined",
            () => html`<div
              class="absolute bottom-0 right-0 mr-1 mb-1 text-xs
                text-neutral-200/90 font-mono bg-neutral-500/40 rounded"
            >
              <small>${slot.occupant.quantity} / ${slot.occupant.max_quantity}</small>
            </div>`
          )}
          <div></div>
        </div>
        <div class="ms-4 mt-auto mb-auto">
          ${when(
            typeof slot.occupant !== "undefined",
            () => html`
              <span class="">
                ${slot.occupant.id} : ${this.slotOccupantPrefabName()}
              </span>
            `,
            () => html`
              <span class="">
                ${slot.typ}
              </span>
            `,
          )}
        </div>
        <div class="ms-auto mt-auto mb-auto me-4">
          ${when(
            typeof slot.occupant !== "undefined",
            () => html`
              <sl-input
                type="number"
                size="small"
                .value=${slot.occupant.quantity.toString()}
                .min=${1}
                .max=${slot.occupant.max_quantity}
              >
                <div slot="help-text">
                  <span><strong>Max Quantity:</strong>${slot.occupant.max_quantity}</span>
                </div>
              </sl-input>
            `,
            () => html`
            `,
          )}
        </div>
      </div>
    `;
  }

  _handleSlotClick(e: Event) {
    console.log(e);
  }

  renderFields() {
    const inputIdBase = `vmDeviceSlot${this.deviceID}Slot${this.slotIndex}Field`;
    const _fields = this.device.getSlotFields(this.slotIndex);
    const fields = Array.from(_fields.entries());

    return html`
      <div class="slot-fields">
        ${fields.map(
        ([name, field], _index, _fields) => html`
        <sl-input
          id="${inputIdBase}${name}"
          key="${name}"
          value="${displayNumber(field.value)}"
          size="small"
          @sl-change=${this._handleChangeSlotField}
        >
          <span slot="prefix">${name}</span>
          <sl-copy-button slot="suffix" from="${inputIdBase}${name}.value"></sl-copy-button>
          <span slot="suffix">${field.field_type}</span>
        </sl-input>
        `,
        )}
      </div>
    `;
  }

  _handleChangeSlotField(e: CustomEvent) {
    const input = e.target as SlInput;
    const field = input.getAttribute("key")! as SlotLogicType;
    const val = parseNumber(input.value);
    window.VM.get().then((vm) => {
      if (!vm.setDeviceSlotField(this.deviceID, this.slotIndex, field, val, true)) {
        input.value = this.device.getSlotField(this.slotIndex, field).toString();
      }
      this.updateDevice();
    });
  }

  render() {
    return html`
      <ic10-details class="slot-card">
        <div class="slot-header w-full" slot="summary">${this.renderHeader()}</div>
        <div class="slot-body">
          ${this.renderFields()}
        </div>
      </ic10-details>
    `;
  }
}
