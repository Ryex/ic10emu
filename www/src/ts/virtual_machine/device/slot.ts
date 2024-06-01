import { html, css } from "lit";
import { customElement, property} from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMTemplateDBMixin, VMObjectMixin } from "virtual_machine/baseDevice";
import {
  clamp,
  crc32,
  displayNumber,
  parseNumber,
} from "utils";
import {
  LogicField,
  LogicSlotType,
  SlotInfo,
  Class as SlotType,
  TemplateDatabase,
} from "ic10emu_wasm";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.component.js";
import { VMDeviceCard } from "./card";
import { when } from "lit/directives/when.js";

export interface SlotModifyEvent {
  deviceID: number;
  slotIndex: number;
}

@customElement("vm-device-slot")
export class VMDeviceSlot extends VMObjectMixin(VMTemplateDBMixin(BaseElement)) {
  private _slotIndex: number;

  get slotIndex() {
    return this._slotIndex;
  }

  @property({ type: Number })
  set slotIndex(val: number) {
    this._slotIndex = val;
    this.unsubscribe((sub) => typeof sub === "object" && "slot" in sub);
    this.subscribe({ slot: val });
  }


  constructor() {
    super();
    this.subscribe("active-ic", "prefabName");
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
      const prefabName = slot.occupant.obj_info.prefab;
      return `img/stationpedia/${prefabName}.png`;
    } else {
      return `img/stationpedia/SlotIcon_${slot.typ}.png`;
    }
  }

  slotOccupantPrefabName(): string {
    const slot = this.slots[this.slotIndex];
    if (typeof slot.occupant !== "undefined") {
      const prefabName = slot.occupant.obj_info.prefab;
      return prefabName;
    } else {
      return undefined;
    }
  }

  slotOcccupantTemplate(): SlotInfo | undefined {
    if ("slots" in this.obj.template) {
      return this.obj.template.slots[this.slotIndex];
    } else {
      return undefined;
    }
  }

  renderHeader() {
    const inputIdBase = `vmDeviceSlot${this.objectID}Slot${this.slotIndex}Head`;
    const slot = this.slots[this.slotIndex];
    const slotImg = this.slotOccupantImg();
    const img = html`<img
      class="w-10 h-10"
      src="${slotImg}"
      onerror="this.src = '${VMDeviceCard.transparentImg}'"
    />`;
    const template = this.slotOcccupantTemplate();

    const thisIsActiveIc = this.activeICId === this.objectID;

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
                <small>
                  ${slot.quantity}/${"item" in slot.occupant.template
                    ? slot.occupant.template.item.max_quantity
                    : 1}
                </small>
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
              ${enableQuantityInput
                ? html` <sl-input
                    type="number"
                    size="small"
                    .value=${slot.quantity.toString()}
                    .min=${1}
                    .max=${"item" in slot.occupant.template
                      ? slot.occupant.template.item.max_quantity
                      : 1}
                    @sl-change=${this._handleSlotQuantityChange}
                  >
                    <div slot="help-text">
                      <span>
                        Max Quantity:
                        ${"item" in slot.occupant.template
                          ? slot.occupant.template.item.max_quantity
                          : 1}
                      </span>
                    </div>
                  </sl-input>`
                : ""}
              <sl-tooltip
                content=${thisIsActiveIc && slot.typ === "ProgrammableChip"
                  ? "Removing the selected Active IC is disabled"
                  : "Remove Occupant"}
              >
                <sl-icon-button
                  class="clear-occupant"
                  name="x-octagon"
                  label="Remove"
                  ?disabled=${thisIsActiveIc && slot.typ === "ProgrammableChip"}
                  @click=${this._handleSlotOccupantRemove}
                ></sl-icon-button>
              </sl-tooltip>
            </div>
          `,
          () => html``,
        )}
      </div>
    `;
  }

  _handleSlotOccupantRemove() {
    window.VM.vm.removeSlotOccupant(this.objectID, this.slotIndex);
  }

  _handleSlotClick(_e: Event) {
    this.dispatchEvent(
      new CustomEvent<SlotModifyEvent>("device-modify-slot", {
        bubbles: true,
        composed: true,
        detail: { deviceID: this.objectID, slotIndex: this.slotIndex },
      }),
    );
  }

  _handleSlotQuantityChange(e: Event) {
    const input = e.currentTarget as SlInput;
    const slot = this.slots[this.slotIndex];
    const val = clamp(
      input.valueAsNumber,
      1,
      "item" in slot.occupant.template
        ? slot.occupant.template.item.max_quantity
        : 1,
    );
    if (
      !window.VM.vm.setObjectSlotField(
        this.objectID,
        this.slotIndex,
        "Quantity",
        val,
        true,
      )
    ) {
      input.value = this.slots[this.slotIndex].quantity.toString();
    }
  }

  renderFields() {
    const inputIdBase = `vmDeviceSlot${this.objectID}Slot${this.slotIndex}Field`;
    const _fields =
      this.slots[this.slotIndex].logicFields ??
      new Map<LogicSlotType, LogicField>();
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
    const field = input.getAttribute("key")! as LogicSlotType;
    let val = parseNumber(input.value);
    if (field === "Quantity") {
      const slot = this.slots[this.slotIndex];
      val = clamp(
        input.valueAsNumber,
        1,
        "item" in slot.occupant.template
          ? slot.occupant.template.item.max_quantity
          : 1,
      );
    }
    window.VM.get().then((vm) => {
      if (
        !vm.setObjectSlotField(this.objectID, this.slotIndex, field, val, true)
      ) {
        input.value = (
          this.slots[this.slotIndex].logicFields ??
          new Map<LogicSlotType, LogicField>()
        )
          .get(field)
          .toString();
      }
      this.updateDevice();
    });
  }

  render() {
    return html`
      <ic10-details
        class="slot-card"
      >
        <div class="slot-header w-full" slot="summary">
          ${this.renderHeader()}
        </div>
        <div class="slot-body">${this.renderFields()}</div>
      </ic10-details>
    `;
  }

}
