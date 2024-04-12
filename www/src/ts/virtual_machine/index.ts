import { DeviceRef, VM, init } from "ic10emu_wasm";
import { DeviceDB } from "./device_db"
import "./base_device";

declare global {
  interface Window {
    VM?: VirtualMachine;
  }
}


class VirtualMachine extends EventTarget {
  ic10vm: VM;
  _devices: Map<number, DeviceRef>;
  _ics: Map<number, DeviceRef>;

  accessor db: DeviceDB;
  dbPromise: Promise<{ default: DeviceDB }>

  constructor() {
    super();
    const vm = init();

    window.VM = this;

    this.ic10vm = vm;

    this._devices = new Map();
    this._ics = new Map();

    this.dbPromise = import("../../../data/database.json");
    this.dbPromise.then((module) => this.setupDeviceDatabase(module.default))

    this.updateDevices();
    this.updateCode();
  }

  get devices() {
    return this._devices;
  }

  get deviceIds() {
    return Array.from(this.ic10vm.devices);
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
        this._devices.get(id)!.free();
        this._devices.delete(id);
        update_flag = true;
      }
    }

    const ics = this.ic10vm.ics;
    for (const id of ics) {
      if (!this._ics.has(id)) {
        this._ics.set(id, this._devices.get(id)!);
        update_flag = true;
      }
    }
    for (const id of this._ics.keys()) {
      if (!ics.includes(id)) {
        this._ics.get(id)!.free();
        this._ics.delete(id);
        update_flag = true;
      }
    }
    if (update_flag) {
      this.dispatchEvent(
        new CustomEvent("vm-devices-update", { detail: device_ids }),
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
        console.time(`CompileProgram_${id}_${attempt}`);
        try {
          this.ics.get(id)!.setCodeInvalid(progs.get(id)!);
          const compiled = this.ics.get(id)?.program!;
          window.App?.session.setProgramErrors(id, compiled.errors);
          this.dispatchEvent(
            new CustomEvent("vm-device-modified", { detail: id }),
          );
        } catch (e) {
          console.log(e);
        }
        console.timeEnd(`CompileProgram_${id}_${attempt}`);
      }
    }
    this.update();
  }

  step() {
    const ic = this.activeIC;
    if (ic) {
      try {
        ic.step(false);
      } catch (e) {
        console.log(e);
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
      } catch (e) {
        console.log(e);
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
    const ic = this.activeIC!;
    window.App!.session.setActiveLine(window.App!.session.activeIC, ic.ip!);
  }

  setRegister(index: number, val: number) {
    const ic = this.activeIC!;
    try {
      ic.setRegister(index, val);
      this.dispatchEvent(
        new CustomEvent("vm-device-modified", { detail: ic.id }),
      );
    } catch (e) {
      console.log(e);
    }
  }

  setStack(addr: number, val: number) {
    const ic = this.activeIC!;
    try {
      ic!.setStack(addr, val);
      this.dispatchEvent(
        new CustomEvent("vm-device-modified", { detail: ic.id }),
      );
    } catch (e) {
      console.log(e);
    }
  }

  setDeviceName(id: number, name: string): boolean {
    const device = this._devices.get(id);
    if (device) {
      device.setName(name);
      this.dispatchEvent(new CustomEvent("vm-device-modified", { detail: id }));
      return true;
    }
    return false;
  }

  setDeviceField(id: number, field: string, val: number) {
    const device = this._devices.get(id);
    if (device) {
      try {
        device.setField(field, val);
        this.dispatchEvent(
          new CustomEvent("vm-device-modified", { detail: id }),
        );
        return true;
      } catch (e) {
        console.log(e);
      }
    }
    return false;
  }

  setDeviceSlotField(id: number, slot: number, field: string, val: number) {
    const device = this._devices.get(id);
    if (device) {
      try {
        device.setSlotField(slot, field, val);
        this.dispatchEvent(
          new CustomEvent("vm-device-modified", { detail: id }),
        );
        return true;
      } catch (e) {
        console.log(e);
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
}

export { VirtualMachine };
