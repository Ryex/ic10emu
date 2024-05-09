use std::{cell::RefCell, ops::Deref, rc::Rc};

use macro_rules_attribute::derive;
use serde_derive::{Deserialize, Serialize};

pub mod errors;
pub mod generic;
pub mod macros;
pub mod stationpedia;
pub mod templates;
pub mod traits;

use traits::*;

use crate::vm::enums::{basic_enums::Class as SlotClass, script_enums::LogicSlotType};

pub type ObjectID = u32;
pub type BoxedObject = Rc<RefCell<dyn Object<ID = ObjectID>>>;

pub struct VMObject(BoxedObject);

impl Deref for VMObject {
    type Target = BoxedObject;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl VMObject {
    pub fn new<T>(val: T) -> Self
    where
        T: Object<ID = ObjectID> + 'static,
    {
        VMObject(Rc::new(RefCell::new(val)))
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MemoryAccess {
    Read,
    Write,
    ReadWrite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicField {
    pub field_type: MemoryAccess,
    pub value: f64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Slot {
    pub typ: SlotClass,
    pub enabled_logic: Vec<LogicSlotType>,
    pub occupant: Option<ObjectID>,
}
