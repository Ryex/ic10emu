import { html, css } from "lit";
import { customElement, property, query, state } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMTemplateDBMixin } from "virtual_machine/base_device";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.component.js";
import SlDialog from "@shoelace-style/shoelace/dist/components/dialog/dialog.component.js";
import { VMDeviceCard } from "./card";
import { when } from "lit/directives/when.js";
import uFuzzy from "@leeoniya/ufuzzy";
import {
  FrozenObject,
  ItemInfo,
  LogicField,
  LogicSlotType,
  ObjectInfo,
  ObjectTemplate,
} from "ic10emu_wasm";

type SlotableItemTemplate = Extract<ObjectTemplate, { item: ItemInfo }>;

@customElement("vm-slot-add-dialog")
export class VMSlotAddDialog extends VMTemplateDBMixin(BaseElement) {
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

  private _items: Map<string, SlotableItemTemplate> = new Map();
  private _filteredItems: SlotableItemTemplate[];
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
    entry: SlotableItemTemplate;
    haystackEntry: string;
    ranges: number[];
  }[] = [];

  postDBSetUpdate(): void {
    this._items = new Map(
      Array.from(this.templateDB.values()).flatMap((template) => {
        if ("item" in template) {
          return [[template.prefab.prefab_name, template]] as [
            string,
            SlotableItemTemplate,
          ][];
        } else {
          return [] as [string, SlotableItemTemplate][];
        }
      }),
    );
    this.setupSearch();
    this.performSearch();
  }

  setupSearch() {
    let filteredItems = Array.from(this._items.values());
    if (
      typeof this.objectID !== "undefined" &&
      typeof this.slotIndex !== "undefined"
    ) {
      const obj = window.VM.vm.objects.get(this.objectID);
      const template = obj.template;
      const slot = "slots" in template ? template.slots[this.slotIndex] : null;
      const typ = slot.typ;

      if (typeof typ === "string" && typ !== "None") {
        filteredItems = Array.from(this._items.values()).filter(
          (item) => item.item.slot_class === typ,
        );
      }
    }
    this._filteredItems = filteredItems;
    const datapoints: [string, string][] = [];
    for (const entry of this._filteredItems) {
      datapoints.push(
        [entry.prefab.name, entry.prefab.prefab_name],
        [entry.prefab.prefab_name, entry.prefab.prefab_name],
        [entry.prefab.desc, entry.prefab.prefab_name],
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

      const filtered =
        order?.map((infoIdx) => ({
          name: this._datapoints[info.idx[infoIdx]][1],
          haystackEntry: this._haystack[info.idx[infoIdx]],
          ranges: info.ranges[infoIdx],
        })) ?? [];

      const uniqueNames = new Set(filtered.map((obj) => obj.name));
      const unique = [...uniqueNames].map((result) => {
        return filtered.find((obj) => obj.name === result);
      });

      this._searchResults = unique.map(({ name, haystackEntry, ranges }) => ({
        entry: this._items.get(name)!,
        haystackEntry,
        ranges,
      }));
    } else {
      // return everything
      this._searchResults = [...this._filteredItems].map((st) => ({
        entry: st,
        haystackEntry: st.prefab.prefab_name,
        ranges: [],
      }));
    }
  }

  renderSearchResults() {
    const enableNone = false;
    const none = html`
      <div
        class="cursor-pointer hover:bg-neutral-600 rounded px-2 py-1 me-1"
        @click=${this._handleClickNone}
      >
        None
      </div>
    `;
    return html`
      <div class="mt-2 max-h-48 overflow-y-auto w-full">
        ${enableNone ? none : ""}
        ${this._searchResults.map((result) => {
      const imgSrc = `img/stationpedia/${result.entry.prefab.prefab_name}.png`;
      const img = html`
            <img
              class="w-8 h-8 mr-2"
              src=${imgSrc}
              onerror="this.src = '${VMDeviceCard.transparentImg}'"
            />
          `;
      return html`
            <div
              class="cursor-pointer hover:bg-neutral-600 rounded px-2 py-1 me-1 flex flex-row"
              key=${result.entry.prefab.prefab_hash.toString()}
              @click=${this._handleClickItem}
            >
              ${img}
              <div>${result.entry.prefab.name}</div>
            </div>
          `;
    })}
      </div>
    `;
  }

  _handleClickNone() {
    window.VM.vm.removeSlotOccupant(this.objectID, this.slotIndex);
    this.hide();
  }

  _handleClickItem(e: Event) {
    const div = e.currentTarget as HTMLDivElement;
    const key = parseInt(div.getAttribute("key"));
    const entry = this.templateDB.get(key) as SlotableItemTemplate;
    const obj = window.VM.vm.objects.get(this.objectID);
    const dbTemplate = obj.template;
    console.log("using entry", dbTemplate);

    const template: FrozenObject = {
      obj_info: {
        prefab: entry.prefab.prefab_name,
      } as ObjectInfo,
      database_template: true,
      template: undefined,
    };
    window.VM.vm.setSlotOccupant(this.objectID, this.slotIndex, template, 1);
    this.hide();
  }

  @query("sl-dialog.slot-add-dialog") dialog: SlDialog;
  @query(".device-search-input") searchInput: SlInput;

  render() {
    const device = window.VM.vm.objects.get(this.objectID);
    const name = device?.obj_info.name ?? device?.obj_info.prefab ?? "";
    const id = this.objectID ?? 0;
    return html`
      <sl-dialog
        label="Edit device ${id} : ${name} Slot ${this.slotIndex}"
        class="slot-add-dialog"
        @sl-hide=${this._handleDialogHide}
      >
        <sl-input
          class="device-search-input"
          autofocus
          placeholder="filter"
          clearable
          @sl-input=${this._handleSearchInput}
        >
          <span slot="prefix">Search Items</span>
          <sl-icon slot="suffix" name="search"></sl-icon>
        </sl-input>
        ${when(
      typeof this.objectID !== "undefined" &&
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
    this.objectID = undefined;
    this.slotIndex = undefined;
  }

  @state() private objectID: number;
  @state() private slotIndex: number;

  show(objectID: number, slotIndex: number) {
    this.objectID = objectID;
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
