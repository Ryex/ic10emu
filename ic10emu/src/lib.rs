use core::f64;
use std::collections::{HashMap, HashSet};

mod grammar;
mod compiler;


#[derive(Debug)]
pub enum FieldType {
    Read,
    Write,
    ReadWrite
}

#[derive(Debug)]
pub struct LogicField {
    field_type: FieldType,
    value: f64,

}

#[derive(Debug, Default)]
pub struct Device {
    pub id: u16,
    pub fields: HashMap<u8, LogicField>,
    pub IC: Option<IC>
}


#[derive(Debug)]
pub struct IC {
    pub id: u16,
    pub registers: [f64; 18], // r[0-15]
    pub ip: u8,
    pub stack: [f64; 512],
    pub aliases: HashMap<String, compiler::Operand>,
    pub pins: [Option<u16>; 6],
    pub code: String,
    pub program: compiler::Program,
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
}

impl Default for Network {
    fn default() -> Self {
        Network {
            devices: HashSet::new(),
            channels: [f64::NAN; 8],
        }
    }
}

impl IC {
    pub fn new(id: u16) -> Self {
        IC {
            id,
            ip: 0,
            registers: [0.0; 18],
            stack: [0.0; 512],
            pins: [None; 6],
            program: compiler::Program::new(),
            code: String::new(),
            aliases: HashMap::new(),
        }
    }
}

impl Device {
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
        };
        let ic = vm.new_ic();
        vm.add_ic(ic);
        vm
    }

    pub fn new_ic(&mut self) -> IC {
        IC::new(self.id_gen.next())
    }

    pub fn add_ic(&mut self, ic: IC) {
        let device = Device::IC(ic);
        self.ics.insert(device.id());
        self.devices.insert(device.id(), device);
    }

    pub fn remove_ic(&mut self, id: u16) {
        if  self.ics.remove(&id) {
            self.devices.remove(&id);
        }
    }

}
