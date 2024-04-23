import { HTMLTemplateResult, html, css } from "lit";
import { customElement, property } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";

import "@shoelace-style/shoelace/dist/components/icon/icon.js";
import "@shoelace-style/shoelace/dist/components/icon-button/icon-button.js";
import "@shoelace-style/shoelace/dist/components/menu/menu.js";
import "@shoelace-style/shoelace/dist/components/divider/divider.js";
import "@shoelace-style/shoelace/dist/components/menu-item/menu-item.js";
import "@shoelace-style/shoelace/dist/components/dropdown/dropdown.js";
import "@shoelace-style/shoelace/dist/components/relative-time/relative-time.js";
import "@shoelace-style/shoelace/dist/components/tooltip/tooltip.js";
import SlMenuItem from "@shoelace-style/shoelace/dist/components/menu-item/menu-item.js";

@customElement("app-nav")
export class Nav extends BaseElement {
  static styles = [
    ...defaultCss,
    css`
      .nav {
        display: flex;
        flex-wrap: wrap;
        padding-left: 0;
        margin-bottom: 0;
        list-style: none;
      }
      .navbar {
        position: relative;
        display: flex;
        flex-wrap: wrap;
        align-items: left;
        justify-content: space-between;
        padding: 0.5rem 0;
      }
      .navbar-nav {
        display: flex;
        flex-direction: column;
        padding-left: 0;
        margin-bottom: 0;
        list-style: none;
      }
      @media screen and (min-width: 768px) {
        .navbar-right {
          display: flex !important;
          flex-direction: row !important;
          margin-left: 0.5rem !important;
        }
      }
      .navbar-right {
        display: none;
      }
      ol,
      ul,
      dl {
        margin-top: 0;
        margin-bottom: 1rem;
      }
      .navbar-text {
        padding: 0;
        padding-right: 10px;
        position: relative;
        color: #fff;
      }
      .navbar-header .version {
        color: var(--sl-color-neutral-500);
        font-size: var(--sl-font-size-small);
      }
      .nav > li > a {
        color: #fff;
        line-height: 20px;
        position: relative;
        display: block;
        padding: 10px 15px;
        padding-top: 10px;
        padding-bottom: 10px;
      }
      .navbar-brand {
        padding-top: 0.3125rem;
        padding-bottom: 0.3125rem;
        margin-right: 1rem;
        font-size: 1.25rem;
        color: #fff;
        text-decoration: none;
        white-space: nowrap;
      }
      .dropdown {
        position: relative;
        top: 50%;
        transform: translateY(-50%);
        z-index: 100;
      }
      nav {
        border-bottom: 1px solid rgb(108, 117, 125);
      }
    `,
  ];

  constructor() {
    super();
  }

  @property() gitVer: string;
  @property() appVer: string;
  @property() buildDate: string;
  protected render(): HTMLTemplateResult {
    return html`
      <nav id="navBar" class="navbar navbar-default">
        <div class="nav navbar-nav ps-2">
          <sl-dropdown class="dropdown">
            <sl-icon-button
              library="fa"
              name="fas-bars"
              slot="trigger"
              label="Main Menu"
            ></sl-icon-button>

            <sl-menu
              class="menu"
              @sl-select=${this._menuClickHandler}
              style="z-index: 10"
            >
              <sl-menu-item value="share">
                Share
                <sl-icon name="share" slot="prefix"></sl-icon>
              </sl-menu-item>
              <sl-menu-item value="openFile">
                Open File
                <sl-icon name="folder2-open" slot="prefix"></sl-icon>
              </sl-menu-item>
              <sl-menu-item value="save">
                Save
                <sl-icon name="box-arrow-in-down" slot="prefix"></sl-icon>
              </sl-menu-item>
              <sl-menu-item value="load">
                Load
                <sl-icon name="box-arrow-up" slot="prefix"></sl-icon>
              </sl-menu-item>
              <sl-menu-item value="export">
                Export current file
                <sl-icon name="file-earmark-arrow-up" slot="prefix"></sl-icon>
              </sl-menu-item>
              <sl-divider></sl-divider>
              <sl-menu-item value="editorSettings">
                Editor Settings
                <sl-icon name="sliders2" slot="prefix"></sl-icon>
              </sl-menu-item>
              <sl-divider></sl-divider>
              <sl-menu-item value="keyboardShortcuts">
                Editor Keyboard Shortcuts
                <sl-icon name="command" slot="prefix"></sl-icon>
              </sl-menu-item>
              <sl-divider></sl-divider>
              <sl-menu-item>
                Presets
                <sl-icon name="code-square" slot="prefix"></sl-icon>
                <sl-menu slot="submenu">
                  <sl-menu-item value="preset-demo"> Demo </sl-menu-item>
                </sl-menu>
              </sl-menu-item>
              <sl-divider></sl-divider>
              <sl-menu-item value="changelog">
                Changelog
                <sl-icon name="journal-text" slot="prefix"></sl-icon>
              </sl-menu-item>
            </sl-menu>
          </sl-dropdown>
        </div>

        <div class="nav navbar-header ms-2 hstack">
          <div>
            <a class="navbar-brand" aria-current="page" href="">
              Stationeers IC10 Emulator
            </a>
          </div>
          <div class="hstack version mt-auto mb-auto">
            <small>v${this.appVer}-${this.gitVer}</small>
            <small class="ms-2">
              <sl-relative-time date=${this.buildDate}></sl-relative-time>
            </small>
          </div>
        </div>

        <div class="nav navbar-nav  ms-auto d-flex flex-row">
          <a
            class="navbar-text mt-auto mb-auto align-self-center"
            href="https://github.com/ryex/ic10emu"
            >View on Github
            <sl-icon library="fa" name="fab-github"></sl-icon>
          </a>
        </div>

        <!-- <div class="flex-grow w-100">&nbsp;</div> -->
        <ul class="nav navbar-nav navbar-right">
          <p class="navbar-text mt-auto mb-auto align-self-center">
            Official Stationeers:
          </p>
          <li role="presentation" class="">
            <a href="https://store.steampowered.com/app/544550/Stationeers/">
              <i class="fa-brands fa-steam fa-w-16"></i>
              <sl-icon library="fa" name="fab-steam"></sl-icon>
            </a>
          </li>
          <li role="presentation" class="">
            <a href="https://stationeers.com/">
              <sl-icon library="fa" name="fas-globe"></sl-icon>
            </a>
          </li>
          <li role="presentation" class="">
            <a href="https://twitter.com/stationeers">
              <sl-icon library="fa" name="fab-x-twitter"></sl-icon>
            </a>
          </li>
          <li role="presentation" class="">
            <a href="https://discordapp.com/invite/CxR3mRy">
              <i class="fa-brands fa-discord"></i>
              <sl-icon library="fa" name="fab-discord"></sl-icon>
            </a>
          </li>
        </ul>
      </nav>
    `;
  }

  firstUpdated(): void {}

  _menuClickHandler(e: CustomEvent) {
    const item = e.detail.item as SlMenuItem;
    switch (item.value) {
      case "share":
        this.dispatchEvent(
          new CustomEvent("app-share-session", { bubbles: true }),
        );
        break;
      case "openFile":
        this.dispatchEvent(new CustomEvent("app-open-file", { bubbles: true }));
        break;
      case "save":
        this.dispatchEvent(new CustomEvent("app-save", { bubbles: true }));
        break;
      case "load":
        this.dispatchEvent(new CustomEvent("app-load", { bubbles: true }));
        break;
      case "export":
        this.dispatchEvent(new CustomEvent("app-export", { bubbles: true }));
        break;
      case "editorSettings":
        window.Editor.settingDialog.show();
        break;
      case "keyboardShortcuts":
        window.Editor.kbShortcuts.show();
        break;
      case "preset-demo":
        window.location.hash = "demo";
        break;
      case "changelog":
        this.dispatchEvent(new CustomEvent("app-changelog", { bubbles: true }));
        break;
      default:
        console.log("Unknown main menu item", item.value);
    }
  }
}
