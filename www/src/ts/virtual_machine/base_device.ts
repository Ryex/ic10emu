import { property, state } from "lit/decorators.js";
import { BaseElement } from "../components";
import {
  DeviceRef,
  Fields,
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
import { structuralEqual } from "../utils";

export class VMBaseDevice extends BaseElement {
  @property({ type: Number }) accessor deviceID: number;
  @state() protected accessor device: DeviceRef;

  @state() accessor name: string | null;
  @state() accessor nameHash: number | null;
  @state() accessor prefabName: string | null;
  @state() accessor fields: Fields;
  @state() accessor slots: Slot[];
  @state() accessor reagents: Reagents;
  @state() accessor connections: Connection[];
  @state() accessor icIP: number;
  @state() accessor icOpCount: number;
  @state() accessor icState: string;
  @state() accessor errors: ICError[];
  @state() accessor registers: Registers | null;
  @state() accessor stack: Stack | null;
  @state() accessor aliases: Aliases | null;
  @state() accessor defines: Defines | null;
  @state() accessor pins: Pins | null;

  constructor() {
    super();
    this.name = null;
    this.nameHash = null;
  }

  connectedCallback(): void {
    const root = super.connectedCallback();
    this.device = window.VM!.devices.get(this.deviceID)!;
    window.VM?.addEventListener(
      "vm-device-modified",
      this._handleDeviceModified.bind(this),
    );
    this.updateDevice();
    return root;
  }

  _handleDeviceModified(e: CustomEvent) {
    const id = e.detail;
    if (this.deviceID === id) {
      this.updateDevice();
    }
  }

  updateDevice() {
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
    this.updateIC();
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
    const errors = this.device.program!.errors ?? null;
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
    if(!structuralEqual(this.pins, pins)) {
      this.pins = pins;
    }
  }
}

export class VMActiveIC extends VMBaseDevice {

  constructor() {
    super();
    this.deviceID = window.App!.session.activeIC;
  }

  connectedCallback(): void {
    const root = super.connectedCallback();
    window.VM?.addEventListener(
      "vm-run-ic",
      this._handleDeviceModified.bind(this),
    );
    window.App?.session.addEventListener(
      "session-active-ic",
      this._handleActiveIC.bind(this),
    );
    return root;
  }

  _handleActiveIC(e: CustomEvent) {
    const id = e.detail;
    if (this.deviceID !== id) {
      this.deviceID = id;
      this.device = window.VM!.devices.get(this.deviceID)!;
    }
    this.updateDevice();
  }

}
