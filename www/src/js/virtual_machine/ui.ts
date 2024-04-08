import { HTMLTemplateResult, html, css } from "lit";
import { customElement, property, query } from "lit/decorators.js";
import { BaseElement, defaultCss } from "../components";

import "./controls.ts";

@customElement("vm-ui")
export class VMUI extends BaseElement {

  constructor() {
    super();
  }

  protected render() {
    return html`<vm-ic-controls>`;
  }
}
