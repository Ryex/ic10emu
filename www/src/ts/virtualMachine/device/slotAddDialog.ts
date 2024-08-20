import { html, css, nothing } from "lit";
import { customElement, property, query, state } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { ComputedObjectSignals, globalObjectSignalMap, VMTemplateDBMixin } from "virtualMachine/baseDevice";
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
import { computed, ReadonlySignal, signal, Signal, watch } from "@lit-labs/preact-signals";
import { repeat } from "lit/directives/repeat.js";

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

  private _items: Signal<Record<string, SlotableItemTemplate>> = signal({});
  private _filteredItems: ReadonlySignal<SlotableItemTemplate[]>;
  private _datapoints: ReadonlySignal<[string, string][]>;
  private _haystack: ReadonlySignal<string[]>;

  private _filter: Signal<string> = signal("");
  get filter() {
    return this._filter.peek();
  }

  set filter(val: string) {
    this._filter.value = val;
  }

  private _searchResults: ReadonlySignal<{
    entry: SlotableItemTemplate;
    haystackEntry: string;
    ranges: number[];
  }[]>;

  constructor() {
    super();
    this.setupSearch();
  }

  postDBSetUpdate(): void {
    this._items.value = Object.fromEntries(
      Array.from(Object.values(this.templateDB)).flatMap((template) => {
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
  }

  setupSearch() {
    const filteredItems = computed(() => {
      let filtered = Array.from(Object.values(this._items.value));
      const obj = globalObjectSignalMap.get(this.objectID.value ?? null);
      if (obj != null) {
        const template = obj.template;
        const slot = "slots" in template.value ? template.value.slots[this.slotIndex.value] : null;
        const typ = slot.typ;

        if (typeof typ === "string" && typ !== "None") {
          filtered = Array.from(Object.values(this._items.value)).filter(
            (item) => item.item.slot_class === typ,
          );
        }
      }
      return filtered;
    });
    this._filteredItems = filteredItems;

    const datapoints = computed(() => {
      const datapoints: [string, string][] = [];
      for (const entry of this._filteredItems.value) {
        datapoints.push(
          [entry.prefab.name, entry.prefab.prefab_name],
          [entry.prefab.prefab_name, entry.prefab.prefab_name],
          [entry.prefab.desc, entry.prefab.prefab_name],
        );
      }
      return datapoints;
    });
    this._datapoints = datapoints;

    const haystack: Signal<string[]> = computed(() => {
      return datapoints.value.map((data) => data[0]);
    });
    this._haystack = haystack;

    const searchResults = computed(() => {
      let results;
      if (this._filter.value) {
        const uf = new uFuzzy({});
        const [_idxs, info, order] = uf.search(
          this._haystack.value,
          this._filter.value,
          0,
          1e3,
        );

        const filtered =
          order?.map((infoIdx) => ({
            name: this._datapoints.value[info.idx[infoIdx]][1],
            haystackEntry: this._haystack.value[info.idx[infoIdx]],
            ranges: info.ranges[infoIdx],
          })) ?? [];

        const uniqueNames = new Set(filtered.map((obj) => obj.name));
        const unique = [...uniqueNames].map((result) => {
          return filtered.find((obj) => obj.name === result);
        });

        results = unique.map(({ name, haystackEntry, ranges }) => ({
          entry: this._items.value[name]!,
          haystackEntry,
          ranges,
        }));
      } else {
        // return everything
        results = [...this._filteredItems.value].map((st) => ({
          entry: st,
          haystackEntry: st.prefab.prefab_name,
          ranges: [],
        }));
      }
      return results;
    });
    this._searchResults = searchResults;
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
    const resultsHtml = computed(() => {
      return repeat(
        this._searchResults.value,
        (result) => {
          return result.entry.prefab.prefab_hash;
        },
        (result) => {
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
        }
      );
    });
    return html`
      <div class="mt-2 max-h-48 overflow-y-auto w-full">
        ${enableNone ? none : ""}
        ${watch(resultsHtml)}
      </div>
    `;
  }

  _handleClickNone() {
    window.VM.vm.removeSlotOccupant(this.objectID.peek(), this.slotIndex.peek());
    this.hide();
  }

  _handleClickItem(e: Event) {
    const div = e.currentTarget as HTMLDivElement;
    const key = parseInt(div.getAttribute("key"));
    const entry = this.templateDB.get(key) as SlotableItemTemplate;
    const obj = window.VM.vm.objects.get(this.objectID.peek());
    const dbTemplate = obj.peek().template;
    console.log("using entry", dbTemplate);

    const template: FrozenObject = {
      obj_info: {
        prefab: entry.prefab.prefab_name,
      } as ObjectInfo,
      database_template: true,
      template: undefined,
    };
    window.VM.vm.setSlotOccupant(this.objectID.peek(), this.slotIndex.peek(), template, 1);
    this.hide();
  }

  @query("sl-dialog.slot-add-dialog") dialog: SlDialog;
  @query(".device-search-input") searchInput: SlInput;

  render() {
    const device = computed(() => {
      return globalObjectSignalMap.get(this.objectID.value) ?? null;
    });
    const name = computed(() => {
      return device.value?.displayName.value ?? nothing;

    });
    const id = computed(() => this.objectID.value ?? 0);
    const resultsHtml = html`
      <div class="flex flex-row overflow-x-auto">
        ${this.renderSearchResults()}
      </div>
    `;
    return html`
      <sl-dialog
        label="Edit device ${watch(id)} : ${watch(name)} Slot ${watch(this.slotIndex)}"
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
        ${resultsHtml}
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

  private objectID: Signal<number> = signal(null);
  private slotIndex: Signal<number> = signal(0);

  show(objectID: number, slotIndex: number) {
    this.objectID.value = objectID;
    this.slotIndex.value = slotIndex;
    this.dialog.show();
    this.searchInput.select();
  }

  hide() {
    this.dialog.hide();
  }
}
