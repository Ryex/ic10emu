import "@popperjs/core";
import "../scss/styles.scss";
import { setBasePath } from "@shoelace-style/shoelace/dist/utilities/base-path.js";
setBasePath("shoelace");
import "./icons";

import "@shoelace-style/shoelace/dist/components/split-panel/split-panel.js";
import "@shoelace-style/shoelace/dist/components/dialog/dialog.js";
import "@shoelace-style/shoelace/dist/components/drawer/drawer.js";
import "@shoelace-style/shoelace/dist/components/icon/icon.js";
import "@shoelace-style/shoelace/dist/components/icon-button/icon-button.js";
import "@shoelace-style/shoelace/dist/components/copy-button/copy-button.js";
import "@shoelace-style/shoelace/dist/components/button-group/button-group.js";
import "@shoelace-style/shoelace/dist/components/button/button.js";
import '@shoelace-style/shoelace/dist/components/switch/switch.js';
import "@shoelace-style/shoelace/dist/components/radio-button/radio-button.js";
import "@shoelace-style/shoelace/dist/components/radio-group/radio-group.js";
import "@shoelace-style/shoelace/dist/components/menu/menu.js";
import "@shoelace-style/shoelace/dist/components/menu-item/menu-item.js";
import "@shoelace-style/shoelace/dist/components/divider/divider.js";
import "@shoelace-style/shoelace/dist/components/dropdown/dropdown.js";
import "@shoelace-style/shoelace/dist/components/tooltip/tooltip.js";
import "@shoelace-style/shoelace/dist/components/input/input.js";
import "@shoelace-style/shoelace/dist/components/spinner/spinner.js";
import "@shoelace-style/shoelace/dist/components/card/card.js";
import "@shoelace-style/shoelace/dist/components/details/details.js";
import "@shoelace-style/shoelace/dist/components/tab/tab.js";
import "@shoelace-style/shoelace/dist/components/tab-panel/tab-panel.js";
import "@shoelace-style/shoelace/dist/components/tab-group/tab-group.js";
import "@shoelace-style/shoelace/dist/components/select/select.js";
import "@shoelace-style/shoelace/dist/components/badge/badge.js";
import "@shoelace-style/shoelace/dist/components/option/option.js";
import "@shoelace-style/shoelace/dist/components/alert/alert.js";
import "@shoelace-style/shoelace/dist/components/format-number/format-number.js";
import "@shoelace-style/shoelace/dist/components/format-date/format-date.js";
import "@shoelace-style/shoelace/dist/components/format-bytes/format-bytes.js";
import "@shoelace-style/shoelace/dist/components/relative-time/relative-time.js";

import "ace-builds";
import "ace-builds/esm-resolver";

class DeferedApp {

  app: App;
  private resolvers: ((value: App) => void)[];

  constructor() {
    this.app = undefined;
    this.resolvers = [];
  }

  get(): Promise<App> {
    const that = this;
    return new Promise(resolve => {
      if (typeof that.app === "undefined") {
        that.resolvers.push(resolve);
      } else {
        resolve(that.app);
      }
    })
  }

  set(app: App) {
    this.app = app;
    while(this.resolvers.length) {
      this.resolvers.shift()(this.app);
    }
  }

}

class DeferedVM {

  vm: VirtualMachine;
  private resolvers: ((value: VirtualMachine) => void)[];

  constructor() {
    this.vm = undefined;
    this.resolvers = [];
  }

  get(): Promise<VirtualMachine> {
    const that = this;
    return new Promise(resolve => {
      if (typeof that.vm === "undefined") {
        that.resolvers.push(resolve);
      } else {
        resolve(that.vm);
      }
    })
  }

  set(vm: VirtualMachine) {
    this.vm = vm;
    while(this.resolvers.length) {
      this.resolvers.shift()(this.vm);
    }
  }

}

declare global {
  interface Window
 {
    App: DeferedApp;
    VM: DeferedVM;
  }
}

window.App = new DeferedApp();
window.VM = new DeferedVM();

import type { App } from "./app";
import type { VirtualMachine } from "./virtualMachine";

import("./app");
