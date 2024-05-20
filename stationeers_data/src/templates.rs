use std::collections::BTreeMap;

use crate::enums::{
    basic_enums::{Class as SlotClass, GasType, SortingClass},
    script_enums::{LogicSlotType, LogicType},
    ConnectionRole, ConnectionType, MemoryAccess,
};
use serde_derive::{Deserialize, Serialize};

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

#[allow(dead_code)]
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
}

impl From<StructureTemplate> for ObjectTemplate {
    fn from(value: StructureTemplate) -> Self {
        Self::Structure(value)
    }
}

impl From<StructureSlotsTemplate> for ObjectTemplate {
    fn from(value: StructureSlotsTemplate) -> Self {
        Self::StructureSlots(value)
    }
}

impl From<StructureLogicTemplate> for ObjectTemplate {
    fn from(value: StructureLogicTemplate) -> Self {
        Self::StructureLogic(value)
    }
}

impl From<StructureLogicDeviceTemplate> for ObjectTemplate {
    fn from(value: StructureLogicDeviceTemplate) -> Self {
        Self::StructureLogicDevice(value)
    }
}

impl From<StructureLogicDeviceMemoryTemplate> for ObjectTemplate {
    fn from(value: StructureLogicDeviceMemoryTemplate) -> Self {
        Self::StructureLogicDeviceMemory(value)
    }
}

impl From<ItemTemplate> for ObjectTemplate {
    fn from(value: ItemTemplate) -> Self {
        Self::Item(value)
    }
}

impl From<ItemSlotsTemplate> for ObjectTemplate {
    fn from(value: ItemSlotsTemplate) -> Self {
        Self::ItemSlots(value)
    }
}

impl From<ItemLogicTemplate> for ObjectTemplate {
    fn from(value: ItemLogicTemplate) -> Self {
        Self::ItemLogic(value)
    }
}

impl From<ItemLogicMemoryTemplate> for ObjectTemplate {
    fn from(value: ItemLogicMemoryTemplate) -> Self {
        Self::ItemLogicMemory(value)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct PrefabInfo {
    pub prefab_name: String,
    pub prefab_hash: i32,
    pub desc: String,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SlotInfo {
    pub name: String,
    pub typ: SlotClass,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct LogicInfo {
    pub logic_slot_types: BTreeMap<u32, BTreeMap<LogicSlotType, MemoryAccess>>,
    pub logic_types: BTreeMap<LogicType, MemoryAccess>,
    pub modes: Option<BTreeMap<u32, String>>,
    pub transmission_receiver: bool,
    pub wireless_logic: bool,
    pub circuit_holder: bool,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ItemInfo {
    pub consumable: bool,
    pub filter_type: Option<GasType>,
    pub ingredient: bool,
    pub max_quantity: u32,
    pub reagents: Option<BTreeMap<String, f64>>,
    pub slot_class: SlotClass,
    pub sorting_class: SortingClass,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub typ: ConnectionType,
    pub role: ConnectionRole,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub connection_list: Vec<ConnectionInfo>,
    pub device_pins_length: Option<u32>,
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
pub struct StructureInfo {
    pub small_grid: bool,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Instruction {
    pub description: String,
    pub typ: String,
    pub value: i64,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub instructions: Option<BTreeMap<String, Instruction>>,
    pub memory_access: MemoryAccess,
    pub memory_size: u32,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct StructureTemplate {
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct StructureSlotsTemplate {
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub slots: Vec<SlotInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct StructureLogicTemplate {
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct StructureLogicDeviceTemplate {
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
    pub device: DeviceInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct StructureLogicDeviceMemoryTemplate {
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
    pub device: DeviceInfo,
    pub memory: MemoryInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ItemTemplate {
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ItemSlotsTemplate {
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
    pub slots: Vec<SlotInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ItemLogicTemplate {
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ItemLogicMemoryTemplate {
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
    pub memory: MemoryInfo,
}
