use std::{collections::BTreeMap, io::Write};

use serde_derive::{Deserialize, Serialize};

use crate::{
    enums,
    stationpedia::{self, Page, Stationpedia},
};

pub fn generate_database(
    stationpedia: &stationpedia::Stationpedia,
    enums: &enums::Enums,
    workspace: &std::path::Path,
) -> color_eyre::Result<()> {
    let templates = generate_templates(stationpedia)?;

    eprintln!("Writing prefab database ...");

    let prefabs: BTreeMap<String, ObjectTemplate> = templates
        .into_iter()
        .map(|obj| (obj.prefab().prefab_name.clone(), obj))
        .collect();
    let prefabs_by_hash: BTreeMap<i32, String> = prefabs
        .iter()
        .map(|(key, val)| (val.prefab().prefab_hash, key.clone()))
        .collect();

    let structures = prefabs
        .iter()
        .filter_map(|(_, val)| {
            use ObjectTemplate::*;
            match val {
                Structure(_)
                | StructureSlots(_)
                | StructureLogic(_)
                | StructureLogicDevice(_)
                | StructureLogicDeviceMemory(_) => Some(val.prefab().prefab_name.clone()),
                Item(_) | ItemSlots(_) | ItemLogic(_) | ItemLogicMemory(_) => None,
            }
        })
        .collect();
    let items = prefabs
        .iter()
        .filter_map(|(_, val)| {
            use ObjectTemplate::*;
            match val {
                Structure(_)
                | StructureSlots(_)
                | StructureLogic(_)
                | StructureLogicDevice(_)
                | StructureLogicDeviceMemory(_) => None,
                Item(_) | ItemSlots(_) | ItemLogic(_) | ItemLogicMemory(_) => {
                    Some(val.prefab().prefab_name.clone())
                }
            }
        })
        .collect();
    let logicable_items = prefabs
        .iter()
        .filter_map(|(_, val)| {
            use ObjectTemplate::*;
            match val {
                Structure(_)
                | StructureSlots(_)
                | StructureLogic(_)
                | StructureLogicDevice(_)
                | StructureLogicDeviceMemory(_)
                | Item(_)
                | ItemSlots(_) => None,
                ItemLogic(_) | ItemLogicMemory(_) => Some(val.prefab().prefab_name.clone()),
            }
        })
        .collect();

    let devices = prefabs
        .iter()
        .filter_map(|(_, val)| {
            use ObjectTemplate::*;
            match val {
                Structure(_) | StructureSlots(_) | StructureLogic(_) | Item(_) | ItemSlots(_)
                | ItemLogic(_) | ItemLogicMemory(_) => None,
                StructureLogicDevice(_) | StructureLogicDeviceMemory(_) => {
                    Some(val.prefab().prefab_name.clone())
                }
            }
        })
        .collect();
    let db: ObjectDatabase = ObjectDatabase {
        prefabs,
        reagents: stationpedia.reagents.clone(),
        enums: enums.clone(),
        prefabs_by_hash,
        structures,
        devices,
        items,
        logicable_items,
    };

    let data_path = workspace.join("data");
    if !data_path.exists() {
        std::fs::create_dir(&data_path)?;
    }
    let database_path = data_path.join("database.json");
    let mut database_file = std::io::BufWriter::new(std::fs::File::create(database_path)?);
    serde_json::to_writer(&mut database_file, &db)?;
    database_file.flush()?;
    Ok(())
}

fn generate_templates(pedia: &Stationpedia) -> color_eyre::Result<Vec<ObjectTemplate>> {
    println!("Generating templates ...");
    let mut templates: Vec<ObjectTemplate> = Vec::new();
    for page in pedia.pages.iter() {
        let prefab = PrefabInfo {
            prefab_name: page.prefab_name.clone(),
            prefab_hash: page.prefab_hash,
            desc: page.description.clone(),
            name: page.title.clone(),
        };
        // every page should either by a item or a structure
        // in theory every device is logicable
        // in theory everything with memory is logicable
        match page {
            Page {
                item: Some(item),
                structure: None,
                logic_info: None,
                slot_inserts,
                memory: None,
                device: None,
                transmission_receiver: None,
                wireless_logic: None,
                circuit_holder: None,
                ..
            } if slot_inserts.is_empty() => {
                templates.push(ObjectTemplate::Item(ItemTemplate {
                    prefab,
                    item: item.into(),
                }));
            }
            Page {
                item: Some(item),
                structure: None,
                logic_info: None,
                slot_inserts,
                memory: None,
                device: None,
                transmission_receiver: None,
                wireless_logic: None,
                circuit_holder: None,
                ..
            } => {
                templates.push(ObjectTemplate::ItemSlots(ItemSlotsTemplate {
                    prefab,
                    item: item.into(),
                    slots: slot_inserts_to_info(slot_inserts),
                }));
            }
            Page {
                item: Some(item),
                structure: None,
                logic_info: Some(logic),
                slot_inserts,
                memory: None,
                device: None,
                transmission_receiver,
                wireless_logic,
                circuit_holder,
                ..
            } => {
                let mut logic: LogicInfo = logic.into();
                if !page.mode_insert.is_empty() {
                    logic.modes = Some(mode_inserts_to_info(&page.mode_insert));
                }
                logic.transmission_receiver = transmission_receiver.unwrap_or(false);
                logic.wireless_logic = wireless_logic.unwrap_or(false);
                logic.circuit_holder = circuit_holder.unwrap_or(false);

                templates.push(ObjectTemplate::ItemLogic(ItemLogicTemplate {
                    prefab,
                    item: item.into(),
                    logic,
                    slots: slot_inserts_to_info(slot_inserts),
                }));
            }
            Page {
                item: Some(item),
                structure: None,
                logic_info: Some(logic),
                slot_inserts,
                memory: Some(memory),
                device: None,
                transmission_receiver,
                wireless_logic,
                circuit_holder,
                ..
            } => {
                let mut logic: LogicInfo = logic.into();
                if !page.mode_insert.is_empty() {
                    logic.modes = Some(mode_inserts_to_info(&page.mode_insert));
                }
                logic.transmission_receiver = transmission_receiver.unwrap_or(false);
                logic.wireless_logic = wireless_logic.unwrap_or(false);
                logic.circuit_holder = circuit_holder.unwrap_or(false);

                templates.push(ObjectTemplate::ItemLogicMemory(ItemLogicMemoryTemplate {
                    prefab,
                    item: item.into(),
                    logic,
                    slots: slot_inserts_to_info(slot_inserts),
                    memory: memory.into(),
                }));
            }
            Page {
                item: None,
                structure: Some(structure),
                slot_inserts,
                logic_info: None,
                memory: None,
                device: None,
                transmission_receiver: None,
                wireless_logic: None,
                circuit_holder: None,
                ..
            } if slot_inserts.is_empty() => {
                templates.push(ObjectTemplate::Structure(StructureTemplate {
                    prefab,
                    structure: structure.into(),
                }));
                // println!("Structure")
            }
            Page {
                item: None,
                structure: Some(structure),
                slot_inserts,
                logic_info: None,
                memory: None,
                device: None,
                transmission_receiver: None,
                wireless_logic: None,
                circuit_holder: None,
                ..
            } => {
                templates.push(ObjectTemplate::StructureSlots(StructureSlotsTemplate {
                    prefab,
                    structure: structure.into(),
                    slots: slot_inserts_to_info(slot_inserts),
                }));
                // println!("Structure")
            }
            Page {
                item: None,
                structure: Some(structure),
                logic_info: Some(logic),
                slot_inserts,
                memory: None,
                device: None,
                transmission_receiver,
                wireless_logic,
                circuit_holder,
                ..
            } => {
                let mut logic: LogicInfo = logic.into();
                if !page.mode_insert.is_empty() {
                    logic.modes = Some(mode_inserts_to_info(&page.mode_insert));
                }
                logic.transmission_receiver = transmission_receiver.unwrap_or(false);
                logic.wireless_logic = wireless_logic.unwrap_or(false);
                logic.circuit_holder = circuit_holder.unwrap_or(false);

                templates.push(ObjectTemplate::StructureLogic(StructureLogicTemplate {
                    prefab,
                    structure: structure.into(),
                    logic,
                    slots: slot_inserts_to_info(slot_inserts),
                }));
                // println!("Structure")
            }
            Page {
                item: None,
                structure: Some(structure),
                logic_info: Some(logic),
                slot_inserts,
                memory: None,
                device: Some(device),
                transmission_receiver,
                wireless_logic,
                circuit_holder,
                ..
            } => {
                let mut logic: LogicInfo = logic.into();
                if !page.mode_insert.is_empty() {
                    logic.modes = Some(mode_inserts_to_info(&page.mode_insert));
                }
                logic.transmission_receiver = transmission_receiver.unwrap_or(false);
                logic.wireless_logic = wireless_logic.unwrap_or(false);
                logic.circuit_holder = circuit_holder.unwrap_or(false);

                templates.push(ObjectTemplate::StructureLogicDevice(
                    StructureLogicDeviceTemplate {
                        prefab,
                        structure: structure.into(),
                        logic,
                        slots: slot_inserts_to_info(slot_inserts),
                        device: device.into(),
                    },
                ));
                // println!("Structure")
            }
            Page {
                item: None,
                structure: Some(structure),
                logic_info: Some(logic),
                slot_inserts,
                memory: Some(memory),
                device: Some(device),
                transmission_receiver,
                wireless_logic,
                circuit_holder,
                ..
            } => {
                let mut logic: LogicInfo = logic.into();
                if !page.mode_insert.is_empty() {
                    logic.modes = Some(mode_inserts_to_info(&page.mode_insert));
                }
                logic.transmission_receiver = transmission_receiver.unwrap_or(false);
                logic.wireless_logic = wireless_logic.unwrap_or(false);
                logic.circuit_holder = circuit_holder.unwrap_or(false);
                templates.push(ObjectTemplate::StructureLogicDeviceMemory(
                    StructureLogicDeviceMemoryTemplate {
                        prefab,
                        structure: structure.into(),
                        logic,
                        slots: slot_inserts_to_info(slot_inserts),
                        device: device.into(),
                        memory: memory.into(),
                    },
                ));
                // println!("Structure")
            }
            _ => panic!(
                    "Non conforming: {:?} \n\titem: {:?}\n\tstructure: {:?}\n\tlogic_info: {:?}\n\tslot_inserts: {:?}\n\tslot_logic: {:?}\n\tmemory: {:?}\n\tdevice: {:?}",
                    page.key,
                    page.item,
                    page.structure,
                    page.logic_info,
                    page.slot_inserts,
                    page.logic_slot_insert,
                    page.memory,
                    page.device,
                ),
        }
    }
    Ok(templates)
}

fn slot_inserts_to_info(slots: &[stationpedia::SlotInsert]) -> Vec<SlotInfo> {
    let mut tmp: Vec<_> = slots.into();
    tmp.sort_by(|a, b| a.slot_index.cmp(&b.slot_index));
    tmp.iter()
        .map(|slot| SlotInfo {
            name: slot.slot_name.clone(),
            typ: slot.slot_type.clone(),
        })
        .collect()
}

fn mode_inserts_to_info(modes: &[stationpedia::ModeInsert]) -> BTreeMap<u32, String> {
    modes
        .iter()
        .map(|mode| (mode.logic_access_types, mode.logic_name.clone()))
        .collect()
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ObjectDatabase {
    pub prefabs: BTreeMap<String, ObjectTemplate>,
    pub reagents: BTreeMap<String, stationpedia::Reagent>,
    pub enums: enums::Enums,
    pub prefabs_by_hash: BTreeMap<i32, String>,
    pub structures: Vec<String>,
    pub devices: Vec<String>,
    pub items: Vec<String>,
    pub logicable_items: Vec<String>,
}

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
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct PrefabInfo {
    pub prefab_name: String,
    pub prefab_hash: i32,
    pub desc: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct SlotInfo {
    pub name: String,
    pub typ: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct LogicInfo {
    pub logic_slot_types: BTreeMap<u32, stationpedia::LogicSlotTypes>,
    pub logic_types: stationpedia::LogicTypes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modes: Option<BTreeMap<u32, String>>,
    pub transmission_receiver: bool,
    pub wireless_logic: bool,
    pub circuit_holder: bool,
}

impl From<&stationpedia::LogicInfo> for LogicInfo {
    fn from(value: &stationpedia::LogicInfo) -> Self {
        LogicInfo {
            logic_slot_types: value.logic_slot_types.clone(),
            logic_types: value.logic_types.clone(),
            modes: None,
            transmission_receiver: false,
            wireless_logic: false,
            circuit_holder: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ItemInfo {
    pub consumable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>,
    pub ingredient: bool,
    pub max_quantity: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reagents: Option<BTreeMap<String, f64>>,
    pub slot_class: String,
    pub sorting_class: String,
}

impl From<&stationpedia::Item> for ItemInfo {
    fn from(item: &stationpedia::Item) -> Self {
        ItemInfo {
            consumable: item.consumable.unwrap_or(false),
            filter_type: item.filter_type.clone(),
            ingredient: item.ingredient.unwrap_or(false),
            max_quantity: item.max_quantity.unwrap_or(1.0) as u32,
            reagents: item
                .reagents
                .as_ref()
                .map(|map| map.iter().map(|(key, val)| (key.clone(), *val)).collect()),
            slot_class: item.slot_class.clone(),
            sorting_class: item.sorting_class.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionInfo {
    pub typ: String,
    pub role: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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

impl From<&stationpedia::Device> for DeviceInfo {
    fn from(value: &stationpedia::Device) -> Self {
        DeviceInfo {
            connection_list: value
                .connection_list
                .iter()
                .map(|(typ, role)| ConnectionInfo {
                    typ: typ.to_string(),
                    role: role.to_string(),
                })
                .collect(),
            device_pins_length: value.devices_length,
            has_activate_state: value.has_activate_state,
            has_atmosphere: value.has_atmosphere,
            has_color_state: value.has_color_state,
            has_lock_state: value.has_lock_state,
            has_mode_state: value.has_mode_state,
            has_on_off_state: value.has_on_off_state,
            has_open_state: value.has_open_state,
            has_reagents: value.has_reagents,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct StructureInfo {
    pub small_grid: bool,
}

impl From<&stationpedia::Structure> for StructureInfo {
    fn from(value: &stationpedia::Structure) -> Self {
        StructureInfo {
            small_grid: value.small_grid,
        }
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
    pub description: String,
    pub typ: String,
    pub value: i64,
}

impl From<&stationpedia::Instruction> for Instruction {
    fn from(value: &stationpedia::Instruction) -> Self {
        Instruction {
            description: value.description.clone(),
            typ: value.type_.clone(),
            value: value.value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct MemoryInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<BTreeMap<String, Instruction>>,
    pub memory_access: String,
    pub memory_size: i64,
}

impl From<&stationpedia::Memory> for MemoryInfo {
    fn from(value: &stationpedia::Memory) -> Self {
        MemoryInfo {
            instructions: value.instructions.as_ref().map(|insts| {
                insts
                    .iter()
                    .map(|(key, value)| (key.clone(), value.into()))
                    .collect()
            }),
            memory_access: value.memory_access.clone(),
            memory_size: value.memory_size,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct StructureTemplate {
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct StructureSlotsTemplate {
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub slots: Vec<SlotInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct StructureLogicTemplate {
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct StructureLogicDeviceTemplate {
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
    pub device: DeviceInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct StructureLogicDeviceMemoryTemplate {
    pub prefab: PrefabInfo,
    pub structure: StructureInfo,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
    pub device: DeviceInfo,
    pub memory: MemoryInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ItemTemplate {
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ItemSlotsTemplate {
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
    pub slots: Vec<SlotInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ItemLogicTemplate {
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ItemLogicMemoryTemplate {
    pub prefab: PrefabInfo,
    pub item: ItemInfo,
    pub logic: LogicInfo,
    pub slots: Vec<SlotInfo>,
    pub memory: MemoryInfo,
}
