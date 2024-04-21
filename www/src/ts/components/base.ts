import { CSSResultGroup, LitElement, css } from "lit";
import shoelaceDark from "@shoelace-style/shoelace/dist/themes/dark.styles.js";

const { cssRules } = Array.from(document.styleSheets).find(
  (sheet) => sheet.href && /index\.([\w\d]+\.)?css/.test(sheet.href),
);
const globalStyle = css([
  Object.values(cssRules)
    .map((rule) => rule.cssText)
    .join("\n"),
] as any);

export const defaultCss = [
  globalStyle,
  shoelaceDark,
  css`
    .d-flex {
      display: flex !important;
    }
    .align-self-center {
      align-self: center !important;
    }
    .hstack {
      display: flex;
      flex-direction: row;
    }
    .vstack {
      display: flex;
      flex-direction: column;
    }
    .flex-g {
      flex-grow: 1;
    }
  `,
];

export class BaseElement extends LitElement {
  // Some default styles
  static styles: CSSResultGroup = defaultCss;
}
