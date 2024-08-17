import { html, css } from "lit";
import { customElement, property } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMTemplateDBMixin, VMObjectMixin, VmObjectSlotInfo, ComputedObjectSignals } from "virtualMachine/baseDevice";
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
import { computed, signal, Signal, SignalWatcher, watch } from "@lit-labs/preact-signals";

export interface SlotModifyEvent {
  deviceID: number;
  slotIndex: number;
}

@customElement("vm-device-slot")
export class VMDeviceSlot extends VMObjectMixin(VMTemplateDBMixin(SignalWatcher(BaseElement))) {
  private _slotIndex: Signal<number>;

  slotSignal: Signal<VmObjectSlotInfo>;

  get slotIndex() {
    return this._slotIndex.value;
  }

  @property({ type: Number })
  set slotIndex(val: number) {
    this._slotIndex.value = val;
  }

  constructor() {
    super();
    this._slotIndex = signal(0);
    this.subscribe("active-ic");
    this.slotSignal = computed(() => {
      const index = this._slotIndex.value;
      return this.objectSignals.slots.value[index];
    });
    this.setupSignals();
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

  setupSignals() {
    this.slotOccupant = computed(() => {
      const slot = this.slotSignal.value ?? null;
      return slot?.occupant ?? null;
    });
    this.slotFieldTypes = computed(() => {
      return Array.from(this.slotSignal.value?.logicFields.keys() ?? []) ;
    });
    this.slotOccupantImg  = computed(() => {
      const slot = this.slotSignal.value ?? null;
      if (slot != null && slot.occupant != null) {
        const prefabName = slot.occupant.prefabName;
        return `img/stationpedia/${watch(prefabName)}.png`;
      } else {
        return `img/stationpedia/SlotIcon_${slot.typ}.png`;
      }
    });
    this.slotOccupantPrefabName = computed(() => {
      const slot = this.slotSignal.value ?? null;
      if (slot != null && slot.occupant != null) {
        const prefabName = slot.occupant.prefabName.value;
        return prefabName;
      } else {
        return null;
      }
    });
    this.slotOccupantTemplate = computed(() => {
      if (this.objectSignals != null && "slots" in this.objectSignals.template.value) {
        return this.objectSignals.template.value.slots[this.slotIndex];
      } else {
        return null;
      }
    });
  }

  slotOccupant: Signal<ComputedObjectSignals | null>;
  slotFieldTypes: Signal<LogicSlotType[]>;
  slotOccupantImg: Signal<string>;
  slotOccupantPrefabName: Signal<string | null>;
  slotOccupantTemplate: Signal<SlotInfo | null>;

  renderHeader() {
    const inputIdBase = `vmDeviceSlot${this.objectID}Slot${this.slotIndex}Head`;
    // const slot = this.slotSignal.value;
    const slotImg = this.slotOccupantImg;
    const img = html`<img
      class="w-10 h-10"
      src="${watch(slotImg)}"
      onerror="this.src = '${VMDeviceCard.transparentImg}'"
    />`;
    const template = this.slotOccupantTemplate;
    const templateName = computed(() => {
      return template.value?.name ?? null;
    });
    const slotTyp = computed(() => {
      return this.slotSignal.value.typ;
    })

    const enableQuantityInput = false;

    const quantity = computed(() => {
      const slot = this.slotSignal.value;
      return slot.quantity;
    });

    const maxQuantity = computed(() => {
      const slotOccupant = this.slotSignal.value.occupant;
      const template = slotOccupant?.template.value ?? null;
      if (template != null && "item" in template) {
        return template.item.max_quantity;
      } else {
        return 1;
      }
    });

    const slotDisplayName = computed(() => {
      return this.slotOccupantPrefabName.value ?? this.slotSignal.value.typ;
    });

    const tooltipContent = computed(() => {
      return this.activeICId === this.objectID && slotTyp.value === "ProgrammableChip"
        ? "Removing the selected Active IC is disabled"
        : "Remove Occupant"
    })

    const removeDisabled = computed(() => {
      return this.activeICId === this.objectID && slotTyp.value === "ProgrammableChip"
    });

    const quantityContent = computed(() => {
      if (this.slotOccupant.value != null) {
        return html`
          <div
            class="absolute bottom-0 right-0 mr-1 mb-1 text-xs
              text-neutral-200/90 font-mono bg-neutral-500/40 rounded pl-1 pr-1"
          >
            <small>
              ${watch(quantity)}/${watch(maxQuantity)}
            </small>
          </div>`
      } else {
        return null
      }
    });

    const slotName = computed(() => {
      if(this.slotOccupant.value != null) {
        return html` <span> ${watch(this.slotOccupantPrefabName)} </span> `
      } else {
       html` <span> ${watch(templateName)} </span> `
      }
    });

    const inputContent = computed(() => {
      if (this.slotOccupant.value != null) {
        return html`
          <div class="quantity-input ms-auto pl-2 mt-auto mb-auto me-2">
            ${enableQuantityInput
                ? html`<sl-input
                    type="number"
                    size="small"
                    .value=${watch(quantity)}
                    .min=${1}
                    .max=${watch(maxQuantity)}
                    @sl-change=${this._handleSlotQuantityChange}
                  >
                    <div slot="help-text">
                      <span>
                        Max Quantity:
                        ${watch(maxQuantity)}
                      </span>
                    </div>
                  </sl-input>`
                : ""}
            <sl-tooltip
              content=${watch(tooltipContent)}
            >
              <sl-icon-button
                class="clear-occupant"
                name="x-octagon"
                label="Remove"
                ?disabled=${watch(removeDisabled)}
                @click=${this._handleSlotOccupantRemove}
              ></sl-icon-button>
            </sl-tooltip>
          </div>
        `
      } else {
        return null;
      }
    });

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
          <sl-tooltip content="${watch(slotDisplayName)}">
            ${img}
          </sl-tooltip>
          ${watch(quantityContent)}
          <div></div>
        </div>
        <div class="flex flex-col justify-end">
          <div class="text-sm mt-auto mb-auto">
            ${watch(slotName)}
          </div>
          <div class="text-neutral-400 text-xs mt-auto flex flex-col mb-1">
            <div>
              <strong class="mt-auto mb-auto">Type:</strong
              ><span class="p-1">${watch(slotTyp)}</span>
            </div>
          </div>
        </div>
        ${watch(inputContent)}
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
    const slot = this.slotSignal.value;
    const val = clamp(
      input.valueAsNumber,
      1,
      "item" in slot.occupant.template.value
        ? slot.occupant.template.value.item.max_quantity
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
      input.value = this.slotSignal.value.quantity.toString();
    }
  }

  renderFields() {
    const inputIdBase = `vmDeviceSlot${this.objectID}Slot${this.slotIndex}Field`;
    const fields = computed(() => {
      const slot = this.slotSignal.value;
      const _fields =
        slot.logicFields??
        new Map<LogicSlotType, LogicField>();
      return this.slotFieldTypes.value.map(
        (name, _index, _types) => {
          const slotField = computed(() => {
            return this.slotSignal.value.logicFields.get(name);
          });
          const fieldValue = computed(() => {
            return displayNumber(slotField.value.value);
          })
          const fieldAccessType = computed(() => {
            return slotField.value.field_type;
          })
          return html`
            <sl-input
              id="${inputIdBase}${name}"
              key="${name}"
              value="${watch(fieldValue)}"
              size="small"
              @sl-change=${this._handleChangeSlotField}
            >
              <span slot="prefix">${name}</span>
              <sl-copy-button
                slot="suffix"
                from="${inputIdBase}${name}.value"
              ></sl-copy-button>
              <span slot="suffix">${watch(fieldAccessType)}</span>
            </sl-input>
          `
        }
      )
    });

    return html`
      <div class="slot-fields">
        ${watch(fields)}
      </div>
    `;
  }

  _handleChangeSlotField(e: CustomEvent) {
    const input = e.target as SlInput;
    const field = input.getAttribute("key")! as LogicSlotType;
    let val = parseNumber(input.value);
    if (field === "Quantity") {
      const slot = this.slotSignal.value;
      val = clamp(
        input.valueAsNumber,
        1,
        "item" in slot.occupant.template.value
          ? slot.occupant.template.value.item.max_quantity
          : 1,
      );
    }
    window.VM.get().then((vm) => {
      if (
        !vm.setObjectSlotField(this.objectID, this.slotIndex, field, val, true)
      ) {
        input.value = (
          this.slotSignal.value.logicFields ??
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
