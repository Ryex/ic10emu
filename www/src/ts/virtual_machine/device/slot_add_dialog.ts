import { html, css } from "lit";
import { customElement, property, query, state } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMDeviceDBMixin } from "virtual_machine/base_device";
import type { DeviceDB, DeviceDBEntry } from "virtual_machine/device_db";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.component.js";
import SlDialog from "@shoelace-style/shoelace/dist/components/dialog/dialog.component.js";
import { VMDeviceCard } from "./card";
import { when } from "lit/directives/when.js";
import uFuzzy from "@leeoniya/ufuzzy";
import { LogicField, SlotLogicType, SlotOccupantTemplate } from "ic10emu_wasm";

@customElement("vm-slot-add-dialog")
export class VMSlotAddDialog extends VMDeviceDBMixin(BaseElement) {
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
    `,
  ];

  private _items: Map<string, DeviceDBEntry> = new Map();
  private _filteredItems: DeviceDBEntry[];
  private _datapoints: [string, string][] = [];
  private _haystack: string[] = [];

  private _filter: string = "";
  get filter() {
    return this._filter;
  }

  @state()
  set filter(val: string) {
    this._filter = val;
    this.performSearch();
  }

  private _searchResults: {
    entry: DeviceDBEntry;
    haystackEntry: string;
    ranges: number[];
  }[] = [];

  postDBSetUpdate(): void {
    this._items = new Map(
      Object.values(this.deviceDB.db)
        .filter((entry) => this.deviceDB.items.includes(entry.name), this)
        .map((entry) => [entry.name, entry]),
    );
    this.setupSearch();
    this.performSearch();
  }


  setupSearch() {
    let filteredItemss = Array.from(this._items.values());
    if( typeof this.deviceID !== "undefined" && typeof this.slotIndex !== "undefined") {
      const device = window.VM.vm.devices.get(this.deviceID);
      const dbDevice = this.deviceDB.db[device.prefabName]
      const slot = dbDevice.slots[this.slotIndex]
      const typ = slot.typ;

      if (typeof typ === "string" && typ !== "None") {
        filteredItemss = Array.from(this._items.values()).filter(item => item.item.slotclass === typ);
      }

    }
    this._filteredItems= filteredItemss;
    const datapoints: [string, string][] = [];
    for (const entry of this._filteredItems) {
      datapoints.push(
        [entry.title, entry.name],
        [entry.name, entry.name],
        [entry.desc, entry.name],
      );
    }

    const haystack: string[] = datapoints.map((data) => data[0]);
    this._datapoints = datapoints;
    this._haystack = haystack;
  }

  performSearch() {
    if (this._filter) {

      const uf = new uFuzzy({});
      const [_idxs, info, order] = uf.search(
        this._haystack,
        this._filter,
        0,
        1e3,
      );

      const filtered = order?.map((infoIdx) => ({
        name: this._datapoints[info.idx[infoIdx]][1],
        haystackEntry: this._haystack[info.idx[infoIdx]],
        ranges: info.ranges[infoIdx],
      })) ?? [];

      const uniqueNames = new Set(filtered.map((obj) => obj.name));
      const unique = [...uniqueNames].map(
        (result) => {
          return filtered.find((obj) => obj.name === result);
        },
      );

      this._searchResults = unique.map(({ name, haystackEntry, ranges }) => ({
        entry: this._items.get(name)!,
        haystackEntry,
        ranges,
      }));
    } else {
      // return everything
      this._searchResults = [...this._filteredItems].map((st) => ({
        entry: st,
        haystackEntry: st.title,
        ranges: [],
      }));
    }
  }

  renderSearchResults() {
    return html`
      <div class="mt-2 max-h-48 overflow-y-auto w-full">
        <div class="cursor-pointer hover:bg-neutral-600 rounded px-2 me-1" @click=${this._handleClickNone}>
          None
        </div>
        ${this._searchResults.map((result) => {
        const imgSrc = `img/stationpedia/${result.entry.name}.png`;
        const img = html`
        <img class="w-8 h-8 mr-2" src=${imgSrc} onerror="this.src = '${VMDeviceCard.transparentImg}'" />
        `;
        return html`
        <div class="cursor-pointer hover:bg-neutral-600 rounded px-2 me-1 flex flex-row" key=${result.entry.name} @click=${this._handleClickItem}>
          ${img}
          <div>${result.entry.title}</div>
        </div>
        `;
        })}
      </div>
    `;
  }

  _handleClickNone() {
    window.VM.vm.removeDeviceSlotOccupant(this.deviceID, this.slotIndex);
    this.hide();
  }

  _handleClickItem(e: Event) {
    const div = e.currentTarget as HTMLDivElement;
    const key = div.getAttribute("key");
    const entry = this.deviceDB.db[key];
    const device = window.VM.vm.devices.get(this.deviceID);
    const dbDevice = this.deviceDB.db[device.prefabName]
    const sorting = this.deviceDB.enums["SortingClass"][entry.item.sorting ?? "Default"] ?? 0;
    console.log("using entry", dbDevice);
    const fields: { [key in SlotLogicType]?: LogicField } = Object.fromEntries(
      Object.entries(dbDevice.slotlogic[this.slotIndex] ?? {})
        .map(([slt_s, field_type]) => {
          let slt = slt_s as SlotLogicType;
          let value = 0.0
          if (slt === "FilterType") {
            value = this.deviceDB.enums["GasType"][entry.item.filtertype]
          }
          const field: LogicField = { field_type, value};
          return [slt, field];
        })
    );
    fields["PrefabHash"] = { field_type: "Read", value: entry.hash };
    fields["MaxQuantity"] = { field_type: "Read", value: entry.item.maxquantity ?? 1.0 };
    fields["SortingClass"] = { field_type: "Read", value: sorting };
    fields["Quantity"] = { field_type: "Read", value: 1 };

    const template: SlotOccupantTemplate = {
      fields
    }
    window.VM.vm.setDeviceSlotOccupant(this.deviceID, this.slotIndex, template);
    this.hide();
  }

  @query("sl-dialog.slot-add-dialog") dialog: SlDialog;
  @query(".device-search-input") searchInput: SlInput;

  render() {
    const device = window.VM.vm.devices.get(this.deviceID);
    const name = device?.name ?? device?.prefabName ?? "";
    const id = this.deviceID ?? 0;
    return html`
      <sl-dialog
        label="Edit device ${id} : ${name} Slot ${this.slotIndex}"
        class="slot-add-dialog"
        @sl-hide=${this._handleDialogHide}
      >
        <sl-input class="device-search-input" autofocus placeholder="filter" clearable
          @sl-input=${this._handleSearchInput}>
          <span slot="prefix">Search Items</span>
          <sl-icon slot="suffix" name="search"></sl-icon>
        </sl-input>
        ${when(
          typeof this.deviceID !== "undefined" &&
          typeof this.slotIndex !== "undefined",
          () => html`
            <div class="flex flex-row overflow-x-auto">
              ${this.renderSearchResults()}
            </div>
          `,
          () => html``,
        )}
      </sl-dialog>
    `;
  }

  private filterTimeout: number | undefined;

  _handleSearchInput(_e: CustomEvent) {
    if (this.filterTimeout) {
      clearTimeout(this.filterTimeout);
    }
    const that = this;
    this.filterTimeout = setTimeout(() => {
      that.filter = that.searchInput.value;
      that.filterTimeout = undefined;
    }, 200);
  }

  _handleDialogHide() {
    this.deviceID = undefined;
    this.slotIndex = undefined;
  }

  @state() private deviceID: number;
  @state() private slotIndex: number;

  show(deviceID: number, slotIndex: number) {
    this.deviceID = deviceID;
    this.slotIndex = slotIndex;
    this.setupSearch();
    this.performSearch();
    this.dialog.show();
    this.searchInput.select();
  }

  hide() {
    this.dialog.hide();
  }
}
