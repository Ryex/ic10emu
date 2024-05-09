#![allow(non_snake_case)]

use std::collections::BTreeMap;

use itertools::Itertools;
use serde_derive::{Deserialize, Serialize};
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

#[serde_as]
#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SlotOccupant {
    pub id: u32,
    pub prefab_hash: i32,
    pub quantity: u32,
    pub max_quantity: u32,
    pub damage: f64,
    pub fields: BTreeMap<ic10emu::grammar::LogicSlotType, ic10emu::device::LogicField>,
}

impl From<&ic10emu::device::SlotOccupant> for SlotOccupant {
    fn from(value: &ic10emu::device::SlotOccupant) -> Self {
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

#[serde_as]
#[derive(Tsify, Debug, Clone, Default, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Slot {
    pub typ: ic10emu::device::SlotType,
    pub occupant: Option<SlotOccupant>,
    pub fields: BTreeMap<ic10emu::grammar::LogicSlotType, ic10emu::device::LogicField>,
}

impl From<&ic10emu::device::Slot> for Slot {
    fn from(value: &ic10emu::device::Slot) -> Self {
        Slot {
            typ: value.typ,
            occupant: value.occupant.as_ref().map(|occupant| occupant.into()),
            fields: value.get_fields(),
        }
    }
}

#[serde_as]
#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Slots(pub Vec<Slot>);

impl<'a> FromIterator<&'a ic10emu::device::Slot> for Slots {
    fn from_iter<T: IntoIterator<Item = &'a ic10emu::device::Slot>>(iter: T) -> Self {
        Slots(iter.into_iter().map(|slot| slot.into()).collect_vec())
    }
}

include!(concat!(env!("OUT_DIR"), "/ts_types.rs"));
