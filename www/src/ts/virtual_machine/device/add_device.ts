
import { html, css } from "lit";
import { customElement, query, state } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";

import SlInput from "@shoelace-style/shoelace/dist/components/input/input.js";

import SlDrawer from "@shoelace-style/shoelace/dist/components/drawer/drawer.js";
import type { DeviceDBEntry } from "virtual_machine/device_db";
import { repeat } from "lit/directives/repeat.js";
import { cache } from "lit/directives/cache.js";
import { default as uFuzzy } from "@leeoniya/ufuzzy";
import { when } from "lit/directives/when.js";
import { unsafeHTML } from "lit/directives/unsafe-html.js";
import { VMTemplateDBMixin } from "virtual_machine/base_device";


@customElement("vm-add-device-button")
export class VMAddDeviceButton extends VMTemplateDBMixin(BaseElement) {
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

  private _structures: Map<string, DeviceDBEntry> = new Map();
  private _datapoints: [string, string][] = [];
  private _haystack: string[] = [];

  postDBSetUpdate(): void {
    this._structures = new Map(
      Object.values(this.templateDB.db)
        .filter((entry) => this.templateDB.structures.includes(entry.name), this)
        .filter(
          (entry) => this.templateDB.logic_enabled.includes(entry.name),
          this,
        )
        .map((entry) => [entry.name, entry]),
    );

    const datapoints: [string, string][] = [];
    for (const entry of this._structures.values()) {
      datapoints.push(
        [entry.title, entry.name],
        [entry.name, entry.name],
        [entry.desc, entry.name],
      );
    }
    const haystack: string[] = datapoints.map((data) => data[0]);
    this._datapoints = datapoints;
    this._haystack = haystack;
    this.performSearch();
  }

  private _filter: string = "";

  get filter() {
    return this._filter;
  }

  @state()
  set filter(val: string) {
    this._filter = val;
    this.page = 0;
    this.performSearch();
  }

  private _searchResults: {
    entry: DeviceDBEntry;
    haystackEntry: string;
    ranges: number[];
  }[] = [];

  private filterTimeout: number | undefined;

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
      }));

      const unique = [...new Set(filtered.map((obj) => obj.name))].map(
        (result) => {
          return filtered.find((obj) => obj.name === result);
        },
      );

      this._searchResults = unique.map(({ name, haystackEntry, ranges }) => ({
        entry: this._structures.get(name)!,
        haystackEntry,
        ranges,
      }));
    } else {
      // return everything
      this._searchResults = [...this._structures.values()].map((st) => ({
        entry: st,
        haystackEntry: st.title,
        ranges: [],
      }));
    }
  }

  connectedCallback(): void {
    super.connectedCallback();
    window.VM.get().then((vm) =>
      vm.addEventListener(
        "vm-device-db-loaded",
        this._handleDeviceDBLoad.bind(this),
      ),
    );
  }

  _handleDeviceDBLoad(e: CustomEvent) {
    this.templateDB = e.detail;
  }

  @state() private page = 0;

  renderSearchResults() {
    const perPage = 40;
    const totalPages = Math.ceil((this._searchResults?.length ?? 0) / perPage);
    let pageKeys = Array.from({ length: totalPages }, (_, index) => index);
    const extra: {
      entry: { title: string; name: string };
      haystackEntry: string;
      ranges: number[];
    }[] = [];
    if (this.page < totalPages - 1) {
      extra.push({
        entry: { title: "", name: this.filter },
        haystackEntry: "...",
        ranges: [],
      });
    }
    return when(
      typeof this._searchResults !== "undefined" &&
        this._searchResults.length < 20,
      () =>
        repeat(
          this._searchResults ?? [],
          (result) => result.entry.name,
          (result) =>
            cache(html`
              <vm-device-template
                prefab_name=${result.entry.name}
                class="card"
                @add-device-template=${this._handleDeviceAdd}
              >
              </vm-device-template>
            `),
        ),
      () => html`
        <div class="p-2">
          <div class="flex flex-row">
            <p class="p-2">
              <sl-format-number
                .value=${this._searchResults?.length}
              ></sl-format-number>
              results, filter more to get cards
            </p>
            <div class="p-2 ml-2">
              Page:
              ${pageKeys.map(
                (key, index) => html`
                  <span
                    class="p-2 cursor-pointer hover:text-purple-400 ${index ===
                    this.page
                      ? " text-purple-500"
                      : ""}"
                    key=${key}
                    @click=${this._handlePageChange}
                    >${key + 1}${index < totalPages - 1 ? "," : ""}</span
                  >
                `,
              )}
            </div>
          </div>
          <div class="flex flex-row flex-wrap">
            ${[
              ...this._searchResults.slice(
                perPage * this.page,
                perPage * this.page + perPage,
              ),
              ...extra,
            ].map((result) => {
              let hay = result.haystackEntry.slice(0, 15);
              if (result.haystackEntry.length > 15) hay += "...";
              const ranges = result.ranges.filter((pos) => pos < 20);
              const key = result.entry.name;
              return html`
                <div
                  class="m-2 text-neutral-200/90 italic cursor-pointer rounded bg-neutral-700 hover:bg-purple-500 px-1"
                  key=${key}
                  @click=${this._handleHaystackClick}
                >
                  ${result.entry.title} (<small class="text-sm">
                    ${ranges.length
                      ? unsafeHTML(uFuzzy.highlight(hay, ranges))
                      : hay} </small
                  >)
                </div>
              `;
            })}
          </div>
        </div>
      `,
    );
  }

  _handlePageChange(e: Event) {
    const span = e.currentTarget as HTMLSpanElement;
    const key = parseInt(span.getAttribute("key"));
    this.page = key;
  }

  _handleHaystackClick(e: Event) {
    const div = e.currentTarget as HTMLDivElement;
    const key = div.getAttribute("key");
    if (key === this.filter) {
      this.page += 1;
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
