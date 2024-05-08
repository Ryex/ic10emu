use macro_rules_attribute::derive;
use serde::{Deserialize, Serialize};

pub mod errors;
pub mod macros;
pub mod stationpedia;
pub mod traits;

use traits::*;

use crate::{device::SlotType, vm::enums::script_enums::LogicSlotType as SlotLogicType};

pub type ObjectID = u32;
pub type BoxedObject = Box<dyn Object<ID = ObjectID>>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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
    pub fn set(&mut self, name: &str) {
        self.value = name.to_owned();
        self.hash = const_crc32::crc32(name.as_bytes()) as i32;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldType {
    Read,
    Write,
    ReadWrite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicField {
    pub field_type: FieldType,
    pub value: f64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Slot {
    pub typ: SlotType,
    pub enabeled_logic: Vec<SlotLogicType>,
    pub occupant: Option<ObjectID>,
}
