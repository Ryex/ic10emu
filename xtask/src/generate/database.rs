#![allow(clippy::module_name_repetitions, clippy::enum_glob_use)]

use std::{
    collections::BTreeMap,
    io::{BufWriter, Write},
    path::PathBuf,
};

use color_eyre::eyre::{eyre, Context};

use itertools::Itertools;
use quote::quote;
use serde_derive::{Deserialize, Serialize};

use crate::{
    enums,
    stationpedia::{self, Memory, Page, Stationpedia},
};

use stationeers_data::{
    enums::{
        script::{LogicSlotType, LogicType},
        MemoryAccess,
    },
    templates::{
        ConnectionInfo, ConsumerInfo, DeviceInfo, FabricatorInfo, Instruction, InstructionPart,
        InstructionPartType, InternalAtmoInfo, ItemCircuitHolderTemplate, ItemConsumerTemplate,
        ItemInfo, ItemLogicMemoryTemplate, ItemLogicTemplate, ItemSlotsTemplate,
        ItemSuitCircuitHolderTemplate, ItemSuitLogicTemplate, ItemSuitTemplate, ItemTemplate,
        LogicInfo, MemoryInfo, ObjectTemplate, PrefabInfo, Reagent, Recipe, RecipeGasMix,
        RecipeRange, SlotInfo, StructureCircuitHolderTemplate, StructureInfo,
        StructureLogicDeviceConsumerMemoryTemplate, StructureLogicDeviceConsumerTemplate,
        StructureLogicDeviceMemoryTemplate, StructureLogicDeviceTemplate, StructureLogicTemplate,
        StructureSlotsTemplate, StructureTemplate, SuitInfo, ThermalInfo,
    },
};

#[allow(clippy::too_many_lines)]
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

    let reagents = stationpedia
        .reagents
        .iter()
        .map(|(name, reagent)| {
            (
                name.clone(),
                Into::<Reagent>::into(reagent).with_name(name.clone()),
            )
        })
        .collect();

    let db: ObjectDatabase = ObjectDatabase {
        prefabs,
        reagents,
        enums: enums.clone(),
        prefabs_by_hash,
        structures,
        devices,
        items,
        logicable_items,
        suits,
        circuit_holders,
    };

    let data_path = workspace
        .join("www")
        .join("src")
        .join("ts")
        .join("database");
    if !data_path.exists() {
        std::fs::create_dir(&data_path)?;
    }
    {
        let database_path = data_path.join("prefabDatabase.ts");
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
        let null_matcher =
            regex::Regex::new(r#"(?:,\n\s*"\w+":\snull)+(,?)|(?:(?:\n)?\s*"\w+":\snull),"#)
                .unwrap();
        let json = null_matcher.replace_all(&json, "$1");
        write!(&mut database_file, "export default {json} as const")?;
        database_file.flush()?;
    }

    let prefab_map_path = workspace
        .join("stationeers_data")
        .join("src")
        .join("database")
        .join("prefab_map.rs");
    let mut prefab_map_file = std::io::BufWriter::new(std::fs::File::create(&prefab_map_path)?);
    write_prefab_map(&mut prefab_map_file, &db.prefabs)?;

    let reagent_map_path = workspace
        .join("stationeers_data")
        .join("src")
        .join("database")
        .join("reagent_map.rs");
    let mut reagent_map_file = std::io::BufWriter::new(std::fs::File::create(&reagent_map_path)?);
    write_reagent_map(&mut reagent_map_file, &db.reagents)?;

    Ok(vec![prefab_map_path, reagent_map_path])
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
    let enum_tag_regex = regex::Regex::new(r#"templateType:\s"\w+"\.into\(\),"#).unwrap();
    let numeric_string_literal_regex = regex::Regex::new(r#""(\d+)"\.into\(\)"#).unwrap();
    let entries = prefabs
        .values()
        .map(|prefab| {
            let hash = prefab.prefab().prefab_hash;
            let uneval_src = &uneval::to_string(prefab)?;
            let fixed = enum_tag_regex.replace_all(&uneval_src, "");
            let fixed = numeric_string_literal_regex
                .replace_all(&fixed, |captures: &regex::Captures| captures[1].to_string());
            let obj = syn::parse_str::<syn::Expr>(&fixed)?;
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

fn write_reagent_map<T: std::io::Write>(
    writer: &mut BufWriter<T>,
    reagents: &BTreeMap<String, Reagent>,
) -> color_eyre::Result<()> {
    write!(
        writer,
        "{}",
        quote! {
            use crate::templates::Reagent;
        }
    )?;
    let entries = reagents
        .values()
        .map(|reagent| {
            let id = reagent.id;
            let uneval_src = &uneval::to_string(reagent)?;
            let obj = syn::parse_str::<syn::Expr>(&uneval_src)?;
            let entry = quote! {
                map.insert(#id, #obj);
            };
            Ok(entry)
        })
        .collect::<Result<Vec<_>, color_eyre::Report>>()?;
    write!(
        writer,
        "{}",
        quote! {
            pub fn build_reagent_database() -> std::collections::BTreeMap<u8, crate::templates::Reagent> {
                #[allow(clippy::unreadable_literal)]
                let mut map: std::collections::BTreeMap<u8, crate::templates::Reagent> = std::collections::BTreeMap::new();
                #(#entries)*
                map
            }
        },
    )?;
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn generate_templates(pedia: &Stationpedia) -> color_eyre::Result<Vec<ObjectTemplate>> {
    eprintln!("Generating templates ...");
    let mut templates: Vec<ObjectTemplate> = Vec::new();
    for page in &pedia.pages {
        let name = &page.prefab_name;

        let span = tracing::span!(tracing::Level::INFO, "generate_template", prefab = name);
        let _enter = span.enter();

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
                slots,
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
                    slots: slot_inserts_to_info(slots, slot_inserts)
                        .wrap_err_with(|| format!("Failed to generate slot info for '{name}'"))?,
                }));
            }
            Page {
                item: Some(item),
                structure: None,
                logic_info: None,
                slots,
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
                    slots: slot_inserts_to_info(slots, slot_inserts)
                        .wrap_err_with(|| format!("Failed to generate slot info for '{name}'"))?,
                    consumer_info: consumer.into(),
                }));
            }
            Page {
                item: Some(item),
                structure: None,
                logic_info: None,
                slots,
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
                    slots: slot_inserts_to_info(slots, slot_inserts)
                        .wrap_err_with(|| format!("Failed to generate slot info for '{name}'"))?,
                    suit_info: item.suit.as_ref().unwrap().into(),
                }));
            }
            Page {
                item: Some(item),
                structure: None,
                logic_info: Some(logic),
                slots,
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
                let mut logic: LogicInfo = logic.try_into()?;
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
                    slots: slot_inserts_to_info(slots, slot_inserts)
                        .wrap_err_with(|| format!("Failed to generate slot info for '{name}'"))?,
                    suit_info: item.suit.as_ref().unwrap().into(),
                }));
            }
            Page {
                item: Some(item),
                structure: None,
                logic_info: Some(logic),
                slots,
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
                let mut logic: LogicInfo = logic.try_into()?;
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
                    slots: slot_inserts_to_info(slots, slot_inserts)
                        .wrap_err_with(|| format!("Failed to generate slot info for '{name}'"))?,
                }));
            }
            Page {
                item: Some(item),
                structure: None,
                logic_info: Some(logic),
                slots,
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
                let mut logic: LogicInfo = logic.try_into()?;
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
                        slots: slot_inserts_to_info(slots, slot_inserts).wrap_err_with(|| {
                            format!("Failed to generate slot info for '{name}'")
                        })?,
                    },
                ));
            }
            Page {
                item: Some(item),
                structure: None,
                logic_info: Some(logic),
                slots,
                slot_inserts,
                memory,
                device: None,
                transmission_receiver,
                wireless_logic,
                circuit_holder: true,
                resource_consumer: None,
                internal_atmosphere,
                thermal,
                ..
            } if item.suit.is_some() => {
                let mut logic: LogicInfo = logic.try_into()?;
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
                        slots: slot_inserts_to_info(slots, slot_inserts).wrap_err_with(|| {
                            format!("Failed to generate slot info for '{name}'")
                        })?,
                        suit_info: item.suit.as_ref().unwrap().into(),
                        memory: memory.as_ref().map(Into::into),
                    },
                ));
            }
            Page {
                item: Some(item),
                structure: None,
                logic_info: Some(logic),
                slots,
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
                let mut logic: LogicInfo = logic.try_into()?;
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
                    slots: slot_inserts_to_info(slots, slot_inserts)
                        .wrap_err_with(|| format!("Failed to generate slot info for '{name}'"))?,
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
            }
            Page {
                item: None,
                structure: Some(structure),
                slots,
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
                    slots: slot_inserts_to_info(slots, slot_inserts)
                        .wrap_err_with(|| format!("Failed to generate slot info for '{name}'"))?,
                }));
            }
            Page {
                item: None,
                structure: Some(structure),
                logic_info: Some(logic),
                slots,
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
                let mut logic: LogicInfo = logic.try_into()?;
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
                    slots: slot_inserts_to_info(slots, slot_inserts)
                        .wrap_err_with(|| format!("Failed to generate slot info for '{name}'"))?,
                }));
            }
            Page {
                item: None,
                structure: Some(structure),
                logic_info: Some(logic),
                slots,
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
                let mut logic: LogicInfo = logic.try_into()?;
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
                        slots: slot_inserts_to_info(slots, slot_inserts).wrap_err_with(|| {
                            format!("Failed to generate slot info for '{name}'")
                        })?,
                        device: device.into(),
                    },
                ));
            }
            Page {
                item: None,
                structure: Some(structure),
                logic_info: Some(logic),
                slots,
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
                let mut logic: LogicInfo = logic.try_into()?;
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
                        slots: slot_inserts_to_info(slots, slot_inserts).wrap_err_with(|| {
                            format!("Failed to generate slot info for '{name}'")
                        })?,
                        device: device.into(),
                    },
                ));
            }
            Page {
                item: None,
                structure: Some(structure),
                logic_info: Some(logic),
                slots,
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
                let mut logic: LogicInfo = logic.try_into()?;
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
                        slots: slot_inserts_to_info(slots, slot_inserts).wrap_err_with(|| {
                            format!("Failed to generate slot info for '{name}'")
                        })?,
                        device: device.into(),
                        consumer_info: consumer.into(),
                        fabricator_info: device.fabricator.as_ref().map(Into::into),
                    },
                ));
            }
            Page {
                item: None,
                structure: Some(structure),
                logic_info: Some(logic),
                slots,
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
                let mut logic: LogicInfo = logic.try_into()?;
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
                        slots: slot_inserts_to_info(slots, slot_inserts).wrap_err_with(|| {
                            format!("Failed to generate slot info for '{name}'")
                        })?,
                        device: device.into(),
                        memory: memory.into(),
                    },
                ));
            }
            Page {
                item: None,
                structure: Some(structure),
                logic_info: Some(logic),
                slots,
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
                let mut logic: LogicInfo = logic.try_into()?;
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
                        slots: slot_inserts_to_info(slots, slot_inserts).wrap_err_with(|| {
                            format!("Failed to generate slot info for '{name}'")
                        })?,
                        device: device.into(),
                        consumer_info: consumer.into(),
                        fabricator_info: device.fabricator.as_ref().map(Into::into),
                        memory: memory.into(),
                    },
                ));
            }
            _ => {
                return Err(eyre!(
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
                ))
            }
        }
    }
    Ok(templates)
}

fn slot_inserts_to_info(
    slots: &[stationpedia::SlotInfo],
    inserts: &[stationpedia::SlotInsert],
) -> color_eyre::Result<BTreeMap<u32, SlotInfo>> {
    let mut tmp: Vec<_> = inserts.into();
    tmp.sort_by(|a, b| a.slot_index.cmp(&b.slot_index));
    Ok(tmp
        .iter()
        .zip_longest(slots.iter())
        .enumerate()
        .map(|(index, pair)| {
            let (slot, insert) = match pair {
                itertools::EitherOrBoth::Both(insert, slot) => (Some(slot), insert),
                itertools::EitherOrBoth::Left(insert) => (None, insert),
                itertools::EitherOrBoth::Right(slot) => {
                    let slot_key = &slot.string_key;
                    let slot_class = &slot.class;
                    return Err(eyre!(
                        "Slot'{slot_key}' without an insert! class: '{slot_class}'"
                    ));
                }
            };
            // Collapse whitespace
            if insert.slot_type == "Proxy" {
                Ok((
                    insert.slot_index,
                    SlotInfo::Proxy {
                        name: insert.slot_name.clone(),
                        index: insert.slot_index,
                    },
                ))
            } else {
                if let Some(slot) = slot {
                    let class = &slot.class;
                    let name = &slot.slot_name;
                    Ok((
                        index.try_into()?,
                        SlotInfo::Direct {
                            name: name.clone(),
                            class: class.parse().map_err(|err| {
                                eyre!("failed to parse slot class '{class}' for '{name}': {err}")
                            })?,
                            index: index.try_into()?,
                        },
                    ))
                } else {
                    let slot_index = &insert.slot_index;
                    let slot_typ = &insert.slot_type;
                    let slot_name = &insert.slot_name;
                    return Err(eyre!("Non Proxy Slot Insert, without Slot: index: {slot_index}, name: {slot_name}, type: {slot_typ}"));
                }
            }
        })
        .collect::<color_eyre::Result<BTreeMap<u32, SlotInfo>>>()?)
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
    pub reagents: BTreeMap<String, Reagent>,
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
            hygiene_reduction_multiplier: value.hygiene_reduction_multiplier,
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

impl TryFrom<&stationpedia::LogicInfo> for LogicInfo {
    type Error = color_eyre::eyre::Report;
    fn try_from(value: &stationpedia::LogicInfo) -> Result<Self, Self::Error> {
        Ok(LogicInfo {
            logic_slot_types: value
                .logic_slot_types
                .iter()
                .map(|(index, slt_map)| {
                    Ok(
                            (
                                *index,
                                slt_map
                                    .slot_types
                                    .iter()
                                    .map(|(key, val)| {
                                        Ok((
                                    key.parse().wrap_err_with(|| {
                                        format!("failed to parse logic slot type '{key}'")
                                    })?,
                                    val.parse().wrap_err_with(|| {
                                        format!("failed to parse memory access '{val}'")
                                    })?,
                                ))
                                    })
                                    .collect::<color_eyre::eyre::Result<
                                        BTreeMap<LogicSlotType, MemoryAccess>,
                                    >>()?,
                            ),
                        )
                })
                .collect::<color_eyre::eyre::Result<BTreeMap<u32, _>>>()?,
            logic_types: value
                .logic_types
                .types
                .iter()
                .map(|(key, val)| {
                    Ok((
                        key.parse()
                            .wrap_err_with(|| format!("failed to parse logic type '{key}'"))?,
                        val.parse()
                            .wrap_err_with(|| format!("failed to parse memory access '{val}'"))?,
                    ))
                })
                .collect::<color_eyre::Result<BTreeMap<LogicType, MemoryAccess>>>()?,
            modes: None,
            transmission_receiver: false,
            wireless_logic: false,
            circuit_holder: false,
        })
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
            slot_class: item.slot_class.parse().unwrap_or_else(|err| {
                let slot_class = &item.slot_class;
                panic!("failed to parse slot class `{slot_class}`: {err}");
            }),
            sorting_class: item.sorting_class.parse().unwrap_or_else(|err| {
                let sorting_class = &item.sorting_class;
                panic!("failed to parse sorting class `{sorting_class}`: {err}");
            }),
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
                    typ: typ.parse().unwrap_or_else(|err| {
                        panic!("failed to parse connection type `{typ}`: {err}")
                    }),
                    role: role.parse().unwrap_or_else(|err| {
                        panic!("failed to parse connection role `{role}`: {err}")
                    }),
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
            consumed_resources: value.consumed_resources.clone(),
            processed_reagents: value.processed_reagents.clone(),
        }
    }
}

impl From<&stationpedia::Reagent> for Reagent {
    fn from(value: &stationpedia::Reagent) -> Self {
        Reagent {
            id: value.id,
            name: String::new(),
            hash: value.hash,
            unit: value.unit.clone(),
            is_organic: value.is_organic,
            sources: value.sources.clone().unwrap_or_default(),
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
                .map(|(prefab, val)| Into::<Recipe>::into(val).with_target(prefab))
                .collect(),
        }
    }
}

impl From<&stationpedia::Recipe> for Recipe {
    fn from(value: &stationpedia::Recipe) -> Self {
        Recipe {
            target_prefab: String::new(),
            target_prefab_hash: 0,
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
