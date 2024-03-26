use core::f64;
use std::collections::{HashMap, HashSet};


mod tokens;
mod grammar;
mod interpreter;
mod rand_mscorlib;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum VMError {
    #[error("Device with id '{0}' does not exist")]
    UnknownId(u16),
    #[error("Device with id '{0}' does not have a IC Slot")]
    NoIC(u16),
    #[error("IC encoutered an error: {0}")]
    ICError(#[from] interpreter::ICError)
}


#[derive(Debug)]
pub enum FieldType {
    Read,
    Write,
    ReadWrite
}

#[derive(Debug)]
pub struct LogicField {
    pub field_type: FieldType,
    pub value: f64,

}

#[derive(Debug, Default)]
pub struct Device {
    pub id: u16,
    pub fields: HashMap<u32, LogicField>,
    pub ic: Option<interpreter::IC>
}

#[derive(Debug)]
pub struct Network {
    pub devices: HashSet<u16>,
    pub channels: [f64; 8],
}

#[derive(Debug)]
struct IdSequenceGenerator {
    next: u16,
}

impl Default for IdSequenceGenerator {
    fn default() -> Self {
        IdSequenceGenerator { next: 1 }
    }
}

impl IdSequenceGenerator {
    pub fn next(&mut self) -> u16 {
        let val = self.next;
        self.next += 1;
        val
    }
}

#[derive(Debug)]
pub struct VM {
    pub ics: HashSet<u16>,
    pub devices: HashMap<u16, Device>,
    pub networks: Vec<Network>,
    id_gen: IdSequenceGenerator,
    random: crate::rand_mscorlib::Random,
}

impl Default for Network {
    fn default() -> Self {
        Network {
            devices: HashSet::new(),
            channels: [f64::NAN; 8],
        }
    }
}

impl Device {
    pub fn new(id: u16) -> Self {
        Device { id, fields: HashMap::new(), ic: None }
    }

    pub fn with_ic(id: u16) -> Self {
        let mut device = Device::new(id);
        device.ic = Some(interpreter::IC::new(id));
        device
    }
}


impl VM {
    pub fn new() -> Self {
        let id_gen = IdSequenceGenerator::default();
        let default_network = Network::default();
        let mut vm = VM {
            ics: HashSet::new(),
            devices: HashMap::new(),
            networks: vec![default_network],
            id_gen,
            random: crate::rand_mscorlib::Random::new(),
        };
        vm.add_ic();
        vm
    }

    pub fn add_ic(&mut self) {
        let device = Device::with_ic(self.id_gen.next());
        self.ics.insert(device.id);
        self.devices.insert(device.id, device);
    }

    pub fn remove_ic(&mut self, id: u16) {
        if  self.ics.remove(&id) {
            self.devices.remove(&id);
        }
    }

    pub fn set_code(&mut self, id: u16, code: &str) -> Result<bool, VMError> {
        let device = self.devices.get_mut(&id).ok_or(VMError::UnknownId(id))?;
        let ic = device.ic.as_mut().ok_or(VMError::NoIC(id))?;
        let new_prog = interpreter::Program::try_from_code(code)?;
        ic.program = new_prog;
        ic.code = code.to_string();
        Ok(true)
    }

}
