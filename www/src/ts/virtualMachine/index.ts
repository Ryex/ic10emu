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
import {
  signal,
  computed,
  effect,
} from '@lit-labs/preact-signals';
import type { Signal } from '@lit-labs/preact-signals';

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

  private _vmState: Signal<FrozenVM>;

  private _objects: Map<number, Signal<FrozenObjectFull>>;
  private _circuitHolders: Map<number, Signal<FrozenObjectFull>>;
  private _networks: Map<number, Signal<FrozenCableNetwork>>;
  private _default_network: Signal<number>;

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
    this.templateDBPromise = this.ic10vm.getTemplateDatabase();
    this.templateDBPromise.then((db) => this.setupTemplateDatabase(db));

    this.vm_worker = new Worker(new URL("./vmWorker.ts", import.meta.url));
    const loaded = (w: Worker) =>
      new Promise((r) => w.addEventListener("message", r, { once: true }));
    await Promise.all([loaded(this.vm_worker)]);
    console.info("VM Worker loaded");
    const vm = Comlink.wrap<VMRef>(this.vm_worker);
    this.ic10vm = vm;
    this._vmState.value = await this.ic10vm.saveVMState();
    window.VM.set(this);

    effect(() => {
      this.updateObjects(this._vmState.value);
      this.updateNetworks(this._vmState.value);
    });

    this.updateCode();
  }

  get state() {
    return this._vmState;
  }

  get objects() {
    return this._objects;
  }

  get objectIds(): ObjectID[] {
    const ids = Array.from(this._objects.keys());
    ids.sort();
    return ids;
  }

  get circuitHolders() {
    return this._circuitHolders;
  }

  get circuitHolderIds(): ObjectID[] {
    const ids = Array.from(this._circuitHolders.keys());
    ids.sort();
    return ids;
  }

  get networks(): ObjectID[] {
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

  async visibleDevices(source: number): Promise<Signal<FrozenObjectFull>[]> {
    try {
      const visDevices = await this.ic10vm.visibleDevices(source);
      const ids = Array.from(visDevices);
      ids.sort();
      return ids.map((id, _index) => this._objects.get(id)!);
    } catch (err) {
      this.handleVmError(err);
    }
  }

  async visibleDeviceIds(source: number): Promise<number[]> {
    const visDevices = await this.ic10vm.visibleDevices(source);
    const ids = Array.from(visDevices);
    ids.sort();
    return ids;
  }

  async updateNetworks(state: FrozenVM) {
    let updateFlag = false;
    const removedNetworks = [];
    const networkIds: ObjectID[] = [];
    const frozenNetworks: FrozenCableNetwork[] = state.networks;
    const updatedNetworks: ObjectID[] = [];

    for (const [index, net] of frozenNetworks.entries()) {
      const id = net.id;
      networkIds.push(id);
      if (!this._networks.has(id)) {
        this._networks.set(id, signal(net));
        updateFlag = true;
        updatedNetworks.push(id);
      } else {
        const mappedNet = this._networks.get(id);
        if (!structuralEqual(mappedNet.peek(), net)) {
          mappedNet.value = net;
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

  async updateObjects(state: FrozenVM) {
    const removedObjects = [];
    const frozenObjects = state.objects;
    const objectIds: ObjectID[] = [];
    const updatedObjects: ObjectID[] = [];
    let updateFlag = false;

    for (const [index, obj] of frozenObjects.entries()) {
      const id = obj.obj_info.id;
      objectIds.push(id);
      if (!this._objects.has(id)) {
        this._objects.set(id, signal(obj));
        updateFlag = true;
        updatedObjects.push(id);
      } else {
        const mappedObject = this._objects.get(id);
        if (!structuralEqual(obj, mappedObject.peek())) {
          mappedObject.value = obj;
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
      if (typeof obj.peek().obj_info.socketed_ic !== "undefined") {
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
        circuitHolder.peek().obj_info.source_code !== prog
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
        await this.ic10vm.stepProgrammable(ic.peek().obj_info.id, false);
      } catch (err) {
        this.handleVmError(err);
      }
      this.update();
      this.dispatchCustomEvent("vm-run-ic", this.activeIC!.peek().obj_info.id);
    }
  }

  async run() {
    const ic = this.activeIC;
    if (ic) {
      try {
        await this.ic10vm.runProgrammable(ic.peek().obj_info.id, false);
      } catch (err) {
        this.handleVmError(err);
      }
      this.update();
      this.dispatchCustomEvent("vm-run-ic", this.activeIC!.peek().obj_info.id);
    }
  }

  async reset() {
    const ic = this.activeIC;
    if (ic) {
      await this.ic10vm.resetProgrammable(ic.peek().obj_info.id);
      await this.update();
    }
  }

  async update(save: boolean = true) {
    try {
      this._vmState.value = await this.ic10vm.saveVMState();
      if (save) this.app.session.save();
    } catch (err) {
      this.handleVmError(err);
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
    this.dispatchCustomEvent("vm-message", message);
  }

  // return the data connected oject ids for a network
  networkDataDevices(network: ObjectID): number[] {
    return this._networks.get(network)?.peek().devices ?? [];
  }

  async changeObjectID(oldID: number, newID: number): Promise<boolean> {
    try {
      await this.ic10vm.changeDeviceId(oldID, newID);
      if (this.app.session.activeIC === oldID) {
        this.app.session.activeIC = newID;
      }
      await this.update();
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
    if (ic) {
      try {
        await this.ic10vm.setRegister(ic.peek().obj_info.id, index, val);
      } catch (err) {
        this.handleVmError(err);
        return false;
      }
      await this.update();
      return true;
    }
  }

  async setStack(addr: number, val: number): Promise<boolean> {
    const ic = this.activeIC!;
    if (ic) {
      try {
        await this.ic10vm.setMemory(ic.peek().obj_info.id, addr, val);
      } catch (err) {
        this.handleVmError(err);
        return false;
      }
      await this.update();
      return true;
    }
  }

  async setObjectName(id: number, name: string): Promise<boolean> {
    const obj = this._objects.get(id);
    if (obj) {
      try {
        await this.ic10vm.setObjectName(obj.peek().obj_info.id, name);
      } catch (e) {
        this.handleVmError(e);
        return false;
      }
      await this.update();
      return true;
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
        await this.ic10vm.setLogicField(obj.peek().obj_info.id, field, val, force);
      } catch (err) {
        this.handleVmError(err);
        return false;
      }
      await this.update();
      return true;
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
          obj.peek().obj_info.id,
          field,
          slot,
          val,
          force,
        );
      } catch (err) {
        this.handleVmError(err);
        return false;
      }
      await this.update();
      return true;
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
      } catch (err) {
        this.handleVmError(err);
        return false;
      }
      await this.update();
      return true;
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
      } catch (err) {
        this.handleVmError(err);
        return false;
      }
      await this.update();
      return true;
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
      await this.update();
      return id;
    } catch (err) {
      this.handleVmError(err);
      return undefined;
    }
  }

  async addObjectsFrozen(
    frozenObjects: FrozenObject[],
  ): Promise<ObjectID[] | undefined> {
    try {
      console.log("adding devices", frozenObjects);
      const ids = await this.ic10vm.addObjectsFrozen(frozenObjects);
      await this.update();
      return Array.from(ids);
    } catch (err) {
      this.handleVmError(err);
      return undefined;
    }
  }

  async removeDevice(id: number): Promise<boolean> {
    try {
      await this.ic10vm.removeDevice(id);
    } catch (err) {
      this.handleVmError(err);
      return false;
    }
    await this.update();
    return true;
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
        await this.update();
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
        await this.ic10vm.removeSlotOccupant(id, index);
        await this.update();
        return true;
      } catch (err) {
        this.handleVmError(err);
      }
    }
    return false;
  }

  async saveVMState(): Promise<FrozenVM> {
    return await this.ic10vm.saveVMState();
  }

  async restoreVMState(state: FrozenVM) {
    try {
      await this.ic10vm.restoreVMState(state);
      this._objects = new Map();
      this._circuitHolders = new Map();
      await this.update();
    } catch (e) {
      this.handleVmError(e);
    }
  }

  getPrograms(): [number, string][] {
    const programs: [number, string][] = Array.from(
      this._circuitHolders.entries(),
    ).map(([id, ic]) => [id, ic.peek().obj_info.source_code]);
    return programs;
  }
}

export { VirtualMachine };
