use std::{collections::BTreeMap, rc::Rc, str::FromStr};

use crate::{
    errors::TemplateError,
    interpreter::ICInfo,
    network::Connection,
    vm::{
        object::{
            generic::structs::{
                Generic, GenericCircuitHolder, GenericItem, GenericItemCircuitHolder,
                GenericItemConsumer, GenericItemLogicable, GenericItemLogicableMemoryReadWriteable,
                GenericItemLogicableMemoryReadable, GenericItemStorage, GenericItemSuit,
                GenericItemSuitCircuitHolder, GenericItemSuitLogic, GenericLogicable,
                GenericLogicableDevice, GenericLogicableDeviceConsumer,
                GenericLogicableDeviceConsumerMemoryReadWriteable,
                GenericLogicableDeviceConsumerMemoryReadable,
                GenericLogicableDeviceMemoryReadWriteable, GenericLogicableDeviceMemoryReadable,
                GenericStorage,
            },
            humans::{EntityInfo, HumanPlayer},
            traits::*,
            LogicField, Name, Slot, SlotOccupantInfo,
        },
        VM,
    },
};
use serde_derive::{Deserialize, Serialize};
use stationeers_data::{
    enums::{
        basic::{Class as SlotClass, GasType, SortingClass},
        prefabs::StationpediaPrefab,
        script::{LogicSlotType, LogicType},
        ConnectionRole, ConnectionType,
    },
    templates::*,
};
use strum::{EnumProperty, IntoEnumIterator};
#[cfg(feature = "tsify")]
use tsify::Tsify;
#[cfg(feature = "tsify")]
use wasm_bindgen::prelude::*;

use super::{stationpedia, MemoryAccess, ObjectID, VMObject};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub enum Prefab {
    Hash(i32),
    Name(String),
}

impl std::fmt::Display for Prefab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (known_prefab, unknown_str) = match self {
            Self::Hash(hash) => (
                StationpediaPrefab::from_repr(*hash),
                format!("Unknown({hash}))"),
            ),
            Self::Name(name) => (StationpediaPrefab::from_str(&name).ok(), name.clone()),
        };
        if let Some(known) = known_prefab {
            write!(f, "{known}")
        } else {
            write!(f, "{unknown_str}")
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct ObjectInfo {
    pub name: Option<String>,
    pub id: Option<ObjectID>,
    pub prefab: Option<Prefab>,
    pub slots: Option<BTreeMap<u32, SlotOccupantInfo>>,
    pub damage: Option<f32>,
    pub device_pins: Option<BTreeMap<u32, ObjectID>>,
    pub connections: Option<BTreeMap<u32, ObjectID>>,
    pub reagents: Option<BTreeMap<i32, f64>>,
    pub memory: Option<Vec<f64>>,
    pub logic_values: Option<BTreeMap<LogicType, f64>>,
    pub entity: Option<EntityInfo>,
    pub source_code: Option<String>,
    pub circuit: Option<ICInfo>,
}

impl From<&VMObject> for ObjectInfo {
    fn from(obj: &VMObject) -> Self {
        let obj_ref = obj.borrow();
        ObjectInfo {
            name: Some(obj_ref.get_name().value.clone()),
            id: Some(*obj_ref.get_id()),
            prefab: Some(Prefab::Hash(obj_ref.get_prefab().hash)),
            slots: None,
            damage: None,
            device_pins: None,
            connections: None,
            reagents: None,
            memory: None,
            logic_values: None,
            entity: None,
            source_code: None,
            circuit: None,
        }
    }
}

impl ObjectInfo {
    pub fn update_from_interfaces(&mut self, interfaces: &ObjectInterfaces<'_>) -> &mut Self {
        if let Some(storage) = interfaces.storage {
            self.update_from_storage(storage);
        }
        if let Some(logic) = interfaces.logicable {
            self.update_from_logic(logic);
        }
        if let Some(device) = interfaces.device {
            self.update_from_device(device);
        }
        if let Some(memory) = interfaces.memory_readable {
            self.update_from_memory(memory);
        }
        if let Some(item) = interfaces.item {
            self.update_from_item(item);
        }
        if let Some(human) = interfaces.human {
            self.update_from_human(human);
        }
        if let Some(source) = interfaces.source_code {
            self.update_from_source_code(source);
        }
        if let Some(circuit) = interfaces.integrated_circuit {
            self.update_from_circuit(circuit);
        }
        self
    }

    pub fn update_from_storage(&mut self, storage: StorageRef<'_>) -> &mut Self {
        let slots = storage.get_slots();
        if slots.is_empty() {
            self.slots = None;
        } else {
            self.slots.replace(
                slots
                    .into_iter()
                    .enumerate()
                    .filter_map(|(index, slot)| {
                        slot.occupant
                            .as_ref()
                            .map(|occupant| (index as u32, occupant.clone()))
                    })
                    .collect(),
            );
        }
        self
    }

    pub fn update_from_item(&mut self, item: ItemRef<'_>) -> &mut Self {
        let damage = item.get_damage();
        if damage == 0.0 {
            self.damage = None;
        } else {
            self.damage.replace(damage);
        }
        self
    }

    pub fn update_from_device(&mut self, device: DeviceRef<'_>) -> &mut Self {
        let pins = device.device_pins();
        if pins.is_some_and(<[Option<u32>]>::is_empty) {
            self.device_pins = None;
        } else {
            self.device_pins = pins.map(|pins| {
                pins.iter()
                    .enumerate()
                    .filter_map(|(index, pin)| pin.as_ref().map(|pin| (index as u32, *pin)))
                    .collect()
            });
        }
        let reagents: BTreeMap<i32, f64> = device.get_reagents().iter().copied().collect();
        if reagents.is_empty() {
            self.reagents = None;
        } else {
            self.reagents.replace(reagents);
        }
        let connections = device.connection_list();
        if connections.is_empty() {
            self.connections = None;
        } else {
            self.connections.replace(
                connections
                    .into_iter()
                    .enumerate()
                    .filter_map(|(index, conn)| match conn.get_network() {
                        Some(net) => Some((index as u32, net)),
                        None => None,
                    })
                    .collect(),
            );
        }
        self
    }

    pub fn update_from_memory(&mut self, memory: MemoryReadableRef<'_>) -> &mut Self {
        if memory.memory_size() != 0 {
            self.memory.replace(memory.get_memory_slice().to_vec());
        } else {
            self.memory = None;
        }
        self
    }

    pub fn update_from_logic(&mut self, logic: LogicableRef<'_>) -> &mut Self {
        self.logic_values.replace(
            logic
                .valid_logic_types()
                .iter()
                .filter_map(|lt| match logic.get_logic(*lt) {
                    Ok(val) => Some((*lt, val)),
                    _ => None,
                })
                .collect(),
        );
        self
    }

    pub fn update_from_human(&mut self, human: HumanRef<'_>) -> &mut Self {
        let damage = human.get_damage();
        if damage == 0.0 {
            self.damage = None;
        } else {
            self.damage.replace(damage);
        }
        self.entity.replace(EntityInfo {
            hydration: human.get_hydration(),
            nutrition: human.get_nutrition(),
            oxygenation: human.get_oxygenation(),
            food_quality: human.get_food_quality(),
            mood: human.get_mood(),
            hygiene: human.get_hygiene(),
        });
        self
    }

    pub fn update_from_source_code(&mut self, source: SourceCodeRef<'_>) -> &mut Self {
        let code = source.get_source_code();
        if !code.is_empty() {
            self.source_code.replace(code);
        }
        self
    }

    pub fn update_from_circuit(&mut self, circuit: IntegratedCircuitRef<'_>) -> &mut Self {
        self.circuit.replace(ICInfo {
            instruction_pointer: circuit.get_instruction_pointer(),
            registers: circuit.get_registers().to_vec(),
            aliases: circuit
                .get_aliases()
                .iter()
                .map(|(key, val)| (key.clone(), val.clone()))
                .collect(),
            defines: circuit
                .get_defines()
                .iter()
                .map(|(key, val)| (key.clone(), *val))
                .collect(),
            labels: circuit
                .get_labels()
                .iter()
                .map(|(key, val)| (key.clone(), *val))
                .collect(),
            state: circuit.get_state(),
            yield_instruciton_count: circuit.get_instructions_since_yield(),
        });
        self
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct FrozenObject {
    pub obj_info: ObjectInfo,
    pub database_template: bool,
    pub template: Option<ObjectTemplate>,
}

impl FrozenObject {
    pub fn new(obj_info: ObjectInfo) -> Self {
        FrozenObject {
            obj_info,
            database_template: false,
            template: None,
        }
    }

    pub fn with_template(obj_info: ObjectInfo, template: ObjectTemplate) -> Self {
        FrozenObject {
            obj_info,
            database_template: false,
            template: Some(template),
        }
    }

    pub fn build_vm_obj(&self, id: ObjectID, vm: &Rc<VM>) -> Result<VMObject, TemplateError> {
        let template = self.template.as_ref().map_or_else(
            || {
                self.obj_info
                    .prefab
                    .as_ref()
                    .map(|prefab| {
                        vm.get_template(prefab.clone())
                            .ok_or(TemplateError::NoTemplateForPrefab(prefab.clone()))
                    })
                    .transpose()?
                    .ok_or(TemplateError::MissingPrefab)
            },
            |template| Ok(template.clone()),
        )?;
        if let Some(obj) = stationpedia::object_from_frozen(&self.obj_info, id, vm)? {
            Ok(obj)
        } else {
            self.build_generic(id, &template, vm.clone())
        }
    }

    pub fn connected_networks(&self) -> Vec<ObjectID> {
        self.obj_info
            .connections
            .as_ref()
            .map(|connections| connections.values().copied().collect())
            .unwrap_or_default()
    }

    pub fn contained_object_ids(&self) -> Vec<ObjectID> {
        self.obj_info
            .slots
            .as_ref()
            .map(|slots| slots.values().map(|slot| slot.id).collect())
            .unwrap_or_default()
    }

    pub fn contained_object_slots(&self) -> Vec<(u32, ObjectID)> {
        self.obj_info
            .slots
            .as_ref()
            .map(|slots| {
                slots
                    .iter()
                    .map(|(index, slot)| (*index, slot.id))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn build_slots(
        &self,
        id: ObjectID,
        slots_info: &[SlotInfo],
        logic_info: Option<&LogicInfo>,
    ) -> Vec<Slot> {
        slots_info
            .iter()
            .enumerate()
            .map(|(index, info)| Slot {
                parent: id,
                index,
                name: info.name.clone(),
                typ: info.typ,
                readable_logic: logic_info
                    .and_then(|info| {
                        info.logic_slot_types.get(&(index as u32)).map(|s_info| {
                            s_info
                                .iter()
                                .filter_map(|(key, access)| match access {
                                    MemoryAccess::Read | MemoryAccess::ReadWrite => Some(key),
                                    _ => None,
                                })
                                .copied()
                                .collect::<Vec<_>>()
                        })
                    })
                    .unwrap_or_else(Vec::new),
                writeable_logic: logic_info
                    .and_then(|info| {
                        info.logic_slot_types.get(&(index as u32)).map(|s_info| {
                            s_info
                                .iter()
                                .filter_map(|(key, access)| match access {
                                    MemoryAccess::Write | MemoryAccess::ReadWrite => Some(key),
                                    _ => None,
                                })
                                .copied()
                                .collect::<Vec<_>>()
                        })
                    })
                    .unwrap_or_else(Vec::new),
                occupant: self
                    .obj_info
                    .slots
                    .as_ref()
                    .and_then(|slots| slots.get(&(index as u32)).cloned()),
            })
            .collect()
    }

    fn build_logic_fields(&self, logic_info: &LogicInfo) -> BTreeMap<LogicType, LogicField> {
        logic_info
            .logic_types
            .iter()
            .map(|(key, access)| {
                (
                    *key,
                    LogicField {
                        field_type: *access,
                        value: self
                            .obj_info
                            .logic_values
                            .as_ref()
                            .and_then(|values| values.get(key))
                            .copied()
                            .unwrap_or(0.0),
                    },
                )
            })
            .collect()
    }

    fn build_connections(&self, device_info: &DeviceInfo) -> Vec<Connection> {
        device_info
            .connection_list
            .iter()
            .enumerate()
            .map(|(index, conn_info)| {
                Connection::from_info(
                    conn_info.typ,
                    conn_info.role,
                    self.obj_info
                        .connections
                        .as_ref()
                        .and_then(|connections| connections.get(&(index as u32)).copied()),
                )
            })
            .collect()
    }

    fn build_pins(&self, device_info: &DeviceInfo) -> Option<Vec<Option<ObjectID>>> {
        let num_pins = device_info.device_pins_length.unwrap_or(0);
        if num_pins > 0 {
            Some(
                (0..num_pins)
                    .map(|index| {
                        self.obj_info
                            .device_pins
                            .as_ref()
                            .and_then(|pins| pins.get(&index).copied())
                    })
                    .collect(),
            )
        } else {
            None
        }
    }

    fn build_memory(&self, memory_info: &MemoryInfo) -> Vec<f64> {
        self.obj_info
            .memory
            .clone()
            .unwrap_or_else(|| vec![0.0; memory_info.memory_size as usize])
    }

    fn build_generic(
        &self,
        id: ObjectID,
        template: &ObjectTemplate,
        vm: Rc<VM>,
    ) -> Result<VMObject, TemplateError> {
        use ObjectTemplate::*;
        match template {
            Structure(s) => Ok(VMObject::new(Generic {
                id,
                prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                name: Name::new(&self.obj_info.name.clone().unwrap_or(s.prefab.name.clone())),
                vm,
                internal_atmo_info: s.internal_atmo_info.clone(),
                thermal_info: s.thermal_info.clone(),
                small_grid: s.structure.small_grid,
            })),
            StructureSlots(s) => Ok(VMObject::new(GenericStorage {
                id,
                prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                name: Name::new(&self.obj_info.name.clone().unwrap_or(s.prefab.name.clone())),
                vm,
                internal_atmo_info: s.internal_atmo_info.clone(),
                thermal_info: s.thermal_info.clone(),
                small_grid: s.structure.small_grid,
                slots: self.build_slots(id, &s.slots, None),
            })),
            StructureLogic(s) => Ok(VMObject::new(GenericLogicable {
                id,
                prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                name: Name::new(&self.obj_info.name.clone().unwrap_or(s.prefab.name.clone())),
                vm,
                internal_atmo_info: s.internal_atmo_info.clone(),
                thermal_info: s.thermal_info.clone(),
                small_grid: s.structure.small_grid,
                slots: self.build_slots(id, &s.slots, Some(&s.logic)),
                fields: self.build_logic_fields(&s.logic),
                modes: s.logic.modes.clone(),
            })),
            StructureLogicDevice(s) => Ok(VMObject::new(GenericLogicableDevice {
                id,
                prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                name: Name::new(&self.obj_info.name.clone().unwrap_or(s.prefab.name.clone())),
                vm,
                internal_atmo_info: s.internal_atmo_info.clone(),
                thermal_info: s.thermal_info.clone(),
                small_grid: s.structure.small_grid,
                slots: self.build_slots(id, &s.slots, Some(&s.logic)),
                fields: self.build_logic_fields(&s.logic),
                modes: s.logic.modes.clone(),
                connections: self.build_connections(&s.device),
                pins: self.build_pins(&s.device),
                device_info: s.device.clone(),
                reagents: self.obj_info.reagents.clone(),
            })),
            StructureLogicDeviceConsumer(s) => Ok(VMObject::new(GenericLogicableDeviceConsumer {
                id,
                prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                name: Name::new(&self.obj_info.name.clone().unwrap_or(s.prefab.name.clone())),
                vm,
                internal_atmo_info: s.internal_atmo_info.clone(),
                thermal_info: s.thermal_info.clone(),
                small_grid: s.structure.small_grid,
                slots: self.build_slots(id, &s.slots, Some(&s.logic)),
                fields: self.build_logic_fields(&s.logic),
                modes: s.logic.modes.clone(),
                connections: self.build_connections(&s.device),
                pins: self.build_pins(&s.device),
                device_info: s.device.clone(),
                reagents: self.obj_info.reagents.clone(),
                consumer_info: s.consumer_info.clone(),
            })),
            StructureCircuitHolder(s) => Ok(VMObject::new(GenericCircuitHolder {
                id,
                prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                name: Name::new(&self.obj_info.name.clone().unwrap_or(s.prefab.name.clone())),
                vm,
                internal_atmo_info: s.internal_atmo_info.clone(),
                thermal_info: s.thermal_info.clone(),
                small_grid: s.structure.small_grid,
                slots: self.build_slots(id, &s.slots, Some(&s.logic)),
                fields: self.build_logic_fields(&s.logic),
                modes: s.logic.modes.clone(),
                connections: self.build_connections(&s.device),
                pins: self.build_pins(&s.device),
                device_info: s.device.clone(),
                reagents: self.obj_info.reagents.clone(),
            })),
            StructureLogicDeviceMemory(s)
                if matches!(s.memory.memory_access, MemoryAccess::Read) =>
            {
                Ok(VMObject::new(GenericLogicableDeviceMemoryReadable {
                    id,
                    prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                    name: Name::new(&self.obj_info.name.clone().unwrap_or(s.prefab.name.clone())),
                    vm,
                    internal_atmo_info: s.internal_atmo_info.clone(),
                    thermal_info: s.thermal_info.clone(),
                    small_grid: s.structure.small_grid,
                    slots: self.build_slots(id, &s.slots, Some(&s.logic)),
                    fields: self.build_logic_fields(&s.logic),
                    modes: s.logic.modes.clone(),
                    connections: self.build_connections(&s.device),
                    pins: self.build_pins(&s.device),
                    device_info: s.device.clone(),
                    reagents: self.obj_info.reagents.clone(),
                    memory: self.build_memory(&s.memory),
                }))
            }
            StructureLogicDeviceMemory(s) => {
                Ok(VMObject::new(GenericLogicableDeviceMemoryReadWriteable {
                    id,
                    prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                    name: Name::new(&self.obj_info.name.clone().unwrap_or(s.prefab.name.clone())),
                    vm,
                    internal_atmo_info: s.internal_atmo_info.clone(),
                    thermal_info: s.thermal_info.clone(),
                    small_grid: s.structure.small_grid,
                    slots: self.build_slots(id, &s.slots, Some(&s.logic)),
                    fields: self.build_logic_fields(&s.logic),
                    modes: s.logic.modes.clone(),
                    connections: self.build_connections(&s.device),
                    pins: self.build_pins(&s.device),
                    device_info: s.device.clone(),
                    reagents: self.obj_info.reagents.clone(),
                    memory: self.build_memory(&s.memory),
                }))
            }
            StructureLogicDeviceConsumerMemory(s)
                if matches!(s.memory.memory_access, MemoryAccess::Read) =>
            {
                Ok(VMObject::new(
                    GenericLogicableDeviceConsumerMemoryReadable {
                        id,
                        prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                        name: Name::new(
                            &self.obj_info.name.clone().unwrap_or(s.prefab.name.clone()),
                        ),
                        vm,
                        internal_atmo_info: s.internal_atmo_info.clone(),
                        thermal_info: s.thermal_info.clone(),
                        small_grid: s.structure.small_grid,
                        slots: self.build_slots(id, &s.slots, Some(&s.logic)),
                        fields: self.build_logic_fields(&s.logic),
                        modes: s.logic.modes.clone(),
                        connections: self.build_connections(&s.device),
                        pins: self.build_pins(&s.device),
                        device_info: s.device.clone(),
                        reagents: self.obj_info.reagents.clone(),
                        consumer_info: s.consumer_info.clone(),
                        fabricator_info: s.fabricator_info.clone(),
                        memory: self.build_memory(&s.memory),
                    },
                ))
            }
            StructureLogicDeviceConsumerMemory(s) => Ok(VMObject::new(
                GenericLogicableDeviceConsumerMemoryReadWriteable {
                    id,
                    prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                    name: Name::new(&self.obj_info.name.clone().unwrap_or(s.prefab.name.clone())),
                    vm,
                    internal_atmo_info: s.internal_atmo_info.clone(),
                    thermal_info: s.thermal_info.clone(),
                    small_grid: s.structure.small_grid,
                    slots: self.build_slots(id, &s.slots, Some(&s.logic)),
                    fields: self.build_logic_fields(&s.logic),
                    modes: s.logic.modes.clone(),
                    connections: self.build_connections(&s.device),
                    pins: self.build_pins(&s.device),
                    device_info: s.device.clone(),
                    reagents: self.obj_info.reagents.clone(),
                    consumer_info: s.consumer_info.clone(),
                    fabricator_info: s.fabricator_info.clone(),
                    memory: self.build_memory(&s.memory),
                },
            )),
            Item(i) => Ok(VMObject::new(GenericItem {
                id,
                prefab: Name::from_prefab_name(&i.prefab.prefab_name),
                name: Name::new(&self.obj_info.name.clone().unwrap_or(i.prefab.name.clone())),
                vm,
                internal_atmo_info: i.internal_atmo_info.clone(),
                thermal_info: i.thermal_info.clone(),
                item_info: i.item.clone(),
                parent_slot: None,
                damage: self.obj_info.damage,
            })),
            ItemSlots(i) => Ok(VMObject::new(GenericItemStorage {
                id,
                prefab: Name::from_prefab_name(&i.prefab.prefab_name),
                name: Name::new(&self.obj_info.name.clone().unwrap_or(i.prefab.name.clone())),
                vm,
                internal_atmo_info: i.internal_atmo_info.clone(),
                thermal_info: i.thermal_info.clone(),
                item_info: i.item.clone(),
                parent_slot: None,
                damage: self.obj_info.damage,
                slots: self.build_slots(id, &i.slots, None),
            })),
            ItemConsumer(i) => Ok(VMObject::new(GenericItemConsumer {
                id,
                prefab: Name::from_prefab_name(&i.prefab.prefab_name),
                name: Name::new(&self.obj_info.name.clone().unwrap_or(i.prefab.name.clone())),
                vm,
                internal_atmo_info: i.internal_atmo_info.clone(),
                thermal_info: i.thermal_info.clone(),
                item_info: i.item.clone(),
                parent_slot: None,
                damage: self.obj_info.damage,
                slots: self.build_slots(id, &i.slots, None),
                consumer_info: i.consumer_info.clone(),
            })),
            ItemLogic(i) => Ok(VMObject::new(GenericItemLogicable {
                id,
                prefab: Name::from_prefab_name(&i.prefab.prefab_name),
                name: Name::new(&self.obj_info.name.clone().unwrap_or(i.prefab.name.clone())),
                vm,
                internal_atmo_info: i.internal_atmo_info.clone(),
                thermal_info: i.thermal_info.clone(),
                item_info: i.item.clone(),
                parent_slot: None,
                damage: self.obj_info.damage,
                slots: self.build_slots(id, &i.slots, Some(&i.logic)),
                fields: self.build_logic_fields(&i.logic),
                modes: i.logic.modes.clone(),
            })),
            ItemLogicMemory(i) if matches!(i.memory.memory_access, MemoryAccess::Read) => {
                Ok(VMObject::new(GenericItemLogicableMemoryReadable {
                    id,
                    prefab: Name::from_prefab_name(&i.prefab.prefab_name),
                    name: Name::new(&self.obj_info.name.clone().unwrap_or(i.prefab.name.clone())),
                    vm,
                    internal_atmo_info: i.internal_atmo_info.clone(),
                    thermal_info: i.thermal_info.clone(),
                    item_info: i.item.clone(),
                    parent_slot: None,
                    damage: self.obj_info.damage,
                    slots: self.build_slots(id, &i.slots, Some(&i.logic)),
                    fields: self.build_logic_fields(&i.logic),
                    modes: i.logic.modes.clone(),
                    memory: self.build_memory(&i.memory),
                }))
            }
            ItemLogicMemory(i) => Ok(VMObject::new(GenericItemLogicableMemoryReadWriteable {
                id,
                prefab: Name::from_prefab_name(&i.prefab.prefab_name),
                name: Name::new(&self.obj_info.name.clone().unwrap_or(i.prefab.name.clone())),
                vm,
                internal_atmo_info: i.internal_atmo_info.clone(),
                thermal_info: i.thermal_info.clone(),
                item_info: i.item.clone(),
                parent_slot: None,
                damage: self.obj_info.damage,
                slots: self.build_slots(id, &i.slots, Some(&i.logic)),
                fields: self.build_logic_fields(&i.logic),
                modes: i.logic.modes.clone(),
                memory: self.build_memory(&i.memory),
            })),
            ItemCircuitHolder(i) => Ok(VMObject::new(GenericItemCircuitHolder {
                id,
                prefab: Name::from_prefab_name(&i.prefab.prefab_name),
                name: Name::new(&self.obj_info.name.clone().unwrap_or(i.prefab.name.clone())),
                vm,
                internal_atmo_info: i.internal_atmo_info.clone(),
                thermal_info: i.thermal_info.clone(),
                item_info: i.item.clone(),
                parent_slot: None,
                damage: self.obj_info.damage,
                slots: self.build_slots(id, &i.slots, Some(&i.logic)),
                fields: self.build_logic_fields(&i.logic),
                modes: i.logic.modes.clone(),
            })),
            ItemSuit(i) => Ok(VMObject::new(GenericItemSuit {
                id,
                prefab: Name::from_prefab_name(&i.prefab.prefab_name),
                name: Name::new(&self.obj_info.name.clone().unwrap_or(i.prefab.name.clone())),
                vm,
                internal_atmo_info: i.internal_atmo_info.clone(),
                thermal_info: i.thermal_info.clone(),
                item_info: i.item.clone(),
                parent_slot: None,
                damage: self.obj_info.damage,
                slots: self.build_slots(id, &i.slots, None),
                suit_info: i.suit_info.clone(),
            })),
            ItemSuitLogic(i) => Ok(VMObject::new(GenericItemSuitLogic {
                id,
                prefab: Name::from_prefab_name(&i.prefab.prefab_name),
                name: Name::new(&self.obj_info.name.clone().unwrap_or(i.prefab.name.clone())),
                vm,
                internal_atmo_info: i.internal_atmo_info.clone(),
                thermal_info: i.thermal_info.clone(),
                item_info: i.item.clone(),
                parent_slot: None,
                damage: self.obj_info.damage,
                slots: self.build_slots(id, &i.slots, Some(&i.logic)),
                suit_info: i.suit_info.clone(),
                fields: self.build_logic_fields(&i.logic),
                modes: i.logic.modes.clone(),
            })),
            ItemSuitCircuitHolder(i) => Ok(VMObject::new(GenericItemSuitCircuitHolder {
                id,
                prefab: Name::from_prefab_name(&i.prefab.prefab_name),
                name: Name::new(&self.obj_info.name.clone().unwrap_or(i.prefab.name.clone())),
                vm,
                internal_atmo_info: i.internal_atmo_info.clone(),
                thermal_info: i.thermal_info.clone(),
                item_info: i.item.clone(),
                parent_slot: None,
                damage: self.obj_info.damage,
                slots: self.build_slots(id, &i.slots, Some(&i.logic)),
                suit_info: i.suit_info.clone(),
                fields: self.build_logic_fields(&i.logic),
                modes: i.logic.modes.clone(),
                memory: self.build_memory(&i.memory),
            })),
            Human(h) => {
                let mut human = HumanPlayer::with_species(id, vm, h.species);
                if let Some(info) = &self.obj_info.entity {
                    human.update_entity_info(info);
                }
                if let Some(slot_info) = &self.obj_info.slots {
                    human.update_slots_from_info(slot_info);
                }
                if let Some(damage) = &self.obj_info.damage {
                    human.damage = *damage;
                }
                Ok(VMObject::new(human))
            }
        }
    }

    pub fn freeze_object(obj: &VMObject, vm: &Rc<VM>) -> Result<Self, TemplateError> {
        let obj_ref = obj.borrow();
        let interfaces = ObjectInterfaces::from_object(&*obj_ref);
        let mut obj_info: ObjectInfo = obj.into();
        obj_info.update_from_interfaces(&interfaces);
        // if the template is known, omit it. else build it from interfaces
        let mut database_template = false;
        let template = vm
            .get_template(Prefab::Hash(obj_ref.get_prefab().hash))
            .map_or_else(
                || Some(try_template_from_interfaces(&interfaces, obj)),
                |template| {
                    database_template = true;
                    Some(Ok(template))
                },
            )
            .transpose()?;

        Ok(FrozenObject {
            obj_info,
            template,
            database_template,
        })
    }

    pub fn freeze_object_sparse(obj: &VMObject, vm: &Rc<VM>) -> Result<Self, TemplateError> {
        let obj_ref = obj.borrow();
        let interfaces = ObjectInterfaces::from_object(&*obj_ref);
        let mut obj_info: ObjectInfo = obj.into();
        obj_info.update_from_interfaces(&interfaces);
        // if the template is known, omit it. else build it from interfaces
        let mut database_template = false;
        let template = vm
            .get_template(Prefab::Hash(obj_ref.get_prefab().hash))
            .map_or_else(
                || Some(try_template_from_interfaces(&interfaces, obj)),
                |_| {
                    database_template = true;
                    None
                },
            )
            .transpose()?;

        Ok(FrozenObject {
            obj_info,
            template,
            database_template,
        })
    }
}

fn try_template_from_interfaces(
    interfaces: &ObjectInterfaces,
    obj: &VMObject,
) -> Result<ObjectTemplate, TemplateError> {
    match *interfaces {
        ObjectInterfaces {
            structure: Some(structure),
            storage: None,
            memory_readable: None,
            memory_writable: None,
            logicable: None,
            source_code: None,
            circuit_holder: None,
            item: None,
            integrated_circuit: None,
            programmable: None,
            instructable: None,
            logic_stack: None,
            device: None,
            wireless_transmit: None,
            wireless_receive: None,
            network: None,
            plant: None,
            suit: None,
            chargeable: None,
            reagent_interface: None,
            fabricator: None,
            internal_atmosphere,
            thermal,
            human: None,
        } => {
            // completely generic structure? not sure how this got created but it technically
            // valid in the data model
            Ok(ObjectTemplate::Structure(StructureTemplate {
                prefab: obj.into(),
                internal_atmo_info: internal_atmosphere.map(Into::into),
                thermal_info: thermal.map(Into::into),
                structure: structure.into(),
            }))
        }
        ObjectInterfaces {
            structure: Some(structure),
            storage: Some(storage),
            memory_readable: None,
            memory_writable: None,
            logicable: None,
            source_code: None,
            circuit_holder: None,
            item: None,
            integrated_circuit: None,
            programmable: None,
            instructable: None,
            logic_stack: None,
            device: None,
            wireless_transmit: None,
            wireless_receive: None,
            network: None,
            plant: None,
            suit: None,
            chargeable: None,
            reagent_interface: None,
            fabricator: None,
            internal_atmosphere,
            thermal,
            human: None,
        } => Ok(ObjectTemplate::StructureSlots(StructureSlotsTemplate {
            prefab: obj.into(),
            internal_atmo_info: internal_atmosphere.map(Into::into),
            thermal_info: thermal.map(Into::into),
            structure: structure.into(),
            slots: storage.into(),
        })),
        ObjectInterfaces {
            structure: Some(structure),
            storage: Some(storage),
            memory_readable: None,
            memory_writable: None,
            logicable: Some(logic),
            source_code: _sc,
            circuit_holder: _ch,
            item: None,
            integrated_circuit: None,
            programmable: None,
            instructable: None,
            logic_stack: None,
            device: None,
            wireless_transmit: _wt,
            wireless_receive: _wr,
            network: None,
            plant: None,
            suit: None,
            chargeable: None,
            reagent_interface: None,
            fabricator: None,
            internal_atmosphere,
            thermal,
            human: None,
        } => Ok(ObjectTemplate::StructureLogic(StructureLogicTemplate {
            prefab: obj.into(),
            internal_atmo_info: internal_atmosphere.map(Into::into),
            thermal_info: thermal.map(Into::into),
            structure: structure.into(),
            slots: storage.into(),
            logic: logic.into(),
        })),
        ObjectInterfaces {
            structure: Some(structure),
            storage: Some(storage),
            memory_readable: None,
            memory_writable: None,
            logicable: Some(logic),
            source_code: _sc,
            circuit_holder: _ch,
            item: None,
            integrated_circuit: None,
            programmable: None,
            instructable: None,
            logic_stack: None,
            device: Some(device),
            wireless_transmit: _wt,
            wireless_receive: _wr,
            network: None,
            plant: None,
            suit: None,
            chargeable: None,
            reagent_interface: None,
            fabricator: None,
            internal_atmosphere,
            thermal,
            human: None,
        } => Ok(ObjectTemplate::StructureLogicDevice(
            StructureLogicDeviceTemplate {
                prefab: obj.into(),
                internal_atmo_info: internal_atmosphere.map(Into::into),
                thermal_info: thermal.map(Into::into),
                structure: structure.into(),
                slots: storage.into(),
                logic: logic.into(),
                device: device.into(),
            },
        )),
        ObjectInterfaces {
            structure: Some(structure),
            storage: Some(storage),
            memory_readable: Some(mem_r),
            memory_writable: _mem_w,
            logicable: Some(logic),
            source_code: _sc,
            circuit_holder: _ch,
            item: None,
            integrated_circuit: None,
            programmable: None,
            instructable: _inst,
            logic_stack: _logic_stack,
            device: Some(device),
            wireless_transmit: _wt,
            wireless_receive: _wr,
            network: None,
            plant: None,
            suit: None,
            chargeable: None,
            reagent_interface: None,
            fabricator: None,
            internal_atmosphere,
            thermal,
            human: None,
        } => Ok(ObjectTemplate::StructureLogicDeviceMemory(
            StructureLogicDeviceMemoryTemplate {
                prefab: obj.into(),
                internal_atmo_info: internal_atmosphere.map(Into::into),
                thermal_info: thermal.map(Into::into),
                structure: structure.into(),
                slots: storage.into(),
                logic: logic.into(),
                device: device.into(),
                memory: mem_r.into(),
            },
        )),

        //  Item Objects
        ObjectInterfaces {
            structure: None,
            storage: None,
            memory_readable: None,
            memory_writable: None,
            logicable: None,
            source_code: None,
            circuit_holder: None,
            item: Some(item),
            integrated_circuit: None,
            programmable: None,
            instructable: None,
            logic_stack: None,
            device: None,
            wireless_transmit: None,
            wireless_receive: None,
            network: None,
            plant: None,
            suit: None,
            chargeable: None,
            reagent_interface: None,
            fabricator: None,
            internal_atmosphere,
            thermal,
            human: None,
        } => Ok(ObjectTemplate::Item(ItemTemplate {
            prefab: obj.into(),
            internal_atmo_info: internal_atmosphere.map(Into::into),
            thermal_info: thermal.map(Into::into),
            item: item.into(),
        })),
        ObjectInterfaces {
            structure: None,
            storage: Some(storage),
            memory_readable: None,
            memory_writable: None,
            logicable: None,
            source_code: None,
            circuit_holder: None,
            item: Some(item),
            integrated_circuit: None,
            programmable: None,
            instructable: None,
            logic_stack: None,
            device: None,
            wireless_transmit: None,
            wireless_receive: None,
            network: None,
            plant: None,
            suit: None,
            chargeable: None,
            reagent_interface: None,
            fabricator: None,
            internal_atmosphere,
            thermal,
            human: None,
        } => Ok(ObjectTemplate::ItemSlots(ItemSlotsTemplate {
            prefab: obj.into(),
            internal_atmo_info: internal_atmosphere.map(Into::into),
            thermal_info: thermal.map(Into::into),
            item: item.into(),
            slots: storage.into(),
        })),
        ObjectInterfaces {
            structure: None,
            storage: Some(storage),
            memory_readable: None,
            memory_writable: None,
            logicable: Some(logic),
            source_code: _sc,
            circuit_holder: _ch,
            item: Some(item),
            integrated_circuit: None,
            programmable: None,
            instructable: None,
            logic_stack: None,
            device: None,
            wireless_transmit: _wt,
            wireless_receive: _wr,
            network: None,
            plant: None,
            suit: None,
            chargeable: None,
            reagent_interface: None,
            fabricator: None,
            internal_atmosphere,
            thermal,
            human: None,
        } => Ok(ObjectTemplate::ItemLogic(ItemLogicTemplate {
            prefab: obj.into(),
            internal_atmo_info: internal_atmosphere.map(Into::into),
            thermal_info: thermal.map(Into::into),
            item: item.into(),
            slots: storage.into(),
            logic: logic.into(),
        })),
        ObjectInterfaces {
            structure: None,
            storage: Some(storage),
            memory_readable: Some(mem_r),
            memory_writable: _mem_w,
            logicable: Some(logic),
            source_code: _sc,
            circuit_holder: _ch,
            item: Some(item),
            integrated_circuit: _ic,
            programmable: None,
            instructable: _inst,
            logic_stack: _logic_stack,
            device: None,
            wireless_transmit: _wt,
            wireless_receive: _wr,
            network: None,
            plant: None,
            suit: None,
            chargeable: None,
            reagent_interface: None,
            fabricator: None,
            internal_atmosphere,
            thermal,
            human: None,
        } => Ok(ObjectTemplate::ItemLogicMemory(ItemLogicMemoryTemplate {
            prefab: obj.into(),
            internal_atmo_info: internal_atmosphere.map(Into::into),
            thermal_info: thermal.map(Into::into),
            item: item.into(),
            slots: storage.into(),
            logic: logic.into(),
            memory: mem_r.into(),
        })),
        ObjectInterfaces {
            structure: None,
            storage: Some(storage),
            memory_readable: None,
            memory_writable: None,
            logicable: None,
            source_code: None,
            circuit_holder: None,
            item: None,
            integrated_circuit: None,
            programmable: None,
            instructable: None,
            logic_stack: None,
            device: None,
            wireless_transmit: None,
            wireless_receive: None,
            network: None,
            plant: None,
            suit: None,
            chargeable: None,
            reagent_interface: None,
            fabricator: None,
            internal_atmosphere: None,
            thermal: Some(_),
            human: Some(human),
        } => Ok(ObjectTemplate::Human(HumanTemplate {
            prefab: PrefabInfo {
                prefab_name: "Character".to_string(),
                prefab_hash: 294335127,
                desc: "Charater".to_string(),
                name: "Charater".to_string(),
            },
            species: human.get_species(),
            slots: storage.into(),
        })),
        _ => Err(TemplateError::NonConformingObject(obj.get_id())),
    }
}

impl From<&VMObject> for PrefabInfo {
    fn from(obj: &VMObject) -> Self {
        let obj_ref = obj.borrow();
        let obj_prefab = obj_ref.get_prefab();
        let prefab_lookup = StationpediaPrefab::from_repr(obj_prefab.hash);
        PrefabInfo {
            prefab_name: obj_prefab.value.clone(),
            prefab_hash: obj_prefab.hash,
            name: prefab_lookup
                .and_then(|prefab| prefab.get_str("name"))
                .unwrap_or("")
                .to_string(),
            desc: prefab_lookup
                .and_then(|prefab| prefab.get_str("desc"))
                .unwrap_or("")
                .to_string(),
        }
    }
}
impl From<LogicableRef<'_>> for LogicInfo {
    fn from(logic: LogicableRef) -> Self {
        // Logicable: Storage -> !None
        let storage = logic.as_storage().unwrap();
        let wt = logic.as_wireless_transmit();
        let wr = logic.as_wireless_receive();
        let circuit_holder = logic.as_circuit_holder();
        LogicInfo {
            logic_slot_types: storage
                .get_slots()
                .iter()
                .enumerate()
                .map(|(index, slot)| {
                    (
                        index as u32,
                        LogicSlotType::iter()
                            .filter_map(|slt| {
                                let readable = slot.readable_logic.contains(&slt);
                                let writeable = slot.writeable_logic.contains(&slt);
                                if readable && writeable {
                                    Some((slt, MemoryAccess::ReadWrite))
                                } else if readable {
                                    Some((slt, MemoryAccess::Read))
                                } else if writeable {
                                    Some((slt, MemoryAccess::Write))
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    )
                })
                .collect(),
            logic_types: logic
                .valid_logic_types()
                .iter()
                .filter_map(|lt| {
                    let readable = logic.can_logic_read(*lt);
                    let writeable = logic.can_logic_write(*lt);
                    if readable && writeable {
                        Some((*lt, MemoryAccess::ReadWrite))
                    } else if readable {
                        Some((*lt, MemoryAccess::Read))
                    } else if writeable {
                        Some((*lt, MemoryAccess::Write))
                    } else {
                        None
                    }
                })
                .collect(),

            modes: logic
                .known_modes()
                .map(|modes| modes.iter().cloned().collect()),
            transmission_receiver: wr.is_some(),
            wireless_logic: wt.is_some(),
            circuit_holder: circuit_holder.is_some(),
        }
    }
}

impl From<ItemRef<'_>> for ItemInfo {
    fn from(item: ItemRef<'_>) -> Self {
        ItemInfo {
            consumable: item.consumable(),
            filter_type: item.filter_type(),
            ingredient: item.ingredient(),
            max_quantity: item.max_quantity(),
            reagents: item.reagents().cloned(),
            slot_class: item.slot_class(),
            sorting_class: item.sorting_class(),
        }
    }
}

impl From<DeviceRef<'_>> for DeviceInfo {
    fn from(device: DeviceRef) -> Self {
        let reagents: BTreeMap<i32, f64> = device.get_reagents().iter().copied().collect();
        DeviceInfo {
            connection_list: device
                .connection_list()
                .iter()
                .map(|conn| conn.to_info())
                .collect(),
            device_pins_length: device.device_pins().map(|pins| pins.len() as u32),
            has_reagents: device.has_reagents(),
            has_lock_state: device.has_lock_state(),
            has_mode_state: device.has_mode_state(),
            has_open_state: device.has_mode_state(),
            has_on_off_state: device.has_on_off_state(),
            has_color_state: device.has_color_state(),
            has_atmosphere: device.has_atmosphere(),
            has_activate_state: device.has_activate_state(),
        }
    }
}

impl From<StructureRef<'_>> for StructureInfo {
    fn from(value: StructureRef) -> Self {
        StructureInfo {
            small_grid: value.is_small_grid(),
        }
    }
}

impl From<MemoryReadableRef<'_>> for MemoryInfo {
    fn from(mem_r: MemoryReadableRef<'_>) -> Self {
        let mem_w = mem_r.as_memory_writable();
        MemoryInfo {
            instructions: None, // TODO: map info from `Instructable` and LogicStack traits
            memory_access: if mem_w.is_some() {
                MemoryAccess::ReadWrite
            } else {
                MemoryAccess::Read
            },
            memory_size: mem_r.memory_size() as u32,
        }
    }
}

impl From<InternalAtmosphereRef<'_>> for InternalAtmoInfo {
    fn from(internal_atmo: InternalAtmosphereRef<'_>) -> Self {
        InternalAtmoInfo {
            volume: internal_atmo.get_volume() as f32,
        }
    }
}

impl From<ThermalRef<'_>> for ThermalInfo {
    fn from(thermal: ThermalRef<'_>) -> Self {
        ThermalInfo {
            convection_factor: thermal.get_convection_factor(),
            radiation_factor: thermal.get_radiation_factor(),
        }
    }
}

impl From<StorageRef<'_>> for Vec<SlotInfo> {
    fn from(storage: StorageRef<'_>) -> Self {
        storage
            .get_slots()
            .iter()
            .map(|slot| SlotInfo {
                name: slot.name.clone(),
                typ: slot.typ,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use serde_derive::Deserialize;
    use serde_json;
    use std::collections::BTreeMap;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    use super::ObjectTemplate;

    static INIT: std::sync::Once = std::sync::Once::new();

    fn setup() {
        INIT.call_once(|| {
            let _ = color_eyre::install();
        })
    }

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct Database {
        pub prefabs: BTreeMap<String, ObjectTemplate>,
    }

    #[test]
    fn all_database_prefabs_parse() -> color_eyre::Result<()> {
        setup();
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d = d.parent().unwrap().join("data").join("database.json");
        println!("loading database from {}", d.display());

        let _database: Database = serde_json::from_reader(BufReader::new(File::open(d)?))?;

        Ok(())
    }
}
