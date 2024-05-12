use std::collections::BTreeMap;

use crate::{
    network::{ConnectionRole, ConnectionType},
    vm::{
        enums::{
            basic_enums::{Class as SlotClass, GasType, SortingClass},
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
            LogicField, Name, Slot,
        },
    },
};
use serde_derive::{Deserialize, Serialize};

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
    pub fn prefab(&self) -> &PrefabInfo {
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

    pub fn object(&self) -> Option<&ObjectInfo> {
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
                        .map(|obj| obj.object().map(|obj_info| obj_info.id).flatten())
                })
                .flatten()
                .collect(),
            StructureLogic(s) => s
                .slots
                .iter()
                .filter_map(|info| {
                    info.occupant
                        .as_ref()
                        .map(|obj| obj.object().map(|obj_info| obj_info.id).flatten())
                })
                .flatten()
                .collect(),
            StructureLogicDevice(s) => s
                .slots
                .iter()
                .filter_map(|info| {
                    info.occupant
                        .as_ref()
                        .map(|obj| obj.object().map(|obj_info| obj_info.id).flatten())
                })
                .flatten()
                .collect(),
            StructureLogicDeviceMemory(s) => s
                .slots
                .iter()
                .filter_map(|info| {
                    info.occupant
                        .as_ref()
                        .map(|obj| obj.object().map(|obj_info| obj_info.id).flatten())
                })
                .flatten()
                .collect(),
            ItemSlots(i) => i
                .slots
                .iter()
                .filter_map(|info| {
                    info.occupant
                        .as_ref()
                        .map(|obj| obj.object().map(|obj_info| obj_info.id).flatten())
                })
                .flatten()
                .collect(),
            ItemLogic(i) => i
                .slots
                .iter()
                .filter_map(|info| {
                    info.occupant
                        .as_ref()
                        .map(|obj| obj.object().map(|obj_info| obj_info.id).flatten())
                })
                .flatten()
                .collect(),
            ItemLogicMemory(i) => i
                .slots
                .iter()
                .filter_map(|info| {
                    info.occupant
                        .as_ref()
                        .map(|obj| obj.object().map(|obj_info| obj_info.id).flatten())
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
            }),
            StructureSlots(s) => VMObject::new(GenericStorage {
                id,
                prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                name: Name::new(&s.prefab.name),
                slots: s
                    .slots
                    .iter()
                    .enumerate()
                    .map(|(index, info)| Slot {
                        parent: id,
                        index,
                        name: info.name.clone(),
                        typ: info.typ,
                        enabled_logic: Vec::new(),
                        occupant: None,
                    })
                    .collect(),
            }),
            StructureLogic(s) => VMObject::new(GenericLogicable {
                id,
                prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                name: Name::new(&s.prefab.name),
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
                slots: s
                    .slots
                    .iter()
                    .enumerate()
                    .map(|(index, info)| Slot {
                        parent: id,
                        index,
                        name: info.name.clone(),
                        typ: info.typ,
                        enabled_logic: s
                            .logic
                            .logic_slot_types
                            .get(&(index as u32))
                            .map(|s_info| s_info.slot_types.keys().copied().collect::<Vec<_>>())
                            .unwrap_or_else(|| Vec::new()),
                        occupant: None,
                    })
                    .collect(),
            }),
            StructureLogicDevice(s) => VMObject::new(GenericLogicableDevice {
                id,
                prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                name: Name::new(&s.prefab.name),
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
                slots: s
                    .slots
                    .iter()
                    .enumerate()
                    .map(|(index, info)| Slot {
                        parent: id,
                        index,
                        name: info.name.clone(),
                        typ: info.typ,
                        enabled_logic: s
                            .logic
                            .logic_slot_types
                            .get(&(index as u32))
                            .map(|s_info| s_info.slot_types.keys().copied().collect::<Vec<_>>())
                            .unwrap_or_else(|| Vec::new()),
                        occupant: None,
                    })
                    .collect(),
            }),
            StructureLogicDeviceMemory(s)
                if matches!(s.memory.memory_access, MemoryAccess::Read) =>
            {
                VMObject::new(GenericLogicableDeviceMemoryReadable {
                    id,
                    prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                    name: Name::new(&s.prefab.name),
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
                    slots: s
                        .slots
                        .iter()
                        .enumerate()
                        .map(|(index, info)| Slot {
                            parent: id,
                            index,
                            name: info.name.clone(),
                            typ: info.typ,
                            enabled_logic: s
                                .logic
                                .logic_slot_types
                                .get(&(index as u32))
                                .map(|s_info| s_info.slot_types.keys().copied().collect::<Vec<_>>())
                                .unwrap_or_else(|| Vec::new()),
                            occupant: None,
                        })
                        .collect(),
                    memory: vec![0.0; s.memory.memory_size as usize],
                })
            }
            StructureLogicDeviceMemory(s) => {
                VMObject::new(GenericLogicableDeviceMemoryReadWriteable {
                    id,
                    prefab: Name::from_prefab_name(&s.prefab.prefab_name),
                    name: Name::new(&s.prefab.name),
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
                    slots: s
                        .slots
                        .iter()
                        .enumerate()
                        .map(|(index, info)| Slot {
                            parent: id,
                            index,
                            name: info.name.clone(),
                            typ: info.typ,
                            enabled_logic: s
                                .logic
                                .logic_slot_types
                                .get(&(index as u32))
                                .map(|s_info| s_info.slot_types.keys().copied().collect::<Vec<_>>())
                                .unwrap_or_else(|| Vec::new()),
                            occupant: None,
                        })
                        .collect(),
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
                        enabled_logic: Vec::new(),
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
                slots: i
                    .slots
                    .iter()
                    .enumerate()
                    .map(|(index, info)| Slot {
                        parent: id,
                        index,
                        name: info.name.clone(),
                        typ: info.typ,
                        enabled_logic: i
                            .logic
                            .logic_slot_types
                            .get(&(index as u32))
                            .map(|s_info| s_info.slot_types.keys().copied().collect::<Vec<_>>())
                            .unwrap_or_else(|| Vec::new()),
                        occupant: None,
                    })
                    .collect(),
            }),
            ItemLogicMemory(i) if matches!(i.memory.memory_access, MemoryAccess::Read) => {
                VMObject::new(GenericItemLogicableMemoryReadable {
                    id,
                    prefab: Name::from_prefab_name(&i.prefab.prefab_name),
                    name: Name::new(&i.prefab.name),
                    item_info: i.item.clone(),
                    parent_slot: None,
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
                    slots: i
                        .slots
                        .iter()
                        .enumerate()
                        .map(|(index, info)| Slot {
                            parent: id,
                            index,
                            name: info.name.clone(),
                            typ: info.typ,
                            enabled_logic: i
                                .logic
                                .logic_slot_types
                                .get(&(index as u32))
                                .map(|s_info| s_info.slot_types.keys().copied().collect::<Vec<_>>())
                                .unwrap_or_else(|| Vec::new()),
                            occupant: None,
                        })
                        .collect(),
                    memory: vec![0.0; i.memory.memory_size as usize],
                })
            }
            ItemLogicMemory(i) => VMObject::new(GenericItemLogicableMemoryReadWriteable {
                id,
                prefab: Name::from_prefab_name(&i.prefab.prefab_name),
                name: Name::new(&i.prefab.name),
                item_info: i.item.clone(),
                parent_slot: None,
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
                slots: i
                    .slots
                    .iter()
                    .enumerate()
                    .map(|(index, info)| Slot {
                        parent: id,
                        index,
                        name: info.name.clone(),
                        typ: info.typ,
                        enabled_logic: i
                            .logic
                            .logic_slot_types
                            .get(&(index as u32))
                            .map(|s_info| s_info.slot_types.keys().copied().collect::<Vec<_>>())
                            .unwrap_or_else(|| Vec::new()),
                        occupant: None,
                    })
                    .collect(),
                memory: vec![0.0; i.memory.memory_size as usize],
            }),
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
    pub device_pins_length: Option<i64>,
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
    pub memory_size: i64,
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
