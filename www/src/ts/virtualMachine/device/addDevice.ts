import { html, css, nothing } from "lit";
import { customElement, query, state } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";

import SlInput from "@shoelace-style/shoelace/dist/components/input/input.js";

import SlDrawer from "@shoelace-style/shoelace/dist/components/drawer/drawer.js";
import { repeat } from "lit/directives/repeat.js";
import { cache } from "lit/directives/cache.js";
import { default as uFuzzy } from "@leeoniya/ufuzzy";
import { when } from "lit/directives/when.js";
import { unsafeHTML } from "lit/directives/unsafe-html.js";
import { LogicInfo, ObjectTemplate, StructureInfo } from "ic10emu_wasm";
import { VMObjectMixin } from "virtualMachine/baseDevice";
import { computed, ReadonlySignal, signal, Signal, watch } from "@lit-labs/preact-signals";
import { isSome, range, structuralEqual } from "utils";

type LogicableStructureTemplate = Extract<
  ObjectTemplate,
  { structure: StructureInfo; logic: LogicInfo }
>;

type SearchResult = {
  entry: LogicableStructureTemplate;
  haystackEntry: string;
  ranges: number[];
};

@customElement("vm-add-device-button")
export class VMAddDeviceButton extends VMObjectMixin(BaseElement) {
  static styles = [
    ...defaultCss,
    css`
      .add-device-drawer {
        --size: 36rem;
        --footer-spacing: var(--sl-spacing-small);
      }

      .card {
        margin-top: var(--sl-spacing-small);
        margin-right: var(--sl-spacing-small);
      }
    `,
  ];

  @query("sl-drawer") drawer: SlDrawer;
  @query(".device-search-input") searchInput: SlInput;

  templateDB = computed(() => {
    return this.vm.value?.state.templateDB.value ?? null;
  });

  structures = (() => {
    let last: Map<string, LogicableStructureTemplate> = null
    return computed(() => {
      const next = new Map(
        Array.from(this.templateDB.value?.values() ?? []).flatMap((template) => {
          if ("structure" in template && "logic" in template) {
            return [[template.prefab.prefab_name, template]] as [
              string,
              LogicableStructureTemplate,
            ][];
          } else {
            return [] as [string, LogicableStructureTemplate][];
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

  datapoints = (() => {
    let last: [string, string][] = null;
    return computed(() => {
      const next = [...this.structures.value.values()].flatMap((entry): [string, string][] => {
        return [
          [entry.prefab.name, entry.prefab.prefab_name],
          [entry.prefab.prefab_name, entry.prefab.prefab_name],
          [entry.prefab.desc, entry.prefab.prefab_name],
        ]
      });
      if (structuralEqual(last, next)) {
        return last;
      }
      last = next;
      return next;
    });
  })();

  haystack = (() => {
    let last: string[] = null;
    return computed(() => {
      const next = this.datapoints.value.map(data => data[0]);
      if (structuralEqual(last, next)) {
        return last;
      }
      last = next;
      return next;
    });
  })();

  private _filter: Signal<string> = signal("");
  private page = signal(0);

  get filter() {
    return this._filter.peek();
  }

  set filter(val: string) {
    this._filter.value = val;
    this.page.value = 0;
  }

  private searchResults: ReadonlySignal<SearchResult[]> = (() => {
    let last: SearchResult[] = null;
    return computed((): SearchResult[] => {
      let next: SearchResult[];
      if (this._filter.value) {
        const uf = new uFuzzy({});
        const [_idxs, info, order] = uf.search(
          this.haystack.value,
          this._filter.value,
          0,
          1e3,
        );

        const filtered = order?.map((infoIdx) => ({
          name: this.datapoints.value[info.idx[infoIdx]][1],
          haystackEntry: this.haystack.value[info.idx[infoIdx]],
          ranges: info.ranges[infoIdx],
        }));

        const unique = [...new Set(filtered.map((obj) => obj.name))].map(
          (result) => {
            return filtered.find((obj) => obj.name === result);
          },
        );

        next = unique.map(({ name, haystackEntry, ranges }) => ({
          entry: this.structures.value.get(name)!,
          haystackEntry,
          ranges,
        }));
      } else {
        // return everything
        next = [...this.structures.value.values()].map((st) => ({
          entry: st,
          haystackEntry: st.prefab.prefab_name,
          ranges: [],
        }));
      }
      if (structuralEqual(last, next)) {
        return last;
      }
      last = next;
      return next;
    });
  })();

  numSearchResults = computed(() => {
    return this.searchResults.value?.length ?? 0;
  })

  private filterTimeout: ReturnType<typeof setTimeout>;

  perPage: Signal<number> = signal(40);
  maxResultsRendered: Signal<number> = signal(20);

  renderSearchResults() {
    const totalPages = computed(() => Math.ceil((this.searchResults.value?.length ?? 0) / this.perPage.value));
    const pageKeys = computed(() => range(totalPages.value));
    const extra = computed((): {
      entry: { prefab: { name: string; prefab_name: string } };
      haystackEntry: string;
      ranges: number[];
    }[] => {
      const next: {
        entry: { prefab: { name: string; prefab_name: string } };
        haystackEntry: string;
        ranges: number[];
      }[] = [];

      if (this.page.value < totalPages.value - 1) {
        next.push({
          entry: { prefab: { name: "", prefab_name: this.filter } },
          haystackEntry: "...",
          ranges: [],
        });
      }
      return next;
    });
    const pageKeyButtons = computed(() => {
      return pageKeys.value.map(
        (key, index) => {
          const textColorClass = computed(() => index === this.page.value ? "text-purple-500" : nothing)
          return html`
            <span
              class="p-2 cursor-pointer hover:text-purple-400 ${watch(textColorClass)}"
              key=${key}
              @click=${this._handlePageChange}
            >
              ${key + 1}${index < totalPages.value - 1 ? "," : nothing}
            </span>
          `
        }
      )
    })

    const results = computed(() => [
      ...this.searchResults.value.slice(
        this.perPage.value * this.page.value,
        this.perPage.value * this.page.value + this.perPage.value,
      ),
      ...extra.value,
    ].map((result) => {
      let hay = result.haystackEntry.slice(0, 15);
      if (result.haystackEntry.length > 15) hay += "...";
      const ranges = result.ranges.filter((pos) => pos < 20);
      const key = result.entry.prefab.prefab_name;
      return html`
        <div
          class="m-2 text-neutral-200/90 italic cursor-pointer rounded bg-neutral-700 hover:bg-purple-500 px-1"
          key=${key}
          @click=${this._handleHaystackClick}
        >
          ${result.entry.prefab.name} (<small class="text-sm">
            ${ranges.length
          ? unsafeHTML(uFuzzy.highlight(hay, ranges))
          : hay
        } </small>)
        </div>
      `;
    }));

    const cards = computed(() => {
      if (this.numSearchResults.value <= this.maxResultsRendered.value) {
        return repeat(
          this.searchResults.value ?? [],
          (result) => result.entry.prefab.prefab_name,
          (result) =>
            html`
          <vm-device-template
            prefabName=${result.entry.prefab.prefab_name}
            class="card"
            @add-device-template=${this._handleDeviceAdd}
          >
          </vm-device-template>
        `,
        );
      } else {
        return nothing;
      }
    });
    const searchResultsHtml = computed(() => {
      if (this.numSearchResults.value > 0 && this.numSearchResults.value <= this.maxResultsRendered.value) {
        return html`${watch(cards)}`
      } else {
        const excessResults = this.numSearchResults.value - this.maxResultsRendered.value
        const filterText = (() => {
          if (this.numSearchResults.value > this.maxResultsRendered.value) {
            return html`, filter <span class="font-mono">${excessResults}</span> more to get cards`
          }
          return nothing
        })();
        return html`
        <div class="p-2">
          <div class="flex flex-row">
            <p class="p-2">
              <sl-format-number
                .value=${this.numSearchResults.value}
              ></sl-format-number>
              results${filterText}
            </p>
            <div class="p-2 ml-2">
              Page:
              ${watch(pageKeyButtons)}
            </div>
          </div>
          <div class="flex flex-row flex-wrap">
            ${watch(results)}
          </div>
        </div>
      `

      }
    });
    return html`${watch(searchResultsHtml)}`;
  }

  _handlePageChange(e: Event) {
    const span = e.currentTarget as HTMLSpanElement;
    const key = parseInt(span.getAttribute("key"));
    this.page.value = key;
  }

  _handleHaystackClick(e: Event) {
    const div = e.currentTarget as HTMLDivElement;
    const key = div.getAttribute("key");
    if (key === this.filter) {
      this.page.value += 1;
    } else {
      this.filter = key;
      this.searchInput.value = key;
    }
  }

  _handleDeviceAdd() {
    this.drawer.hide();
  }

  render() {
    return html`
      <sl-button
        variant="neutral"
        outline
        pill
        @click=${this._handleAddButtonClick}
      >
        Add Device
      </sl-button>
      <sl-drawer class="add-device-drawer" placement="bottom" no-header>
        <sl-input
          class="device-search-input"
          autofocus
          placeholder="filter"
          clearable
          @sl-input=${this._handleSearchInput}
        >
          <span slot="prefix">Search Structures</span>
          <sl-icon slot="suffix" name="search"></sl-icon>
        </sl-input>
        <div class="flex flex-row overflow-x-auto">
          ${this.renderSearchResults()}
        </div>
        <sl-button
          slot="footer"
          variant="primary"
          @click=${() => {
        this.drawer.hide();
      }}
        >
          Close
        </sl-button>
      </sl-drawer>
    `;
  }

  _handleSearchInput(e: CustomEvent) {
    if (this.filterTimeout) {
      clearTimeout(this.filterTimeout);
    }
    const that = this;
    this.filterTimeout = setTimeout(() => {
      that.filter = that.searchInput.value;
      that.filterTimeout = undefined;
    }, 200);
  }

  _handleAddButtonClick() {
    this.drawer.show();
    this.searchInput.select();
  }
}
