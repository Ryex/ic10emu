import { HTMLTemplateResult, html, css, CSSResultGroup } from "lit";
import { customElement, property, query, state } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { VMState } from "session";

import "@shoelace-style/shoelace/dist/components/dialog/dialog.js";
import "@shoelace-style/shoelace/dist/components/format-date/format-date.js";
import "@shoelace-style/shoelace/dist/components/relative-time/relative-time.js";
import "@shoelace-style/shoelace/dist/components/format-bytes/format-bytes.js";
import "@shoelace-style/shoelace/dist/components/spinner/spinner.js";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.js";
import { repeat } from "lit/directives/repeat.js";
import SlDialog from "@shoelace-style/shoelace/dist/components/dialog/dialog.js";
import { when } from "lit/directives/when.js";
import uFuzzy from "@leeoniya/ufuzzy";
import SlButton from "@shoelace-style/shoelace/dist/components/button/button.js";
import { SlIconButton } from "@shoelace-style/shoelace";

export type SaveDialogMode = "save" | "load";

@customElement("save-dialog")
export class SaveDialog extends BaseElement {
  static styles = [
    ...defaultCss,
    css`
      .save-dialog {
        --width: 42rem;
      }
      sl-icon-button.delete-button::part(base) {
        color: var(--sl-color-danger-600);
      }
      sl-icon-button.delete-button::part(base):hover,
      sl-icon-button.delete-button::part(base):focus {
        color: var(--sl-color-danger-500);
      }
      sl-icon-button.delete-button::part(base):active {
        color: var(--sl-color-danger-600);
      }
    `,
  ];

  private _saves: { name: string; date: Date; session: VMState }[];

  get saves() {
    return this._saves;
  }

  @state()
  set saves(val: { name: string; date: Date; session: VMState }[]) {
    this._saves = val;
    this.performSearch();
  }

  @state() mode: SaveDialogMode;

  private searchResults: { name: string; date: Date; session: VMState }[];

  constructor() {
    super();
    this.mode = "save";
  }

  connectedCallback(): void {
    super.connectedCallback();
    window.App.get().then((app) =>
      app.session.addEventListener(
        "sessions-local-update",
        this._handleSessionsUpdate.bind(this),
      ),
    );
    this.loadsaves();
  }

  _handleSessionsUpdate() {
    this.loadsaves();
  }

  loadsaves() {
    window.App.get().then(async (app) => {
      const saves = await app.session.getLocalSaved();
      this.saves = saves;
    });
  }

  @query("sl-dialog.save-dialog") saveDialog: SlDialog;

  show(mode: SaveDialogMode) {
    this.mode = mode;
    this.saveDialog.show();
  }

  hide() {
    this.saveDialog.hide();
  }

  private _filter: string = "";

  get filter() {
    return this._filter;
  }

  @state()
  set filter(val: string) {
    this._filter = val;
    this.performSearch();
  }

  performSearch() {
    if (this.filter) {
      const source = this.saves ?? [];
      const haystack: string[] = source.map((save) => save.name);
      const uf = new uFuzzy({});
      const [_idxs, info, order] = uf.search(haystack, this._filter, 0, 1e3);
      const filtered = order?.map((infoIdx) => source[info.idx[infoIdx]]);
      this.searchResults = filtered ?? [];
    } else {
      this.searchResults = this.saves;
    }
  }

  render() {
    const label = this.mode === "save" ? "Save Session" : "Load session";
    return html`
      <sl-dialog label=${label} class="save-dialog" @sl-hide=${this._saveDialogHide}>
        ${when(
          this.mode === "save",
          () => html`
            <div class="flex flex-row mb-4">
              <sl-input
                class="save-name-input grow"
                autofocus
                pill
                @sl-input=${this._saveInputChange}
              ></sl-input>
              <sl-button
                class="ms-2 save-button"
                variant="success"
                pill
                @click=${this._handleSaveButtonClick}
                >Save</sl-button
              >
            </div>
          `,
        )}
        ${when(
          typeof this.saves !== "undefined",
          () => html`
            ${when(
              this.saves.length === 0,
              () => html`<strong>No local saves found</strong>`,
              () => html`
                <div
                  class="p-2 border-1 border-solid border-neutral-700 rounded-lg"
                >
                  <sl-input
                    class="filter-input mb-2"
                    ?autofocus=${this.mode === "load"}
                    placeholder="Filter Saves"
                    clearable
                    size="small"
                    @sl-input=${this._handleSearchInput}
                  >
                    <sl-icon slot="suffix" name="search"></sl-icon>
                  </sl-input>
                  <div class="overflow-auto max-h-40">
                    ${repeat(
                      this.searchResults,
                      (save) => save.name,
                      (save, index) => {
                        const size = JSON.stringify(save.session).length;
                        let classList =
                          index !== 0
                            ? "flex flex-row space-x-2 justify-between justify-self-start border-t border-neutral-400/10"
                            : "flex flex-row space-x-2 justify-between justify-self-start";
                        classList += " hover:bg-neutral-700 cursor-pointer";
                        return html`
                          <div
                            class=${classList}
                            value=${save.name}
                            @click=${this._handleLocalSaveClick}
                          >
                            <div class="py-2 ms-2 text-nowrap">
                              ${save.name}
                            </div>
                            <div class="py-2 ms-auto text-nowrap">
                              <sl-relative-time
                                .date=${save.date}
                              ></sl-relative-time>
                              <sl-format-date
                                .date=${save.date}
                              ></sl-format-date>
                            </div>
                            <div class="py-2 me-2 text-nowrap">
                              <sl-format-bytes .value=${size}></sl-format-bytes>
                            </div>
                            <div class="py-2 mx-3">
                              <sl-tooltip content="Delete this save">
                                <sl-icon-button
                                  name="trash"
                                  label="Delete"
                                  key=${save.name}
                                  @click=${this._handleDeleteSave}
                                  class="delete-button"
                                ></sl-icon-button>
                              </sl-tooltip>
                           </div>
                          </div>
                        `;
                      },
                    )}
                  </div>
                </div>
              `,
            )}
          `,
          () => html` <sl-spinner></sl-spinner> `,
        )}
      </sl-dialog>
      <sl-dialog class="delete-dialog" no-header @sl-request-close=${this._preventOverlayClose} @sl-hide=${this._deleteDialogHide}>
          <div class="w-full">
            <p><strong>Are you sure you want to remove this save?</strong></p>
          </div>
        <div slot="footer">
          <sl-button variant="primary" autofocus @click=${this._closeDeleteDialog}>Close</sl-button>
          <sl-button variant="danger" @click=${this._deleteDialogDelete}>Delete</sl-button>
        </div>
      </sl-dialog>
    `;
  }

  @query("sl-dialog.delete-dialog") deleteDialog: SlDialog;
  private _toDelete: string | undefined;

  _preventOverlayClose(event: CustomEvent) {
    if (event.detail.source === "overlay") {
      event.preventDefault();
    }
    this._toDelete = undefined;
  }

  _closeDeleteDialog() {
    this.deleteDialog.hide();
  }

  _deleteDialogHide(e: CustomEvent) {
    if (e.target !== e.currentTarget) return;
    this._toDelete = undefined;
  }

  _saveDialogHide(e: CustomEvent) {
    if (e.target !== e.currentTarget) return;
    if (this.filterInput != null) this.filterInput.value = "";
    if (this.saveInput != null) this.saveInput.value = "";
    this._filter = undefined;
  }

  @query(".save-name-input") saveInput: SlInput;
  @query(".filter-input") filterInput: SlInput;
  @query(".save-button") saveButton: SlButton;

  async _handleSaveButtonClick(_e: CustomEvent) {
    const name = this.saveInput.value;
    const app = await window.App.get();
    console.log(app);
    await app.session.saveLocal(name);
    this.saveDialog.hide();
  }

  _handleLocalSaveClick(e: Event) {
    const saveName = (e.currentTarget as HTMLDivElement).getAttribute("value");
    if (this.mode === "save") {
      this.saveInput.value = saveName;
      this.checkSaveName();
    } else {
      window.App.get().then((app) => app.session.loadFromLocal(saveName));
      this.saveDialog.hide();
    }
  }

  checkSaveName() {
    const saveName = this.saveInput.value;
    const saves = this.saves ?? [];
    let found = false;
    for (const save of saves) {
      if (save.name === saveName) {
        found = true;
        break;
      }
    }
    if (found) {
      this.saveButton.variant = "danger";
      this.saveButton.textContent = "Overwrite";
    } else {
      this.saveButton.variant = "success";
      this.saveButton.textContent = "Save";
    }
  }

  _saveInputChange(e: CustomEvent) {
    this.checkSaveName();
  }

  private _searchTimeout: number | undefined;
  _handleSearchInput() {
    if (this._searchTimeout) clearTimeout(this._searchTimeout);
    const that = this;
    this._searchTimeout = setTimeout(() => {
      that.filter = that.filterInput.value;
      that._searchTimeout = undefined;
    }, 200);
  }

  _handleDeleteSave(e: Event) {
    const saveName = (e.target as SlIconButton).getAttribute("key");
    this._toDelete = saveName;
    this.deleteDialog.show();
  }

  _deleteDialogDelete() {
    if (typeof this._toDelete === "string") {
      const toDelete = this._toDelete;
      window.App.get().then(app => app.session.deleteLocalSave(toDelete))
    }
    this.deleteDialog.hide();
    this._toDelete = undefined;
    this.saveDialog.show();
  }
}
