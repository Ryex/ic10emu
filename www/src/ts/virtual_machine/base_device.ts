import { property, state } from "lit/decorators.js";

import type {
  DeviceRef,
  LogicFields,
  Reagents,
  Slot,
  Connection,
  ICError,
  Registers,
  Stack,
  Aliases,
  Defines,
  Pins,
  LogicType,
} from "ic10emu_wasm";
import { structuralEqual } from "utils";
import { LitElement, PropertyValueMap } from "lit";
import type { DeviceDB } from "./device_db";

type Constructor<T = {}> = new (...args: any[]) => T;

export declare class VMDeviceMixinInterface {
  deviceID: number;
  activeICId: number;
  device: DeviceRef;
  name: string | null;
  nameHash: number | null;
  prefabName: string | null;
  fields: LogicFields;
  slots: Slot[];
  reagents: Reagents;
  connections: Connection[];
  icIP: number;
  icOpCount: number;
  icState: string;
  errors: ICError[];
  registers: Registers | null;
  stack: Stack | null;
  aliases: Aliases | null;
  defines: Defines | null;
  pins: Pins | null;
  _handleDeviceModified(e: CustomEvent): void;
  updateDevice(): void;
  updateIC(): void;
  subscribe(...sub: VMDeviceMixinSubscription[]): void;
  unsubscribe(filter: (sub: VMDeviceMixinSubscription) => boolean): void;
}

export type VMDeviceMixinSubscription =
  | "name"
  | "nameHash"
  | "prefabName"
  | "fields"
  | "slots"
  | "slots-count"
  | "reagents"
  | "connections"
  | "ic"
  | "active-ic"
  | { field: LogicType }
  | { slot: number };

export const VMDeviceMixin = <T extends Constructor<LitElement>>(
  superClass: T,
) => {
  class VMDeviceMixinClass extends superClass {
    private _deviceID: number;
    get deviceID() {
      return this._deviceID;
    }
    @property({ type: Number })
    set deviceID(val: number) {
      this._deviceID = val;
      this.updateDevice();
    }

    @state() private deviceSubscriptions: VMDeviceMixinSubscription[] = [];

    subscribe(...sub: VMDeviceMixinSubscription[]) {
      this.deviceSubscriptions = this.deviceSubscriptions.concat(sub);
    }

    // remove subscripotions matching the filter
    unsubscribe(filter: (sub: VMDeviceMixinSubscription) => boolean) {
      this.deviceSubscriptions = this.deviceSubscriptions.filter(
        (sub) => !filter(sub),
      );
    }

    device: DeviceRef;

    @state() activeICId: number;

    @state() name: string | null = null;
    @state() nameHash: number | null = null;
    @state() prefabName: string | null;
    @state() fields: LogicFields;
    @state() slots: Slot[];
    @state() reagents: Reagents;
    @state() connections: Connection[];
    @state() icIP: number;
    @state() icOpCount: number;
    @state() icState: string;
    @state() errors: ICError[];
    @state() registers: Registers | null;
    @state() stack: Stack | null;
    @state() aliases: Aliases | null;
    @state() defines: Defines | null;
    @state() pins: Pins | null;

    connectedCallback(): void {
      const root = super.connectedCallback();
      window.VM.get().then((vm) => {
        vm.addEventListener(
          "vm-device-modified",
          this._handleDeviceModified.bind(this),
        );
        vm.addEventListener(
          "vm-devices-update",
          this._handleDevicesModified.bind(this),
        );
        vm.addEventListener(
          "vm-device-id-change",
          this._handleDeviceIdChange.bind(this),
        );
      });
      this.updateDevice();
      return root;
    }

    disconnectedCallback(): void {
      window.VM.get().then((vm) => {
        vm.removeEventListener(
          "vm-device-modified",
          this._handleDeviceModified.bind(this),
        );
        vm.removeEventListener(
          "vm-devices-update",
          this._handleDevicesModified.bind(this),
        );
        vm.removeEventListener(
          "vm-device-id-change",
          this._handleDeviceIdChange.bind(this),
        );
      });
    }

    _handleDeviceModified(e: CustomEvent) {
      const id = e.detail;
      const activeIcId = window.App.app.session.activeIC;
      if (this.deviceID === id) {
        this.updateDevice();
      } else if (
        id === activeIcId &&
        this.deviceSubscriptions.includes("active-ic")
      ) {
        this.updateDevice();
      }
    }

    _handleDevicesModified(e: CustomEvent<number[]>) {
      const activeIcId = window.App.app.session.activeIC;
      const ids = e.detail;
      if (ids.includes(this.deviceID)) {
        this.updateDevice();
      } else if (
        ids.includes(activeIcId) &&
        this.deviceSubscriptions.includes("active-ic")
      ) {
        this.updateDevice();
      }
    }

    _handleDeviceIdChange(e: CustomEvent<{ old: number; new: number }>) {
      if (this.deviceID === e.detail.old) {
        this.deviceID = e.detail.new;
      }
    }

    updateDevice() {
      this.device = window.VM.vm.devices.get(this.deviceID)!;

      if (typeof this.device === "undefined") {
        return;
      }

      for (const sub of this.deviceSubscriptions) {
        if (typeof sub === "string") {
          if (sub == "name") {
            const name = this.device.name ?? null;
            if (this.name !== name) {
              this.name = name;
            }
          } else if (sub === "nameHash") {
            const nameHash = this.device.nameHash ?? null;
            if (this.nameHash !== nameHash) {
              this.nameHash = nameHash;
            }
          } else if (sub === "prefabName") {
            const prefabName = this.device.prefabName ?? null;
            if (this.prefabName !== prefabName) {
              this.prefabName = prefabName;
            }
          } else if (sub === "fields") {
            const fields = this.device.fields;
            if (!structuralEqual(this.fields, fields)) {
              this.fields = fields;
            }
          } else if (sub === "slots") {
            const slots = this.device.slots;
            if (!structuralEqual(this.slots, slots)) {
              this.slots = slots;
            }
          } else if (sub === "slots-count") {
            const slots = this.device.slots;
            if (typeof this.slots === "undefined") {
              this.slots = slots;
            } else if (this.slots.length !== slots.length) {
              this.slots = slots;
            }
          } else if (sub === "reagents") {
            const reagents = this.device.reagents;
            if (!structuralEqual(this.reagents, reagents)) {
              this.reagents = reagents;
            }
          } else if (sub === "connections") {
            const connections = this.device.connections;
            if (!structuralEqual(this.connections, connections)) {
              this.connections = connections;
            }
          } else if (sub === "ic") {
            if (typeof this.device.ic !== "undefined") {
              this.updateIC();
            }
          } else if (sub === "active-ic") {
            const activeIc = window.VM.vm?.activeIC;
            if (this.activeICId !== activeIc.id) {
              this.activeICId = activeIc.id;
            }
          }
        } else {
          if ("field" in sub) {
            const fields = this.device.fields;
            if (this.fields.get(sub.field) !== fields.get(sub.field)) {
              this.fields = fields;
            }
          } else if ("slot" in sub) {
            const slots = this.device.slots;
            if (
              typeof this.slots === "undefined" ||
              this.slots.length < sub.slot
            ) {
              this.slots = slots;
            } else if (
              !structuralEqual(this.slots[sub.slot], slots[sub.slot])
            ) {
              this.slots = slots;
            }
          }
        }
      }
    }

    updateIC() {
      const ip = this.device.ip!;
      if (this.icIP !== ip) {
        this.icIP = ip;
      }
      const opCount = this.device.instructionCount!;
      if (this.icOpCount !== opCount) {
        this.icOpCount = opCount;
      }
      const state = this.device.state!;
      if (this.icState !== state) {
        this.icState = state;
      }
      const errors = this.device.program?.errors ?? null;
      if (!structuralEqual(this.errors, errors)) {
        this.errors = errors;
      }
      const registers = this.device.registers ?? null;
      if (!structuralEqual(this.registers, registers)) {
        this.registers = registers;
      }
      const stack = this.device.stack ?? null;
      if (!structuralEqual(this.stack, stack)) {
        this.stack = stack;
      }
      const aliases = this.device.aliases ?? null;
      if (!structuralEqual(this.aliases, aliases)) {
        this.aliases = aliases;
      }
      const defines = this.device.defines ?? null;
      if (!structuralEqual(this.defines, defines)) {
        this.defines = defines;
      }
      const pins = this.device.pins ?? null;
      if (!structuralEqual(this.pins, pins)) {
        this.pins = pins;
      }
    }
  }
  return VMDeviceMixinClass as Constructor<VMDeviceMixinInterface> & T;
};

export const VMActiveICMixin = <T extends Constructor<LitElement>>(
  superClass: T,
) => {
  class VMActiveICMixinClass extends VMDeviceMixin(superClass) {
    constructor() {
      super();
      this.deviceID = window.App.app.session.activeIC;
    }

    connectedCallback(): void {
      const root = super.connectedCallback();
      window.VM.get().then((vm) =>
        vm.addEventListener("vm-run-ic", this._handleDeviceModified.bind(this)),
      );
      window.App.app.session.addEventListener(
        "session-active-ic",
        this._handleActiveIC.bind(this),
      );
      return root;
    }

    disconnectedCallback(): void {
      window.VM.get().then((vm) =>
        vm.removeEventListener(
          "vm-run-ic",
          this._handleDeviceModified.bind(this),
        ),
      );
      window.App.app.session.removeEventListener(
        "session-active-ic",
        this._handleActiveIC.bind(this),
      );
    }

    _handleActiveIC(e: CustomEvent) {
      const id = e.detail;
      if (this.deviceID !== id) {
        this.deviceID = id;
        this.device = window.VM.vm.devices.get(this.deviceID)!;
      }
      this.updateDevice();
    }
  }

  return VMActiveICMixinClass as Constructor<VMDeviceMixinInterface> & T;
};

export declare class VMDeviceDBMixinInterface {
  deviceDB: DeviceDB;
  _handleDeviceDBLoad(e: CustomEvent): void;
  postDBSetUpdate(): void;
}

export const VMDeviceDBMixin = <T extends Constructor<LitElement>>(
  superClass: T,
) => {
  class VMDeviceDBMixinClass extends superClass {
    connectedCallback(): void {
      const root = super.connectedCallback();
      window.VM.vm.addEventListener(
        "vm-device-db-loaded",
        this._handleDeviceDBLoad.bind(this),
      );
      if (typeof window.VM.vm.db !== "undefined") {
        this.deviceDB = window.VM.vm.db!;
      }
      return root;
    }

    disconnectedCallback(): void {
      window.VM.vm.removeEventListener(
        "vm-device-db-loaded",
        this._handleDeviceDBLoad.bind(this),
      );
    }

    _handleDeviceDBLoad(e: CustomEvent) {
      this.deviceDB = e.detail;
    }

    private _deviceDB: DeviceDB;

    get deviceDB(): DeviceDB {
      return this._deviceDB;
    }

    postDBSetUpdate(): void { }

    @state()
    set deviceDB(val: DeviceDB) {
      this._deviceDB = val;
      this.postDBSetUpdate();
    }
  }

  return VMDeviceDBMixinClass as Constructor<VMDeviceDBMixinInterface> & T;
};
