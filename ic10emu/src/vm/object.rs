use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use macro_rules_attribute::derive;
use serde_derive::{Deserialize, Serialize};

pub mod errors;
pub mod generic;
pub mod humans;
pub mod macros;
pub mod stationpedia;
pub mod templates;
pub mod traits;

use traits::Object;

use crate::vm::VM;
#[cfg(feature = "tsify")]
use tsify::{declare, Tsify};
#[cfg(feature = "tsify")]
use wasm_bindgen::prelude::*;

use stationeers_data::enums::{
    basic::Class, prefabs::StationpediaPrefab, script::LogicSlotType, MemoryAccess,
};

#[cfg_attr(feature = "tsify", declare)]
pub type ObjectID = u32;
pub type BoxedObject = Rc<RefCell<dyn Object>>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "tsify", wasm_bindgen)]
pub struct VMObject(BoxedObject);

impl Deref for VMObject {
    type Target = BoxedObject;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VMObject {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl VMObject {
    pub fn new<T>(val: T) -> Self
    where
        T: Object + 'static,
    {
        VMObject(Rc::new(RefCell::new(val)))
    }

    pub fn set_vm(&mut self, vm: Rc<VM>) {
        self.borrow_mut().set_vm(vm);
    }

    pub fn get_vm(&self) -> Rc<VM> {
        self.borrow().get_vm().clone()
    }

    pub fn get_id(&self) -> ObjectID {
        *self.borrow().get_id()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Name {
    pub value: String,
    pub hash: i32,
}

#[allow(unused)]
impl Name {
    pub fn new(name: &str) -> Self {
        Name {
            value: name.to_owned(),
            hash: const_crc32::crc32(name.as_bytes()) as i32,
        }
    }
    pub fn from_prefab_name(name: &str) -> Self {
        Name {
            value: name.to_string(),
            hash: name
                .parse::<StationpediaPrefab>()
                .map(|prefab| prefab as i32)
                .unwrap_or_else(|_| const_crc32::crc32(name.as_bytes()) as i32),
        }
    }
    pub fn from_prefab_hash(hash: i32) -> Option<Self> {
        StationpediaPrefab::from_repr(hash).map(|prefab| Name {
            value: prefab.to_string(),
            hash,
        })
    }
    pub fn set(&mut self, name: &str) {
        self.value = name.to_owned();
        self.hash = const_crc32::crc32(name.as_bytes()) as i32;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct LogicField {
    pub field_type: MemoryAccess,
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct SlotOccupantInfo {
    pub quantity: u32,
    pub id: ObjectID,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Slot {
    pub parent: ObjectID,
    pub index: usize,
    pub name: String,
    pub typ: Class,
    pub readable_logic: Vec<LogicSlotType>,
    pub writeable_logic: Vec<LogicSlotType>,
    pub occupant: Option<SlotOccupantInfo>,
}

impl Slot {
    #[must_use]
    pub fn new(parent: ObjectID, index: usize, name: String, typ: Class) -> Self {
        Slot {
            parent,
            index,
            name,
            typ,
            readable_logic: vec![
                LogicSlotType::Class,
                LogicSlotType::Damage,
                LogicSlotType::MaxQuantity,
                LogicSlotType::OccupantHash,
                LogicSlotType::Occupied,
                LogicSlotType::PrefabHash,
                LogicSlotType::Quantity,
                LogicSlotType::ReferenceId,
                LogicSlotType::SortingClass,
            ],
            writeable_logic: vec![],
            occupant: None,
        }
    }
}
