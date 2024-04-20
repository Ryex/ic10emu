import { BaseElement, defaultCss } from "../components";
import { html, css, PropertyValueMap } from "lit";
import { customElement, property, query } from "lit/decorators.js";
import { ace, Ace } from "./ace";

import "@shoelace-style/shoelace/dist/components/drawer/drawer.js";

import { SlDrawer } from "@shoelace-style/shoelace";
import { Ref, createRef, ref } from "lit/directives/ref.js";

@customElement("ace-kb-menu")
export class AceKeyboardShortcuts extends BaseElement {
  static styles = [
    defaultCss,
    css`
      .command {
        color: #c678dd;
        font-weight: normal;
      }
      .entry:hover {
        background-color: var(--sl-color-neutral-100);
        transition: all 0.3s;
      }
      .key {
        color: var(--sl-color-neutral-1000);
        font-weight: bold;
      }
    `,
  ];

  editor?: Ace.Editor;
  @query(".drawer") drawer: SlDrawer;

  constructor() {
    super();
  }

  // protected shouldUpdate(_changedProperties: PropertyValueMap<any> | Map<PropertyKey, unknown>): boolean {
  //   return true;
  // }

  protected async firstUpdated() {
    if (!ace.require("ace/ext/menu_tools/get_editor_keyboard_shortcuts")) {
      await import("ace-builds/src-noconflict/ext-keybinding_menu");
    }
  }

  protected render() {
    var kbs: any[] = [];
    if (this.editor) {
      const getEditorKeybordShortcuts = ace.require(
        "ace/ext/menu_tools/get_editor_keyboard_shortcuts",
      ).getEditorKeybordShortcuts;
      kbs = getEditorKeybordShortcuts(this.editor);
    }
    return html`
      <sl-drawer label="Editor Keyboard Shortcuts" class="drawer">
        <div>
          ${kbs.map(
            (kb: any) =>
              html`<div class="entry">
                <span class="command">${kb.command}</span> :
                <span class="key">${kb.key}</span>
              </div>`,
          )}
        </div>
      </sl-drawer>
    `;
  }

  show() {
    this.drawer.show();
  }

  hide() {
    this.drawer.hide();
  }
}
