use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[serde_as]
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Stack(#[serde_as(as = "[_; 512]")] pub [f64; 512]);

#[serde_as]
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Registers(#[serde_as(as = "[_; 18]")] pub [f64; 18]);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotOccupant {
    pub id: u32,
    pub prefab_hash: i32,
    pub quantity: u32,
    pub max_quantity: u32,
    pub damage: f64,
    pub fields: HashMap<ic10emu::grammar::SlotLogicType, ic10emu::LogicField>,
}

impl From<&ic10emu::SlotOccupant> for SlotOccupant {
    fn from(value: &ic10emu::SlotOccupant) -> Self {
        SlotOccupant {
            id: value.id,
            prefab_hash: value.prefab_hash,
            quantity: value.quantity,
            max_quantity: value.max_quantity,
            damage: value.damage,
            fields: value.get_fields(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Slot {
    pub typ: ic10emu::SlotType,
    pub occupant: Option<SlotOccupant>,
    pub fields: HashMap<ic10emu::grammar::SlotLogicType, ic10emu::LogicField>,
}

impl From<&ic10emu::Slot> for Slot {
    fn from(value: &ic10emu::Slot) -> Self {
        Slot {
            typ: value.typ,
            occupant: value.occupant.as_ref().map(|occupant| occupant.into()),
            fields: value.get_fields(),
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/ts_types.rs"));

