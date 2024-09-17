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
import { comlinkSpecialJsonTransferHandler, structuralEqual, TypedEventTarget } from "utils";
export interface ToastMessage {
  variant: "warning" | "danger" | "success" | "primary" | "neutral";
  icon: string;
  title: string;
  msg: string;
  id: string;
}
import {
  computed,
} from '@lit-labs/preact-signals';
import { getJsonContext } from "./jsonErrorUtils";
import { VMState } from "./state";

import * as log from "log";

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

Comlink.transferHandlers.set("SpecialJson", comlinkSpecialJsonTransferHandler);

const jsonErrorRegex = /((invalid type: .*)|(missing field .*)) at line (?<errorLine>\d+) column (?<errorColumn>\d+)/;

class VirtualMachine extends TypedEventTarget<VirtualMachineEventMap, typeof EventTarget>(EventTarget) {
  ic10vm: Comlink.Remote<VMRef>;
  templateDBPromise: Promise<TemplateDatabase>;

  state: VMState = new VMState();

  private vmWorker: Worker;
  private app: App;

  constructor(app: App) {
    super();
    this.app = app;
    this.setupVM();
    this.app.session.vmState.value = this.state;
  }

  async setupVM() {

    this.vmWorker = new Worker(new URL("./vmWorker.ts", import.meta.url), { name: "ic10emu-Worker"});
    const loaded = (w: Worker) =>
      new Promise((r) => w.addEventListener("message", r, { once: true }));
    await Promise.all([loaded(this.vmWorker)]);
    log.info("VM Worker loaded");
    const vm = Comlink.wrap<VMRef>(this.vmWorker);
    this.ic10vm = vm;
    this.state.vm.value = await this.ic10vm.saveVMState();
    this.templateDBPromise = this.ic10vm.getTemplateDatabase();
    this.templateDBPromise.then((db) => this.setupTemplateDatabase(db));

    window.VM.set(this);
  }

  get activeIC() {
    return computed(() => this.app.session.activeIC.value);
  }

  async visibleDeviceIds(source: ObjectID): Promise<ObjectID[]> {
    const visDevices = await this.ic10vm.visibleDevices(source);
    const ids = Array.from(visDevices);
    ids.sort();
    return ids;
  }

  async step(ignoreError: boolean = false) {
    const ic = this.activeIC.peek();
    if (ic) {
      try {
        await this.ic10vm.stepProgrammable(ic, ignoreError);
      } catch (err) {
        this.handleVmError(err);
      }
      this.update();
      this.dispatchCustomEvent("vm-run-ic", ic);
    }
  }

  async run() {
    const ic = this.activeIC.peek();
    if (ic) {
      try {
        await this.ic10vm.runProgrammable(ic, false);
      } catch (err) {
        this.handleVmError(err);
      }
      this.update();
      this.dispatchCustomEvent("vm-run-ic", this.activeIC.peek());
    }
  }

  async reset() {
    const ic = this.activeIC.peek();
    if (ic) {
      await this.ic10vm.resetProgrammable(ic);
      await this.update();
    }
  }

  async update(save: boolean = true) {
    try {
      const newState = await this.ic10vm.saveVMState();
      this.state.vm.value = newState;
      if (save) this.app.session.save();
    } catch (err) {
      this.handleVmError(err);
    }
  }

  handleVmError(err: Error, args: { context?: string, jsonContext?: string, trace?: boolean } = {}) {
    const message = args.context ? `Error in Virtual Machine {${args.context}}` : "Error in Virtual Machine";
    log.error(message, err);
    if (args.jsonContext != null) {
      const jsonTypeError = err.message.match(jsonErrorRegex)
      if (jsonTypeError) {
        log.debug(
          "Json Error context",
          getJsonContext(
            parseInt(jsonTypeError.groups["errorLine"]),
            parseInt(jsonTypeError.groups["errorColumn"]),
            args.jsonContext,
            100
          )
        )
      }
    }
    if (args.trace) {
      console.trace();
    }
    const toastMessage: ToastMessage = {
      variant: "danger",
      icon: "bug",
      title: `Error in Virtual Machine ${err.name}`,
      msg: err.message,
      id: Date.now().toString(16),
    };
    this.dispatchCustomEvent("vm-message", toastMessage);
  }

  async changeObjectID(oldID: number, newID: number): Promise<boolean> {
    try {
      await this.ic10vm.changeDeviceId(oldID, newID);
    } catch (err) {
      this.handleVmError(err);
      return false;
    }
    if (this.app.session.activeIC.peek() === oldID) {
      this.app.session.activeIC = newID;
    }
    await this.update();
    this.dispatchCustomEvent("vm-object-id-change", {
      old: oldID,
      new: newID,
    });
    return true;
  }

  async setCode(id: ObjectID, prog: string): Promise<boolean> {
    const attempt = Date.now().toString(16);
    const vmProg = this.state.getObjectProgramSource(id).peek();
    if (
      vmProg &&
      prog &&
      vmProg !== prog
    ) {
      try {
        console.time(`CompileProgram_${id}_${attempt}`);
        await this.ic10vm.setCodeInvalid(id, prog);
      } catch (err) {
        this.handleVmError(err);
        return false;
      } finally {
        console.timeEnd(`CompileProgram_${id}_${attempt}`);
      }
      await this.update();
    }
    return true;
  }

  async setRegister(id: ObjectID, index: number, val: number): Promise<boolean> {
    try {
      await this.ic10vm.setRegister(id, index, val);
    } catch (err) {
      this.handleVmError(err);
      return false;
    }
    await this.update();
    return true;
  }

  async setStack(id: ObjectID, addr: number, val: number): Promise<boolean> {
    try {
      await this.ic10vm.setMemory(id, addr, val);
    } catch (err) {
      this.handleVmError(err);
      return false;
    }
    await this.update();
    return true;
  }

  async setObjectName(id: number, name: string): Promise<boolean> {
    try {
      await this.ic10vm.setObjectName(id, name);
    } catch (e) {
      this.handleVmError(e);
      return false;
    }
    await this.update();
    return true;
  }

  async setObjectField(
    id: number,
    field: LogicType,
    val: number,
    force?: boolean,
  ): Promise<boolean> {
    force = force ?? false;
    try {
      await this.ic10vm.setLogicField(id, field, val, force);
    } catch (err) {
      this.handleVmError(err);
      return false;
    }
    await this.update();
    return true;
  }

  async setObjectSlotField(
    id: number,
    slot: number,
    field: LogicSlotType,
    val: number,
    force?: boolean,
  ): Promise<boolean> {
    force = force ?? false;
    try {
      await this.ic10vm.setSlotLogicField(
        id,
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

  async setDeviceConnection(
    id: number,
    conn: number,
    val: number | undefined,
  ): Promise<boolean> {
    try {
      await this.ic10vm.setDeviceConnection(id, conn, val);
    } catch (err) {
      this.handleVmError(err);
      return false;
    }
    await this.update();
    return true;
  }

  async setDevicePin(
    id: number,
    pin: number,
    val: number | undefined,
  ): Promise<boolean> {
    try {
      await this.ic10vm.setPin(id, pin, val);
    } catch (err) {
      this.handleVmError(err);
      return false;
    }
    await this.update();
    return true;
  }

  setupTemplateDatabase(db: TemplateDatabase) {
    this.state.templateDB.value = db;
    log.debug("Loaded Template Database", this.state.templateDB.value);
    this.dispatchCustomEvent("vm-template-db-loaded", this.state.templateDB.value);
  }

  async addObjectFrozen(frozen: FrozenObject): Promise<ObjectID | undefined> {
    let id = undefined;
    try {
      log.trace("adding device", frozen);
      id = await this.ic10vm.addObjectFrozen(frozen);
    } catch (err) {
      this.handleVmError(err);
      return undefined;
    }
    await this.update();
    return id;
  }

  async addObjectsFrozen(
    frozenObjects: FrozenObject[],
  ): Promise<ObjectID[] | undefined> {
    let ids = undefined;
    try {
      log.trace("adding devices", frozenObjects);
      ids = await this.ic10vm.addObjectsFrozen(frozenObjects);
    } catch (err) {
      this.handleVmError(err);
      return undefined;
    }
    await this.update();
    return Array.from(ids ?? []);
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
    try {
      log.trace("setting slot occupant", frozen);
      await this.ic10vm.setSlotOccupant(id, index, frozen, quantity);
    } catch (err) {
      this.handleVmError(err);
      return false;
    }
    await this.update();
    return true;
  }

  async removeSlotOccupant(id: number, index: number): Promise<boolean> {
    try {
      await this.ic10vm.removeSlotOccupant(id, index);
    } catch (err) {
      this.handleVmError(err);
      return false;
    }
    await this.update();
    return true;
  }

  async saveVMState(): Promise<FrozenVM> {
    return await this.ic10vm.saveVMState();
  }

  async restoreVMState(state: FrozenVM) {
    try {
      log.info("Restoring VM State from", state);
      await this.ic10vm.restoreVMState(state);
    } catch (e) {
      this.handleVmError(e, { jsonContext: JSON.stringify(state) });
      return;
    }
    // TODO: Cleanup old state
    await this.update();
  }

  getPrograms(): [number, string][] {
    const programs: [number, string][] = this.state.circuitHolderIds.value.map((id) => [id, this.state.getObjectProgramSource(id).value]);
    return programs;
  }
}

export { VirtualMachine };
