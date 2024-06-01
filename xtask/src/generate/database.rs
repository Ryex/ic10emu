#![allow(clippy::module_name_repetitions, clippy::enum_glob_use)]

use std::{
    collections::BTreeMap,
    io::{BufWriter, Write},
    path::PathBuf,
};

use quote::quote;
use serde_derive::{Deserialize, Serialize};

use crate::{
    enums,
    stationpedia::{self, Memory, Page, Stationpedia},
};

use stationeers_data::templates::{
    ConnectionInfo, ConsumerInfo, DeviceInfo, FabricatorInfo, Instruction, InstructionPart,
    InstructionPartType, InternalAtmoInfo, ItemCircuitHolderTemplate, ItemConsumerTemplate,
    ItemInfo, ItemLogicMemoryTemplate, ItemLogicTemplate, ItemSlotsTemplate,
    ItemSuitCircuitHolderTemplate, ItemSuitLogicTemplate, ItemSuitTemplate, ItemTemplate,
    LogicInfo, MemoryInfo, ObjectTemplate, PrefabInfo, Recipe, RecipeGasMix, RecipeRange, SlotInfo,
    StructureCircuitHolderTemplate, StructureInfo, StructureLogicDeviceConsumerMemoryTemplate,
    StructureLogicDeviceConsumerTemplate, StructureLogicDeviceMemoryTemplate,
    StructureLogicDeviceTemplate, StructureLogicTemplate, StructureSlotsTemplate,
    StructureTemplate, SuitInfo, ThermalInfo,
};

#[allow(clippy::too_many_lines)]
pub fn generate_database(
    stationpedia: &stationpedia::Stationpedia,
    enums: &enums::Enums,
    workspace: &std::path::Path,
) -> color_eyre::Result<Vec<PathBuf>> {
    let templates = generate_templates(stationpedia);

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
                | StructureCircuitHolder(_)
                | StructureLogicDeviceConsumer(_)
                | StructureLogicDeviceMemory(_)
                | StructureLogicDeviceConsumerMemory(_) => Some(val.prefab().prefab_name.clone()),
                Item(_)
                | ItemSlots(_)
                | ItemConsumer(_)
                | ItemLogic(_)
                | ItemCircuitHolder(_)
                | ItemLogicMemory(_)
                | ItemSuit(_)
                | ItemSuitLogic(_)
                | ItemSuitCircuitHolder(_)
                | Human(_) => None,
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
                | StructureCircuitHolder(_)
                | StructureLogicDeviceConsumer(_)
                | StructureLogicDeviceMemory(_)
                | StructureLogicDeviceConsumerMemory(_)
                | Human(_) => None,
                Item(_)
                | ItemSlots(_)
                | ItemConsumer(_)
                | ItemLogic(_)
                | ItemCircuitHolder(_)
                | ItemLogicMemory(_)
                | ItemSuit(_)
                | ItemSuitLogic(_)
                | ItemSuitCircuitHolder(_) => Some(val.prefab().prefab_name.clone()),
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
                | StructureCircuitHolder(_)
                | StructureLogicDeviceConsumer(_)
                | StructureLogicDeviceMemory(_)
                | StructureLogicDeviceConsumerMemory(_)
                | Item(_)
                | ItemSlots(_)
                | ItemSuit(_)
                | ItemConsumer(_)
                | Human(_) => None,
                ItemLogic(_)
                | ItemCircuitHolder(_)
                | ItemLogicMemory(_)
                | ItemSuitLogic(_)
                | ItemSuitCircuitHolder(_) => Some(val.prefab().prefab_name.clone()),
            }
        })
        .collect();

    let devices = prefabs
        .iter()
        .filter_map(|(_, val)| {
            use ObjectTemplate::*;
            match val {
                Structure(_)
                | StructureSlots(_)
                | StructureLogic(_)
                | Item(_)
                | ItemSlots(_)
                | ItemConsumer(_)
                | ItemLogic(_)
                | ItemCircuitHolder(_)
                | ItemLogicMemory(_)
                | ItemSuit(_)
                | ItemSuitLogic(_)
                | ItemSuitCircuitHolder(_)
                | Human(_) => None,
                StructureLogicDevice(_)
                | StructureCircuitHolder(_)
                | StructureLogicDeviceMemory(_)
                | StructureLogicDeviceConsumer(_)
                | StructureLogicDeviceConsumerMemory(_) => Some(val.prefab().prefab_name.clone()),
            }
        })
        .collect();
    let suits = prefabs
        .iter()
        .filter_map(|(_, val)| {
            use ObjectTemplate::*;
            match val {
                ItemSuitCircuitHolder(_) | ItemSuitLogic(_) | ItemSuit(_) => {
                    Some(val.prefab().prefab_name.clone())
                }
                _ => None,
            }
        })
        .collect();
    let circuit_holders = prefabs
        .iter()
        .filter_map(|(_, val)| {
            use ObjectTemplate::*;
            match val {
                ItemSuitCircuitHolder(_) | ItemCircuitHolder(_) | StructureCircuitHolder(_) => {
                    Some(val.prefab().prefab_name.clone())
                }
                _ => None,
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
        suits,
        circuit_holders,
    };

    let data_path = workspace.join("www").join("data");
    if !data_path.exists() {
        std::fs::create_dir(&data_path)?;
    }
    {
        let database_path = data_path.join("database.json");
        let mut database_file = std::io::BufWriter::new(std::fs::File::create(database_path)?);
        let json = serde_json::to_string_pretty(&db)?;
        // this may seem anathema but I don't want to write a separate struct set to skip Nones
        // the current set can't skip Nones to be uneval compatible
        // we are pretty printing and I know the keys are well formed and that all nulls are from a
        // None so a regex to replace them is easy and sound
        //
        // remove preceding comma if it exists, leave trailing comma intact if it exists, capture
        // repeating groups of null fields
        //
        // https://regex101.com/r/WFpjHV/1
        //
        let null_matcher = regex::Regex::new(r#"(?:,\n\s*"\w+":\snull)+(,?)|(?:(?:\n)?\s*"\w+":\snull),"#).unwrap();
        let json = null_matcher.replace_all(&json, "$1");
        write!(&mut database_file, "{json}")?;
        database_file.flush()?;
    }

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
            use crate::enums::script::*;
            use crate::enums::basic::*;
            use crate::enums::{MemoryAccess, ConnectionType, ConnectionRole, MachineTier};
            use crate::templates::*;
        }
    )?;
    let entries = prefabs
        .values()
        .map(|prefab| {
            let hash = prefab.prefab().prefab_hash;
            let obj = syn::parse_str::<syn::Expr>(&uneval::to_string(prefab)?)?;
            let entry = quote! {
                map.insert(#hash, #obj.into());
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
                let mut map: std::collections::BTreeMap<i32, crate::templates::ObjectTemplate> = std::collections::BTreeMap::new();
                #(#entries)*
                map
            }
        },
    )?;
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn generate_templates(pedia: &Stationpedia) -> Vec<ObjectTemplate> {
    println!("Generating templates ...");
    let mut templates: Vec<ObjectTemplate> = Vec::new();
    for page in &pedia.pages {
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
                transmission_receiver: false,
                wireless_logic: false,
                circuit_holder: false,
                resource_consumer: None,
                internal_atmosphere,
                thermal,
                ..
            } if slot_inserts.is_empty() && item.suit.is_none() => {
                templates.push(ObjectTemplate::Item(ItemTemplate {
                    prefab,
                    item: item.into(),
                    thermal_info: thermal.as_ref().map(Into::into),
                    internal_atmo_info: internal_atmosphere.as_ref().map(Into::into),
                }));
            }
            Page {
                item: Some(item),
                structure: None,
                logic_info: None,
                slot_inserts,
                memory: None,
                device: None,
                transmission_receiver: false,
                wireless_logic: false,
                circuit_holder: false,
                resource_consumer: None,
                internal_atmosphere,
                thermal,
                ..
            } if item.suit.is_none() => {
                templates.push(ObjectTemplate::ItemSlots(ItemSlotsTemplate {
                    prefab,
                    item: item.into(),
                    thermal_info: thermal.as_ref().map(Into::into),
                    internal_atmo_info: internal_atmosphere.as_ref().map(Into::into),
                    slots: slot_inserts_to_info(slot_inserts),
                }));
            }
            Page {
                item: Some(item),
                structure: None,
                logic_info: None,
                slot_inserts,
                memory: None,
                device: None,
                transmission_receiver: false,
                wireless_logic: false,
                circuit_holder: false,
                resource_consumer: Some(consumer),
                internal_atmosphere,
                thermal,
                ..
            } if item.suit.is_none() => {
                templates.push(ObjectTemplate::ItemConsumer(ItemConsumerTemplate {
                    prefab,
                    item: item.into(),
                    thermal_info: thermal.as_ref().map(Into::into),
                    internal_atmo_info: internal_atmosphere.as_ref().map(Into::into),
                    slots: slot_inserts_to_info(slot_inserts),
                    consumer_info: consumer.into(),
                }));
            }
            Page {
                item: Some(item),
                structure: None,
                logic_info: None,
                slot_inserts,
                memory: None,
                device: None,
                transmission_receiver: false,
                wireless_logic: false,
                circuit_holder: false,
                resource_consumer: None,
                internal_atmosphere,
                thermal,
                ..
            } if item.suit.is_some() => {
                templates.push(ObjectTemplate::ItemSuit(ItemSuitTemplate {
                    prefab,
                    item: item.into(),
                    thermal_info: thermal.as_ref().map(Into::into),
                    internal_atmo_info: internal_atmosphere.as_ref().map(Into::into),
                    slots: slot_inserts_to_info(slot_inserts),
                    suit_info: item.suit.as_ref().unwrap().into(),
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
                circuit_holder: false,
                resource_consumer: None,
                internal_atmosphere,
                thermal,
                ..
            } if item.suit.is_some() => {
                let mut logic: LogicInfo = logic.into();
                if !page.mode_insert.is_empty() {
                    logic.modes = Some(mode_inserts_to_info(&page.mode_insert));
                }
                logic.transmission_receiver = *transmission_receiver;
                logic.wireless_logic = *wireless_logic;
                logic.circuit_holder = false;

                templates.push(ObjectTemplate::ItemSuitLogic(ItemSuitLogicTemplate {
                    prefab,
                    item: item.into(),
                    thermal_info: thermal.as_ref().map(Into::into),
                    internal_atmo_info: internal_atmosphere.as_ref().map(Into::into),
                    logic,
                    slots: slot_inserts_to_info(slot_inserts),
                    suit_info: item.suit.as_ref().unwrap().into(),
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
                circuit_holder: false,
                resource_consumer: None,
                internal_atmosphere,
                thermal,
                ..
            } if item.suit.is_none() => {
                let mut logic: LogicInfo = logic.into();
                if !page.mode_insert.is_empty() {
                    logic.modes = Some(mode_inserts_to_info(&page.mode_insert));
                }
                logic.transmission_receiver = *transmission_receiver;
                logic.wireless_logic = *wireless_logic;
                logic.circuit_holder = false;

                templates.push(ObjectTemplate::ItemLogic(ItemLogicTemplate {
                    prefab,
                    item: item.into(),
                    thermal_info: thermal.as_ref().map(Into::into),
                    internal_atmo_info: internal_atmosphere.as_ref().map(Into::into),
                    logic,
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
                circuit_holder: true,
                resource_consumer: None,
                internal_atmosphere,
                thermal,
                ..
            } if item.suit.is_none() => {
                let mut logic: LogicInfo = logic.into();
                if !page.mode_insert.is_empty() {
                    logic.modes = Some(mode_inserts_to_info(&page.mode_insert));
                }
                logic.transmission_receiver = *transmission_receiver;
                logic.wireless_logic = *wireless_logic;
                logic.circuit_holder = true;

                templates.push(ObjectTemplate::ItemCircuitHolder(
                    ItemCircuitHolderTemplate {
                        prefab,
                        item: item.into(),
                        thermal_info: thermal.as_ref().map(Into::into),
                        internal_atmo_info: internal_atmosphere.as_ref().map(Into::into),
                        logic,
                        slots: slot_inserts_to_info(slot_inserts),
                    },
                ));
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
                circuit_holder: true,
                resource_consumer: None,
                internal_atmosphere,
                thermal,
                ..
            } if item.suit.is_some() => {
                let mut logic: LogicInfo = logic.into();
                if !page.mode_insert.is_empty() {
                    logic.modes = Some(mode_inserts_to_info(&page.mode_insert));
                }
                logic.transmission_receiver = *transmission_receiver;
                logic.wireless_logic = *wireless_logic;
                logic.circuit_holder = true;

                templates.push(ObjectTemplate::ItemSuitCircuitHolder(
                    ItemSuitCircuitHolderTemplate {
                        prefab,
                        item: item.into(),
                        thermal_info: thermal.as_ref().map(Into::into),
                        internal_atmo_info: internal_atmosphere.as_ref().map(Into::into),
                        logic,
                        slots: slot_inserts_to_info(slot_inserts),
                        suit_info: item.suit.as_ref().unwrap().into(),
                        memory: memory.into(),
                    },
                ));
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
                circuit_holder: false,
                resource_consumer: None,
                internal_atmosphere,
                thermal,
                ..
            } if item.suit.is_none() => {
                let mut logic: LogicInfo = logic.into();
                if !page.mode_insert.is_empty() {
                    logic.modes = Some(mode_inserts_to_info(&page.mode_insert));
                }
                logic.transmission_receiver = *transmission_receiver;
                logic.wireless_logic = *wireless_logic;
                logic.circuit_holder = false;

                templates.push(ObjectTemplate::ItemLogicMemory(ItemLogicMemoryTemplate {
                    prefab,
                    item: item.into(),
                    thermal_info: thermal.as_ref().map(Into::into),
                    internal_atmo_info: internal_atmosphere.as_ref().map(Into::into),
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
                transmission_receiver: false,
                wireless_logic: false,
                circuit_holder: false,
                resource_consumer: None,
                internal_atmosphere,
                thermal,
                ..
            } if slot_inserts.is_empty() => {
                templates.push(ObjectTemplate::Structure(StructureTemplate {
                    prefab,
                    structure: structure.into(),
                    thermal_info: thermal.as_ref().map(Into::into),
                    internal_atmo_info: internal_atmosphere.as_ref().map(Into::into),
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
                transmission_receiver: false,
                wireless_logic: false,
                circuit_holder: false,
                resource_consumer: None,
                internal_atmosphere,
                thermal,
                ..
            } => {
                templates.push(ObjectTemplate::StructureSlots(StructureSlotsTemplate {
                    prefab,
                    structure: structure.into(),
                    thermal_info: thermal.as_ref().map(Into::into),
                    internal_atmo_info: internal_atmosphere.as_ref().map(Into::into),
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
                circuit_holder: false,
                resource_consumer: None,
                internal_atmosphere,
                thermal,
                ..
            } => {
                let mut logic: LogicInfo = logic.into();
                if !page.mode_insert.is_empty() {
                    logic.modes = Some(mode_inserts_to_info(&page.mode_insert));
                }
                logic.transmission_receiver = *transmission_receiver;
                logic.wireless_logic = *wireless_logic;
                logic.circuit_holder = false;

                templates.push(ObjectTemplate::StructureLogic(StructureLogicTemplate {
                    prefab,
                    structure: structure.into(),
                    thermal_info: thermal.as_ref().map(Into::into),
                    internal_atmo_info: internal_atmosphere.as_ref().map(Into::into),
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
                circuit_holder: false,
                resource_consumer: None,
                internal_atmosphere,
                thermal,
                ..
            } => {
                let mut logic: LogicInfo = logic.into();
                if !page.mode_insert.is_empty() {
                    logic.modes = Some(mode_inserts_to_info(&page.mode_insert));
                }
                logic.transmission_receiver = *transmission_receiver;
                logic.wireless_logic = *wireless_logic;
                logic.circuit_holder = false;

                templates.push(ObjectTemplate::StructureLogicDevice(
                    StructureLogicDeviceTemplate {
                        prefab,
                        structure: structure.into(),
                        thermal_info: thermal.as_ref().map(Into::into),
                        internal_atmo_info: internal_atmosphere.as_ref().map(Into::into),
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
                // NOTE: at the time of writing StructureCircuitHolder structure has a read write 0b memory, useless
                // other holders have no memory
                memory:
                    Some(Memory {
                        instructions: None,
                        memory_size: 0,
                        ..
                    })
                    | None,
                device: Some(device),
                transmission_receiver,
                wireless_logic,
                circuit_holder: true,
                resource_consumer: None,
                internal_atmosphere,
                thermal,
                ..
            } => {
                let mut logic: LogicInfo = logic.into();
                if !page.mode_insert.is_empty() {
                    logic.modes = Some(mode_inserts_to_info(&page.mode_insert));
                }
                logic.transmission_receiver = *transmission_receiver;
                logic.wireless_logic = *wireless_logic;
                logic.circuit_holder = true;

                templates.push(ObjectTemplate::StructureCircuitHolder(
                    StructureCircuitHolderTemplate {
                        prefab,
                        structure: structure.into(),
                        thermal_info: thermal.as_ref().map(Into::into),
                        internal_atmo_info: internal_atmosphere.as_ref().map(Into::into),
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
                memory: None,
                device: Some(device),
                transmission_receiver,
                wireless_logic,
                circuit_holder: false,
                resource_consumer: Some(consumer),
                internal_atmosphere,
                thermal,
                ..
            } => {
                let mut logic: LogicInfo = logic.into();
                if !page.mode_insert.is_empty() {
                    logic.modes = Some(mode_inserts_to_info(&page.mode_insert));
                }
                logic.transmission_receiver = *transmission_receiver;
                logic.wireless_logic = *wireless_logic;
                logic.circuit_holder = false;

                templates.push(ObjectTemplate::StructureLogicDeviceConsumer(
                    StructureLogicDeviceConsumerTemplate {
                        prefab,
                        structure: structure.into(),
                        thermal_info: thermal.as_ref().map(Into::into),
                        internal_atmo_info: internal_atmosphere.as_ref().map(Into::into),
                        logic,
                        slots: slot_inserts_to_info(slot_inserts),
                        device: device.into(),
                        consumer_info: consumer.into(),
                        fabricator_info: device.fabricator.as_ref().map(Into::into),
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
                circuit_holder: false,
                resource_consumer: None,
                internal_atmosphere,
                thermal,
                ..
            } => {
                let mut logic: LogicInfo = logic.into();
                if !page.mode_insert.is_empty() {
                    logic.modes = Some(mode_inserts_to_info(&page.mode_insert));
                }
                logic.transmission_receiver = *transmission_receiver;
                logic.wireless_logic = *wireless_logic;
                logic.circuit_holder = false;
                templates.push(ObjectTemplate::StructureLogicDeviceMemory(
                    StructureLogicDeviceMemoryTemplate {
                        prefab,
                        structure: structure.into(),
                        thermal_info: thermal.as_ref().map(Into::into),
                        internal_atmo_info: internal_atmosphere.as_ref().map(Into::into),
                        logic,
                        slots: slot_inserts_to_info(slot_inserts),
                        device: device.into(),
                        memory: memory.into(),
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
                circuit_holder: false,
                resource_consumer: Some(consumer),
                internal_atmosphere,
                thermal,
                ..
            } => {
                let mut logic: LogicInfo = logic.into();
                if !page.mode_insert.is_empty() {
                    logic.modes = Some(mode_inserts_to_info(&page.mode_insert));
                }
                logic.transmission_receiver = *transmission_receiver;
                logic.wireless_logic = *wireless_logic;
                logic.circuit_holder = false;
                templates.push(ObjectTemplate::StructureLogicDeviceConsumerMemory(
                    StructureLogicDeviceConsumerMemoryTemplate {
                        prefab,
                        structure: structure.into(),
                        thermal_info: thermal.as_ref().map(Into::into),
                        internal_atmo_info: internal_atmosphere.as_ref().map(Into::into),
                        logic,
                        slots: slot_inserts_to_info(slot_inserts),
                        device: device.into(),
                        consumer_info: consumer.into(),
                        fabricator_info: device.fabricator.as_ref().map(Into::into),
                        memory: memory.into(),
                    },
                ));
                // println!("Structure")
            }
            _ => panic!(
                "\
                    Non conforming: {:?} \n\t\
                        item: {:?}\n\t\
                        structure: {:?}\n\t\
                        logic_info: {:?}\n\t\
                        slot_inserts: {:?}\n\t\
                        slot_logic: {:?}\n\t\
                        memory: {:?}\n\t\
                        circuit_holder: {:?}\n\t\
                        device: {:?}\n\t\
                        resource_consumer: {:?}\n\t\
                        internal_atmosphere: {:?}\n\t\
                        thermal: {:?}\n\t\
                    ",
                page.key,
                page.item,
                page.structure,
                page.logic_info,
                page.slot_inserts,
                page.logic_slot_insert,
                page.memory,
                page.circuit_holder,
                page.device,
                page.resource_consumer,
                page.internal_atmosphere,
                page.thermal,
            ),
        }
    }
    templates
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
    pub suits: Vec<String>,
    pub circuit_holders: Vec<String>,
}

impl From<&stationpedia::SuitInfo> for SuitInfo {
    fn from(value: &stationpedia::SuitInfo) -> Self {
        SuitInfo {
            hygine_reduction_multiplier: value.hygine_reduction_multiplier,
            waste_max_pressure: value.waste_max_pressure,
        }
    }
}

impl From<&stationpedia::ThermalInfo> for ThermalInfo {
    fn from(value: &stationpedia::ThermalInfo) -> Self {
        ThermalInfo {
            convection_factor: value.convection,
            radiation_factor: value.radiation,
        }
    }
}

impl From<&stationpedia::InternalAtmosphereInfo> for InternalAtmoInfo {
    fn from(value: &stationpedia::InternalAtmosphereInfo) -> Self {
        InternalAtmoInfo {
            volume: value.volume,
        }
    }
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
            consumable: item.consumable,
            filter_type: item.filter_type.as_ref().map(|typ| {
                typ.parse()
                    .unwrap_or_else(|err| panic!("failed to parse filter type: {err}"))
            }),
            ingredient: item.ingredient,
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
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
        let color_re = regex::Regex::new(r"<color=.*?>|</color>").unwrap();
        let description_stripped = color_re.replace_all(&value.description, "").to_string();
        // https://regex101.com/r/GVNgq3/1
        let valid_range_re =
            regex::Regex::new(r"VALID ONLY AT ADDRESS(?:ES)? (?<start>\d+) (?:TO (?<end>\d+))?")
                .unwrap();
        // https://regex101.com/r/jwbISO/1
        let part_re =
            regex::Regex::new(r"[ \|]+(?<start>\d+)-(?<end>\d+)[ \|]+(?<name>[A-Z_]+)[ \|]+(?:(?<type>[A-Z]+_[0-9]+)|(?<unused_len>\d+))")
                .unwrap();
        let valid = {
            if let Some(caps) = valid_range_re.captures(&description_stripped) {
                (
                    caps.name("start").unwrap().as_str().parse().unwrap(),
                    caps.name("end").map(|cap| cap.as_str().parse().unwrap()),
                )
            } else {
                (0, None)
            }
        };
        let parts = {
            part_re
                .captures_iter(&description_stripped)
                .map(|caps| {
                    let typ = caps
                        .name("type")
                        .map(|cap| match cap.as_str() {
                            "BOOL_8" => InstructionPartType::Bool8,
                            "BYTE_8" => InstructionPartType::Byte8,
                            "INT_32" => InstructionPartType::Int32,
                            "UINT_32" => InstructionPartType::UInt32,
                            "SHORT_16" => InstructionPartType::Short16,
                            "USHORT_16" => InstructionPartType::UShort16,
                            s => InstructionPartType::Unknown(s.to_string()),
                        })
                        .unwrap_or_else(|| {
                            let len = caps
                                .name("unused_len")
                                .and_then(|cap| cap.as_str().parse().ok())
                                .unwrap_or(0);
                            InstructionPartType::Unused(len)
                        });
                    InstructionPart {
                        range: (
                            caps.name("start").unwrap().as_str().parse().unwrap(),
                            caps.name("end").unwrap().as_str().parse().unwrap(),
                        ),
                        name: caps.name("name").unwrap().as_str().to_string(),
                        typ,
                    }
                })
                .collect()
        };
        Instruction {
            description: value.description.clone(),
            description_stripped,
            typ: value.type_.clone(),
            value: value.value,
            valid,
            parts,
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

impl From<&stationpedia::ResourceConsumer> for ConsumerInfo {
    fn from(value: &stationpedia::ResourceConsumer) -> Self {
        ConsumerInfo {
            consumed_resouces: value.consumed_resources.clone(),
            processed_reagents: value.processed_reagents.clone(),
        }
    }
}

impl From<&stationpedia::Fabricator> for FabricatorInfo {
    fn from(value: &stationpedia::Fabricator) -> Self {
        FabricatorInfo {
            tier: value
                .tier_name
                .parse()
                .unwrap_or_else(|_| panic!("Unknown MachineTier {}", value.tier_name)),
            recipes: value
                .recipes
                .iter()
                .map(|(key, val)| (key.clone(), val.into()))
                .collect(),
        }
    }
}

impl From<&stationpedia::Recipe> for Recipe {
    fn from(value: &stationpedia::Recipe) -> Self {
        Recipe {
            tier: value
                .tier_name
                .parse()
                .unwrap_or_else(|_| panic!("Unknown MachineTier {}", value.tier_name)),
            time: value.time,
            energy: value.energy,
            temperature: (&value.temperature).into(),
            pressure: (&value.pressure).into(),
            required_mix: (&value.required_mix).into(),
            count_types: value.count_types,
            reagents: value
                .reagents
                .iter()
                .filter_map(|(key, val)| {
                    if *val == 0.0 {
                        None
                    } else {
                        Some((key.clone(), *val))
                    }
                })
                .collect(),
        }
    }
}

impl From<&stationpedia::RecipeTemperature> for RecipeRange {
    fn from(value: &stationpedia::RecipeTemperature) -> Self {
        RecipeRange {
            start: value.start,
            stop: value.stop,
            is_valid: value.is_valid,
        }
    }
}

impl From<&stationpedia::RecipePressure> for RecipeRange {
    fn from(value: &stationpedia::RecipePressure) -> Self {
        RecipeRange {
            start: value.start,
            stop: value.stop,
            is_valid: value.is_valid,
        }
    }
}

impl From<&stationpedia::RecipeGasMix> for RecipeGasMix {
    fn from(value: &stationpedia::RecipeGasMix) -> Self {
        RecipeGasMix {
            rule: value.rule,
            is_any: value.is_any,
            is_any_to_remove: value.is_any_to_remove,
            reagents: value
                .reagents
                .iter()
                .filter_map(|(key, val)| {
                    if *val == 0.0 {
                        None
                    } else {
                        Some((key.clone(), *val))
                    }
                })
                .collect(),
        }
    }
}
