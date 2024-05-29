import { property, state } from "lit/decorators.js";

import type {
  Slot,
  Connection,
  ICError,
  LogicType,
  LogicField,
  Operand,
  ObjectID,
  TemplateDatabase,
  FrozenObjectFull,
} from "ic10emu_wasm";
import { crc32, structuralEqual } from "utils";
import { LitElement, PropertyValueMap } from "lit";

type Constructor<T = {}> = new (...args: any[]) => T;

export declare class VMObjectMixinInterface {
  objectID: ObjectID;
  activeICId: ObjectID;
  obj: FrozenObjectFull;
  name: string | null;
  nameHash: number | null;
  prefabName: string | null;
  prefabHash: number | null;
  fields: Map<LogicType, LogicField>;
  slots: Slot[];
  reagents: Map<number, number>;
  connections: Connection[];
  icIP: number;
  icOpCount: number;
  icState: string;
  errors: ICError[];
  registers: number[] | null;
  memory: number[] | null;
  aliases: Map<string, Operand> | null;
  defines: Map<string, string> | null;
  numPins: number | null;
  pins: Map<number, ObjectID> | null;
  _handleDeviceModified(e: CustomEvent): void;
  updateDevice(): void;
  updateIC(): void;
  subscribe(...sub: VMObjectMixinSubscription[]): void;
  unsubscribe(filter: (sub: VMObjectMixinSubscription) => boolean): void;
}

export type VMObjectMixinSubscription =
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
  | { slot: number }
  | "visible-devices";

export const VMObjectMixin = <T extends Constructor<LitElement>>(
  superClass: T,
) => {
  class VMObjectMixinClass extends superClass {
    private _deviceID: number;
    get deviceID() {
      return this._deviceID;
    }
    @property({ type: Number })
    set deviceID(val: number) {
      this._deviceID = val;
      this.updateDevice();
    }

    @state() private objectSubscriptions: VMObjectMixinSubscription[] = [];

    subscribe(...sub: VMObjectMixinSubscription[]) {
      this.objectSubscriptions = this.objectSubscriptions.concat(sub);
    }

    // remove subscripotions matching the filter
    unsubscribe(filter: (sub: VMObjectMixinSubscription) => boolean) {
      this.objectSubscriptions = this.objectSubscriptions.filter(
        (sub) => !filter(sub),
      );
    }

    obj: FrozenObjectFull;

    @state() activeICId: number;

    @state() name: string | null = null;
    @state() nameHash: number | null = null;
    @state() prefabName: string | null;
    @state() prefabHash: number | null;
    @state() fields: Map<LogicType, LogicField>;
    @state() slots: Slot[];
    @state() reagents: Map<number, number>;
    @state() connections: Connection[];
    @state() icIP: number;
    @state() icOpCount: number;
    @state() icState: string;
    @state() errors: ICError[];
    @state() registers: number[] | null;
    @state() memory: number[] | null;
    @state() aliases: Map<string, Operand> | null;
    @state() defines: Map<string, string> | null;
    @state() numPins: number | null;
    @state() pins: Map<number, ObjectID> | null;

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
        vm.addEventListener(
          "vm-devices-removed",
          this._handleDevicesRemoved.bind(this),
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
        vm.removeEventListener(
          "vm-devices-removed",
          this._handleDevicesRemoved.bind(this),
        );
      });
    }

    async _handleDeviceModified(e: CustomEvent) {
      const id = e.detail;
      const activeIcId = window.App.app.session.activeIC;
      if (this.deviceID === id) {
        this.updateDevice();
      } else if (
        id === activeIcId &&
        this.objectSubscriptions.includes("active-ic")
      ) {
        this.updateDevice();
        this.requestUpdate();
      } else if (this.objectSubscriptions.includes("visible-devices")) {
        const visibleDevices = await window.VM.vm.visibleDeviceIds(
          this.deviceID,
        );
        if (visibleDevices.includes(id)) {
          this.updateDevice();
          this.requestUpdate();
        }
      }
    }

    async _handleDevicesModified(e: CustomEvent<number[]>) {
      const activeIcId = window.App.app.session.activeIC;
      const ids = e.detail;
      if (ids.includes(this.deviceID)) {
        this.updateDevice();
        if (this.objectSubscriptions.includes("visible-devices")) {
          this.requestUpdate();
        }
      } else if (
        ids.includes(activeIcId) &&
        this.objectSubscriptions.includes("active-ic")
      ) {
        this.updateDevice();
        this.requestUpdate();
      } else if (this.objectSubscriptions.includes("visible-devices")) {
        const visibleDevices = await window.VM.vm.visibleDeviceIds(
          this.deviceID,
        );
        if (ids.some((id) => visibleDevices.includes(id))) {
          this.updateDevice();
          this.requestUpdate();
        }
      }
    }

    async _handleDeviceIdChange(e: CustomEvent<{ old: number; new: number }>) {
      if (this.deviceID === e.detail.old) {
        this.deviceID = e.detail.new;
      } else if (this.objectSubscriptions.includes("visible-devices")) {
        const visibleDevices = await window.VM.vm.visibleDeviceIds(
          this.deviceID,
        );
        if (
          visibleDevices.some(
            (id) => id === e.detail.old || id === e.detail.new,
          )
        ) {
          this.requestUpdate();
        }
      }
    }

    _handleDevicesRemoved(e: CustomEvent<number[]>) {
      const _ids = e.detail;
      if (this.objectSubscriptions.includes("visible-devices")) {
        this.requestUpdate();
      }
    }

    updateDevice() {
      this.obj = window.VM.vm.objects.get(this.deviceID)!;

      if (typeof this.obj === "undefined") {
        return;
      }

      if (
        this.objectSubscriptions.includes("slots") ||
        this.objectSubscriptions.includes("slots-count")
      ) {
        const slotsOccupantInfo = this.obj.obj_info.slots;
        const logicTemplate =
          "logic" in this.obj.template ? this.obj.template.logic : null;
        const slotsTemplate =
          "slots" in this.obj.template ? this.obj.template.slots : [];
        let slots: Slot[] | null = null;
        if (slotsOccupantInfo.size !== 0) {
          slots = slotsTemplate.map((template, index) => {
            let slot = {
              parent: this.obj.obj_info.id,
              index: index,
              name: template.name,
              typ: template.typ,
              readable_logic: Array.from(
                logicTemplate?.logic_slot_types.get(index)?.entries() ?? [],
              )
                .filter(([_, val]) => val === "Read" || val === "ReadWrite")
                .map(([key, _]) => key),
              writeable_logic: Array.from(
                logicTemplate?.logic_slot_types.get(index)?.entries() ?? [],
              )
                .filter(([_, val]) => val === "Write" || val === "ReadWrite")
                .map(([key, _]) => key),
              occupant: slotsOccupantInfo.get(index),
            };
            return slot;
          });
        }

        if (!structuralEqual(this.slots, slots)) {
          this.slots = slots;
        }
      }

      for (const sub of this.objectSubscriptions) {
        if (typeof sub === "string") {
          if (sub == "name") {
            const name = this.obj.obj_info.name ?? null;
            if (this.name !== name) {
              this.name = name;
            }
          } else if (sub === "nameHash") {
            const nameHash =
              typeof this.obj.obj_info.name !== "undefined"
                ? crc32(this.obj.obj_info.name)
                : null;
            if (this.nameHash !== nameHash) {
              this.nameHash = nameHash;
            }
          } else if (sub === "prefabName") {
            const prefabName = this.obj.obj_info.prefab ?? null;
            if (this.prefabName !== prefabName) {
              this.prefabName = prefabName;
              this.prefabHash = crc32(prefabName);
            }
          } else if (sub === "fields") {
            const fields = this.obj.obj_info.logic_values ?? null;
            const logicTemplate =
              "logic" in this.obj.template ? this.obj.template.logic : null;
            let logic_fields: Map<LogicType, LogicField> | null = null;
            if (fields !== null) {
              logic_fields = new Map();
              for (const [lt, val] of fields) {
                const access = logicTemplate?.logic_types.get(lt) ?? "Read";
                logic_fields.set(lt, {
                  value: val,
                  field_type: access,
                });
              }
            }
            if (!structuralEqual(this.fields, logic_fields)) {
              this.fields = logic_fields;
            }
          } else if (sub === "reagents") {
            const reagents = this.obj.obj_info.reagents;
            if (!structuralEqual(this.reagents, reagents)) {
              this.reagents = reagents;
            }
          } else if (sub === "connections") {
            const connectionsMap = this.obj.obj_info.connections ?? new Map();
            const connectionList =
              "device" in this.obj.template
                ? this.obj.template.device.connection_list
                : [];
            let connections: Connection[] | null = null;
            if (connectionList.length !== 0) {
              connections = connectionList.map((conn, index) => {
                if (conn.typ === "Data") {
                  return {
                    CableNetwork: {
                      typ: "Data",
                      role: conn.role,
                      net: connectionsMap.get(index),
                    },
                  };
                } else if (conn.typ === "Power") {
                  return {
                    CableNetwork: {
                      typ: "Power",
                      role: conn.role,
                      net: connectionsMap.get(index),
                    },
                  };
                } else if (conn.typ === "PowerAndData") {
                  return {
                    CableNetwork: {
                      typ: "Data",
                      role: conn.role,
                      net: connectionsMap.get(index),
                    },
                  };
                } else if (conn.typ === "Pipe") {
                  return { Pipe: { role: conn.role } };
                } else if (conn.typ === "Chute") {
                  return { Chute: { role: conn.role } };
                } else if (conn.typ === "Elevator") {
                  return { Elevator: { role: conn.role } };
                } else if (conn.typ === "LaunchPad") {
                  return { LaunchPad: { role: conn.role } };
                } else if (conn.typ === "LandingPad") {
                  return { LandingPad: { role: conn.role } };
                } else if (conn.typ === "PipeLiquid") {
                  return { PipeLiquid: { role: conn.role } };
                }
                return "None";
              });
            }
            if (!structuralEqual(this.connections, connections)) {
              this.connections = connections;
            }
          } else if (sub === "ic") {
            if (
              typeof this.obj.obj_info.circuit !== "undefined" ||
              this.obj.obj_info.socketed_ic !== "undefined"
            ) {
              this.updateIC();
            }
          } else if (sub === "active-ic") {
            const activeIc = window.VM.vm?.activeIC;
            if (this.activeICId !== activeIc.obj_info.id) {
              this.activeICId = activeIc.obj_info.id;
            }
          }
        } else {
          if ("field" in sub) {
            const fields = this.obj.obj_info.logic_values;
            if (this.fields.get(sub.field) !== fields.get(sub.field)) {
              this.fields = fields;
            }
          } else if ("slot" in sub) {
            const slots = this.obj.slots;
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
      const ip = this.obj.ip!;
      if (this.icIP !== ip) {
        this.icIP = ip;
      }
      const opCount = this.obj.instructionCount!;
      if (this.icOpCount !== opCount) {
        this.icOpCount = opCount;
      }
      const state = this.obj.state!;
      if (this.icState !== state) {
        this.icState = state;
      }
      const errors = this.obj.program?.errors ?? null;
      if (!structuralEqual(this.errors, errors)) {
        this.errors = errors;
      }
      const registers = this.obj.registers ?? null;
      if (!structuralEqual(this.registers, registers)) {
        this.registers = registers;
      }
      const stack = this.obj.stack ?? null;
      if (!structuralEqual(this.memory, stack)) {
        this.memory = stack;
      }
      const aliases = this.obj.aliases ?? null;
      if (!structuralEqual(this.aliases, aliases)) {
        this.aliases = aliases;
      }
      const defines = this.obj.defines ?? null;
      if (!structuralEqual(this.defines, defines)) {
        this.defines = defines;
      }
      const pins = this.obj.pins ?? null;
      if (!structuralEqual(this.pins, pins)) {
        this.pins = pins;
      }
    }
  }
  return VMObjectMixinClass as Constructor<VMObjectMixinInterface> & T;
};

export const VMActiveICMixin = <T extends Constructor<LitElement>>(
  superClass: T,
) => {
  class VMActiveICMixinClass extends VMObjectMixin(superClass) {
    constructor() {
      super();
      this.objectID = window.App.app.session.activeIC;
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
      if (this.objectID !== id) {
        this.objectID = id;
        this.obj = window.VM.vm.objects.get(this.objectID)!;
      }
      this.updateDevice();
    }
  }

  return VMActiveICMixinClass as Constructor<VMObjectMixinInterface> & T;
};

export declare class VMDeviceDBMixinInterface {
  templateDB: TemplateDatabase;
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
      if (typeof window.VM.vm.templateDB !== "undefined") {
        this.templateDB = window.VM.vm.templateDB!;
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
      this.templateDB = e.detail;
    }

    private _templateDB: TemplateDatabase;

    get templateDB(): TemplateDatabase {
      return this._templateDB;
    }

    postDBSetUpdate(): void { }

    @state()
    set templateDB(val: TemplateDatabase) {
      this._templateDB = val;
      this.postDBSetUpdate();
    }
  }

  return VMDeviceDBMixinClass as Constructor<VMDeviceDBMixinInterface> & T;
};
