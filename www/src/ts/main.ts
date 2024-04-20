import "@popperjs/core";
import "../scss/styles.scss";
import { Dropdown, Modal } from "bootstrap";

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
      if (typeof that.app !== "undefined") {
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
      if (typeof that.vm !== "undefined") {
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
import type { VirtualMachine } from "./virtual_machine";

import("./app");
