import { LitElement, css } from "lit";
import shoelaceDark from "@shoelace-style/shoelace/dist/themes/dark.styles.js";

export class BaseElement extends LitElement {
  // Some default styles
  static styles = shoelaceDark;
}
