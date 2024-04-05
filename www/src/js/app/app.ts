import { HTMLTemplateResult, html, css } from "lit";
import { customElement, property } from "lit/decorators.js";
import { BaseElement } from "../components";
import "./nav.ts";

@customElement('ic10emu-app')
export class App extends BaseElement {
  constructor() {
    super();
  }

  protected render(): HTMLTemplateResult {
    return html`<app-nav></app-nav>`;
  }
}
