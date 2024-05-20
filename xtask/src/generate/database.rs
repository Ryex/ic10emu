use std::{
    collections::BTreeMap,
    io::{BufWriter, Write},
    path::PathBuf,
};

use quote::quote;
use serde_derive::{Deserialize, Serialize};

use crate::{
    enums,
    stationpedia::{self, Page, Stationpedia},
};

use stationeers_data::templates::{
    ConnectionInfo, DeviceInfo, Instruction, ItemInfo, ItemLogicMemoryTemplate, ItemLogicTemplate,
    ItemSlotsTemplate, ItemTemplate, LogicInfo, MemoryInfo, ObjectTemplate, PrefabInfo, SlotInfo,
    StructureInfo, StructureLogicDeviceMemoryTemplate, StructureLogicDeviceTemplate,
    StructureLogicTemplate, StructureSlotsTemplate, StructureTemplate,
};

pub fn generate_database(
    stationpedia: &stationpedia::Stationpedia,
    enums: &enums::Enums,
    workspace: &std::path::Path,
) -> color_eyre::Result<Vec<PathBuf>> {
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

    let prefab_map_path = workspace
        .join("stationeers_data")
        .join("src")
        .join("database")
        .join("prefab_map.rs");
    let mut prefab_map_file = std::io::BufWriter::new(std::fs::File::create(&prefab_map_path)?);
    write_prefab_map(&mut prefab_map_file, &db.prefabs)?;

    Ok(vec![prefab_map_path])
}

fn write_prefab_map<T: std::io::Write>(
    writer: &mut BufWriter<T>,
    prefabs: &BTreeMap<String, ObjectTemplate>,
) -> color_eyre::Result<()> {
    write!(
        writer,
        "{}",
        quote! {
            use crate::enums::script_enums::*;
            use crate::enums::basic_enums::*;
            use crate::enums::{MemoryAccess, ConnectionType, ConnectionRole};
            use crate::templates::*;
        }
    )?;
    let entries = prefabs
        .values()
        .map(|prefab| {
            let hash = prefab.prefab().prefab_hash;
            let obj = syn::parse_str::<syn::Expr>(&uneval::to_string(prefab)?)?;
            let entry = quote! {
                (
                    #hash,
                    #obj.into(),
                )
            };
            Ok(entry)
        })
        .collect::<Result<Vec<_>, color_eyre::Report>>()?;
    write!(
        writer,
        "{}",
        quote! {
            pub fn build_prefab_database() -> std::collections::BTreeMap<i32, crate::templates::ObjectTemplate> {
                #[allow(clippy::unreadable_literal)]
                std::collections::BTreeMap::from([
                    #(#entries),*
                ])
            }
        },
    )?;
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
            typ: slot
                .slot_type
                .parse()
                .unwrap_or_else(|err| panic!("faild to parse slot class: {err}")),
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

impl From<&stationpedia::LogicInfo> for LogicInfo {
    fn from(value: &stationpedia::LogicInfo) -> Self {
        LogicInfo {
            logic_slot_types: value
                .logic_slot_types
                .iter()
                .map(|(index, slt_map)| {
                    (
                        *index,
                        slt_map
                            .slot_types
                            .iter()
                            .map(|(key, val)| {
                                (
                                    key.parse().unwrap_or_else(|err| {
                                        panic!("failed to parse logic slot type: {err}")
                                    }),
                                    val.parse().unwrap_or_else(|err| {
                                        panic!("failed to parse memory access: {err}")
                                    }),
                                )
                            })
                            .collect(),
                    )
                })
                .collect(),
            logic_types: value
                .logic_types
                .types
                .iter()
                .map(|(key, val)| {
                    (
                        key.parse()
                            .unwrap_or_else(|err| panic!("failed to parse logic type: {err}")),
                        val.parse()
                            .unwrap_or_else(|err| panic!("failed to parse memory access: {err}")),
                    )
                })
                .collect(),
            modes: None,
            transmission_receiver: false,
            wireless_logic: false,
            circuit_holder: false,
        }
    }
}

impl From<&stationpedia::Item> for ItemInfo {
    fn from(item: &stationpedia::Item) -> Self {
        ItemInfo {
            consumable: item.consumable.unwrap_or(false),
            filter_type: item.filter_type.as_ref().map(|typ| {
                typ.parse()
                    .unwrap_or_else(|err| panic!("failed to parse filter type: {err}"))
            }),
            ingredient: item.ingredient.unwrap_or(false),
            max_quantity: item.max_quantity.unwrap_or(1.0) as u32,
            reagents: item
                .reagents
                .as_ref()
                .map(|map| map.iter().map(|(key, val)| (key.clone(), *val)).collect()),
            slot_class: item
                .slot_class
                .parse()
                .unwrap_or_else(|err| panic!("failed to parse slot class: {err}")),
            sorting_class: item
                .sorting_class
                .parse()
                .unwrap_or_else(|err| panic!("failed to parse sorting class: {err}")),
        }
    }
}

impl From<&stationpedia::Device> for DeviceInfo {
    fn from(value: &stationpedia::Device) -> Self {
        DeviceInfo {
            connection_list: value
                .connection_list
                .iter()
                .map(|(typ, role)| ConnectionInfo {
                    typ: typ
                        .parse()
                        .unwrap_or_else(|err| panic!("failed to parse connection type: {err}")),
                    role: role
                        .parse()
                        .unwrap_or_else(|err| panic!("failed to parse connection role: {err}")),
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

impl From<&stationpedia::Structure> for StructureInfo {
    fn from(value: &stationpedia::Structure) -> Self {
        StructureInfo {
            small_grid: value.small_grid,
        }
    }
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

impl From<&stationpedia::Memory> for MemoryInfo {
    fn from(value: &stationpedia::Memory) -> Self {
        MemoryInfo {
            instructions: value.instructions.as_ref().map(|insts| {
                insts
                    .iter()
                    .map(|(key, value)| (key.clone(), value.into()))
                    .collect()
            }),
            memory_access: value
                .memory_access
                .parse()
                .unwrap_or_else(|err| panic!("failed to parse memory access: {err}")),
            memory_size: value.memory_size,
        }
    }
}
