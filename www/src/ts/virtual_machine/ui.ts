import { HTMLTemplateResult, html, css } from "lit";
import { customElement, property, query } from "lit/decorators.js";
import { BaseElement, defaultCss } from "../components";
import "@shoelace-style/shoelace/dist/components/details/details.js";

import "./controls";
import "./registers";
import "./stack";

@customElement("vm-ui")
export class VMUI extends BaseElement {
  static styles = [
    ...defaultCss,
    css`
      sl-details {
        margin-left: 1rem;
        margin-right: 1rem;
      }
      sl-details::part(header) {
        padding: 0.3rem;
      }
      sl-details::part(content) {
        padding: 0.5rem;
      }
    `,
  ];

  constructor() {
    super();
  }

  protected render() {
    return html`
      <vm-ic-controls></vm-ic-controls>
      <sl-details summary="Registers">
        <vm-ic-registers></vm-ic-registers>
      </sl-details>
      <sl-details summary="Stack">
        <vm-ic-stack></vm-ic-stack>
      </sl-details>
    `;
  }
}
