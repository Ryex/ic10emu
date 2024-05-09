use serde_derive::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Stationpedia", deny_unknown_fields)]
pub struct Stationpedia {
    pub pages: Vec<Page>,
    pub reagents: BTreeMap<String, Reagent>,
    #[serde(rename = "scriptCommands")]
    pub script_commands: BTreeMap<String, Command>,
}

#[allow(dead_code)]
impl Stationpedia {
    pub fn lookup_prefab_name(&self, prefab_name: &'_ str) -> Option<&Page> {
        self.pages.iter().find(|p| p.prefab_name == prefab_name)
    }

    pub fn lookup_key(&self, key: &str) -> Option<&Page> {
        self.pages.iter().find(|p| p.key == key)
    }

    pub fn lookup_hash(&self, hash: i32) -> Option<&Page> {
        self.pages.iter().find(|p| p.prefab_hash == hash)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reagent {
    #[serde(rename = "Hash")]
    pub hash: i64,
    #[serde(rename = "Unit")]
    pub unit: String,
    #[serde(rename = "Sources")]
    pub sources: Option<BTreeMap<String, f64>>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Command {
    pub desc: String,
    pub example: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Page {
    #[serde(rename = "ConnectionInsert")]
    pub connection_insert: Vec<ConnectionInsert>,
    #[serde(rename = "ConstructedByKits")]
    pub constructs: Vec<Constructs>,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Device")]
    pub device: Option<Device>,
    #[serde(rename = "Item")]
    pub item: Option<Item>,
    #[serde(rename = "Structure")]
    pub structure: Option<Structure>,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "LogicInfo")]
    pub logic_info: Option<LogicInfo>,
    #[serde(rename = "LogicInsert")]
    pub logic_insert: Vec<LogicInsert>,
    #[serde(rename = "LogicSlotInsert")]
    pub logic_slot_insert: Vec<LogicSlotInsert>,
    #[serde(rename = "Memory")]
    pub memory: Option<Memory>,
    #[serde(rename = "ModeInsert")]
    pub mode_insert: Vec<ModeInsert>,
    #[serde(rename = "PrefabHash")]
    pub prefab_hash: i32,
    #[serde(rename = "PrefabName")]
    pub prefab_name: String,
    #[serde(rename = "SlotInserts")]
    pub slot_inserts: Vec<SlotInsert>,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "TransmissionReceiver")]
    pub transmission_receiver: Option<bool>,
    #[serde(rename = "WirelessLogic")]
    pub wireless_logic: Option<bool>,
    #[serde(rename = "CircuitHolder")]
    pub circuit_holder: Option<bool>,
    #[serde(rename = "BasePowerDraw")]
    pub base_power_draw: Option<String>,
    #[serde(rename = "MaxPressure")]
    pub max_pressure: Option<String>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Constructs {
    #[serde(rename = "NameOfThing")]
    pub name_of_thing: String,
    #[serde(rename = "PageLink")]
    pub page_link: String,
    #[serde(rename = "PrefabHash")]
    pub prefab_hash: i32,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Structure {
    #[serde(rename = "SmallGrid")]
    pub small_grid: bool,
    #[serde(rename = "BuildStates")]
    pub build_states: BuildStates,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct BuildStates(pub Vec<BuildState>);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct BuildState {
    #[serde(rename = "Tool")]
    pub tool: Option<Vec<Tool>>,
    #[serde(rename = "ToolExit")]
    pub tool_exit: Option<Vec<Tool>>,
    #[serde(rename = "CanManufacture", default)]
    pub can_manufacture: bool,
    #[serde(rename = "MachineTier")]
    pub machine_tier: Option<MachineTier>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum MachineTier {
    Undefined,
    TierOne,
    TierTwo,
    TierThree,
    Max,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Tool {
    #[serde(rename = "IsTool", default)]
    pub is_tool: bool,
    #[serde(rename = "PrefabName")]
    pub prefab_name: String,
    #[serde(rename = "Quantity")]
    pub quantity: Option<i64>,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct SlotInsert {
    #[serde(rename = "SlotIndex")]
    #[serde_as(as = "DisplayFromStr")]
    pub slot_index: u32,
    #[serde(rename = "SlotName")]
    pub slot_name: String,
    #[serde(rename = "SlotType")]
    pub slot_type: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct LogicInsert {
    #[serde(rename = "LogicAccessTypes")]
    pub logic_access_types: String,
    #[serde(rename = "LogicName")]
    pub logic_name: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct LogicSlotInsert {
    #[serde(rename = "LogicAccessTypes")]
    pub logic_access_types: String,
    #[serde(rename = "LogicName")]
    pub logic_name: String,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct ModeInsert {
    #[serde(rename = "LogicAccessTypes")]
    #[serde_as(as = "DisplayFromStr")]
    pub logic_access_types: u32,
    #[serde(rename = "LogicName")]
    pub logic_name: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct ConnectionInsert {
    #[serde(rename = "LogicAccessTypes")]
    pub logic_access_types: String,
    #[serde(rename = "LogicName")]
    pub logic_name: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct LogicInfo {
    #[serde(rename = "LogicSlotTypes")]
    pub logic_slot_types: BTreeMap<u32, LogicSlotTypes>,
    #[serde(rename = "LogicTypes")]
    pub logic_types: LogicTypes,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct LogicSlotTypes {
    #[serde(flatten)]
    pub slot_types: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct LogicTypes {
    #[serde(flatten)]
    pub types: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Memory {
    #[serde(rename = "Instructions")]
    pub instructions: Option<BTreeMap<String, Instruction>>,
    #[serde(rename = "MemoryAccess")]
    pub memory_access: String,
    #[serde(rename = "MemorySize")]
    pub memory_size: i64,
    #[serde(rename = "MemorySizeReadable")]
    pub memory_size_readable: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Instruction {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Type")]
    pub type_: String,
    #[serde(rename = "Value")]
    pub value: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "Consumable")]
    pub consumable: Option<bool>,
    #[serde(rename = "FilterType")]
    pub filter_type: Option<String>,
    #[serde(rename = "Ingredient")]
    pub ingredient: Option<bool>,
    #[serde(rename = "MaxQuantity")]
    pub max_quantity: Option<f64>,
    #[serde(rename = "Reagents")]
    pub reagents: Option<indexmap::IndexMap<String, f64>>,
    #[serde(rename = "SlotClass")]
    pub slot_class: String,
    #[serde(rename = "SortingClass")]
    pub sorting_class: String,
    #[serde(rename = "Recipes", default)]
    pub recipes: Vec<Recipe>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    #[serde(rename = "CreatorPrefabName")]
    pub creator_prefab_name: String,
    #[serde(rename = "TierName")]
    pub tier_name: String,
    #[serde(rename = "Time")]
    pub time: f64,
    #[serde(rename = "Energy")]
    pub energy: f64,
    #[serde(rename = "Temperature")]
    pub temperature: RecipeTemperature,
    #[serde(rename = "Pressure")]
    pub pressure: RecipePressure,
    #[serde(rename = "RequiredMix")]
    pub required_mix: RecipeGasMix,
    #[serde(rename = "CountTypes")]
    pub count_types: i64,
    #[serde(flatten)]
    pub reagents: indexmap::IndexMap<String, f64>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RecipeTemperature {
    #[serde(rename = "Start")]
    pub start: f64,
    #[serde(rename = "Stop")]
    pub stop: f64,
    #[serde(rename = "IsValid")]
    pub is_valid: bool,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RecipePressure {
    #[serde(rename = "Start")]
    pub start: f64,
    #[serde(rename = "Stop")]
    pub stop: f64,
    #[serde(rename = "IsValid")]
    pub is_valid: bool,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RecipeGasMix {
    #[serde(rename = "Rule")]
    pub rule: i64,
    #[serde(rename = "IsAny")]
    pub is_any: bool,
    #[serde(rename = "IsAnyToRemove")]
    pub is_any_to_remove: bool,
    #[serde(flatten)]
    pub reagents: BTreeMap<String, f64>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "ConnectionList")]
    pub connection_list: Vec<(String, String)>,
    #[serde(rename = "DevicesLength")]
    pub devices_length: Option<i64>,
    #[serde(rename = "HasActivateState")]
    pub has_activate_state: bool,
    #[serde(rename = "HasAtmosphere")]
    pub has_atmosphere: bool,
    #[serde(rename = "HasColorState")]
    pub has_color_state: bool,
    #[serde(rename = "HasLockState")]
    pub has_lock_state: bool,
    #[serde(rename = "HasModeState")]
    pub has_mode_state: bool,
    #[serde(rename = "HasOnOffState")]
    pub has_on_off_state: bool,
    #[serde(rename = "HasOpenState")]
    pub has_open_state: bool,
    #[serde(rename = "HasReagents")]
    pub has_reagents: bool,
}
