import { DeviceRef, DeviceTemplate, LogicType, SlotLogicType, VM, init } from "ic10emu_wasm";
import { DeviceDB } from "./device_db";
import "./base_device";

declare global {
  interface Window {
    VM?: VirtualMachine;
  }
}

export interface ToastMessage {
  variant: "warning" | "danger" | "success" | "primary" | "neutral";
  icon: string;
  title: string;
  msg: string;
  id: string;
}

class VirtualMachine extends EventTarget {
  ic10vm: VM;
  _devices: Map<number, DeviceRef>;
  _ics: Map<number, DeviceRef>;

  accessor db: DeviceDB;
  dbPromise: Promise<{ default: DeviceDB }>;

  constructor() {
    super();
    const vm = init();

    window.VM = this;

    this.ic10vm = vm;

    this._devices = new Map();
    this._ics = new Map();

    this.dbPromise = import("../../../data/database.json", {
      assert: { type: "json" },
    }) as Promise<{ default: DeviceDB }>;

    this.dbPromise.then((module) =>
      this.setupDeviceDatabase(module.default as DeviceDB),
    );

    this.updateDevices();
    this.updateCode();
  }

  get devices() {
    return this._devices;
  }

  get deviceIds() {
    const ids = Array.from(this.ic10vm.devices);
    ids.sort();
    return ids;
  }

  get ics() {
    return this._ics;
  }

  get icIds() {
    return Array.from(this.ic10vm.ics);
  }

  get networks() {
    return Array.from(this.ic10vm.networks);
  }

  get defaultNetwork() {
    return this.ic10vm.defaultNetwork;
  }

  get activeIC() {
    return this._ics.get(window.App!.session.activeIC);
  }

  visibleDevices(source: number) {
    const ids = Array.from(this.ic10vm.visibleDevices(source));
    return ids.map((id, _index) => this._devices.get(id)!);
  }

  updateDevices() {
    var update_flag = false;
    const device_ids = this.ic10vm.devices;
    for (const id of device_ids) {
      if (!this._devices.has(id)) {
        this._devices.set(id, this.ic10vm.getDevice(id)!);
        update_flag = true;
      }
    }
    for (const id of this._devices.keys()) {
      if (!device_ids.includes(id)) {
        this._devices.delete(id);
        update_flag = true;
      }
    }

    for (const [id, device] of this._devices) {
      if (typeof device.ic !== "undefined") {
        if (!this._ics.has(id)) {
          this._ics.set(id, device);
          update_flag = true;
        }
      }
    }

    for (const id of this._ics.keys()) {
      if (!this._devices.has(id)) {
        this._ics.delete(id);
        update_flag = true;
      }
    }

    if (update_flag) {
      const ids = Array.from(device_ids);
      ids.sort();
      this.dispatchEvent(
        new CustomEvent("vm-devices-update", {
          detail: ids,
        }),
      );
    }
  }

  updateCode() {
    const progs = window.App!.session.programs;
    for (const id of progs.keys()) {
      const attempt = Date.now().toString(16);
      const ic = this._ics.get(id);
      const prog = progs.get(id);
      if (ic && prog) {
        try {
          console.time(`CompileProgram_${id}_${attempt}`);
          this.ics.get(id)!.setCodeInvalid(progs.get(id)!);
          const compiled = this.ics.get(id)?.program!;
          window.App?.session.setProgramErrors(id, compiled.errors);
          this.dispatchEvent(
            new CustomEvent("vm-device-modified", { detail: id }),
          );
        } catch (err) {
          this.handleVmError(err);
        } finally{
          console.timeEnd(`CompileProgram_${id}_${attempt}`);
        }
      }
    }
    this.update();
  }

  step() {
    const ic = this.activeIC;
    if (ic) {
      try {
        ic.step(false);
      } catch (err) {
        this.handleVmError(err);
      }
      this.update();
      this.dispatchEvent(
        new CustomEvent("vm-run-ic", { detail: this.activeIC!.id }),
      );
    }
  }

  run() {
    const ic = this.activeIC;
    if (ic) {
      try {
        ic.run(false);
      } catch (err) {
        this.handleVmError(err);
      }
      this.update();
      this.dispatchEvent(
        new CustomEvent("vm-run-ic", { detail: this.activeIC!.id }),
      );
    }
  }

  reset() {
    const ic = this.activeIC;
    if (ic) {
      ic.reset();
      this.update();
    }
  }

  update() {
    this.updateDevices();
    this.ic10vm.lastOperationModified.forEach((id, _index, _modifiedIds) => {
      if (this.devices.has(id)) {
        this.dispatchEvent(
          new CustomEvent("vm-device-modified", { detail: id }),
        );
      }
    }, this);
    this.updateDevice(this.activeIC);
  }

  updateDevice(device: DeviceRef) {
    this.dispatchEvent(
      new CustomEvent("vm-device-modified", { detail: device.id }),
    );
    if (typeof device.ic !== "undefined") {
      window.App!.session.setActiveLine(device.id, device.ip!);
    }
  }

  handleVmError(err: Error) {
    console.log("Error in Virtual Machine", err);
    const message: ToastMessage = {
      variant: "danger",
      icon: "bug",
      title: `Error in Virtual Machine ${err.name}`,
      msg: err.message,
      id: Date.now().toString(16),
    };
    this.dispatchEvent(new CustomEvent("vm-message", { detail: message }));
  }

  changeDeviceId(old_id: number, new_id: number): boolean {
    try {
      this.ic10vm.changeDeviceId(old_id, new_id);
      this.updateDevices();
      if (window.App.session.activeIC === old_id) {
        window.App.session.activeIC = new_id;
      }
      return true;
    } catch (err) {
      this.handleVmError(err);
      return false;
    }
  }

  setRegister(index: number, val: number): boolean {
    const ic = this.activeIC!;
    try {
      ic.setRegister(index, val);
      this.updateDevice(ic);
      return true;
    } catch (err) {
      this.handleVmError(err);
      return false;
    }
  }

  setStack(addr: number, val: number): boolean {
    const ic = this.activeIC!;
    try {
      ic!.setStack(addr, val);
      this.updateDevice(ic);
      return true;
    } catch (err) {
      this.handleVmError(err);
      return false;
    }
  }

  setDeviceName(id: number, name: string): boolean {
    const device = this._devices.get(id);
    if (device) {
      try {
        device.setName(name);
        this.dispatchEvent(new CustomEvent("vm-device-modified", { detail: id }));
        return true;
      } catch(e) {
        this.handleVmError(e);
      }
    }
    return false;
  }

  setDeviceField(id: number, field: LogicType, val: number, force?: boolean): boolean {
    force = force ?? false;
    const device = this._devices.get(id);
    if (device) {
      try {
        device.setField(field, val, force);
        this.updateDevice(device);
        return true;
      } catch (err) {
        this.handleVmError(err);
      }
    }
    return false;
  }

  setDeviceSlotField(id: number, slot: number, field: SlotLogicType, val: number, force?: boolean): boolean {
    force = force ?? false;
    const device = this._devices.get(id);
    if (device) {
      try {
        device.setSlotField(slot, field, val, false);
        this.updateDevice(device);
        return true;
      } catch (err) {
        this.handleVmError(err);
      }
    }
    return false;
  }

  setDeviceConnection(id: number, conn: number, val: number | undefined): boolean {
    const device = this._devices.get(id);
    if (typeof device !== "undefined") {
      try {
        this.ic10vm.setDeviceConnection(id, conn, val);
        this.updateDevice(device);
        return true
      } catch (err) {
        this.handleVmError(err);
      }
    }
    return false
  }

  setDevicePin(id: number, pin: number, val: number | undefined): boolean {
    const device = this._devices.get(id);
    if (typeof device !== "undefined") {
      try {
        this.ic10vm.setPin(id, pin, val);
        this.updateDevice(device);
        return true;
      } catch (err) {
        this.handleVmError(err);
      }
    }
    return false;
  }

  setupDeviceDatabase(db: DeviceDB) {
    this.db = db;
    console.log("Loaded Device Database", this.db);
    this.dispatchEvent(
      new CustomEvent("vm-device-db-loaded", { detail: this.db }),
    );
  }

  addDeviceFromTemplate(template: DeviceTemplate): boolean {
    try {
      console.log("adding device", template);
      const id = this.ic10vm.addDeviceFromTemplate(template);
      this._devices.set(id, this.ic10vm.getDevice(id)!);
      const device_ids = this.ic10vm.devices;
      this.dispatchEvent(
        new CustomEvent("vm-devices-update", {
          detail: Array.from(device_ids),
        }),
      );
      return true;
    } catch (err) {
      this.handleVmError(err);
      return false;
    }
  }

  removeDevice(id: number): boolean {
    try {
      this.ic10vm.removeDevice(id);
      this.updateDevices();
      return true;
    } catch (err) {
      this.handleVmError(err);
      return false;
    }
  }
}

export { VirtualMachine };
