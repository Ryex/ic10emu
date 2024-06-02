use std::collections::BTreeMap;

use crate::enums::{
    basic::{Class, GasType, SortingClass},
    script::{LogicSlotType, LogicType},
    ConnectionRole, ConnectionType, MachineTier, MemoryAccess, Species,
};

use serde_with::{serde_as, DisplayFromStr, Map};

use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "tsify")]
use tsify::Tsify;
#[cfg(feature = "tsify")]
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "templateType")]
pub enum ObjectTemplate {
    Structure(StructureTemplate),
    StructureSlots(StructureSlotsTemplate),
    StructureLogic(StructureLogicTemplate),
    StructureLogicDevice(StructureLogicDeviceTemplate),
    StructureLogicDeviceConsumer(StructureLogicDeviceConsumerTemplate),
    StructureLogicDeviceMemory(StructureLogicDeviceMemoryTemplate),
    StructureLogicDeviceConsumerMemory(StructureLogicDeviceConsumerMemoryTemplate),
    StructureCircuitHolder(StructureCircuitHolderTemplate),
    Item(ItemTemplate),
    ItemSlots(ItemSlotsTemplate),
    ItemConsumer(ItemConsumerTemplate),
    ItemLogic(ItemLogicTemplate),
    ItemLogicMemory(ItemLogicMemoryTemplate),
    ItemCircuitHolder(ItemCircuitHolderTemplate),
    ItemSuit(ItemSuitTemplate),
    ItemSuitLogic(ItemSuitLogicTemplate),
    ItemSuitCircuitHolder(ItemSuitCircuitHolderTemplate),
    Human(HumanTemplate),
}

#[allow(dead_code)]
impl ObjectTemplate {
    #[allow(clippy::must_use_candidate)]
    pub fn prefab(&self) -> &PrefabInfo {
        #[allow(clippy::enum_glob_use)]
        use ObjectTemplate::*;
        match self {
            Structure(s) => &s.prefab,
            StructureSlots(s) => &s.prefab,
            StructureLogic(s) => &s.prefab,
            StructureLogicDevice(s) => &s.prefab,
            StructureLogicDeviceConsumer(s) => &s.prefab,
            StructureLogicDeviceMemory(s) => &s.prefab,
            StructureLogicDeviceConsumerMemory(s) => &s.prefab,
            StructureCircuitHolder(s) => &s.prefab,
            Item(i) => &i.prefab,
            ItemSlots(i) => &i.prefab,
            ItemConsumer(i) => &i.prefab,
            ItemLogic(i) => &i.prefab,
            ItemLogicMemory(i) => &i.prefab,
            ItemCircuitHolder(i) => &i.prefab,
            ItemSuit(i) => &i.prefab,
            ItemSuitLogic(i) => &i.prefab,
            ItemSuitCircuitHolder(i) => &i.prefab,
            Human(h) => &h.prefab,
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
impl From<StructureLogicDeviceConsumerTemplate> for ObjectTemplate {
    fn from(value: StructureLogicDeviceConsumerTemplate) -> Self {
        Self::StructureLogicDeviceConsumer(value)
    }
}

impl From<StructureLogicDeviceMemoryTemplate> for ObjectTemplate {
    fn from(value: StructureLogicDeviceMemoryTemplate) -> Self {
        Self::StructureLogicDeviceMemory(value)
    }
}

impl From<StructureLogicDeviceConsumerMemoryTemplate> for ObjectTemplate {
    fn from(value: StructureLogicDeviceConsumerMemoryTemplate) -> Self {
        Self::StructureLogicDeviceConsumerMemory(value)
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

impl From<ItemConsumerTemplate> for ObjectTemplate {
    fn from(value: ItemConsumerTemplate) -> Self {
        Self::ItemConsumer(value)
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

impl From<ItemSuitCircuitHolderTemplate> for ObjectTemplate {
    fn from(value: ItemSuitCircuitHolderTemplate) -> Self {
        Self::ItemSuitCircuitHolder(value)
    }
}

impl From<ItemSuitTemplate> for ObjectTemplate {
    fn from(value: ItemSuitTemplate) -> Self {
        Self::ItemSuit(value)
    }
}

impl From<ItemSuitLogicTemplate> for ObjectTemplate {
    fn from(value: ItemSuitLogicTemplate) -> Self {
        Self::ItemSuitLogic(value)
    }
}

impl From<ItemCircuitHolderTemplate> for ObjectTemplate {
    fn from(value: ItemCircuitHolderTemplate) -> Self {
        Self::ItemCircuitHolder(value)
    }
}

impl From<StructureCircuitHolderTemplate> for ObjectTemplate {
    fn from(value: StructureCircuitHolderTemplate) -> Self {
        Self::StructureCircuitHolder(value)
    }
}

impl From<HumanTemplate> for ObjectTemplate {
    fn from(value: HumanTemplate) -> Self {
        Self::Human(value)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct HumanTemplate {
    pub prefab: PrefabInfo,
    pub species: Species,
    pub slots: Vec<SlotInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct PrefabInfo {
    pub prefab_name: String,
    pub prefab_hash: i32,
    pub desc: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct SlotInfo {
    pub name: String,
    pub typ: Class,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct LogicInfo {
    #[serde_as( as = "BTreeMap<DisplayFromStr, _>")]
    pub logic_slot_types: BTreeMap<u32, BTreeMap<LogicSlotType, MemoryAccess>>,
    pub logic_types: BTreeMap<LogicType, MemoryAccess>,
    #[serde_as( as = "Option<BTreeMap<DisplayFromStr, _>>")]
    pub modes: Option<BTreeMap<u32, String>>,
    pub transmission_receiver: bool,
    pub wireless_logic: bool,
    pub circuit_holder: bool,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct ItemInfo {
    pub consumable: bool,
    pub filter_type: Option<GasType>,
    pub ingredient: bool,
    pub max_quantity: u32,
    pub reagents: Option<BTreeMap<String, f64>>,
    pub slot_class: Class,
    pub sorting_class: SortingClass,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct ConnectionInfo {
    pub typ: ConnectionType,
    pub role: ConnectionRole,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
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

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct ConsumerInfo {
    pub consumed_resouces: Vec<String>,
    pub processed_reagents: Vec<i32>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct RecipeRange {
    pub start: f64,
    pub stop: f64,
    pub is_valid: bool,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct RecipeGasMix {
    pub rule: i64,
    pub is_any: bool,
    pub is_any_to_remove: bool,
    pub reagents: BTreeMap<String, f64>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct Recipe {
    pub tier: MachineTier,
    pub time: f64,
    pub energy: f64,
    pub temperature: RecipeRange,
    pub pressure: RecipeRange,
    pub required_mix: RecipeGasMix,
    pub count_types: i64,
    pub reagents: BTreeMap<String, f64>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct FabricatorInfo {
    pub tier: MachineTier,
    pub recipes: BTreeMap<String, Recipe>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct StructureInfo {
    pub small_grid: bool,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub enum InstructionPartType {
    Bool8,
    Byte8,
    Int32,
    UInt32,
    Short16,
    UShort16,
    Unused(u32),
    Unknown(String),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct InstructionPart {
    pub range: (u32, u32),
    pub name: String,
    pub typ: InstructionPartType,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct Instruction {
    pub description: String,
    pub description_stripped: String,
    pub typ: String,
    pub value: i64,
    pub valid: (u32, Option<u32>),
    pub parts: Vec<InstructionPart>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct MemoryInfo {
    pub instructions: Option<BTreeMap<String, Instruction>>,
    pub memory_access: MemoryAccess,
    pub memory_size: u32,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct ThermalInfo {
    pub convection_factor: f32,
    pub radiation_factor: f32,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct InternalAtmoInfo {
    pub volume: f32,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct SuitInfo {
    pub hygine_reduction_multiplier: f32,
    pub waste_max_pressure: f32,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct StructureTemplate {
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct StructureSlotsTemplate {
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub slots: Vec<SlotInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct StructureLogicTemplate {
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct StructureLogicDeviceTemplate {
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
    pub device: DeviceInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct StructureLogicDeviceConsumerTemplate {
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
    pub device: DeviceInfo,
    pub consumer_info: ConsumerInfo,
    pub fabricator_info: Option<FabricatorInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct StructureLogicDeviceMemoryTemplate {
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
    pub device: DeviceInfo,
    pub memory: MemoryInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct StructureCircuitHolderTemplate {
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
    pub device: DeviceInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct StructureLogicDeviceConsumerMemoryTemplate {
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
    pub device: DeviceInfo,
    pub consumer_info: ConsumerInfo,
    pub fabricator_info: Option<FabricatorInfo>,
    pub memory: MemoryInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct ItemTemplate {
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct ItemSlotsTemplate {
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub slots: Vec<SlotInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct ItemConsumerTemplate {
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub slots: Vec<SlotInfo>,
    pub consumer_info: ConsumerInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct ItemLogicTemplate {
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct ItemLogicMemoryTemplate {
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
    pub memory: MemoryInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct ItemCircuitHolderTemplate {
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct ItemSuitTemplate {
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub slots: Vec<SlotInfo>,
    pub suit_info: SuitInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct ItemSuitLogicTemplate {
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
    pub suit_info: SuitInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct ItemSuitCircuitHolderTemplate {
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
    pub suit_info: SuitInfo,
    pub memory: MemoryInfo,
}
