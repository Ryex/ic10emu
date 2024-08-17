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
  Class,
  LogicSlotType,
  SlotOccupantInfo,
  ICState,
  ObjectTemplate,
} from "ic10emu_wasm";
import { crc32, structuralEqual } from "utils";
import { LitElement, PropertyValueMap } from "lit";

import {
  computed,
} from '@lit-labs/preact-signals';
import type { Signal } from '@lit-labs/preact-signals';

export interface VmObjectSlotInfo {
  parent: ObjectID;
  index: number;
  name: string;
  typ: Class;
  logicFields: Map<LogicSlotType, LogicField>;
  quantity: number;
  occupant: ComputedObjectSignals | null;
}

export class ComputedObjectSignals {
  obj: Signal<FrozenObjectFull>;
  id: Signal<number>;
  template: Signal<ObjectTemplate>;

  name: Signal<string | null>;
  nameHash: Signal<number | null>;
  prefabName: Signal<string | null>;
  prefabHash: Signal<number | null>;
  displayName: Signal<string>;
  logicFields: Signal<Map<LogicType, LogicField> | null>;
  slots: Signal<VmObjectSlotInfo[] | null>;
  slotsCount: Signal<number | null>;
  reagents: Signal<Map<number, number> | null>;

  connections: Signal<Connection[] | null>;
  visibleDevices: Signal<ComputedObjectSignals[]>;

  memory: Signal<number[] | null>;
  icIP: Signal<number | null>;
  icOpCount: Signal<number | null>;
  icState: Signal<ICState | null>;
  errors: Signal<ICError[] | null>;
  registers: Signal<number[] | null>;
  aliases: Signal<Map<string, Operand> | null>;
  defines: Signal<Map<string, number> | null>;

  numPins: Signal<number | null>;
  pins: Signal<Map<number, ObjectID> | null>;


  constructor(obj: Signal<FrozenObjectFull>) {
    this.obj = obj
    this.id = computed(() => { return this.obj.value.obj_info.id; });

    this.template = computed(() => { return this.obj.value.template; });

    this.name = computed(() => { return this.obj.value.obj_info.name; });
    this.nameHash = computed(() => { return this.name.value !== "undefined" ? crc32(this.name.value) : null; });
    this.prefabName = computed(() => { return this.obj.value.obj_info.prefab; });
    this.prefabHash = computed(() => { return this.obj.value.obj_info.prefab_hash; });
    this.displayName = computed(() => { return this.obj.value.obj_info.name ?? this.obj.value.obj_info.prefab; });

    this.logicFields = computed(() => {
      const obj_info = this.obj.value.obj_info;
      const template = this.obj.value.template;

      const logicValues =
        obj_info.logic_values != null
          ? (new Map(Object.entries(obj_info.logic_values)) as Map<
            LogicType,
            number
          >)
          : null;
      const logicTemplate =
        "logic" in template ? template.logic : null;

      return new Map(
        Array.from(Object.entries(logicTemplate?.logic_types) ?? []).map(
          ([lt, access]) => {
            let field: LogicField = {
              field_type: access,
              value: logicValues.get(lt as LogicType) ?? 0,
            };
            return [lt as LogicType, field];
          },
        ),
      )
    });

    this.slots = computed(() => {
      const obj_info = this.obj.value.obj_info;
      const template = this.obj.value.template;

      const slotsOccupantInfo =
        obj_info.slots != null
          ? new Map(
            Object.entries(obj_info.slots).map(([key, val]) => [
              parseInt(key),
              val,
            ]),
          )
          : null;
      const slotsLogicValues =
        obj_info.slot_logic_values != null
          ? new Map<number, Map<LogicSlotType, number>>(
            Object.entries(obj_info.slot_logic_values).map(
              ([index, values]) => [
                parseInt(index),
                new Map(Object.entries(values)) as Map<
                  LogicSlotType,
                  number
                >,
              ],
            ),
          )
          : null;
      const logicTemplate =
        "logic" in template ? template.logic : null;
      const slotsTemplate =
        "slots" in template ? template.slots : [];

      return slotsTemplate.map((template, index) => {
        const fieldEntryInfos = Array.from(
          Object.entries(logicTemplate?.logic_slot_types[index]) ?? [],
        );
        const logicFields = new Map(
          fieldEntryInfos.map(([slt, access]) => {
            let field: LogicField = {
              field_type: access,
              value:
                slotsLogicValues.get(index)?.get(slt as LogicSlotType) ?? 0,
            };
            return [slt as LogicSlotType, field];
          }),
        );
        let occupantInfo = slotsOccupantInfo.get(index);
        let occupant =
          typeof occupantInfo !== "undefined"
            ? globalObjectSignalMap.get(occupantInfo.id) ?? null
            : null;
        let slot: VmObjectSlotInfo = {
          parent: obj_info.id,
          index: index,
          name: template.name,
          typ: template.typ,
          logicFields: logicFields,
          occupant: occupant,
          quantity: occupantInfo?.quantity ?? 0,
        };
        return slot;
      });
    });

    this.slotsCount = computed(() => {
      const slotsTemplate =
        "slots" in this.obj.value.template ? this.obj.value.template.slots : [];
      return slotsTemplate.length;
    });

    this.reagents = computed(() => {
      const reagents =
        this.obj.value.obj_info.reagents != null
          ? new Map(
            Object.entries(this.obj.value.obj_info.reagents).map(
              ([key, val]) => [parseInt(key), val],
            ),
          )
          : null;
      return reagents;
    });

    this.connections = computed(() => {
      const obj_info = this.obj.value.obj_info;
      const template = this.obj.value.template;

      const connectionsMap =
        obj_info.connections != null
          ? new Map(
            Object.entries(obj_info.connections).map(
              ([key, val]) => [parseInt(key), val],
            ),
          )
          : null;
      const connectionList =
        "device" in template
          ? template.device.connection_list
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
      return connections;
    });

    this.visibleDevices = computed(() => {
      return this.obj.value.obj_info.visible_devices.map((id) => globalObjectSignalMap.get(id))
    });

    this.memory = computed(() => {
      return this.obj.value.obj_info.memory ?? null;
    });

    this.icIP = computed(() => {
      return this.obj.value.obj_info.circuit?.instruction_pointer ?? null;
    });

    this.icOpCount = computed(() => {
      return this.obj.value.obj_info.circuit?.yield_instruction_count ?? null;
    });

    this.icState = computed(() => {
      return this.obj.value.obj_info.circuit?.state ?? null;
    });

    this.errors = computed(() => {
      return this.obj.value.obj_info.compile_errors ?? null;
    });

    this.registers = computed(() => {
      return this.obj.value.obj_info.circuit?.registers ?? null;
    });

    this.aliases = computed(() => {
      const aliases = this.obj.value.obj_info.circuit?.aliases ?? null;
      return aliases != null ? new Map(Object.entries(aliases)) : null;
    });

    this.defines = computed(() => {
      const defines = this.obj.value.obj_info.circuit?.defines ?? null;
      return defines != null ? new Map(Object.entries(defines)) : null;
    });

    this.pins = computed(() => {
      const pins = this.obj.value.obj_info.device_pins;
      return pins != null ? new Map(Object.entries(pins).map(([key, val]) => [parseInt(key), val])) : null;
    });

    this.numPins = computed(() => {
      return "device" in this.obj.value.template
        ? this.obj.value.template.device.device_pins_length
        : Math.max(...Array.from(this.pins.value?.keys() ?? [0]));
    });

  }
}

class ObjectComputedSignalMap extends Map {
  get(id: ObjectID): ComputedObjectSignals {
    if (!this.has(id)) {
      const obj = window.VM.vm.objects.get(id)
      if (typeof obj !== "undefined") {
        this.set(id, new ComputedObjectSignals(obj));
      }
    }
    return super.get(id);
  }
  set(id: ObjectID, value: ComputedObjectSignals): this {
    super.set(id, value);
    return this
  }
}

export const globalObjectSignalMap = new ObjectComputedSignalMap();

type Constructor<T = {}> = new (...args: any[]) => T;

export declare class VMObjectMixinInterface {
  objectID: ObjectID;
  activeICId: ObjectID;
  objectSignals: ComputedObjectSignals | null;
  _handleDeviceModified(e: CustomEvent): void;
  updateDevice(): void;
  subscribe(...sub: VMObjectMixinSubscription[]): void;
  unsubscribe(filter: (sub: VMObjectMixinSubscription) => boolean): void;
}

export type VMObjectMixinSubscription =
  | "active-ic"
  | "visible-devices";

export const VMObjectMixin = <T extends Constructor<LitElement>>(
  superClass: T,
) => {
  class VMObjectMixinClass extends superClass {
    private _objectID: number;
    get objectID() {
      return this._objectID;
    }
    @property({ type: Number })
    set objectID(val: number) {
      this._objectID = val;
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

    @state() objectSignals: ComputedObjectSignals | null;

    @state() activeICId: number;

    connectedCallback(): void {
      const root = super.connectedCallback();
      window.VM.get().then((vm) => {
        vm.addEventListener(
          "vm-object-modified",
          this._handleDeviceModified.bind(this),
        );
        vm.addEventListener(
          "vm-objects-update",
          this._handleDevicesModified.bind(this),
        );
        vm.addEventListener(
          "vm-object-id-change",
          this._handleDeviceIdChange.bind(this),
        );
        vm.addEventListener(
          "vm-objects-removed",
          this._handleDevicesRemoved.bind(this),
        );
      });
      this.updateDevice();
      return root;
    }

    disconnectedCallback(): void {
      window.VM.get().then((vm) => {
        vm.removeEventListener(
          "vm-object-modified",
          this._handleDeviceModified.bind(this),
        );
        vm.removeEventListener(
          "vm-objects-update",
          this._handleDevicesModified.bind(this),
        );
        vm.removeEventListener(
          "vm-object-id-change",
          this._handleDeviceIdChange.bind(this),
        );
        vm.removeEventListener(
          "vm-objects-removed",
          this._handleDevicesRemoved.bind(this),
        );
      });
    }

    async _handleDeviceModified(e: CustomEvent) {
      const id = e.detail;
      const activeIcId = window.App.app.session.activeIC;
      if (this.objectID === id) {
        this.updateDevice();
      } else if (
        id === activeIcId &&
        this.objectSubscriptions.includes("active-ic")
      ) {
        this.updateDevice();
        this.requestUpdate();
      } else if (this.objectSubscriptions.includes("visible-devices")) {
        const visibleDevices = await window.VM.vm.visibleDeviceIds(
          this.objectID,
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
      if (ids.includes(this.objectID)) {
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
          this.objectID,
        );
        if (ids.some((id) => visibleDevices.includes(id))) {
          this.updateDevice();
          this.requestUpdate();
        }
      }
    }

    async _handleDeviceIdChange(e: CustomEvent<{ old: number; new: number }>) {
      if (this.objectID === e.detail.old) {
        this.objectID = e.detail.new;
      } else if (this.objectSubscriptions.includes("visible-devices")) {
        const visibleDevices = await window.VM.vm.visibleDeviceIds(
          this.objectID,
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
      const newObjSignals = globalObjectSignalMap.get(this.objectID);
      if (newObjSignals !== this.objectSignals) {
        this.objectSignals = newObjSignals
      }

      if (typeof this.objectSignals === "undefined") {
        return;
      }

      // other updates needed

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
      }
      this.updateDevice();
    }
  }

  return VMActiveICMixinClass as Constructor<VMObjectMixinInterface> & T;
};

export declare class VMTemplateDBMixinInterface {
  templateDB: TemplateDatabase;
  _handleDeviceDBLoad(e: CustomEvent): void;
  postDBSetUpdate(): void;
}

export const VMTemplateDBMixin = <T extends Constructor<LitElement>>(
  superClass: T,
) => {
  class VMTemplateDBMixinClass extends superClass {
    connectedCallback(): void {
      const root = super.connectedCallback();
      window.VM.vm.addEventListener(
        "vm-template-db-loaded",
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

  return VMTemplateDBMixinClass as Constructor<VMTemplateDBMixinInterface> & T;
};
