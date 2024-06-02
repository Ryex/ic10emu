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
  logicFields: Map<LogicType, LogicField> | null;
  slots: VmObjectSlotInfo[] | null;
  slotsCount: number | null;
  reagents: Map<number, number> | null;
  connections: Connection[] | null;
  icIP: number | null;
  icOpCount: number | null;
  icState: string | null;
  errors: ICError[] | null;
  registers: number[] | null;
  memory: number[] | null;
  aliases: Map<string, Operand> | null;
  defines: Map<string, number> | null;
  numPins: number | null;
  pins: Map<number, ObjectID> | null;
  visibleDevices: ObjectID[] | null;
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
  | "memory"
  | "ic"
  | "active-ic"
  | { field: LogicType }
  | { slot: number }
  | "visible-devices";

export interface VmObjectSlotInfo {
  parent: ObjectID;
  index: number;
  name: string;
  typ: Class;
  logicFields: Map<LogicSlotType, LogicField>;
  quantity: number;
  occupant: FrozenObjectFull | undefined;
}

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

    obj: FrozenObjectFull;

    @state() activeICId: number;

    @state() name: string | null = null;
    @state() nameHash: number | null = null;
    @state() prefabName: string | null = null;
    @state() prefabHash: number | null = null;
    @state() logicFields: Map<LogicType, LogicField> | null = null;
    @state() slots: VmObjectSlotInfo[] | null = null;
    @state() slotsCount: number | null = null;
    @state() reagents: Map<number, number> | null = null;
    @state() connections: Connection[] | null = null;
    @state() icIP: number | null = null;
    @state() icOpCount: number | null = null;
    @state() icState: ICState | null = null;
    @state() errors: ICError[] | null = null;
    @state() registers: number[] | null = null;
    @state() memory: number[] | null = null;
    @state() aliases: Map<string, Operand> | null = null;
    @state() defines: Map<string, number> | null = null;
    @state() numPins: number | null = null;
    @state() pins: Map<number, ObjectID> | null = null;
    @state() visibleDevices: ObjectID[] | null = null;

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
      this.obj = window.VM.vm.objects.get(this.objectID)!;

      if (typeof this.obj === "undefined") {
        return;
      }

      let newFields: Map<LogicType, LogicField> | null = null;
      if (
        this.objectSubscriptions.some(
          (sub) =>
            sub === "fields" || (typeof sub === "object" && "field" in sub),
        )
      ) {
        const logicValues =
          this.obj.obj_info.logic_values != null
            ? (new Map(Object.entries(this.obj.obj_info.logic_values)) as Map<
                LogicType,
                number
              >)
            : null;
        const logicTemplate =
          "logic" in this.obj.template ? this.obj.template.logic : null;
        newFields = new Map(
          Array.from(Object.entries(logicTemplate?.logic_types) ?? []).map(
            ([lt, access]) => {
              let field: LogicField = {
                field_type: access,
                value: logicValues.get(lt as LogicType) ?? 0,
              };
              return [lt as LogicType, field];
            },
          ),
        );
      }

      const visibleDevices = this.obj.obj_info.visible_devices ?? [];
      if (!structuralEqual(this.visibleDevices, visibleDevices)) {
        this.visibleDevices = visibleDevices;
      }

      let newSlots: VmObjectSlotInfo[] | null = null;
      if (
        this.objectSubscriptions.some(
          (sub) =>
            sub === "slots" || (typeof sub === "object" && "slot" in sub),
        )
      ) {
        const slotsOccupantInfo =
          this.obj.obj_info.slots != null
            ? new Map(
                Object.entries(this.obj.obj_info.slots).map(([key, val]) => [
                  parseInt(key),
                  val,
                ]),
              )
            : null;
        const slotsLogicValues =
          this.obj.obj_info.slot_logic_values != null
            ? new Map<number, Map<LogicSlotType, number>>(
                Object.entries(this.obj.obj_info.slot_logic_values).map(
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
          "logic" in this.obj.template ? this.obj.template.logic : null;
        const slotsTemplate =
          "slots" in this.obj.template ? this.obj.template.slots : [];
        newSlots = slotsTemplate.map((template, index) => {
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
              ? window.VM.vm.objects.get(occupantInfo.id)
              : null;
          let slot: VmObjectSlotInfo = {
            parent: this.obj.obj_info.id,
            index: index,
            name: template.name,
            typ: template.typ,
            logicFields: logicFields,
            occupant: occupant,
            quantity: occupantInfo?.quantity ?? 0,
          };
          return slot;
        });
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
            if (!structuralEqual(this.logicFields, newFields)) {
              this.logicFields = newFields;
            }
          } else if (sub === "slots") {
            if (!structuralEqual(this.slots, newSlots)) {
              this.slots = newSlots;
              this.slotsCount = newSlots.length;
            }
          } else if (sub === "slots-count") {
            const slotsTemplate =
              "slots" in this.obj.template ? this.obj.template.slots : [];
            const slotsCount = slotsTemplate.length;
            if (this.slotsCount !== slotsCount) {
              this.slotsCount = slotsCount;
            }
          } else if (sub === "reagents") {
            const reagents =
              this.obj.obj_info.reagents != null
                ? new Map(
                    Object.entries(this.obj.obj_info.reagents).map(
                      ([key, val]) => [parseInt(key), val],
                    ),
                  )
                : null;
            if (!structuralEqual(this.reagents, reagents)) {
              this.reagents = reagents;
            }
          } else if (sub === "connections") {
            const connectionsMap =
              this.obj.obj_info.connections != null
                ? new Map(
                    Object.entries(this.obj.obj_info.connections).map(
                      ([key, val]) => [parseInt(key), val],
                    ),
                  )
                : null;
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
          } else if (sub === "memory") {
            const stack = this.obj.obj_info.memory ?? null;
            if (!structuralEqual(this.memory, stack)) {
              this.memory = stack;
            }
          } else if (sub === "ic") {
            if (
              typeof this.obj.obj_info.circuit !== "undefined" ||
              typeof this.obj.obj_info.socketed_ic !== "undefined"
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
            if (this.logicFields.get(sub.field) !== newFields.get(sub.field)) {
              this.logicFields = newFields;
            }
          } else if ("slot" in sub) {
            if (
              typeof this.slots === "undefined" ||
              this.slots.length < sub.slot
            ) {
              this.slots = newSlots;
            } else if (
              !structuralEqual(this.slots[sub.slot], newSlots[sub.slot])
            ) {
              this.slots = newSlots;
            }
          }
        }
      }
    }

    updateIC() {
      const ip = this.obj.obj_info.circuit?.instruction_pointer ?? null;
      if (this.icIP !== ip) {
        this.icIP = ip;
      }
      const opCount =
        this.obj.obj_info.circuit?.yield_instruction_count ?? null;
      if (this.icOpCount !== opCount) {
        this.icOpCount = opCount;
      }
      const state = this.obj.obj_info.circuit?.state ?? null;
      if (this.icState !== state) {
        this.icState = state;
      }
      const errors = this.obj.obj_info.compile_errors ?? null;
      if (!structuralEqual(this.errors, errors)) {
        this.errors = errors;
      }
      const registers = this.obj.obj_info.circuit?.registers ?? null;
      if (!structuralEqual(this.registers, registers)) {
        this.registers = registers;
      }
      const aliases =
        this.obj.obj_info.circuit?.aliases != null
          ? new Map(Object.entries(this.obj.obj_info.circuit.aliases))
          : null;
      if (!structuralEqual(this.aliases, aliases)) {
        this.aliases = aliases;
      }
      const defines =
        this.obj.obj_info.circuit?.defines != null
          ? new Map(
              Object.entries(this.obj.obj_info.circuit.defines),
              // .map(([key, val]) => [])
            )
          : null;
      if (!structuralEqual(this.defines, defines)) {
        this.defines = new Map(defines);
      }
      const pins =
        this.obj.obj_info.device_pins != null
          ? new Map(
              Object.entries(this.obj.obj_info.device_pins).map(
                ([key, val]) => [parseInt(key), val],
              ),
            )
          : null;
      if (!structuralEqual(this.pins, pins)) {
        this.pins = pins;
        this.numPins =
          "device" in this.obj.template
            ? this.obj.template.device.device_pins_length
            : Math.max(...Array.from(this.pins?.keys() ?? [0]));
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

    postDBSetUpdate(): void {}

    @state()
    set templateDB(val: TemplateDatabase) {
      this._templateDB = val;
      this.postDBSetUpdate();
    }
  }

  return VMTemplateDBMixinClass as Constructor<VMTemplateDBMixinInterface> & T;
};
