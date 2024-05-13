use std::collections::BTreeMap;

use crate::{
    errors::TemplateError,
    network::{Connection, ConnectionRole, ConnectionType},
    vm::{
        enums::{
            basic_enums::{Class as SlotClass, GasType, SortingClass},
            prefabs::StationpediaPrefab,
            script_enums::{LogicSlotType, LogicType},
        },
        object::{
            generic::structs::{
                Generic, GenericItem, GenericItemLogicable,
                GenericItemLogicableMemoryReadWriteable, GenericItemLogicableMemoryReadable,
                GenericItemStorage, GenericLogicable, GenericLogicableDevice,
                GenericLogicableDeviceMemoryReadWriteable, GenericLogicableDeviceMemoryReadable,
                GenericStorage,
            },
            traits::*,
            LogicField, Name, Slot,
        },
    },
};
use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use super::{stationpedia, MemoryAccess, ObjectID, VMObject};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObjectTemplate {
    Structure(StructureTemplate),
    StructureSlots(StructureSlotsTemplate),
    StructureLogic(StructureLogicTemplate),
    StructureLogicDevice(StructureLogicDeviceTemplate),
    StructureLogicDeviceMemory(StructureLogicDeviceMemoryTemplate),
    Item(ItemTemplate),
    ItemSlots(ItemSlotsTemplate),
    ItemLogic(ItemLogicTemplate),
    ItemLogicMemory(ItemLogicMemoryTemplate),
}

impl ObjectTemplate {
    pub fn prefab_info(&self) -> &PrefabInfo {
        use ObjectTemplate::*;
        match self {
            Structure(s) => &s.prefab,
            StructureSlots(s) => &s.prefab,
            StructureLogic(s) => &s.prefab,
            StructureLogicDevice(s) => &s.prefab,
            StructureLogicDeviceMemory(s) => &s.prefab,
            Item(i) => &i.prefab,
            ItemSlots(i) => &i.prefab,
            ItemLogic(i) => &i.prefab,
            ItemLogicMemory(i) => &i.prefab,
        }
    }

    pub fn object_info(&self) -> Option<&ObjectInfo> {
        use ObjectTemplate::*;
        match self {
            Structure(s) => s.object.as_ref(),
            StructureSlots(s) => s.object.as_ref(),
            StructureLogic(s) => s.object.as_ref(),
            StructureLogicDevice(s) => s.object.as_ref(),
            StructureLogicDeviceMemory(s) => s.object.as_ref(),
            Item(i) => i.object.as_ref(),
            ItemSlots(i) => i.object.as_ref(),
            ItemLogic(i) => i.object.as_ref(),
            ItemLogicMemory(i) => i.object.as_ref(),
        }
    }

    pub fn build(&self, id: ObjectID) -> VMObject {
        if let Some(obj) = stationpedia::object_from_prefab_template(&self, id) {
            obj
        } else {
            self.build_generic(id)
        }
    }

    pub fn connected_networks(&self) -> Vec<ObjectID> {
        use ObjectTemplate::*;
        match self {
            StructureLogicDevice(s) => s
                .device
                .connection_list
                .iter()
                .filter_map(|conn| conn.network.as_ref())
                .copied()
                .collect(),
            StructureLogicDeviceMemory(s) => s
                .device
                .connection_list
                .iter()
                .filter_map(|conn| conn.network.as_ref())
                .copied()
                .collect(),
            _ => vec![],
        }
    }

    pub fn contained_object_ids(&self) -> Vec<ObjectID> {
        use ObjectTemplate::*;
        match self {
            StructureSlots(s) => s
                .slots
                .iter()
                .filter_map(|info| {
                    info.occupant
                        .as_ref()
                        .map(|obj| obj.object_info().map(|obj_info| obj_info.id).flatten())
                })
                .flatten()
                .collect(),
            StructureLogic(s) => s
                .slots
                .iter()
                .filter_map(|info| {
                    info.occupant
                        .as_ref()
                        .map(|obj| obj.object_info().map(|obj_info| obj_info.id).flatten())
                })
                .flatten()
                .collect(),
            StructureLogicDevice(s) => s
                .slots
                .iter()
                .filter_map(|info| {
                    info.occupant
                        .as_ref()
                        .map(|obj| obj.object_info().map(|obj_info| obj_info.id).flatten())
                })
                .flatten()
                .collect(),
            StructureLogicDeviceMemory(s) => s
                .slots
                .iter()
                .filter_map(|info| {
                    info.occupant
                        .as_ref()
                        .map(|obj| obj.object_info().map(|obj_info| obj_info.id).flatten())
                })
                .flatten()
                .collect(),
            ItemSlots(i) => i
                .slots
                .iter()
                .filter_map(|info| {
                    info.occupant
                        .as_ref()
                        .map(|obj| obj.object_info().map(|obj_info| obj_info.id).flatten())
                })
                .flatten()
                .collect(),
            ItemLogic(i) => i
                .slots
                .iter()
                .filter_map(|info| {
                    info.occupant
                        .as_ref()
                        .map(|obj| obj.object_info().map(|obj_info| obj_info.id).flatten())
                })
                .flatten()
                .collect(),
            ItemLogicMemory(i) => i
                .slots
                .iter()
                .filter_map(|info| {
                    info.occupant
                        .as_ref()
                        .map(|obj| obj.object_info().map(|obj_info| obj_info.id).flatten())
                })
                .flatten()
                .collect(),
            _ => vec![],
        }
    }

    pub fn templates_from_slots(&self) -> Vec<Option<ObjectTemplate>> {
        use ObjectTemplate::*;
        match self {
            StructureSlots(s) => s.slots.iter().map(|info| info.occupant.clone()).collect(),
            StructureLogic(s) => s.slots.iter().map(|info| info.occupant.clone()).collect(),
            StructureLogicDevice(s) => s.slots.iter().map(|info| info.occupant.clone()).collect(),
            StructureLogicDeviceMemory(s) => {
                s.slots.iter().map(|info| info.occupant.clone()).collect()
            }
            ItemSlots(i) => i.slots.iter().map(|info| info.occupant.clone()).collect(),
            ItemLogic(i) => i.slots.iter().map(|info| info.occupant.clone()).collect(),
            ItemLogicMemory(i) => i.slots.iter().map(|info| info.occupant.clone()).collect(),
            _ => vec![],
        }
    }

    fn build_generic(&self, id: ObjectID) -> VMObject {
        use ObjectTemplate::*;
        match self {
            Structure(s) => VMObject::new(Generic {
                id,
                prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                name: Name::new(&s.prefab.name),
                small_grid: s.structure.small_grid,
            }),
            StructureSlots(s) => VMObject::new(GenericStorage {
                id,
                prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                name: Name::new(&s.prefab.name),
                small_grid: s.structure.small_grid,
                slots: s
                    .slots
                    .iter()
                    .enumerate()
                    .map(|(index, info)| Slot {
                        parent: id,
                        index,
                        name: info.name.clone(),
                        typ: info.typ,
                        readable_logic: Vec::new(),
                        writeable_logic: Vec::new(),
                        occupant: None,
                    })
                    .collect(),
            }),
            StructureLogic(s) => VMObject::new(GenericLogicable {
                id,
                prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                name: Name::new(&s.prefab.name),
                small_grid: s.structure.small_grid,
                slots: s
                    .slots
                    .iter()
                    .enumerate()
                    .map(|(index, info)| Slot {
                        parent: id,
                        index,
                        name: info.name.clone(),
                        typ: info.typ,
                        readable_logic: s
                            .logic
                            .logic_slot_types
                            .get(&(index as u32))
                            .map(|s_info| {
                                s_info
                                    .slot_types
                                    .iter()
                                    .filter_map(|(key, access)| match access {
                                        MemoryAccess::Read | MemoryAccess::ReadWrite => Some(key),
                                        _ => None,
                                    })
                                    .copied()
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_else(|| Vec::new()),
                        writeable_logic: s
                            .logic
                            .logic_slot_types
                            .get(&(index as u32))
                            .map(|s_info| {
                                s_info
                                    .slot_types
                                    .iter()
                                    .filter_map(|(key, access)| match access {
                                        MemoryAccess::Write | MemoryAccess::ReadWrite => Some(key),
                                        _ => None,
                                    })
                                    .copied()
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_else(|| Vec::new()),
                        occupant: None,
                    })
                    .collect(),
                fields: s
                    .logic
                    .logic_types
                    .types
                    .iter()
                    .map(|(key, access)| {
                        (
                            *key,
                            LogicField {
                                field_type: *access,
                                value: 0.0,
                            },
                        )
                    })
                    .collect(),
                modes: s.logic.modes.clone(),
            }),
            StructureLogicDevice(s) => VMObject::new(GenericLogicableDevice {
                id,
                prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                name: Name::new(&s.prefab.name),
                small_grid: s.structure.small_grid,
                slots: s
                    .slots
                    .iter()
                    .enumerate()
                    .map(|(index, info)| Slot {
                        parent: id,
                        index,
                        name: info.name.clone(),
                        typ: info.typ,
                        readable_logic: s
                            .logic
                            .logic_slot_types
                            .get(&(index as u32))
                            .map(|s_info| {
                                s_info
                                    .slot_types
                                    .iter()
                                    .filter_map(|(key, access)| match access {
                                        MemoryAccess::Read | MemoryAccess::ReadWrite => Some(key),
                                        _ => None,
                                    })
                                    .copied()
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_else(|| Vec::new()),
                        writeable_logic: s
                            .logic
                            .logic_slot_types
                            .get(&(index as u32))
                            .map(|s_info| {
                                s_info
                                    .slot_types
                                    .iter()
                                    .filter_map(|(key, access)| match access {
                                        MemoryAccess::Write | MemoryAccess::ReadWrite => Some(key),
                                        _ => None,
                                    })
                                    .copied()
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_else(|| Vec::new()),
                        occupant: None,
                    })
                    .collect(),
                fields: s
                    .logic
                    .logic_types
                    .types
                    .iter()
                    .map(|(key, access)| {
                        (
                            *key,
                            LogicField {
                                field_type: *access,
                                value: 0.0,
                            },
                        )
                    })
                    .collect(),
                modes: s.logic.modes.clone(),
                connections: s
                    .device
                    .connection_list
                    .iter()
                    .map(|conn_info| Connection::from_info(conn_info.typ, conn_info.role))
                    .collect(),
                pins: s
                    .device
                    .device_pins_length
                    .map(|pins_len| vec![None; pins_len]),
                device_info: s.device.clone(),
            }),
            StructureLogicDeviceMemory(s)
                if matches!(s.memory.memory_access, MemoryAccess::Read) =>
            {
                VMObject::new(GenericLogicableDeviceMemoryReadable {
                    id,
                    prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                    name: Name::new(&s.prefab.name),
                    small_grid: s.structure.small_grid,
                    slots: s
                        .slots
                        .iter()
                        .enumerate()
                        .map(|(index, info)| Slot {
                            parent: id,
                            index,
                            name: info.name.clone(),
                            typ: info.typ,
                            readable_logic: s
                                .logic
                                .logic_slot_types
                                .get(&(index as u32))
                                .map(|s_info| {
                                    s_info
                                        .slot_types
                                        .iter()
                                        .filter_map(|(key, access)| match access {
                                            MemoryAccess::Read | MemoryAccess::ReadWrite => {
                                                Some(key)
                                            }
                                            _ => None,
                                        })
                                        .copied()
                                        .collect::<Vec<_>>()
                                })
                                .unwrap_or_else(|| Vec::new()),
                            writeable_logic: s
                                .logic
                                .logic_slot_types
                                .get(&(index as u32))
                                .map(|s_info| {
                                    s_info
                                        .slot_types
                                        .iter()
                                        .filter_map(|(key, access)| match access {
                                            MemoryAccess::Write | MemoryAccess::ReadWrite => {
                                                Some(key)
                                            }
                                            _ => None,
                                        })
                                        .copied()
                                        .collect::<Vec<_>>()
                                })
                                .unwrap_or_else(|| Vec::new()),
                            occupant: None,
                        })
                        .collect(),
                    fields: s
                        .logic
                        .logic_types
                        .types
                        .iter()
                        .map(|(key, access)| {
                            (
                                *key,
                                LogicField {
                                    field_type: *access,
                                    value: 0.0,
                                },
                            )
                        })
                        .collect(),
                    modes: s.logic.modes.clone(),
                    connections: s
                        .device
                        .connection_list
                        .iter()
                        .map(|conn_info| Connection::from_info(conn_info.typ, conn_info.role))
                        .collect(),
                    pins: s
                        .device
                        .device_pins_length
                        .map(|pins_len| vec![None; pins_len]),
                    device_info: s.device.clone(),
                    memory: vec![0.0; s.memory.memory_size as usize],
                })
            }
            StructureLogicDeviceMemory(s) => {
                VMObject::new(GenericLogicableDeviceMemoryReadWriteable {
                    id,
                    prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                    name: Name::new(&s.prefab.name),
                    small_grid: s.structure.small_grid,
                    slots: s
                        .slots
                        .iter()
                        .enumerate()
                        .map(|(index, info)| Slot {
                            parent: id,
                            index,
                            name: info.name.clone(),
                            typ: info.typ,
                            readable_logic: s
                                .logic
                                .logic_slot_types
                                .get(&(index as u32))
                                .map(|s_info| {
                                    s_info
                                        .slot_types
                                        .iter()
                                        .filter_map(|(key, access)| match access {
                                            MemoryAccess::Read | MemoryAccess::ReadWrite => {
                                                Some(key)
                                            }
                                            _ => None,
                                        })
                                        .copied()
                                        .collect::<Vec<_>>()
                                })
                                .unwrap_or_else(|| Vec::new()),
                            writeable_logic: s
                                .logic
                                .logic_slot_types
                                .get(&(index as u32))
                                .map(|s_info| {
                                    s_info
                                        .slot_types
                                        .iter()
                                        .filter_map(|(key, access)| match access {
                                            MemoryAccess::Write | MemoryAccess::ReadWrite => {
                                                Some(key)
                                            }
                                            _ => None,
                                        })
                                        .copied()
                                        .collect::<Vec<_>>()
                                })
                                .unwrap_or_else(|| Vec::new()),
                            occupant: None,
                        })
                        .collect(),
                    fields: s
                        .logic
                        .logic_types
                        .types
                        .iter()
                        .map(|(key, access)| {
                            (
                                *key,
                                LogicField {
                                    field_type: *access,
                                    value: 0.0,
                                },
                            )
                        })
                        .collect(),
                    modes: s.logic.modes.clone(),
                    connections: s
                        .device
                        .connection_list
                        .iter()
                        .map(|conn_info| Connection::from_info(conn_info.typ, conn_info.role))
                        .collect(),
                    pins: s
                        .device
                        .device_pins_length
                        .map(|pins_len| vec![None; pins_len]),
                    device_info: s.device.clone(),
                    memory: vec![0.0; s.memory.memory_size as usize],
                })
            }
            Item(i) => VMObject::new(GenericItem {
                id,
                prefab: Name::from_prefab_name(&i.prefab.prefab_name),
                name: Name::new(&i.prefab.name),
                item_info: i.item.clone(),
                parent_slot: None,
            }),
            ItemSlots(i) => VMObject::new(GenericItemStorage {
                id,
                prefab: Name::from_prefab_name(&i.prefab.prefab_name),
                name: Name::new(&i.prefab.name),
                item_info: i.item.clone(),
                parent_slot: None,
                slots: i
                    .slots
                    .iter()
                    .enumerate()
                    .map(|(index, info)| Slot {
                        parent: id,
                        index,
                        name: info.name.clone(),
                        typ: info.typ,
                        readable_logic: Vec::new(),
                        writeable_logic: Vec::new(),
                        occupant: None,
                    })
                    .collect(),
            }),
            ItemLogic(i) => VMObject::new(GenericItemLogicable {
                id,
                prefab: Name::from_prefab_name(&i.prefab.prefab_name),
                name: Name::new(&i.prefab.name),
                item_info: i.item.clone(),
                parent_slot: None,
                slots: i
                    .slots
                    .iter()
                    .enumerate()
                    .map(|(index, info)| Slot {
                        parent: id,
                        index,
                        name: info.name.clone(),
                        typ: info.typ,
                        readable_logic: i
                            .logic
                            .logic_slot_types
                            .get(&(index as u32))
                            .map(|s_info| {
                                s_info
                                    .slot_types
                                    .iter()
                                    .filter_map(|(key, access)| match access {
                                        MemoryAccess::Read | MemoryAccess::ReadWrite => Some(key),
                                        _ => None,
                                    })
                                    .copied()
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_else(|| Vec::new()),
                        writeable_logic: i
                            .logic
                            .logic_slot_types
                            .get(&(index as u32))
                            .map(|s_info| {
                                s_info
                                    .slot_types
                                    .iter()
                                    .filter_map(|(key, access)| match access {
                                        MemoryAccess::Write | MemoryAccess::ReadWrite => Some(key),
                                        _ => None,
                                    })
                                    .copied()
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_else(|| Vec::new()),
                        occupant: None,
                    })
                    .collect(),
                fields: i
                    .logic
                    .logic_types
                    .types
                    .iter()
                    .map(|(key, access)| {
                        (
                            *key,
                            LogicField {
                                field_type: *access,
                                value: 0.0,
                            },
                        )
                    })
                    .collect(),
                modes: i.logic.modes.clone(),
            }),
            ItemLogicMemory(i) if matches!(i.memory.memory_access, MemoryAccess::Read) => {
                VMObject::new(GenericItemLogicableMemoryReadable {
                    id,
                    prefab: Name::from_prefab_name(&i.prefab.prefab_name),
                    name: Name::new(&i.prefab.name),
                    item_info: i.item.clone(),
                    parent_slot: None,
                    slots: i
                        .slots
                        .iter()
                        .enumerate()
                        .map(|(index, info)| Slot {
                            parent: id,
                            index,
                            name: info.name.clone(),
                            typ: info.typ,
                            readable_logic: i
                                .logic
                                .logic_slot_types
                                .get(&(index as u32))
                                .map(|s_info| {
                                    s_info
                                        .slot_types
                                        .iter()
                                        .filter_map(|(key, access)| match access {
                                            MemoryAccess::Read | MemoryAccess::ReadWrite => {
                                                Some(key)
                                            }
                                            _ => None,
                                        })
                                        .copied()
                                        .collect::<Vec<_>>()
                                })
                                .unwrap_or_else(|| Vec::new()),
                            writeable_logic: i
                                .logic
                                .logic_slot_types
                                .get(&(index as u32))
                                .map(|s_info| {
                                    s_info
                                        .slot_types
                                        .iter()
                                        .filter_map(|(key, access)| match access {
                                            MemoryAccess::Write | MemoryAccess::ReadWrite => {
                                                Some(key)
                                            }
                                            _ => None,
                                        })
                                        .copied()
                                        .collect::<Vec<_>>()
                                })
                                .unwrap_or_else(|| Vec::new()),
                            occupant: None,
                        })
                        .collect(),
                    fields: i
                        .logic
                        .logic_types
                        .types
                        .iter()
                        .map(|(key, access)| {
                            (
                                *key,
                                LogicField {
                                    field_type: *access,
                                    value: 0.0,
                                },
                            )
                        })
                        .collect(),
                    modes: i.logic.modes.clone(),
                    memory: vec![0.0; i.memory.memory_size as usize],
                })
            }
            ItemLogicMemory(i) => VMObject::new(GenericItemLogicableMemoryReadWriteable {
                id,
                prefab: Name::from_prefab_name(&i.prefab.prefab_name),
                name: Name::new(&i.prefab.name),
                item_info: i.item.clone(),
                parent_slot: None,
                slots: i
                    .slots
                    .iter()
                    .enumerate()
                    .map(|(index, info)| Slot {
                        parent: id,
                        index,
                        name: info.name.clone(),
                        typ: info.typ,
                        readable_logic: i
                            .logic
                            .logic_slot_types
                            .get(&(index as u32))
                            .map(|s_info| {
                                s_info
                                    .slot_types
                                    .iter()
                                    .filter_map(|(key, access)| match access {
                                        MemoryAccess::Read | MemoryAccess::ReadWrite => Some(key),
                                        _ => None,
                                    })
                                    .copied()
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_else(|| Vec::new()),
                        writeable_logic: i
                            .logic
                            .logic_slot_types
                            .get(&(index as u32))
                            .map(|s_info| {
                                s_info
                                    .slot_types
                                    .iter()
                                    .filter_map(|(key, access)| match access {
                                        MemoryAccess::Write | MemoryAccess::ReadWrite => Some(key),
                                        _ => None,
                                    })
                                    .copied()
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_else(|| Vec::new()),
                        occupant: None,
                    })
                    .collect(),
                fields: i
                    .logic
                    .logic_types
                    .types
                    .iter()
                    .map(|(key, access)| {
                        (
                            *key,
                            LogicField {
                                field_type: *access,
                                value: 0.0,
                            },
                        )
                    })
                    .collect(),
                modes: i.logic.modes.clone(),
                memory: vec![0.0; i.memory.memory_size as usize],
            }),
        }
    }

    pub fn freeze_object(obj: &VMObject) -> Result<Self, TemplateError> {
        let obj_ref = obj.borrow();
        let interfaces = ObjectInterfaces::from_object(&*obj_ref);
        match interfaces {
            ObjectInterfaces {
                structure: Some(stru),
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
            } => {
                let prefab_lookup = StationpediaPrefab::from_repr(obj_ref.get_prefab().hash);
                // completely generic structure? not sure how this got created but it technically
                // valid in the data model
                Ok(ObjectTemplate::Structure(StructureTemplate {
                    object: Some(ObjectInfo {
                        name: Some(obj_ref.get_name().value()),
                        id: Some(*obj_ref.get_id()),
                    }),
                    prefab: PrefabInfo {
                        prefab_name: obj_ref.get_prefab().value,
                        prefab_hash: obj_ref.get_prefab().hash,
                        name: prefab_lookup
                            .map(|prefab| prefab.get_str("name"))
                            .flatten()
                            .unwrap_or("")
                            .to_string(),
                        desc: prefab_lookup
                            .map(|prefab| prefab.get_str("desc"))
                            .flatten()
                            .unwrap_or("")
                            .to_string(),
                    },
                    structure: StructureInfo {
                        small_grid: stru.is_small_grid(),
                    },
                }))
            }
            ObjectInterfaces {
                structure: Some(stru),
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
            } => {
                let prefab_lookup = StationpediaPrefab::from_repr(obj_ref.get_prefab().hash);
                Ok(ObjectTemplate::StructureSlots(StructureSlotsTemplate {
                    object: Some(ObjectInfo {
                        name: Some(obj_ref.get_name().value()),
                        id: Some(*obj_ref.get_id()),
                    }),
                    prefab: PrefabInfo {
                        prefab_name: obj_ref.get_prefab().value,
                        prefab_hash: obj_ref.get_prefab().hash,
                        name: prefab_lookup
                            .map(|prefab| prefab.get_str("name"))
                            .flatten()
                            .unwrap_or("")
                            .to_string(),
                        desc: prefab_lookup
                            .map(|prefab| prefab.get_str("desc"))
                            .flatten()
                            .unwrap_or("")
                            .to_string(),
                    },
                    structure: StructureInfo {
                        small_grid: stru.is_small_grid(),
                    },
                    slots: storage
                        .get_slots()
                        .iter()
                        .map(|slot| {
                            Ok(SlotInfo {
                                name: slot.name.clone(),
                                typ: slot.typ,
                                occupant: slot
                                    .occupant
                                    .map(|occupant| ObjectTemplate::freeze_object(occupant))
                                    .map_or(Ok(None), |v| v.map(Some))?,
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                }))
            }
            ObjectInterfaces {
                structure: Some(stru),
                storage: Some(storage),
                memory_readable: None,
                memory_writable: None,
                logicable: Some(logic),
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
            } => {
                let prefab_lookup = StationpediaPrefab::from_repr(obj_ref.get_prefab().hash);
                Ok(ObjectTemplate::StructureLogic(StructureLogicTemplate {
                    object: Some(ObjectInfo {
                        name: Some(obj_ref.get_name().value()),
                        id: Some(*obj_ref.get_id()),
                    }),
                    prefab: PrefabInfo {
                        prefab_name: obj_ref.get_prefab().value,
                        prefab_hash: obj_ref.get_prefab().hash,
                        name: prefab_lookup
                            .map(|prefab| prefab.get_str("name"))
                            .flatten()
                            .unwrap_or("")
                            .to_string(),
                        desc: prefab_lookup
                            .map(|prefab| prefab.get_str("desc"))
                            .flatten()
                            .unwrap_or("")
                            .to_string(),
                    },
                    structure: StructureInfo {
                        small_grid: stru.is_small_grid(),
                    },
                    slots: storage
                        .get_slots()
                        .iter()
                        .map(|slot| {
                            Ok(SlotInfo {
                                name: slot.name.clone(),
                                typ: slot.typ,
                                occupant: slot
                                    .occupant
                                    .map(|occupant| ObjectTemplate::freeze_object(occupant))
                                    .map_or(Ok(None), |v| v.map(Some))?,
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                    logic: LogicInfo {
                        logic_slot_types: storage
                            .get_slots()
                            .iter()
                            .enumerate()
                            .map(|(index, slot)| {
                                (
                                    index as u32,
                                    LogicSlotTypes {
                                        slot_types: LogicSlotType::iter()
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
                                    },
                                )
                            })
                            .collect(),
                        logic_types: LogicTypes {
                            types: logic
                                .valid_logic_types()
                                .iter()
                                .filter_map(|lt| {
                                    let readable = logic.can_logic_read(lt);
                                    let writeable = logic.can_logic_write(lt);
                                    if readable && writeable {
                                        Some((lt, MemoryAccess::ReadWrite))
                                    } else if readable {
                                        Some((lt, MemoryAccess::Read))
                                    } else if writeable {
                                        Some((lt, MemoryAccess::Write))
                                    } else {
                                        None
                                    }
                                })
                                .collect(),
                        },
                        modes: logic.known_modes().map(|modes| modes.iter().collect()),
                        logic_values: logic
                            .valid_logic_types()
                            .iter()
                            .filter_map(|lt| logic.get_logic(lt).ok()),
                        transmission_receiver: false,
                        wireless_logic: false,
                        circuit_holder: false,
                    },
                }))
            }
            ObjectInterfaces {
                structure: Some(stru),
                storage: Some(storage),
                memory_readable: None,
                memory_writable: None,
                logicable: Some(logic),
                source_code: None,
                circuit_holder: None,
                item: None,
                integrated_circuit: None,
                programmable: None,
                instructable: None,
                logic_stack: None,
                device: Some(device),
                wireless_transmit: None,
                wireless_receive: None,
                network: None,
            } => {
                let prefab_lookup = StationpediaPrefab::from_repr(obj_ref.get_prefab().hash);
                Ok(ObjectTemplate::StructureLogicDevice(
                    StructureLogicDeviceTemplate {
                        object: Some(ObjectInfo {
                            name: Some(obj_ref.get_name().value()),
                            id: Some(*obj_ref.get_id()),
                        }),
                        prefab: PrefabInfo {
                            prefab_name: obj_ref.get_prefab().value,
                            prefab_hash: obj_ref.get_prefab().hash,
                            name: prefab_lookup
                                .map(|prefab| prefab.get_str("name"))
                                .flatten()
                                .unwrap_or("")
                                .to_string(),
                            desc: prefab_lookup
                                .map(|prefab| prefab.get_str("desc"))
                                .flatten()
                                .unwrap_or("")
                                .to_string(),
                        },
                        structure: StructureInfo {
                            small_grid: stru.is_small_grid(),
                        },
                        slots: storage
                            .get_slots()
                            .iter()
                            .map(|slot| {
                                Ok(SlotInfo {
                                    name: slot.name.clone(),
                                    typ: slot.typ,
                                    occupant: slot
                                        .occupant
                                        .map(|occupant| ObjectTemplate::freeze_object(occupant))
                                        .map_or(Ok(None), |v| v.map(Some))?,
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()?,
                        logic: LogicInfo {
                            logic_slot_types: storage
                                .get_slots()
                                .iter()
                                .enumerate()
                                .map(|(index, slot)| {
                                    (
                                        index as u32,
                                        LogicSlotTypes {
                                            slot_types: LogicSlotType::iter()
                                                .filter_map(|slt| {
                                                    let readable =
                                                        slot.readable_logic.contains(&slt);
                                                    let writeable =
                                                        slot.writeable_logic.contains(&slt);
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
                                        },
                                    )
                                })
                                .collect(),
                            logic_types: LogicTypes {
                                types: logic
                                    .valid_logic_types()
                                    .iter()
                                    .filter_map(|lt| {
                                        let readable = logic.can_logic_read(lt);
                                        let writeable = logic.can_logic_write(lt);
                                        if readable && writeable {
                                            Some((lt, MemoryAccess::ReadWrite))
                                        } else if readable {
                                            Some((lt, MemoryAccess::Read))
                                        } else if writeable {
                                            Some((lt, MemoryAccess::Write))
                                        } else {
                                            None
                                        }
                                    })
                                    .collect(),
                            },
                            modes: logic.known_modes().map(|modes| modes.iter().collect()),
                            logic_values: logic
                                .valid_logic_types()
                                .iter()
                                .filter_map(|lt| logic.get_logic(lt).ok()),
                            transmission_receiver: false,
                            wireless_logic: false,
                            circuit_holder: false,
                        },
                        device: DeviceInfo {
                            connection_list: device
                                .connection_list()
                                .iter()
                                .map(|conn| conn.to_info()),
                            device_pins_length: device.device_pins().map(|pins| pins.len()),
                            device_pins: device.device_pins().map(|pins| pins.iter().collect()),
                            has_reagents: device.has_reagents(),
                            has_lock_state: device.has_lock_state(),
                            has_mode_state: device.has_mode_state(),
                            has_open_state: device.has_mode_state(),
                            has_on_off_state: device.has_on_off_state(),
                            has_color_state: device.has_color_state(),
                            has_atmosphere: device.has_atmosphere(),
                            has_activate_state: device.has_activate_state(),
                        },
                    },
                ))
            }
            ObjectInterfaces {
                structure: Some(stru),
                storage: Some(storage),
                memory_readable: Some(mem_r),
                memory_writable: mem_w,
                logicable: Some(logic),
                source_code: None,
                circuit_holder: None,
                item: None,
                integrated_circuit: None,
                programmable: None,
                instructable: inst,
                logic_stack: None,
                device: Some(device),
                wireless_transmit: None,
                wireless_receive: None,
                network: None,
            } => {
                let prefab_lookup = StationpediaPrefab::from_repr(obj_ref.get_prefab().hash);
                Ok(ObjectTemplate::StructureLogicDeviceMemory(
                    StructureLogicDeviceMemoryTemplate {
                        object: Some(ObjectInfo {
                            name: Some(obj_ref.get_name().value()),
                            id: Some(*obj_ref.get_id()),
                        }),
                        prefab: PrefabInfo {
                            prefab_name: obj_ref.get_prefab().value,
                            prefab_hash: obj_ref.get_prefab().hash,
                            name: prefab_lookup
                                .map(|prefab| prefab.get_str("name"))
                                .flatten()
                                .unwrap_or("")
                                .to_string(),
                            desc: prefab_lookup
                                .map(|prefab| prefab.get_str("desc"))
                                .flatten()
                                .unwrap_or("")
                                .to_string(),
                        },
                        structure: StructureInfo {
                            small_grid: stru.is_small_grid(),
                        },
                        slots: storage
                            .get_slots()
                            .iter()
                            .map(|slot| {
                                Ok(SlotInfo {
                                    name: slot.name.clone(),
                                    typ: slot.typ,
                                    occupant: slot
                                        .occupant
                                        .map(|occupant| ObjectTemplate::freeze_object(occupant))
                                        .map_or(Ok(None), |v| v.map(Some))?,
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()?,
                        logic: LogicInfo {
                            logic_slot_types: storage
                                .get_slots()
                                .iter()
                                .enumerate()
                                .map(|(index, slot)| {
                                    (
                                        index as u32,
                                        LogicSlotTypes {
                                            slot_types: LogicSlotType::iter()
                                                .filter_map(|slt| {
                                                    let readable =
                                                        slot.readable_logic.contains(&slt);
                                                    let writeable =
                                                        slot.writeable_logic.contains(&slt);
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
                                        },
                                    )
                                })
                                .collect(),
                            logic_types: LogicTypes {
                                types: logic
                                    .valid_logic_types()
                                    .iter()
                                    .filter_map(|lt| {
                                        let readable = logic.can_logic_read(lt);
                                        let writeable = logic.can_logic_write(lt);
                                        if readable && writeable {
                                            Some((lt, MemoryAccess::ReadWrite))
                                        } else if readable {
                                            Some((lt, MemoryAccess::Read))
                                        } else if writeable {
                                            Some((lt, MemoryAccess::Write))
                                        } else {
                                            None
                                        }
                                    })
                                    .collect(),
                            },
                            modes: logic.known_modes().map(|modes| modes.iter().collect()),
                            logic_values: logic
                                .valid_logic_types()
                                .iter()
                                .filter_map(|lt| logic.get_logic(lt).ok()),
                            transmission_receiver: false,
                            wireless_logic: false,
                            circuit_holder: false,
                        },
                        device: DeviceInfo {
                            connection_list: device
                                .connection_list()
                                .iter()
                                .map(|conn| conn.to_info()),
                            device_pins_length: device.device_pins().map(|pins| pins.len()),
                            device_pins: device.device_pins().map(|pins| pins.iter().collect()),
                            has_reagents: device.has_reagents(),
                            has_lock_state: device.has_lock_state(),
                            has_mode_state: device.has_mode_state(),
                            has_open_state: device.has_mode_state(),
                            has_on_off_state: device.has_on_off_state(),
                            has_color_state: device.has_color_state(),
                            has_atmosphere: device.has_atmosphere(),
                            has_activate_state: device.has_activate_state(),
                        },
                        memory: MemoryInfo {
                            instructions: None, // TODO: map info from `Instructable` trait
                            memory_access: if mem_w.is_some() {
                                MemoryAccess::ReadWrite
                            } else {
                                MemoryAccess::Read
                            },
                            memory_size: mem_r.memory_size(),
                            values: Some(mem_r.get_memory_slice().iter().collect()),
                        },
                    },
                ))
            }

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
            } => {
                let prefab_lookup = StationpediaPrefab::from_repr(obj_ref.get_prefab().hash);
                Ok(ObjectTemplate::Item(ItemTemplate {
                    object: Some(ObjectInfo {
                        name: Some(obj_ref.get_name().value()),
                        id: Some(*obj_ref.get_id()),
                    }),
                    prefab: PrefabInfo {
                        prefab_name: obj_ref.get_prefab().value,
                        prefab_hash: obj_ref.get_prefab().hash,
                        name: prefab_lookup
                            .map(|prefab| prefab.get_str("name"))
                            .flatten()
                            .unwrap_or("")
                            .to_string(),
                        desc: prefab_lookup
                            .map(|prefab| prefab.get_str("desc"))
                            .flatten()
                            .unwrap_or("")
                            .to_string(),
                    },
                    item: ItemInfo {
                        consumable: item.consumable(),
                        filter_type: item.filter_type(),
                        ingredient: item.ingredient(),
                        max_quantity: item.max_quantity(),
                        reagents: item.reagents(),
                        slot_class: item.slot_class(),
                        sorting_class: item.sorting_class(),
                    },
                }))
            }
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
            } => {
                let prefab_lookup = StationpediaPrefab::from_repr(obj_ref.get_prefab().hash);
                Ok(ObjectTemplate::ItemSlots(ItemSlotsTemplate {
                    object: Some(ObjectInfo {
                        name: Some(obj_ref.get_name().value()),
                        id: Some(*obj_ref.get_id()),
                    }),
                    prefab: PrefabInfo {
                        prefab_name: obj_ref.get_prefab().value,
                        prefab_hash: obj_ref.get_prefab().hash,
                        name: prefab_lookup
                            .map(|prefab| prefab.get_str("name"))
                            .flatten()
                            .unwrap_or("")
                            .to_string(),
                        desc: prefab_lookup
                            .map(|prefab| prefab.get_str("desc"))
                            .flatten()
                            .unwrap_or("")
                            .to_string(),
                    },
                    item: ItemInfo {
                        consumable: item.consumable(),
                        filter_type: item.filter_type(),
                        ingredient: item.ingredient(),
                        max_quantity: item.max_quantity(),
                        reagents: item.reagents(),
                        slot_class: item.slot_class(),
                        sorting_class: item.sorting_class(),
                    },
                    slots: storage
                        .get_slots()
                        .iter()
                        .map(|slot| {
                            Ok(SlotInfo {
                                name: slot.name.clone(),
                                typ: slot.typ,
                                occupant: slot
                                    .occupant
                                    .map(|occupant| ObjectTemplate::freeze_object(occupant))
                                    .map_or(Ok(None), |v| v.map(Some))?,
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                }))
            }

            ObjectInterfaces {
                structure: None,
                storage: Some(storage),
                memory_readable: None,
                memory_writable: None,
                logicable: Some(logic),
                source_code: None,
                circuit_holder: None,
                item: Some(item),
                integrated_circuit: None,
                programmable: None,
                instructable: None,
                logic_stack: None,
                device: None,
                wireless_transmit: wt,
                wireless_receive: wr,
                network: None,
            } => {
                let prefab_lookup = StationpediaPrefab::from_repr(obj_ref.get_prefab().hash);
                Ok(ObjectTemplate::ItemLogic(ItemLogicTemplate {
                    object: Some(ObjectInfo {
                        name: Some(obj_ref.get_name().value()),
                        id: Some(*obj_ref.get_id()),
                    }),
                    prefab: PrefabInfo {
                        prefab_name: obj_ref.get_prefab().value,
                        prefab_hash: obj_ref.get_prefab().hash,
                        name: prefab_lookup
                            .map(|prefab| prefab.get_str("name"))
                            .flatten()
                            .unwrap_or("")
                            .to_string(),
                        desc: prefab_lookup
                            .map(|prefab| prefab.get_str("desc"))
                            .flatten()
                            .unwrap_or("")
                            .to_string(),
                    },
                    item: ItemInfo {
                        consumable: item.consumable(),
                        filter_type: item.filter_type(),
                        ingredient: item.ingredient(),
                        max_quantity: item.max_quantity(),
                        reagents: item.reagents(),
                        slot_class: item.slot_class(),
                        sorting_class: item.sorting_class(),
                    },
                    slots: storage
                        .get_slots()
                        .iter()
                        .map(|slot| {
                            Ok(SlotInfo {
                                name: slot.name.clone(),
                                typ: slot.typ,
                                occupant: slot
                                    .occupant
                                    .map(|occupant| ObjectTemplate::freeze_object(occupant))
                                    .map_or(Ok(None), |v| v.map(Some))?,
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                }))
            }
            _ => {
                Err(TemplateError::NonConformingObject(obj_ref.get_id()))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrefabInfo {
    pub prefab_name: String,
    pub prefab_hash: i32,
    pub desc: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectInfo {
    pub name: Option<String>,
    pub id: Option<ObjectID>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlotInfo {
    pub name: String,
    pub typ: SlotClass,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occupant: Option<ObjectTemplate>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct LogicSlotTypes {
    #[serde(flatten)]
    pub slot_types: BTreeMap<LogicSlotType, MemoryAccess>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct LogicTypes {
    #[serde(flatten)]
    pub types: BTreeMap<LogicType, MemoryAccess>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogicInfo {
    pub logic_slot_types: BTreeMap<u32, LogicSlotTypes>,
    pub logic_types: LogicTypes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modes: Option<BTreeMap<u32, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logic_values: Option<BTreeMap<LogicType, f64>>,
    pub transmission_receiver: bool,
    pub wireless_logic: bool,
    pub circuit_holder: bool,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemInfo {
    pub consumable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<GasType>,
    pub ingredient: bool,
    pub max_quantity: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reagents: Option<BTreeMap<String, f64>>,
    pub slot_class: SlotClass,
    pub sorting_class: SortingClass,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionInfo {
    pub typ: ConnectionType,
    pub role: ConnectionRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<ObjectID>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub connection_list: Vec<ConnectionInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_pins_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_pins: Option<Vec<Option<ObjectID>>>,
    pub has_activate_state: bool,
    pub has_atmosphere: bool,
    pub has_color_state: bool,
    pub has_lock_state: bool,
    pub has_mode_state: bool,
    pub has_on_off_state: bool,
    pub has_open_state: bool,
    pub has_reagents: bool,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureInfo {
    pub small_grid: bool,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
    pub description: String,
    pub typ: String,
    pub value: i64,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<BTreeMap<String, Instruction>>,
    pub memory_access: MemoryAccess,
    pub memory_size: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<f64>>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectInfo>,
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureSlotsTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectInfo>,
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub slots: Vec<SlotInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureLogicTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectInfo>,
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureLogicDeviceTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectInfo>,
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
    pub device: DeviceInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureLogicDeviceMemoryTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectInfo>,
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
    pub device: DeviceInfo,
    pub memory: MemoryInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectInfo>,
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSlotsTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectInfo>,
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
    pub slots: Vec<SlotInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemLogicTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectInfo>,
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemLogicMemoryTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectInfo>,
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
    pub memory: MemoryInfo,
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

        let database: Database = serde_json::from_reader(BufReader::new(File::open(d)?))?;

        Ok(())
    }
}
