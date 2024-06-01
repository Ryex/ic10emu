import type {
  ObjectTemplate,
  FrozenObject,
  FrozenVM,
  LogicType,
  LogicSlotType,
  VMRef,
  TemplateDatabase,
  FrozenCableNetwork,
  FrozenObjectFull,
  ObjectID,
} from "ic10emu_wasm";
import * as Comlink from "comlink";
import "./baseDevice";
import "./device";
import { App } from "app";
import { structuralEqual, TypedEventTarget } from "utils";
export interface ToastMessage {
  variant: "warning" | "danger" | "success" | "primary" | "neutral";
  icon: string;
  title: string;
  msg: string;
  id: string;
}

export interface VirtualMachineEventMap {
  "vm-template-db-loaded": CustomEvent<TemplateDatabase>;
  "vm-objects-update": CustomEvent<number[]>;
  "vm-objects-removed": CustomEvent<number[]>;
  "vm-object-modified": CustomEvent<number>;
  "vm-run-ic": CustomEvent<number>;
  "vm-object-id-change": CustomEvent<{ old: number; new: number }>;
  "vm-networks-update": CustomEvent<number[]>;
  "vm-networks-removed": CustomEvent<number[]>;
  "vm-message": CustomEvent<ToastMessage>;
}

class VirtualMachine extends TypedEventTarget<VirtualMachineEventMap>() {
  ic10vm: Comlink.Remote<VMRef>;
  templateDBPromise: Promise<TemplateDatabase>;
  templateDB: TemplateDatabase;

  private _objects: Map<number, FrozenObjectFull>;
  private _circuitHolders: Map<number, FrozenObjectFull>;
  private _networks: Map<number, FrozenCableNetwork>;
  private _default_network: number;

  private vm_worker: Worker;

  private app: App;

  constructor(app: App) {
    super();
    this.app = app;

    this._objects = new Map();
    this._circuitHolders = new Map();
    this._networks = new Map();

    this.setupVM();
  }

  async setupVM() {
    this.vm_worker = new Worker(new URL("./vmWorker.ts", import.meta.url));
    const loaded = (w: Worker) =>
      new Promise((r) => w.addEventListener("message", r, { once: true }));
    await Promise.all([loaded(this.vm_worker)]);
    console.info("VM Worker loaded");
    const vm = Comlink.wrap<VMRef>(this.vm_worker);
    this.ic10vm = vm;
    window.VM.set(this);

    this.templateDBPromise = this.ic10vm.getTemplateDatabase();

    this.templateDBPromise.then((db) => this.setupTemplateDatabase(db));

    this.updateObjects();
    this.updateNetworks();
    this.updateCode();
  }

  get objects() {
    return this._objects;
  }

  get objectIds() {
    const ids = Array.from(this._objects.keys());
    ids.sort();
    return ids;
  }

  get circuitHolders() {
    return this._circuitHolders;
  }

  get circuitHolderIds() {
    const ids = Array.from(this._circuitHolders.keys());
    ids.sort();
    return ids;
  }

  get networks() {
    const ids = Array.from(this._networks.keys());
    ids.sort();
    return ids;
  }

  get defaultNetwork() {
    return this._default_network;
  }

  get activeIC() {
    return this._circuitHolders.get(this.app.session.activeIC);
  }

  async visibleDevices(source: number) {
    const visDevices = await this.ic10vm.visibleDevices(source);
    const ids = Array.from(visDevices);
    ids.sort();
    return ids.map((id, _index) => this._objects.get(id)!);
  }

  async visibleDeviceIds(source: number) {
    const visDevices = await this.ic10vm.visibleDevices(source);
    const ids = Array.from(visDevices);
    ids.sort();
    return ids;
  }

  async updateNetworks() {
    let updateFlag = false;
    const removedNetworks = [];
    let networkIds: Uint32Array;
    let frozenNetworks: FrozenCableNetwork[];
    try {
      networkIds = await this.ic10vm.networks;
      frozenNetworks = await this.ic10vm.freezeNetworks(networkIds);
    } catch (e) {
      this.handleVmError(e);
      return;
    }
    const updatedNetworks: ObjectID[] = [];
    for (const [index, id] of networkIds.entries()) {
      if (!this._networks.has(id)) {
        this._networks.set(id, frozenNetworks[index]);
        updateFlag = true;
        updatedNetworks.push(id);
      } else {
        if (!structuralEqual(this._networks.get(id), frozenNetworks[index])) {
          this._networks.set(id, frozenNetworks[index]);
          updatedNetworks.push(id);
          updateFlag = true;
        }
      }
    }

    for (const id of this._networks.keys()) {
      if (!networkIds.includes(id)) {
        this._networks.delete(id);
        updateFlag = true;
        removedNetworks.push(id);
      }
    }

    if (updateFlag) {
      const ids = Array.from(updatedNetworks);
      ids.sort();
      this.dispatchCustomEvent("vm-networks-update", ids);
      if (removedNetworks.length > 0) {
        this.dispatchCustomEvent("vm-networks-removed", removedNetworks);
      }
      this.app.session.save();
    }
  }

  async updateObjects() {
    let updateFlag = false;
    const removedObjects = [];
    let objectIds: Uint32Array;
    let frozenObjects: FrozenObjectFull[];
    try {
      objectIds = await this.ic10vm.objects;
      frozenObjects = await this.ic10vm.freezeObjects(objectIds);
    } catch (e) {
      this.handleVmError(e);
      return;
    }
    const updatedObjects: ObjectID[] = [];

    for (const [index, id] of objectIds.entries()) {
      if (!this._objects.has(id)) {
        this._objects.set(id, frozenObjects[index]);
        updateFlag = true;
        updatedObjects.push(id);
      } else {
        if (!structuralEqual(this._objects.get(id), frozenObjects[index])) {
          this._objects.set(id, frozenObjects[index]);
          updatedObjects.push(id);
          updateFlag = true;
        }
      }
    }

    for (const id of this._objects.keys()) {
      if (!objectIds.includes(id)) {
        this._objects.delete(id);
        updateFlag = true;
        removedObjects.push(id);
      }
    }

    for (const [id, obj] of this._objects) {
      if (typeof obj.obj_info.socketed_ic !== "undefined") {
        if (!this._circuitHolders.has(id)) {
          this._circuitHolders.set(id, obj);
          updateFlag = true;
          if (!updatedObjects.includes(id)) {
            updatedObjects.push(id);
          }
        }
      } else {
        if (this._circuitHolders.has(id)) {
          updateFlag = true;
          if (!updatedObjects.includes(id)) {
            updatedObjects.push(id);
          }
          this._circuitHolders.delete(id);
        }
      }
    }

    for (const id of this._circuitHolders.keys()) {
      if (!this._objects.has(id)) {
        this._circuitHolders.delete(id);
        updateFlag = true;
        if (!removedObjects.includes(id)) {
          removedObjects.push(id);
        }
      }
    }

    if (updateFlag) {
      const ids = Array.from(updatedObjects);
      ids.sort();
      this.dispatchCustomEvent("vm-objects-update", ids);
      if (removedObjects.length > 0) {
        this.dispatchCustomEvent("vm-objects-removed", removedObjects);
      }
      this.app.session.save();
    }
  }

  async updateCode() {
    const progs = this.app.session.programs;
    for (const id of progs.keys()) {
      const attempt = Date.now().toString(16);
      const circuitHolder = this._circuitHolders.get(id);
      const prog = progs.get(id);
      if (
        circuitHolder &&
        prog &&
        circuitHolder.obj_info.source_code !== prog
      ) {
        try {
          console.time(`CompileProgram_${id}_${attempt}`);
          await this.ic10vm.setCodeInvalid(id, progs.get(id)!);
          const errors = await this.ic10vm.getCompileErrors(id);
          this.app.session.setProgramErrors(id, errors);
          this.dispatchCustomEvent("vm-object-modified", id);
        } catch (err) {
          this.handleVmError(err);
        } finally {
          console.timeEnd(`CompileProgram_${id}_${attempt}`);
        }
      }
    }
    this.update(false);
  }

  async step() {
    const ic = this.activeIC;
    if (ic) {
      try {
        await this.ic10vm.stepProgrammable(ic.obj_info.id, false);
      } catch (err) {
        this.handleVmError(err);
      }
      this.update();
      this.dispatchCustomEvent("vm-run-ic", this.activeIC!.obj_info.id);
    }
  }

  async run() {
    const ic = this.activeIC;
    if (ic) {
      try {
        await this.ic10vm.runProgrammable(ic.obj_info.id, false);
      } catch (err) {
        this.handleVmError(err);
      }
      this.update();
      this.dispatchCustomEvent("vm-run-ic", this.activeIC!.obj_info.id);
    }
  }

  async reset() {
    const ic = this.activeIC;
    if (ic) {
      await this.ic10vm.resetProgrammable(ic.obj_info.id);
      await this.update();
    }
  }

  async update(save: boolean = true) {
    await this.updateObjects();
    await this.updateNetworks();
    const lastModified = await this.ic10vm.lastOperationModified;
    lastModified.forEach((id, _index, _modifiedIds) => {
      if (this.objects.has(id)) {
        this.updateObject(id, false);
      }
    }, this);
    this.updateObject(this.activeIC.obj_info.id, false);
    if (save) this.app.session.save();
  }

  async updateObject(id: number, save: boolean = true) {
    let frozen;
    try {
      frozen = await this.ic10vm.freezeObject(id);
      this._objects.set(id, frozen);
    } catch (e) {
      this.handleVmError(e);
    }
    const device = this._objects.get(id);
    this.dispatchCustomEvent("vm-object-modified", device.obj_info.id);
    if (typeof device.obj_info.socketed_ic !== "undefined") {
      const ic = this._objects.get(device.obj_info.socketed_ic);
      const ip = ic.obj_info.circuit?.instruction_pointer;
      this.app.session.setActiveLine(device.obj_info.id, ip);
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
    this.dispatchCustomEvent("vm-message", message);
  }

  // return the data connected oject ids for a network
  networkDataDevices(network: ObjectID): number[] {
    return this._networks.get(network)?.devices ?? []
  }

  async changeObjectID(oldID: number, newID: number): Promise<boolean> {
    try {
      await this.ic10vm.changeDeviceId(oldID, newID);
      if (this.app.session.activeIC === oldID) {
        this.app.session.activeIC = newID;
      }
      await this.updateObjects();
      this.dispatchCustomEvent("vm-object-id-change", {
        old: oldID,
        new: newID,
      });
      this.app.session.changeID(oldID, newID);
      return true;
    } catch (err) {
      this.handleVmError(err);
      return false;
    }
  }

  async setRegister(index: number, val: number): Promise<boolean> {
    const ic = this.activeIC!;
    try {
      await this.ic10vm.setRegister(ic.obj_info.id, index, val);
      this.updateObject(ic.obj_info.id);
      return true;
    } catch (err) {
      this.handleVmError(err);
      return false;
    }
  }

  async setStack(addr: number, val: number): Promise<boolean> {
    const ic = this.activeIC!;
    try {
      await this.ic10vm.setMemory(ic.obj_info.id, addr, val);
      this.updateObject(ic.obj_info.id);
      return true;
    } catch (err) {
      this.handleVmError(err);
      return false;
    }
  }

  async setObjectName(id: number, name: string): Promise<boolean> {
    const obj = this._objects.get(id);
    if (obj) {
      try {
        await this.ic10vm.setObjectName(obj.obj_info.id, name);
        this.updateObject(obj.obj_info.id);
        this.app.session.save();
        return true;
      } catch (e) {
        this.handleVmError(e);
      }
    }
    return false;
  }

  async setObjectField(
    id: number,
    field: LogicType,
    val: number,
    force?: boolean,
  ): Promise<boolean> {
    force = force ?? false;
    const obj = this._objects.get(id);
    if (obj) {
      try {
        await this.ic10vm.setLogicField(obj.obj_info.id, field, val, force);
        this.updateObject(obj.obj_info.id);
        return true;
      } catch (err) {
        this.handleVmError(err);
      }
    }
    return false;
  }

  async setObjectSlotField(
    id: number,
    slot: number,
    field: LogicSlotType,
    val: number,
    force?: boolean,
  ): Promise<boolean> {
    force = force ?? false;
    const obj = this._objects.get(id);
    if (obj) {
      try {
        await this.ic10vm.setSlotLogicField(
          obj.obj_info.id,
          field,
          slot,
          val,
          force,
        );
        this.updateObject(obj.obj_info.id);
        return true;
      } catch (err) {
        this.handleVmError(err);
      }
    }
    return false;
  }

  async setDeviceConnection(
    id: number,
    conn: number,
    val: number | undefined,
  ): Promise<boolean> {
    const device = this._objects.get(id);
    if (typeof device !== "undefined") {
      try {
        await this.ic10vm.setDeviceConnection(id, conn, val);
        this.updateObject(device.obj_info.id);
        return true;
      } catch (err) {
        this.handleVmError(err);
      }
    }
    return false;
  }

  async setDevicePin(
    id: number,
    pin: number,
    val: number | undefined,
  ): Promise<boolean> {
    const device = this._objects.get(id);
    if (typeof device !== "undefined") {
      try {
        await this.ic10vm.setPin(id, pin, val);
        this.updateObject(device.obj_info.id);
        return true;
      } catch (err) {
        this.handleVmError(err);
      }
    }
    return false;
  }

  setupTemplateDatabase(db: TemplateDatabase) {
    this.templateDB = db;
    console.log("Loaded Template Database", this.templateDB);
    this.dispatchCustomEvent("vm-template-db-loaded", this.templateDB);
  }

  async addObjectFrozen(frozen: FrozenObject): Promise<ObjectID | undefined> {
    try {
      console.log("adding device", frozen);
      const id = await this.ic10vm.addObjectFrozen(frozen);
      const refrozen = await this.ic10vm.freezeObject(id);
      this._objects.set(id, refrozen);
      const device_ids = await this.ic10vm.objects;
      this.dispatchCustomEvent("vm-objects-update", Array.from(device_ids));
      this.app.session.save();
      return id;
    } catch (err) {
      this.handleVmError(err);
      return undefined;
    }
  }

  async addObjectsFrozen(frozenObjects: FrozenObject[]): Promise<ObjectID[] | undefined> {
    try {
      console.log("adding devices", frozenObjects);
      const ids = await this.ic10vm.addObjectsFrozen(frozenObjects);
      const refrozen = await this.ic10vm.freezeObjects(ids);
      ids.forEach((id, index) => {
        this._objects.set(id, refrozen[index]);
      })
      const device_ids = await this.ic10vm.objects;
      this.dispatchCustomEvent("vm-objects-update", Array.from(device_ids));
      this.app.session.save();
      return Array.from(ids);
    } catch (err) {
      this.handleVmError(err);
      return undefined;
    }
  }

  async removeDevice(id: number): Promise<boolean> {
    try {
      await this.ic10vm.removeDevice(id);
      await this.updateObjects();
      return true;
    } catch (err) {
      this.handleVmError(err);
      return false;
    }
  }

  async setSlotOccupant(
    id: number,
    index: number,
    frozen: FrozenObject,
    quantity: number,
  ): Promise<boolean> {
    const device = this._objects.get(id);
    if (typeof device !== "undefined") {
      try {
        console.log("setting slot occupant", frozen);
        await this.ic10vm.setSlotOccupant(id, index, frozen, quantity);
        this.updateObject(device.obj_info.id);
        return true;
      } catch (err) {
        this.handleVmError(err);
      }
    }
    return false;
  }

  async removeSlotOccupant(id: number, index: number): Promise<boolean> {
    const device = this._objects.get(id);
    if (typeof device !== "undefined") {
      try {
        this.ic10vm.removeSlotOccupant(id, index);
        this.updateObject(device.obj_info.id);
        return true;
      } catch (err) {
        this.handleVmError(err);
      }
    }
    return false;
  }

  async saveVMState(): Promise<FrozenVM> {
    return this.ic10vm.saveVMState();
  }

  async restoreVMState(state: FrozenVM) {
    try {
      await this.ic10vm.restoreVMState(state);
      this._objects = new Map();
      this._circuitHolders = new Map();
      await this.updateObjects();
    } catch (e) {
      this.handleVmError(e);
    }
  }

  getPrograms(): [number, string][] {
    const programs: [number, string][] = Array.from(
      this._circuitHolders.entries(),
    ).map(([id, ic]) => [id, ic.obj_info.source_code]);
    return programs;
  }
}

export { VirtualMachine };
