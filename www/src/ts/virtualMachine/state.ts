import { computed, ReadonlySignal, signal, Signal } from "@lit-labs/preact-signals";
import { Obj } from "@popperjs/core";
import { Class, Connection, FrozenCableNetwork, FrozenNetworks, FrozenObject, FrozenObjectFull, FrozenVM, ICInfo, LogicField, LogicSlotType, LogicType, ObjectID, Operand, Slot, TemplateDatabase } from "ic10emu_wasm";
import { fromJson, isSome, structuralEqual } from "utils";


export interface ObjectSlotInfo {
  parent: ObjectID;
  index: number;
  name: string;
  typ: Class;
  quantity: number;
  occupant: ObjectID;
}

export class VMState {

  vm: Signal<FrozenVM> = signal(null);
  templateDB: Signal<TemplateDatabase> = signal(null);

  objectIds: ReadonlySignal<ObjectID[]> = computed(() => this.vm.value?.objects.map((obj) => obj.obj_info.id) ?? []);
  circuitHolderIds: ReadonlySignal<ObjectID[]> = computed(() => this.vm.value?.circuit_holders ?? []);
  programHolderIds: ReadonlySignal<ObjectID[]> = computed(() => this.vm.value?.program_holders ?? []);
  networkIds: ReadonlySignal<ObjectID[]> = computed(() => this.vm.value?.networks.map((net) => net.id) ?? []);
  wirelessTransmitterIds: ReadonlySignal<ObjectID[]> = computed(() => this.vm.value?.wireless_transmitters ?? []);
  wirelessReceivers: ReadonlySignal<ObjectID[]> = computed(() => this.vm.value?.wireless_receivers ?? []);
  defaultNetworkId: ReadonlySignal<ObjectID> = computed(() => this.vm.value?.default_network_key ?? null);

  private _signalCache: Map<string, WeakRef<ReadonlySignal<any>>> = new Map();
  private _signalRegistry = new FinalizationRegistry((key: string) => {
    const s = this._signalCache.get(key);
    if (s && !s.deref()) this._signalCache.delete(key);
  });

  signalCacheHas(key: string): boolean {
    return this._signalCache.has(key) && typeof this._signalCache.get(key).deref() !== undefined
  }
  signalCacheGet<T>(key: string): any {
    return this._signalCache.get(key).deref();
  }
  signalCacheSet<T>(key: string, s: ReadonlySignal<T>) {
    this._signalCache.set(key, new WeakRef(s));
    this._signalRegistry.register(s, key);
  }

  getObject(id: ObjectID): ReadonlySignal<FrozenObject> {
    const key = `obj:${id}`;
    if (!this.signalCacheHas(key)) {
      let last: FrozenObject = null;
      const s = computed(() => {
        const obj = this.vm.value?.objects.find((o) => o.obj_info.id === id) ?? null;
        if (obj?.database_template ?? false) {
          return { ...obj, template: this.templateDB.value?.get(obj.obj_info.prefab_hash) }
        }
        if (structuralEqual(last, obj)) {
          return last;
        }
        last = obj;
        return obj;
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getObjectName(id: ObjectID): ReadonlySignal<string> {
    const key = `obj:${id},name`;
    if (!this.signalCacheHas(key)) {
      const s = computed(() => {
        const obj = this.getObject(id);
        return obj.value?.obj_info.name ?? "";
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getObjectPrefabName(id: ObjectID): ReadonlySignal<string> {
    const key = `obj:${id},prefabName`;
    if (!this.signalCacheHas(key)) {
      const s = computed(() => {
        const obj = this.getObject(id);
        return obj.value?.obj_info.prefab ?? "";
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getObjectDisplayName(id: ObjectID): ReadonlySignal<string> {
    const key = `obj:${id},DisplayName`;
    if (!this.signalCacheHas(key)) {
      const s = computed(() => {
        const obj = this.getObject(id).value;
        return obj?.obj_info.name ?? obj?.obj_info.prefab ?? "";
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getNetwork(id: ObjectID): Signal<FrozenCableNetwork> {
    const key = `network:${id}`;
    if (!this.signalCacheHas(key)) {
      let last: FrozenCableNetwork = null
      const s = computed(() => {
        const next = this.vm.value?.networks.find((n) => n.id === id) ?? null
        if (structuralEqual(last, next)) {
          return last;
        }
        last = next;
        return next;
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getObjectFieldNames(id: ObjectID): ReadonlySignal<LogicType[]> {
    const key = `obj:${id},fieldNames`;
    if (!this.signalCacheHas(key)) {
      let last: LogicType[] = null;
      const s = computed((): LogicType[] => {
        const obj = this.getObject(id).value;
        const template = obj?.template;
        const logicAccess = isSome(template) && "logic" in template ? template.logic.logic_types : null;
        const next = Array.from(logicAccess?.keys() ?? [])
        if (structuralEqual(last, next)) {
          return last;
        }
        last = next;
        return next;
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getObjectField(id: ObjectID, field: LogicType): ReadonlySignal<LogicField> {
    const key = `obj:${id},field:${field}`;
    if (!this.signalCacheHas(key)) {
      const s = computed((): LogicField => {
        const obj = this.getObject(id).value;
        const template = obj?.template;
        const logicAccess = isSome(template) && "logic" in template ? template.logic.logic_types.get(field) : null;
        const logicValue = obj?.obj_info.logic_values.get(field) ?? null;
        return isSome(logicAccess) || isSome(logicValue) ? {
          field_type: logicAccess,
          value: logicValue,
        } : null;
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getObjectSlotCount(id: ObjectID): ReadonlySignal<number> {
    const key = `obj:${id},slotsCount`;
    if (!this.signalCacheHas(key)) {
      const s = computed((): number => {
        const obj = this.getObject(id).value;
        const template = obj?.template;
        return isSome(template) && "slots" in template ? template.slots.length : 0
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getObjectSlotInfo(id: ObjectID, index: number): ReadonlySignal<ObjectSlotInfo> {
    const key = `obj:${id},slot${index}`;
    if (!this.signalCacheHas(key)) {
      let last: ObjectSlotInfo = null;
      const s = computed((): ObjectSlotInfo => {
        const obj = this.getObject(id).value;
        const info = obj?.obj_info.slots.get(index);
        const template = obj?.template;
        const slotTemplate = isSome(template) && "slots" in template ? template.slots[index] : null;
        if (isSome(obj)) {
          const next = {
            parent: obj?.obj_info.id,
            index,
            name: slotTemplate?.name,
            typ: slotTemplate?.typ,
            quantity: info?.quantity,
            occupant: info?.id
          }
          if (structuralEqual(last, next)) {
            return last;
          }
          last = next;
          return next;
        }
        return null;
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getObjectSlotFieldNames(id: ObjectID, index: number): ReadonlySignal<LogicSlotType[]> {
    const key = `obj:${id},slot:${index},fieldNames`;
    if (!this.signalCacheHas(key)) {
      let last: LogicSlotType[] = null;
      const s = computed((): LogicSlotType[] => {
        const obj = this.getObject(id).value;
        const template = obj?.template;
        let logicTemplate = null;
        if (isSome(template) && ("logic" in template)) {
          logicTemplate = template.logic.logic_slot_types.get(index.toString());
        }
        const next = Array.from(logicTemplate?.keys() ?? []);
        if (structuralEqual(last, next)) {
          return last;
        }
        last = next;
        return next;
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getObjectSlotField(id: ObjectID, index: number, field: LogicSlotType): ReadonlySignal<LogicField> {
    const key = `obj:${id},slot:${index},field:${field}`;
    if (!this.signalCacheHas(key)) {
      let last: LogicField = null
      const s = computed((): LogicField => {
        const obj = this.getObject(id).value;
        const template = obj?.template;
        const logicTemplate = isSome(template) && "logic" in template ? template.logic.logic_slot_types.get(index.toString()) : null;
        const slotFieldValue = obj?.obj_info.slot_logic_values?.get(index)?.get(field) ?? null;
        const slotFieldAccess = logicTemplate?.get(field)
        const next = isSome(slotFieldValue) || isSome(slotFieldAccess) ? {
          field_type: slotFieldAccess,
          value: slotFieldValue

        } : null
        if (structuralEqual(last, next)) {
          return last;
        }
        last = next;
        return next;
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getObjectSlotOccupantId(id: ObjectID, index: number): ReadonlySignal<ObjectID> {
    const key = `obj:${id},slot:${index},occupant`
    if (!this.signalCacheHas(key)) {
      const s = computed((): ObjectID => {
        const obj = this.getObject(id).value;
        const info = obj?.obj_info.slots.get(index);
        return info?.id ?? null;
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getObjectSocketedIcId(id: ObjectID): ReadonlySignal<ObjectID> {
    const key = `obj:${id},socketedIc`
    if (!this.signalCacheHas(key)) {
      const s = computed((): ObjectID => {
        const obj = this.getObject(id).value;
        return obj?.obj_info.socketed_ic ?? null;
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getObjectConnectionCount(id: ObjectID): ReadonlySignal<number> {
    const key = `obj:${id},connectionCount`;
    if (!this.signalCacheHas(key)) {
      const s = computed((): number => {
        const obj = this.getObject(id).value;
        const template = obj?.template;
        const connectionList =
          isSome(template) && "device" in template
            ? template.device.connection_list
            : [];
        return connectionList.length;
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getObjectConnections(id: ObjectID): ReadonlySignal<Connection[]> {
    const key = `obj:${id},connections`
    if (!this.signalCacheHas(key)) {
      let last: Connection[] = null;
      const s = computed((): Connection[] => {
        const obj = this.getObject(id).value;
        const template = obj?.template;
        const connectionsMap = obj?.obj_info.connections ?? null;
        const connectionList =
          isSome(template) && "device" in template
            ? template.device.connection_list
            : [];
        const connections = connectionList.map((conn, index): Connection => {
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
          } else if (conn.typ === "RoboticArmRail") {
            return { RoboticArmRail: { role: conn.role } }
          }
          return "None";
        });
        if (structuralEqual(last, connections)) {
          return last;
        }
        last = connections;
        return connections;
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getObjectConnection(id: ObjectID, index: number): ReadonlySignal<Connection> {
    const key = `obj:${id},connection:${index}`;
    if (!this.signalCacheHas(key)) {
      let last: Connection = null;
      const s = computed((): Connection => {
        const connections = this.getObjectConnections(id).value ?? [];
        const next = connections[index] ?? null;
        if (structuralEqual(last, next)) {
          return last;
        }
        last = next;
        return next;
      })
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key)
  }

  getCircuitInfo(id: ObjectID): ReadonlySignal<ICInfo> {
    const key = `obj:${id},socketedIc`
    if (!this.signalCacheHas(key)) {
      let last: ICInfo = null;
      const s = computed((): ICInfo => {
        const obj = this.getObject(id).value;
        if (!isSome(obj)) {
          return null;
        }
        let circuitInfo: ICInfo = obj.obj_info.circuit;
        if (!isSome(circuitInfo)) {
          const icObj = this.getObject(obj.obj_info.socketed_ic).value;
          if (!isSome(icObj)) {
            return null;
          }
          circuitInfo = icObj.obj_info.circuit;
        }
        if (structuralEqual(last, circuitInfo)) {
          return last;
        }
        last = circuitInfo;
        return circuitInfo;
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getObjectMemorySize(id: ObjectID): ReadonlySignal<number> {
    const key = `obj:${id},memorySize`;
    if (!this.signalCacheHas(key)) {
      const s = computed((): number => {
        return this.getObject(id).value?.obj_info.memory?.length ?? null;
      })
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getObjectMemory(id: ObjectID): ReadonlySignal<number[]> {
    const key = `obj:${id},memory`;
    if (!this.signalCacheHas(key)) {
      let last: number[] = null;
      const s = computed((): number[] => {
        const next = this.getObject(id).value?.obj_info.memory ?? null;
        if (structuralEqual(last, next)) {
          return last;
        }
        last = next;
        return next;
      })
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getObjectMemoryAt(id: ObjectID, index: number): ReadonlySignal<number> {
    const key = `obj:${id},memory:${index}`;
    if (!this.signalCacheHas(key)) {
      const s = computed((): number => {
        return (this.getObject(id).value?.obj_info.memory ?? [])[index] ?? null;
      })
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getCircuitRegistersCount(id: ObjectID): ReadonlySignal<number> {
    const key = `obj:${id},registersCount`;
    if (!this.signalCacheHas(key)) {
      const s = computed((): number => {
        return this.getCircuitInfo(id).value?.registers.length ?? null;
      })
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getCircuitRegisters(id: ObjectID): ReadonlySignal<number[]> {
    const key = `obj:${id},registers`;
    if (!this.signalCacheHas(key)) {
      let last: number[] = null;
      const s = computed((): number[] => {
        const next = this.getCircuitInfo(id).value?.registers ?? null;
        if (structuralEqual(last, next)) {
          return last;
        }
        last = next;
        return next;
      })
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getCircuitRegistersAt(id: ObjectID, index: number): ReadonlySignal<number> {
    const key = `obj:${id},register:${index}`;
    if (!this.signalCacheHas(key)) {
      const s = computed((): number => {
        return (this.getCircuitInfo(id).value?.registers ?? [])[index] ?? null;
      })
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getCircuitAliases(id: ObjectID): ReadonlySignal<Record<string, Operand>> {
    const key = `obj:${id},circuitAliases`;
    if (!this.signalCacheHas(key)) {
      let last: Record<string, Operand> = null;
      const s = computed(() => {
        const circuit = this.getCircuitInfo(id).value;
        const aliases = circuit?.aliases;
        const next = Object.fromEntries(aliases?.entries() ?? [])
        if (structuralEqual(last, next)) {
          return last;
        }
        last = next;
        return next;
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key)
  }

  getDeviceNumPins(id: ObjectID): ReadonlySignal<number> {
    const key = `obj:${id},numPins`;
    if (!this.signalCacheHas(key)) {
      const s = computed((): number => {
        const obj = this.getObject(id).value;
        return [...obj?.obj_info.device_pins?.keys() ?? []].length;
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getDevicePins(id: ObjectID): ReadonlySignal<[number, ObjectID][]> {
    const key = `obj:${id},pins`;
    if (!this.signalCacheHas(key)) {
      let last: [number, ObjectID][] = null;
      const s = computed((): [number, ObjectID][] => {
        const obj = this.getObject(id).value;
        const next = [...obj?.obj_info.device_pins?.entries() ?? []];
        if (structuralEqual(last, next)) {
          return last;
        }
        last = next;
        return next;
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getDevicePin(id: ObjectID, pin: number): ReadonlySignal<ObjectID> {
    const key = `obj:${id},pin:${id}`;
    if (!this.signalCacheHas(key)) {
      const s = computed((): ObjectID => {
        const obj = this.getObject(id).value;
        return obj?.obj_info.device_pins?.get(pin);
      });
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key);
  }

  getNetworkDevices(id: ObjectID): ReadonlySignal<ObjectID[]> {
    const key = `network:${id},devices`;
    if (!this.signalCacheHas(key)) {
      let last: ObjectID[] = null;
      const s = computed(() => {
        const next = this.vm.value.networks.find((net) => net.id === id)?.devices ?? null;
        if (structuralEqual(last, next)) {
          return last;
        }
        last = next;
        return next;
      })
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key)
  }

  getObjectProgramSource(id: ObjectID): ReadonlySignal<string> {
    const key = `obj:${id},source`;
    if (!this.signalCacheHas(key)) {
      const s = computed(() => {
        return this.getObject(id).value?.obj_info.source_code ?? null;
      })
      this.signalCacheSet(key, s);
      return s;
    }
    return this.signalCacheGet(key)
  }


}
