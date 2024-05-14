use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
    str::FromStr,
};

use macro_rules_attribute::derive;
use serde_derive::{Deserialize, Serialize};

pub mod errors;
pub mod generic;
pub mod macros;
pub mod stationpedia;
pub mod templates;
pub mod traits;

use traits::Object;

use crate::vm::{
    enums::{basic_enums::Class as SlotClass, script_enums::LogicSlotType},
    VM,
};

use super::enums::prefabs::StationpediaPrefab;

pub type ObjectID = u32;
pub type BoxedObject = Rc<RefCell<dyn Object>>;

#[derive(Debug, Clone)]
pub struct VMObject {
    obj: BoxedObject,
    vm: Rc<VM>,
}

impl Deref for VMObject {
    type Target = BoxedObject;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}

impl DerefMut for VMObject {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.obj
    }
}

impl VMObject {
    pub fn new<T>(val: T, vm: Rc<VM>) -> Self
    where
        T: Object + 'static,
    {
        VMObject {
            obj: Rc::new(RefCell::new(val)),
            vm,
        }
    }

    pub fn set_vm(&mut self, vm: Rc<VM>) {
        self.vm = vm;
    }

    pub fn get_vm(&self) -> &Rc<VM> {
        &self.vm
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
    pub fn from_prefab_name(name: &str) -> Self {
        Name {
            value: name.to_string(),
            hash: StationpediaPrefab::from_str(name)
                .map(|prefab| prefab as i32)
                .unwrap_or_else(|_| const_crc32::crc32(name.as_bytes()) as i32),
        }
    }
    pub fn from_prefab_hash(hash: i32) -> Option<Self> {
        if let Some(prefab) = StationpediaPrefab::from_repr(hash) {
            Some(Name {
                value: prefab.to_string(),
                hash,
            })
        } else {
            None
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
    pub parent: ObjectID,
    pub index: usize,
    pub name: String,
    pub typ: SlotClass,
    pub readable_logic: Vec<LogicSlotType>,
    pub writeable_logic: Vec<LogicSlotType>,
    pub occupant: Option<ObjectID>,
}
