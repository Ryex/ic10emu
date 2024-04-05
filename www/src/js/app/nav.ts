import { HTMLTemplateResult, html, css } from "lit";
import { customElement, property } from "lit/decorators.js";
import { BaseElement } from "../components";
import "@shoelace-style/shoelace/dist/components/icon-button/icon-button.js";
import "@shoelace-style/shoelace/dist/components/menu/menu.js";
import "@shoelace-style/shoelace/dist/components/divider/divider.js";
import "@shoelace-style/shoelace/dist/components/menu-item/menu-item.js";
import "@shoelace-style/shoelace/dist/components/dropdown/dropdown.js";

@customElement("app-nav")
export class Nav extends BaseElement {
  static styles = css`
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
    @media (min-width: 768px) .d-md-flex {
      display: flex !important;
    }
    @media (min-width: 576px) .d-sm-none {
      display: none !important;
    }
    .d-none {
      display: none !important;
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
  `;

  constructor() {
    super();
  }
  protected render(): HTMLTemplateResult {
    return html`
      <nav id="navBar" class="navbar navbar-default">
        <div class="nav navbar-nav ps-2">
          <sl-dropdown>
            <sl-icon-button
              library="fa"
              name="fas-bars"
              slot="trigger"
              label="Main Menu"
            ></sl-icon-button>

            <sl-menu>
              <sl-menu-item value="share">Share</sl-menu-item>
              <sl-menu-item value="openFile">Open File</sl-menu-item>
              <sl-menu-item value="saveAs">Save As</sl-menu-item>
              <sl-devider></sl-devider>
              <sl-menu-item value="editorSettings"
                >Editor Settings</sl-menu-item
              >
              <sl-devider></sl-devider>
              <sl-menu-item value="keyboardShortcuts"
                >Editor Keyboard Shortcuts</sl-menu-item
              >
            </sl-menu>
          </sl-dropdown>
        </div>

        <div class="nav navbar-nav navbar-header ms-2">
          <a class="navbar-brand" aria-current="page" href=""
            >Stationeers IC10 Emulator</a
          >
        </div>

        <div class="nav navbar-nav  ms-auto navbar-right d-flex flex-row">
          <a
            class="navbar-text mt-auto mb-auto align-self-center"
            href="https://github.com/ryex/ic10emu"
            >View on Github <i class="fa-brands fa-github"></i
          ></a>
        </div>

        <!-- <div class="flex-grow w-100">&nbsp;</div> -->
        <ul
          class="nav navbar-nav navbar-right flex-row d-sm-none d-none d-md-flex"
        >
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
}
