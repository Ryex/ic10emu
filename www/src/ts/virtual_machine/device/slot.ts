import { html, css } from "lit";
import { customElement, property, query, state } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMDeviceDBMixin, VMDeviceMixin } from "virtual_machine/base_device";
import type { DeviceDB, DeviceDBEntry } from "virtual_machine/device_db";
import SlSelect from "@shoelace-style/shoelace/dist/components/select/select.component.js";
import { clamp, displayNumber, parseIntWithHexOrBinary, parseNumber } from "utils";
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

export interface SlotModifyEvent {
  deviceID: number;
  slotIndex: number;
}

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
        --padding: var(--sl-spacing-x-small);
      }
      .slot-card::part(header) {
        padding: var(--sl-spacing-x-small);
      }
      .slot-card::part(base) {
        background-color: var(--sl-color-neutral-50);
      }
      .quantity-input sl-input::part(input) {
        width: 3rem;
      }
      .clear-occupant::part(base) {
        color: var(--sl-color-warning-500);
      }
      .clear-occupant::part(base):hover,
      .clear-occupant::part(base):focus {
        color: var(--sl-color-warning-400);
      }
      .clear-occupant::part(base):active {
        color: var(--sl-color-warning-500);
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

  slotOcccupantTemplate(): { name: string; typ: SlotType } | undefined {
    if (this.deviceDB) {
      const entry = this.deviceDB.db[this.prefabName];
      return entry?.slots[this.slotIndex];
    } else {
      return undefined;
    }
  }

  renderHeader() {
    const inputIdBase = `vmDeviceSlot${this.deviceID}Slot${this.slotIndex}Head`;
    const slot = this.slots[this.slotIndex];
    const slotImg = this.slotOccupantImg();
    const img = html`<img
      class="w-10 h-10"
      src="${slotImg}"
      onerror="this.src = '${VMDeviceCard.transparentImg}'"
    />`;
    const template = this.slotOcccupantTemplate();

    const activeIc = window.VM.vm.activeIC;
    const thisIsActiveIc = activeIc.id === this.deviceID;

    const enableQuantityInput = false;

    return html`
      <div class="flex flex-row me-2">
        <div
          class="relative shrink-0 border border-neutral-200/40 rounded-lg p-1
                  hover:ring-2 hover:ring-purple-500 hover:ring-offset-1
                  hover:ring-offset-purple-500 cursor-pointer me-2"
          @click=${this._handleSlotClick}
        >
          <div
            class="absolute top-0 left-0 ml-1 mt-1 text-xs
                  text-neutral-200/90 font-mono bg-neutral-500/40 rounded pl-1 pr-1"
          >
            <small>${this.slotIndex}</small>
          </div>
          <sl-tooltip content="${this.slotOccupantPrefabName() ?? slot.typ}">
            ${img}
          </sl-tooltip>
          ${when(
            typeof slot.occupant !== "undefined",
            () =>
              html`<div
                class="absolute bottom-0 right-0 mr-1 mb-1 text-xs
                      text-neutral-200/90 font-mono bg-neutral-500/40 rounded pl-1 pr-1"
              >
                <small
                  >${slot.occupant.quantity}/${slot.occupant
                    .max_quantity}</small
                >
              </div>`,
          )}
          <div></div>
        </div>
        <div class="flex flex-col justify-end">
          <div class="text-sm mt-auto mb-auto">
            ${when(
              typeof slot.occupant !== "undefined",
              () => html` <span> ${this.slotOccupantPrefabName()} </span> `,
              () => html` <span> ${template?.name} </span> `,
            )}
          </div>
          <div class="text-neutral-400 text-xs mt-auto flex flex-col mb-1">
            <div>
              <strong class="mt-auto mb-auto">Type:</strong
              ><span class="p-1">${slot.typ}</span>
            </div>
          </div>
        </div>
        ${when(
          typeof slot.occupant !== "undefined",
          () => html`
            <div class="quantity-input ms-auto pl-2 mt-auto mb-auto me-2">
              ${ enableQuantityInput ? html`
              <sl-input
                type="number"
                size="small"
                .value=${slot.occupant.quantity.toString()}
                .min=${1}
                .max=${slot.occupant.max_quantity}
                @sl-change=${this._handleSlotQuantityChange}
              >
                <div slot="help-text">
                  <span>Max Quantity: ${slot.occupant.max_quantity}</span>
                </div>
              </sl-input>` : "" }
              <sl-tooltip content=${thisIsActiveIc ? "Removing the selected Active IC is disabled" : "Remove Occupant" }>
                <sl-icon-button class="clear-occupant" name="x-octagon" label="Remove" ?disabled=${thisIsActiveIc}
                  @click=${this._handleSlotOccupantRemove}></sl-icon-button>
              </sl-tooltip>
            </div>
          `,
          () => html``,
        )}
      </div>
    `;
  }

  _handleSlotOccupantRemove() {
    window.VM.vm.removeDeviceSlotOccupant(this.deviceID, this.slotIndex);
  }

  _handleSlotClick(_e: Event) {
    this.dispatchEvent(
      new CustomEvent<SlotModifyEvent>("device-modify-slot", {
        bubbles: true,
        composed: true,
        detail: { deviceID: this.deviceID, slotIndex: this.slotIndex },
      }),
    );
  }

  _handleSlotQuantityChange(e: Event) {
    const input = e.currentTarget as SlInput;
    const slot = this.slots[this.slotIndex];
    const val = clamp(input.valueAsNumber, 1, slot.occupant.max_quantity);
    if (!window.VM.vm.setDeviceSlotField(this.deviceID, this.slotIndex, "Quantity", val, true)) {
      input.value = this.device.getSlotField(this.slotIndex, "Quantity").toString();
    }
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
              <sl-copy-button
                slot="suffix"
                from="${inputIdBase}${name}.value"
              ></sl-copy-button>
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
    let val = parseNumber(input.value);
    if (field === "Quantity") {
      const slot = this.slots[this.slotIndex];
      val = clamp(input.valueAsNumber, 1, slot.occupant.max_quantity);
    }
    window.VM.get().then((vm) => {
      if (
        !vm.setDeviceSlotField(this.deviceID, this.slotIndex, field, val, true)
      ) {
        input.value = this.device
          .getSlotField(this.slotIndex, field)
          .toString();
      }
      this.updateDevice();
    });
  }

  render() {
    return html`
      <ic10-details class="slot-card">
        <div class="slot-header w-full" slot="summary">
          ${this.renderHeader()}
        </div>
        <div class="slot-body">${this.renderFields()}</div>
      </ic10-details>
    `;
  }
}
