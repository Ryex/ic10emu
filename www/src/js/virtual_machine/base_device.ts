import { property, state } from "lit/decorators.js";
import { BaseElement } from "../components";
import {
  DeviceRef,
  Fields,
  Reagents,
  Slot,
  Connection,
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
  }
}
