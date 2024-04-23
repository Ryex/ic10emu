import { html, css } from "lit";
import { unsafeHTML } from 'lit/directives/unsafe-html.js';
import { customElement, property, query } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";
import { SlDialog, SlSwitch } from "@shoelace-style/shoelace";
import { until } from "lit/directives/until.js";

import "@shoelace-style/shoelace/dist/components/spinner/spinner.js";
import '@shoelace-style/shoelace/dist/components/switch/switch.js';

import { marked } from "marked";
import { gfmStyles } from "./gfm-styles";

@customElement("app-welcome")
export class AppWelcome extends BaseElement {
  static styles = [
    ...defaultCss,
    gfmStyles,
    css`
      .welcome-dialog {
        --width: 42rem;
      }
    `,
  ];

  @property({ type: Boolean }) dontShowAgain: boolean;

  constructor() {
    super();
    this.dontShowAgain = true;
  }

  @query("sl-dialog.welcome-dialog") dialog: SlDialog;
  @query("sl-switch.dont-show-switch") dontShowSwitch: SlSwitch;

  hide() {
    this.dialog?.hide();
  }

  show() {
    this.dialog?.show();
  }

  async getChangelog() {
    const response = await fetch("static/CHANGELOG.md");
    const blob = await response.blob();
    const markdown = await blob.text();
    const renderedText = await marked(markdown, {
      async: true,
      gfm: true,
    });
    return unsafeHTML(renderedText);
  }

  render() {
    return html`
      <sl-dialog class="welcome-dialog" label="Changelog">
        <h6>Hey there!</h6>
        <p>Looks like there have been some updates since you've last visit.</p>
        <br />
        <p>Check out the changelog below.</p>
        <div class="p-4 border-1 border-solid rounded-lg max-h-80 mt-4 overflow-y-auto bg-neutral-900 markdown-body">
          ${until(this.getChangelog(), html`<sl-spinner class="ml-2 my-4" style="font-size: 2rem;"></sl-spinner>`)}
        </div>
        <div slot="footer">
          <sl-switch class="dont-show-switch" size="small" ?checked=${this.dontShowAgain} @sl-change=${this._dontShowSwitchChange} >Don't show again</sl-switch>
        </div>
      </sl-dialog>
    `;
  }

  _dontShowSwitchChange(e: CustomEvent) {
    this.dontShowAgain = this.dontShowSwitch.checked;
  }
}
