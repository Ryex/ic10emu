use std::collections::BTreeMap;

use crate::{
    network::{ConnectionRole, ConnectionType},
    vm::enums::{
        basic_enums::{Class as SlotClass, GasType, SortingClass},
        script_enums::{LogicSlotType, LogicType},
    },
};
use serde_derive::{Deserialize, Serialize};

use super::{MemoryAccess, ObjectID};

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
    fn prefab(&self) -> &PrefabInfo {
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
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrefabInfo {
    pub prefab_name: String,
    pub prefab_hash: i32,
    pub desc: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectInfo {
    pub name: Option<String>,
    pub id: Option<ObjectID>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlotInfo {
    pub name: String,
    pub typ: String,
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

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogicInfo {
    pub logic_slot_types: BTreeMap<u32, LogicSlotTypes>,
    pub logic_types: LogicTypes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modes: Option<BTreeMap<u32, String>>,
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
    pub max_quantity: f64,
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
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub connection_list: Vec<ConnectionInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_pins_length: Option<i64>,
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
