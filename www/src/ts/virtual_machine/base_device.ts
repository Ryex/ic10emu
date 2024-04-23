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
} from "ic10emu_wasm";
import { structuralEqual } from "utils";
import { LitElement, PropertyValueMap } from "lit";
import type { DeviceDB } from "./device_db";

type Constructor<T = {}> = new (...args: any[]) => T;

export declare class VMDeviceMixinInterface {
  deviceID: number;
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
}

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

    device: DeviceRef;

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
      window.VM.get().then((vm) =>
        vm.addEventListener(
          "vm-device-modified",
          this._handleDeviceModified.bind(this),
        ),
      );
      window.VM.get().then((vm) =>
        vm.addEventListener(
          "vm-devices-update",
          this._handleDevicesModified.bind(this),
        ),
      );
      this.updateDevice();
      return root;
    }

    _handleDeviceModified(e: CustomEvent) {
      const id = e.detail;
      if (this.deviceID === id) {
        this.updateDevice();
      } else {
        this.requestUpdate();
      }
    }

    _handleDevicesModified(e: CustomEvent) {
      const ids = e.detail;
      this.requestUpdate();
    }

    updateDevice() {
      this.device = window.VM.vm.devices.get(this.deviceID)!;

      const name = this.device.name ?? null;
      if (this.name !== name) {
        this.name = name;
      }
      const nameHash = this.device.nameHash ?? null;
      if (this.nameHash !== nameHash) {
        this.nameHash = nameHash;
      }
      const prefabName = this.device.prefabName ?? null;
      if (this.prefabName !== prefabName) {
        this.prefabName = prefabName;
      }
      const fields = this.device.fields;
      if (!structuralEqual(this.fields, fields)) {
        this.fields = fields;
      }
      const slots = this.device.slots;
      if (!structuralEqual(this.slots, slots)) {
        this.slots = slots;
      }
      const reagents = this.device.reagents;
      if (!structuralEqual(this.reagents, reagents)) {
        this.reagents = reagents;
      }
      const connections = this.device.connections;
      if (!structuralEqual(this.connections, connections)) {
        this.connections = connections;
      }
      if (typeof this.device.ic !== "undefined") {
        this.updateIC();
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

    update(changedProperties: PropertyValueMap<any> | Map<PropertyKey, unknown>): void {
      super.update(changedProperties);
      this.updateDevice();
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
  _handleDeviceDBLoad(e: CustomEvent): void
}

export const VMDeviceDBMixin = <T extends Constructor<LitElement>>(superClass: T) => {
  class VMDeviceDBMixinClass extends superClass {

    connectedCallback(): void {
      const root = super.connectedCallback();
      window.VM.vm.addEventListener(
        "vm-device-db-loaded",
        this._handleDeviceDBLoad.bind(this),
      );
      return root;
    }

    _handleDeviceDBLoad(e: CustomEvent) {
      this.deviceDB = e.detail;
    }

    private _deviceDB: DeviceDB;

    get deviceDB(): DeviceDB {
      return this._deviceDB;
    }

    @state()
    set deviceDB(val: DeviceDB) {
      this._deviceDB = val;
    }
  }

  return VMDeviceDBMixinClass as Constructor<VMDeviceDBMixinInterface> & T
}
