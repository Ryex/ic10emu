import { CSSResultGroup, LitElement, css, unsafeCSS } from "lit";
import shoelaceDark from "@shoelace-style/shoelace/dist/themes/dark.styles.js";

export const defaultCss = [
  shoelaceDark,
  css`
    .ps-2 {
      padding-left: 0.5rem !important;
    }
    .ms-2 {
      margin-left: 0.5rem !important;
    }
    .ms-auto {
      margin-left: auto !important;
    }
    .flex-row {
      flex-direction: row !important;
    }
    .d-flex {
      display: flex !important;
    }
    .align-self-center {
      align-self: center !important;
    }
    .mb-auto {
      margin-bottom: auto !important;
    }
    .mt-auto {
      margin-top: auto !important;
    }
    .hstack {
      display: flex;
      flex-direction: row;
    }
    .vstack {
      display: flex;
      flex-direction: column;
    }
  `,
];

export class BaseElement extends LitElement {
  // Some default styles
  static styles: CSSResultGroup = defaultCss;
}
