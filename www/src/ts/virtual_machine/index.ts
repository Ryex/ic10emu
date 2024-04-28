import {
  DeviceRef,
  DeviceTemplate,
  FrozenVM,
  LogicType,
  SlotLogicType,
  SlotOccupantTemplate,
  Slots,
  VMRef,
  init,
} from "ic10emu_wasm";
import { DeviceDB } from "./device_db";
import "./base_device";
import "./device";
import { App } from "app";
export interface ToastMessage {
  variant: "warning" | "danger" | "success" | "primary" | "neutral";
  icon: string;
  title: string;
  msg: string;
  id: string;
}

export interface CacheDeviceRef extends DeviceRef {
  dirty: boolean;
}

function cachedDeviceRef(ref: DeviceRef) {
  let slotsDirty = true;
  let cachedSlots: Slots = undefined;
  return new Proxy<DeviceRef>(ref, {
    get(target, prop, receiver) {
      if (prop === "slots") {
        if (typeof cachedSlots === undefined || slotsDirty) {
          cachedSlots = target.slots;
          slotsDirty = false;
        }
        return cachedSlots;
      } else if (prop === "dirty") {
        return slotsDirty;
      }
      return Reflect.get(target, prop, receiver);
    },
    set(target, prop, value) {
      if (prop === "dirty") {
        slotsDirty = value;
        return true;
      }
      return Reflect.set(target, prop, value);
    },
  }) as CacheDeviceRef;
}

class VirtualMachine extends EventTarget {
  ic10vm: VMRef;
  _devices: Map<number, CacheDeviceRef>;
  _ics: Map<number, DeviceRef>;

  db: DeviceDB;
  dbPromise: Promise<{ default: DeviceDB }>;

  private app: App;

  constructor(app: App) {
    super();
    this.app = app;
    const vm = init();
    window.VM.set(this);

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
    return this._ics.get(this.app.session.activeIC);
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
        this._devices.set(id, cachedDeviceRef(this.ic10vm.getDevice(id)!));
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
      device.dirty = true;
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
      this.app.session.save();
    }
  }

  updateCode() {
    const progs = this.app.session.programs;
    for (const id of progs.keys()) {
      const attempt = Date.now().toString(16);
      const ic = this._ics.get(id);
      const prog = progs.get(id);
      if (ic && prog && ic.code !== prog) {
        try {
          console.time(`CompileProgram_${id}_${attempt}`);
          this.ics.get(id)!.setCodeInvalid(progs.get(id)!);
          const compiled = this.ics.get(id)?.program!;
          this.app.session.setProgramErrors(id, compiled.errors);
          this.dispatchEvent(
            new CustomEvent("vm-device-modified", { detail: id }),
          );
        } catch (err) {
          this.handleVmError(err);
        } finally {
          console.timeEnd(`CompileProgram_${id}_${attempt}`);
        }
      }
    }
    this.update(false);
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

  update(save: boolean = true) {
    this.updateDevices();
    this.ic10vm.lastOperationModified.forEach((id, _index, _modifiedIds) => {
      if (this.devices.has(id)) {
        this.dispatchEvent(
          new CustomEvent("vm-device-modified", { detail: id }),
        );
      }
    }, this);
    this.updateDevice(this.activeIC.id, save);
    if (save) this.app.session.save();
  }

  updateDevice(id: number, save: boolean = true) {
    const device = this._devices.get(id);
    device.dirty = true;
    this.dispatchEvent(
      new CustomEvent("vm-device-modified", { detail: device.id }),
    );
    if (typeof device.ic !== "undefined") {
      this.app.session.setActiveLine(device.id, device.ip!);
    }
    if (save) this.app.session.save();
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

  changeDeviceID(oldID: number, newID: number): boolean {
    try {
      this.ic10vm.changeDeviceId(oldID, newID);
      if (this.app.session.activeIC === oldID) {
        this.app.session.activeIC = newID;
      }
      this.updateDevices();
      this.dispatchEvent(
        new CustomEvent("vm-device-id-change", {
          detail: {
            old: oldID,
            new: newID,
          },
        }),
      );
      this.app.session.changeID(oldID, newID);
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
      this.updateDevice(ic.id);
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
      this.updateDevice(ic.id);
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
        this.dispatchEvent(
          new CustomEvent("vm-device-modified", { detail: id }),
        );
        this.app.session.save();
        return true;
      } catch (e) {
        this.handleVmError(e);
      }
    }
    return false;
  }

  setDeviceField(
    id: number,
    field: LogicType,
    val: number,
    force?: boolean,
  ): boolean {
    force = force ?? false;
    const device = this._devices.get(id);
    if (device) {
      try {
        device.setField(field, val, force);
        this.updateDevice(device.id);
        return true;
      } catch (err) {
        this.handleVmError(err);
      }
    }
    return false;
  }

  setDeviceSlotField(
    id: number,
    slot: number,
    field: SlotLogicType,
    val: number,
    force?: boolean,
  ): boolean {
    force = force ?? false;
    const device = this._devices.get(id);
    if (device) {
      try {
        device.setSlotField(slot, field, val, force);
        this.updateDevice(device.id);
        return true;
      } catch (err) {
        this.handleVmError(err);
      }
    }
    return false;
  }

  setDeviceConnection(
    id: number,
    conn: number,
    val: number | undefined,
  ): boolean {
    const device = this._devices.get(id);
    if (typeof device !== "undefined") {
      try {
        this.ic10vm.setDeviceConnection(id, conn, val);
        this.updateDevice(device.id);
        return true;
      } catch (err) {
        this.handleVmError(err);
      }
    }
    return false;
  }

  setDevicePin(id: number, pin: number, val: number | undefined): boolean {
    const device = this._devices.get(id);
    if (typeof device !== "undefined") {
      try {
        this.ic10vm.setPin(id, pin, val);
        this.updateDevice(device.id);
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
      this._devices.set(id, cachedDeviceRef(this.ic10vm.getDevice(id)!));
      const device_ids = this.ic10vm.devices;
      this.dispatchEvent(
        new CustomEvent("vm-devices-update", {
          detail: Array.from(device_ids),
        }),
      );
      this.app.session.save();
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

  setDeviceSlotOccupant(
    id: number,
    index: number,
    template: SlotOccupantTemplate,
  ): boolean {
    const device = this._devices.get(id);
    if (typeof device !== "undefined") {
      try {
        console.log("setting slot occupant", template);
        this.ic10vm.setSlotOccupant(id, index, template);
        this.updateDevice(device.id);
        return true;
      } catch (err) {
        this.handleVmError(err);
      }
    }
    return false;
  }

  removeDeviceSlotOccupant(id: number, index: number): boolean {
    const device = this._devices.get(id);
    if (typeof device !== "undefined") {
      try {
        this.ic10vm.removeSlotOccupant(id, index);
        this.updateDevice(device.id);
        return true;
      } catch (err) {
        this.handleVmError(err);
      }
    }
    return false;
  }

  saveVMState(): FrozenVM {
    return this.ic10vm.saveVMState();
  }

  restoreVMState(state: FrozenVM) {
    try {
      this.ic10vm.restoreVMState(state);
      this._devices = new Map();
      this._ics = new Map();
      this.updateDevices();
    } catch (e) {
      this.handleVmError(e);
    }
  }

  getPrograms() {
    const programs: [number, string][] = Array.from(this._ics.entries()).map(
      ([id, ic]) => [id, ic.code],
    );
    return programs;
  }
}

export { VirtualMachine };
