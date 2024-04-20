import { HTMLTemplateResult, html, css, CSSResultGroup } from "lit";
import { customElement, property, query, state } from "lit/decorators.js";
import { BaseElement, defaultCss } from "../components";
import { VMState } from "../session";

import "@shoelace-style/shoelace/dist/components/dialog/dialog.js";
import "@shoelace-style/shoelace/dist/components/format-date/format-date.js";
import "@shoelace-style/shoelace/dist/components/relative-time/relative-time.js";
import "@shoelace-style/shoelace/dist/components/format-bytes/format-bytes.js";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.js";
import { repeat } from "lit/directives/repeat.js";
import SlDialog from "@shoelace-style/shoelace/dist/components/dialog/dialog.js";
import { when } from "lit/directives/when.js";

export type SaveDialogMode = "save" | "load";

@customElement("save-dialog")
export class SaveDialog extends BaseElement {
  static styles = [...defaultCss];

  @state() saves: { name: string; date: Date; session: VMState }[];
  @state() mode: SaveDialogMode;

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

  @query("sl-dialog") dialog: SlDialog;

  show(mode: SaveDialogMode) {
    this.mode = mode;
    this.dialog.show();
  }

  hide() {
    this.dialog.hide();
  }

  render() {
    return html`
      <sl-dialog label="Save Session">
        ${when(
          this.mode === "save",
          () => html`
            <div class="hstack mb-2">
              <sl-input class="save-name-input" autofocus></sl-input>
              <sl-button
                class="ms-2"
                variant="success"
                @click=${this._handleSaveClick}
                >Save</sl-button
              >
            </div>
          `,
        )}
        <sl-input
          class="filter-input"
          ?autofocus=${this.mode === "load"}
          placeholder="Filter Saves"
          clearable
          @sl-input=${this._handleSearchInput}
        >
          <sl-icon slot="suffix" name="search"></sl-icon>
        </sl-input>
        <table>
          <tr>
            <th>Name</th>
            <th>Date</th>
            <th>Size</th>
          </tr>
          ${when(typeof this.saves !== "undefined", () =>
            repeat(
              this.saves,
              (save) => save.name,
              (save) => {
                const size = JSON.stringify(save.session).length;
                return html`
                  <tr>
                    <td>${save.name}</td>
                    <td>
                      <sl-format-date .date=${save.date}></sl-format-date>
                      <sl-relative-time .date=${save.date}></sl-relative-time>
                    </td>
                    <td>
                      <sl-format-bytes .value=${size}></sl-format-bytes>
                    </td>
                  </tr>
                `;
              },
            ),
          )}
        </table>
      </sl-dialog>
    `;
  }

  @query(".save-name-input") saveInput: SlInput;

  async _handleSaveClick(_e: CustomEvent) {
    const name = this.saveInput.value;
    const app = await window.App.get();
    console.log(app);
    await app.session.saveLocal(name);
  }

  _handleSearchInput() {}
}
