import type {
  ObjectID,
  TemplateDatabase,
} from "ic10emu_wasm";
import { LitElement } from "lit";

import {
  signal,
} from '@lit-labs/preact-signals';
import type { Signal } from '@lit-labs/preact-signals';
import { VirtualMachine } from "virtualMachine";
import { property } from "lit/decorators.js";

type Constructor<T = {}> = new (...args: any[]) => T;

export declare class VMObjectMixinInterface {
  objectIDSignal: Signal<ObjectID>;
  objectID: ObjectID;
  vm: Signal<VirtualMachine>;
}

export type VMObjectMixinSubscription =
  | "active-ic"
  | "visible-devices";

export const VMObjectMixin = <T extends Constructor<LitElement>>(
  superClass: T,
) => {
  class VMObjectMixinClass extends superClass {
    objectIDSignal: Signal<ObjectID | null> = signal(null);
    vm: Signal<VirtualMachine> = signal(null);

    get objectID(): number {
      return this.objectIDSignal.peek();
    }

    @property({type: Number})
    set objectID(value: number) {
      this.objectIDSignal.value = value;
    }

    constructor (...args: any[]) {
      super(...args);
      this.setupVM();
    }

    private async setupVM() {
      this.vm.value = await window.VM.get();
    }
  }

  return VMObjectMixinClass as Constructor<VMObjectMixinInterface> & T;
};

export declare class VMTemplateDBMixinInterface {
  templateDB: Signal<TemplateDatabase>;
  _handleDeviceDBLoad(e: CustomEvent): void;
  postDBSetUpdate(): void;
}
