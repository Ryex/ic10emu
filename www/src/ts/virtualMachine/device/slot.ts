import { html, css } from "lit";
import { customElement, property } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMObjectMixin, } from "virtualMachine/baseDevice";
import {
  clamp,
  crc32,
  displayNumber,
  isSome,
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
  objectID: number;
  slotIndex: number;
}

@customElement("vm-object-slot")
export class VMDeviceSlot extends VMObjectMixin(BaseElement) {

  slotIndexSignal: Signal<number> = signal(0);

  @property({ type: Number })
  get slotIndex() {
    return this.slotIndexSignal.peek();
  }

  set slotIndex(val: number) {
    this.slotIndexSignal.value = val;
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


  slotInfo = computed(() => {
    return this.vm.value?.state.getObjectSlotInfo(this.objectIDSignal.value, this.slotIndexSignal.value).value ?? null;
  });

  slotOccupantId = computed(() => {
    const slot = this.slotInfo.value ?? null;
    return slot?.occupant ?? null;
  });

  slotOccupant = computed(() => {
    return this.vm.value?.state.getObject(this.slotOccupantId.value).value;
  });

  slotFieldTypes = computed(() => {
    return this.vm.value?.state.getObjectSlotFieldNames(this.objectIDSignal.value, this.slotIndexSignal.value).value ?? [];
  });

  slotOccupantImg = computed(() => {
    const occupant = this.slotOccupant.value;
    if (isSome(occupant)) {
      const prefabName = occupant.obj_info.prefab;
      return `img/stationpedia/${prefabName}.png`;
    } else {
      const slot = this.vm.value?.state.getObjectSlotInfo(this.objectIDSignal.value, this.slotIndexSignal.value).value ?? null;
      return `img/stationpedia/SlotIcon_${slot?.typ}.png`;
    }
  });

  slotOccupantPrefabName = computed(() => {
    const occupant = this.slotOccupant.value;
    return occupant?.obj_info.prefab ?? null;
  });

  slotQuantity = computed(() => {
    const slot = this.slotInfo.value ?? null;
    return slot?.quantity;
  });

  slotTyp = computed(() => {
    const slot = this.slotInfo.value ?? null;
    return slot?.typ;
  });

  slotName = computed(() => {
    const slot = this.slotInfo.value ?? null;
    return slot?.name ?? slot?.typ;
  });

  slotDisplayName = computed(() => {
    return this.slotOccupantPrefabName.value ?? this.slotName ?? "";
  });

  maxQuantity = computed(() => {
    const occupant = this.slotOccupant.value;
    const template = occupant?.template ?? null;
    if (isSome(template) && "item" in template) {
      return template.item.max_quantity;
    }
    return 1;
  });

  renderHeader() {
    // const inputIdBase = computed(() => `vmDeviceSlot${this.objectIDSignal.value}Slot${this.slotIndexSignal.value}Head`);
    const img = html`<img
      class="w-10 h-10"
      src="${watch(this.slotOccupantImg)}"
      onerror="this.src = '${VMDeviceCard.transparentImg}'"
    />`;

    const enableQuantityInput = false;

    const removeDisabled = computed(() => {
      return this.vm.value?.activeIC.value === this.objectIDSignal.value && this.slotTyp.value === "ProgrammableChip"
    });

    const tooltipContent = computed(() => {
      return removeDisabled.value
        ? "Removing the selected Active IC is disabled"
        : "Remove Occupant"
    });

    const quantityContent = computed(() => {
      if (this.slotOccupant.value != null) {
        return html`
          <div
            class="absolute bottom-0 right-0 mr-1 mb-1 text-xs
              text-neutral-200/90 font-mono bg-neutral-500/40 rounded pl-1 pr-1"
          >
            <small>
              ${watch(this.slotQuantity)}/${watch(this.maxQuantity)}
            </small>
          </div>`
      } else {
        return null
      }
    });

    const slotName = computed(() => {
      return html` <span> ${watch(this.slotDisplayName)} </span> `
    });

    const inputContent = computed(() => {
      if (isSome(this.slotOccupant.value)) {
        return html`
          <div class="quantity-input ms-auto pl-2 mt-auto mb-auto me-2">
            ${enableQuantityInput
            ? html`<sl-input
                    type="number"
                    size="small"
                    .value=${watch(this.slotQuantity)}
                    .min=${1}
                    .max=${watch(this.maxQuantity)}
                    @sl-change=${this._handleSlotQuantityChange}
                  >
                    <div slot="help-text">
                      <span>
                        Max Quantity:
                        ${watch(this.maxQuantity)}
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
          <sl-tooltip content="${watch(this.slotDisplayName)}">
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
              ><span class="p-1">${watch(this.slotTyp)}</span>
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
        detail: { objectID: this.objectID, slotIndex: this.slotIndex },
      }),
    );
  }

  _handleSlotQuantityChange(e: Event) {
    const input = e.currentTarget as SlInput;
    const val = clamp(
      input.valueAsNumber,
      1,
      this.maxQuantity.peek()
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
      input.value = this.slotQuantity.value.toString();
    }
  }

  renderFields() {
    const inputIdBase = computed(() => `vmDeviceSlot${this.objectIDSignal.value}Slot${this.slotIndexSignal.value}Field`);
    const fields = computed(() => {
      return this.slotFieldTypes.value.map(
        field => {
          const slotField = computed(() => {
            return this.vm.value?.state.getObjectSlotField(this.objectIDSignal.value, this.slotIndexSignal.value, field).value ?? null;
          });
          const fieldValue = computed(() => {
            return displayNumber(slotField.value?.value ?? null);
          })
          const fieldAccessType = computed(() => {
            return slotField.value?.field_type ?? null;
          })
          return html`
            <sl-input
              id="${watch(inputIdBase)}${field}"
              key="${field}"
              value="${watch(fieldValue)}"
              size="small"
              @sl-change=${this._handleChangeSlotField}
            >
              <span slot="prefix">${field}</span>
              <sl-copy-button
                slot="suffix"
                from="${watch(inputIdBase)}${field}.value"
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
      const slot = this.slotIndexSignal.value;
      val = clamp(
        input.valueAsNumber,
        1,
        this.maxQuantity.peek(),
      );
    }
    window.VM.get().then((vm) => {
      if (
        !vm.setObjectSlotField(this.objectID, this.slotIndex, field, val, true)
      ) {
        input.value = (vm.state.getObjectSlotField(this.objectIDSignal.value, this.slotIndexSignal.value, field).value.value ?? null).toString();
      }
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
