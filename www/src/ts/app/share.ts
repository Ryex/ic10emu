import { HTMLTemplateResult, html, css } from "lit";
import { customElement, property, query } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";

import "@shoelace-style/shoelace/dist/components/dialog/dialog.js";
import "@shoelace-style/shoelace/dist/components/input/input.js";
import "@shoelace-style/shoelace/dist/components/icon/icon.js";
import "@shoelace-style/shoelace/dist/components/icon-button/icon-button.js";
import "@shoelace-style/shoelace/dist/components/copy-button/copy-button.js";
import SlDialog from "@shoelace-style/shoelace/dist/components/dialog/dialog.js";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.js";

@customElement("session-share-dialog")
export class ShareSessionDialog extends BaseElement {
  @query(".dialog")  dialog: SlDialog;
  @query(".input")  input: SlInput;
  @property({ type: String })  link: string;

  constructor() {
    super();
  }

  protected render() {
    return html`
      <sl-dialog label="Share This Code" class="dialog" style="--header-spacing: 0.75em;">
        <sl-input class="input" id="session-link-input" value="${this.link}">
          <sl-icon name="link-45deg" slot="prefix"></sl-icon>
          <sl-copy-button from="session-link-input.value" slot="suffix">
            <sl-icon slot="copy-icon" name="clipboard"></sl-icon>
            <sl-icon slot="success-icon" name="clipboard-check"></sl-icon>
            <sl-icon slot="error-icon" name="clipboard-x"></sl-icon>
          </sl-copy-button>
        </sl-input>
      </sl-dialog>
    `;
  }

  show() {
    this.dialog.show();
  }

  hide() {
    this.dialog.hide();
  }

  _handleCopyClick() {
    this.input.value;
  }
}
