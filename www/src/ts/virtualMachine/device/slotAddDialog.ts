import { html, css, nothing } from "lit";
import { customElement, query } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMObjectMixin } from "virtualMachine/baseDevice";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.component.js";
import SlDialog from "@shoelace-style/shoelace/dist/components/dialog/dialog.component.js";
import { VMDeviceCard } from "./card";
import uFuzzy from "@leeoniya/ufuzzy";
import {
  FrozenObject,
  ItemInfo,
  ObjectInfo,
  ObjectTemplate,
  TemplateDatabase,
} from "ic10emu_wasm";
import { computed, ReadonlySignal, signal, Signal, watch } from "@lit-labs/preact-signals";
import { repeat } from "lit/directives/repeat.js";
import { isSome, structuralEqual } from "utils";

type SlotableItemTemplate = Extract<ObjectTemplate, { item: ItemInfo }>;

@customElement("vm-slot-add-dialog")
export class VMSlotAddDialog extends VMObjectMixin(BaseElement) {
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

  private _filter: Signal<string> = signal("");
  get filter() {
    return this._filter.peek();
  }

  set filter(val: string) {
    this._filter.value = val;
  }

  templateDB = computed((): TemplateDatabase => {
    return this.vm.value?.state.templateDB.value ?? null;
  });

  items = (() => {
    let last: { [k: string]: SlotableItemTemplate } = null;
    return computed(() => {
      const next = Object.fromEntries(
        Array.from(this.templateDB.value?.values() ?? []).flatMap((template) => {
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
      if (structuralEqual(last, next)) {
        return last;
      }
      last = next;
      return next;
    });
  })();

  filteredItems = (() => {
    let last: SlotableItemTemplate[] = null;
    return computed(() => {
      let filtered = Array.from(Object.values(this.items.value));
      const obj = this.vm.value?.state.getObject(this.objectIDSignal.value).value;
      if (isSome(obj)) {
        const template = obj.template;
        const slot = "slots" in template ? template.slots.get(this.slotIndex.value.toString()) : null;
        const typ = typeof slot === "object" && "Direct" in slot ? slot.Direct.class : null;

        if (typeof typ === "string" && typ !== "None") {
          filtered = Array.from(Object.values(this.items.value)).filter(
            (item) => item.item.slot_class === typ,
          );
        }
      }
      if (structuralEqual(last, filtered)) {
        return last;
      }
      last = filtered;
      return filtered;
    });
  })();

  datapoints = (() => {
    let last: [string, string][] = null;
    return computed(() => {
      const datapoints: [string, string][] = [];
      for (const entry of this.filteredItems.value) {
        datapoints.push(
          [entry.prefab.name, entry.prefab.prefab_name],
          [entry.prefab.prefab_name, entry.prefab.prefab_name],
          [entry.prefab.desc, entry.prefab.prefab_name],
        );
      }
      if (structuralEqual(last, datapoints)) {
        return last;
      }
      last = datapoints;
      return datapoints;
    });
  })();

  haystack = (() => {
    let last: string[] = null;
    return computed(() => {
      const hay = this.datapoints.value.map(data => data[0])
      if (structuralEqual(last, hay)) {
        return last;
      }
      last = hay;
      return hay
    });
  })();

  searchResults: ReadonlySignal<{
    entry: SlotableItemTemplate
    haystackEntry: string,
    ranges: number[]
  }[]> = (() => {
    let last: {
      entry: SlotableItemTemplate
      haystackEntry: string,
      ranges: number[]
    }[] = null;
    return computed(() => {
      let results;
      if (this._filter.value) {
        const uf = new uFuzzy({});
        const [_idxs, info, order] = uf.search(
          this.haystack.value,
          this._filter.value,
          0,
          1e3,
        );

        const filtered =
          order?.map((infoIdx) => ({
            name: this.datapoints.value[info.idx[infoIdx]][1],
            haystackEntry: this.haystack.value[info.idx[infoIdx]],
            ranges: info.ranges[infoIdx],
          })) ?? [];

        const uniqueNames = new Set(filtered.map((obj) => obj.name));
        const unique = [...uniqueNames].map((result) => {
          return filtered.find((obj) => obj.name === result);
        });

        results = unique.map(({ name, haystackEntry, ranges }) => ({
          entry: this.items.value[name]!,
          haystackEntry,
          ranges,
        }));
      } else {
        // return everything
        results = [...this.filteredItems.value].map((st) => ({
          entry: st,
          haystackEntry: st.prefab.prefab_name,
          ranges: [],
        }));
      }
      if (structuralEqual(last, results)) {
        return last;
      }
      last = results
      return results;
    });
  })();

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
        this.searchResults.value,
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
    window.VM.vm.removeSlotOccupant(this.objectID, this.slotIndex.peek());
    this.hide();
  }

  _handleClickItem(e: Event) {
    const div = e.currentTarget as HTMLDivElement;
    const key = parseInt(div.getAttribute("key"));
    const entry = this.templateDB.value.get(key) as SlotableItemTemplate;
    const template: FrozenObject = {
      obj_info: {
        prefab: entry.prefab.prefab_name,
      } as ObjectInfo,
      database_template: true,
      template: undefined,
    };
    window.VM.vm.setSlotOccupant(this.objectID, this.slotIndex.peek(), template, 1);
    this.hide();
  }

  @query("sl-dialog.slot-add-dialog") accessor dialog: SlDialog;
  @query(".device-search-input") accessor searchInput: SlInput;

  render() {
   const name = computed(() => {
      return this.vm.value?.state.getObjectDisplayName(this.objectIDSignal.value) ?? "";
    });
    const id = computed(() => this.objectIDSignal.value ?? 0);
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
    this.objectIDSignal.value = null;
    this.slotIndex.value = null;
  }

  private slotIndex: Signal<number> = signal(0);

  show(objectID: number, slotIndex: number) {
    this.objectIDSignal.value = objectID;
    this.slotIndex.value = slotIndex;
    this.dialog.show();
    this.searchInput.select();
  }

  hide() {
    this.dialog.hide();
  }
}
