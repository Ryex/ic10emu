use core::f64;
use serde_derive::{Deserialize, Serialize};
use std::{
    cell::{Cell, RefCell},
    ops::Deref,
    string::ToString,
};
use std::{
    collections::{BTreeMap, HashSet},
    fmt::Display,
    u32,
};

use itertools::Itertools;

use time::format_description;

use crate::{
    errors::{ICError, LineError},
    grammar,
    vm::{
        enums::{
            basic_enums::Class as SlotClass,
            script_enums::{LogicSlotType, LogicType},
        },
        instructions::{
            enums::InstructionOp,
            operands::{DeviceSpec, Operand, RegisterSpec},
            Instruction,
        },
        VM,
    },
};

use serde_with::serde_as;

pub mod instructions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ICState {
    Start,
    Running,
    Yield,
    Sleep(time::OffsetDateTime, f64),
    HasCaughtFire,
    Error(LineError),
}

impl Display for ICState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            ICState::Start => "Not Run".to_owned(),
            ICState::Running => "Running".to_owned(),
            ICState::Yield => "Ic has yielded, Resume on next tick".to_owned(),
            ICState::Sleep(then, sleep_for) => {
                let format = format_description::parse("[hour]:[minute]:[second]").unwrap();
                let resume = *then + time::Duration::new(*sleep_for as i64, 0);
                format!(
                    "Sleeping for {sleep_for} seconds, will resume at {}",
                    resume.format(&format).unwrap()
                )
            }
            ICState::Error(err) => format!("{err}"),
            ICState::HasCaughtFire => "IC has caught fire! this is not a joke!".to_owned(),
        };
        write!(f, "{out}")
    }
}

#[derive(Debug)]
pub struct IC {
    pub device: u32,
    pub id: u32,
    pub registers: RefCell<[f64; 18]>,
    /// Instruction Pointer
    pub ip: Cell<u32>,
    /// Instruction Count since last yield
    pub ic: Cell<u16>,
    pub stack: RefCell<[f64; 512]>,
    pub aliases: RefCell<BTreeMap<String, Operand>>,
    pub defines: RefCell<BTreeMap<String, f64>>,
    pub pins: RefCell<[Option<u32>; 6]>,
    pub code: RefCell<String>,
    pub program: RefCell<Program>,
    pub state: RefCell<ICState>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrozenIC {
    pub device: u32,
    pub id: u32,
    pub registers: [f64; 18],
    /// Instruction Pointer
    pub ip: u32,
    /// Instruction Count since last yield
    pub ic: u16,
    #[serde_as(as = "[_; 512]")]
    pub stack: [f64; 512],
    pub aliases: BTreeMap<String, Operand>,
    pub defines: BTreeMap<String, f64>,
    pub pins: [Option<u32>; 6],
    pub state: ICState,
    pub code: String,
}

impl<T> From<T> for FrozenIC
where
    T: Deref<Target = IC>,
{
    fn from(ic: T) -> Self {
        FrozenIC {
            device: ic.device,
            id: ic.id,
            registers: *ic.registers.borrow(),
            ip: ic.ip.get(),
            ic: ic.ic.get(),
            stack: *ic.stack.borrow(),
            aliases: ic.aliases.borrow().clone(),
            defines: ic.defines.borrow().clone(),
            pins: *ic.pins.borrow(),
            state: ic.state.borrow().clone(),
            code: ic.code.borrow().clone(),
        }
    }
}

impl From<FrozenIC> for IC {
    fn from(value: FrozenIC) -> Self {
        IC {
            device: value.device,
            id: value.id,
            registers: RefCell::new(value.registers),
            ip: Cell::new(value.ip),
            ic: Cell::new(value.ic),
            stack: RefCell::new(value.stack),
            aliases: RefCell::new(value.aliases),
            defines: RefCell::new(value.defines),
            pins: RefCell::new(value.pins),
            state: RefCell::new(value.state),
            code: RefCell::new(value.code.clone()),
            program: RefCell::new(Program::from_code_with_invalid(&value.code)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub errors: Vec<ICError>,
    pub labels: BTreeMap<String, u32>,
}

impl Default for Program {
    fn default() -> Self {
        Program::new()
    }
}

impl Program {
    pub fn new() -> Self {
        Program {
            instructions: Vec::new(),
            errors: Vec::new(),
            labels: BTreeMap::new(),
        }
    }

    pub fn try_from_code(code: &str) -> Result<Self, ICError> {
        let parse_tree = grammar::parse(code)?;
        let mut labels_set = HashSet::new();
        let mut labels = BTreeMap::new();
        let errors = Vec::new();
        let instructions = parse_tree
            .into_iter()
            .enumerate()
            .map(|(line_number, line)| match line.code {
                None => Ok(Instruction {
                    instruction: InstructionOp::Nop,
                    operands: vec![],
                }),
                Some(code) => match code {
                    grammar::Code::Label(label) => {
                        if labels_set.contains(&label.id.name) {
                            Err(ICError::DuplicateLabel(label.id.name))
                        } else {
                            labels_set.insert(label.id.name.clone());
                            labels.insert(label.id.name, line_number as u32);
                            Ok(Instruction {
                                instruction: InstructionOp::Nop,
                                operands: vec![],
                            })
                        }
                    }
                    grammar::Code::Instruction(instruction) => Ok(instruction),
                    grammar::Code::Invalid(err) => Err(err.into()),
                },
            })
            .try_collect()?;
        Ok(Program {
            instructions,
            errors,
            labels,
        })
    }

    pub fn from_code_with_invalid(code: &str) -> Self {
        let parse_tree = grammar::parse_with_invalid(code);
        let mut labels_set = HashSet::new();
        let mut labels = BTreeMap::new();
        let mut errors = Vec::new();
        let instructions = parse_tree
            .into_iter()
            .enumerate()
            .map(|(line_number, line)| match line.code {
                None => Instruction {
                    instruction: InstructionOp::Nop,
                    operands: vec![],
                },
                Some(code) => match code {
                    grammar::Code::Label(label) => {
                        if labels_set.contains(&label.id.name) {
                            errors.push(ICError::DuplicateLabel(label.id.name));
                        } else {
                            labels_set.insert(label.id.name.clone());
                            labels.insert(label.id.name, line_number as u32);
                        }
                        Instruction {
                            instruction: InstructionOp::Nop,
                            operands: vec![],
                        }
                    }
                    grammar::Code::Instruction(instruction) => instruction,
                    grammar::Code::Invalid(err) => {
                        errors.push(err.into());
                        Instruction {
                            instruction: InstructionOp::Nop,
                            operands: vec![],
                        }
                    }
                },
            })
            .collect_vec();
        Program {
            instructions,
            errors,
            labels,
        }
    }

    pub fn get_line(&self, line: usize) -> Result<&Instruction, ICError> {
        self.instructions
            .get(line)
            .ok_or(ICError::InstructionPointerOutOfRange(line))
    }
}

impl IC {
    pub fn new(id: u32, device: u32) -> Self {
        IC {
            device,
            id,
            ip: Cell::new(0),
            ic: Cell::new(0),
            registers: RefCell::new([0.0; 18]),
            stack: RefCell::new([0.0; 512]),
            pins: RefCell::new([None; 6]),
            program: RefCell::new(Program::new()),
            code: RefCell::new(String::new()),
            aliases: RefCell::new(BTreeMap::new()),
            defines: RefCell::new(BTreeMap::new()),
            state: RefCell::new(ICState::Start),
        }
    }

    pub fn reset(&self) {
        self.ip.replace(0);
        self.ic.replace(0);
        self.registers.replace([0.0; 18]);
        self.stack.replace([0.0; 512]);
        self.aliases.replace(BTreeMap::new());
        self.defines.replace(BTreeMap::new());
        self.state.replace(ICState::Start);
    }

    /// Set program code if it's valid
    pub fn set_code(&self, code: &str) -> Result<(), ICError> {
        let prog = Program::try_from_code(code)?;
        self.program.replace(prog);
        self.code.replace(code.to_string());
        Ok(())
    }

    /// Set program code and translate invalid lines to Nop, collecting errors
    pub fn set_code_invalid(&mut self, code: &str) {
        let prog = Program::from_code_with_invalid(code);
        self.program.replace(prog);
        self.code.replace(code.to_string());
    }

    pub fn get_real_target(&self, indirection: u32, target: u32) -> Result<f64, ICError> {
        let mut i = indirection;
        let mut t = target as f64;
        while i > 0 {
            if let Some(new_t) = self.registers.borrow().get(t as usize) {
                t = *new_t;
            } else {
                return Err(ICError::RegisterIndexOutOfRange(t));
            }
            i -= 1;
        }
        Ok(t)
    }

    pub fn get_register(&self, indirection: u32, target: u32) -> Result<f64, ICError> {
        let t = self.get_real_target(indirection, target)?;
        self.registers
            .borrow()
            .get(t as usize)
            .ok_or(ICError::RegisterIndexOutOfRange(t))
            .copied()
    }

    /// sets a register thorough, recursing through provided indirection, returns value previously
    pub fn set_register(&self, indirection: u32, target: u32, val: f64) -> Result<f64, ICError> {
        let t = self.get_real_target(indirection, target)?;
        let mut registers = self.registers.borrow_mut();
        let old_val = registers
            .get(t as usize)
            .ok_or(ICError::RegisterIndexOutOfRange(t))
            .copied()?;
        registers[t as usize] = val;
        Ok(old_val)
    }

    /// save ip to 'ra' or register 18
    fn al(&self) {
        self.registers.borrow_mut()[17] = self.ip() as f64 + 1.0;
    }

    pub fn push(&self, val: f64) -> Result<f64, ICError> {
        let mut registers = self.registers.borrow_mut();
        let mut stack = self.stack.borrow_mut();
        let sp = (registers[16].round()) as i32;
        if sp < 0 {
            Err(ICError::StackUnderflow)
        } else if sp >= 512 {
            Err(ICError::StackOverflow)
        } else {
            let last = stack[sp as usize];
            stack[sp as usize] = val;
            registers[16] += 1.0;
            Ok(last)
        }
    }

    pub fn pop(&self) -> Result<f64, ICError> {
        let mut registers = self.registers.borrow_mut();
        registers[16] -= 1.0;
        let sp = (registers[16].round()) as i32;
        if sp < 0 {
            Err(ICError::StackUnderflow)
        } else if sp >= 512 {
            Err(ICError::StackOverflow)
        } else {
            let last = self.stack.borrow()[sp as usize];
            Ok(last)
        }
    }

    pub fn poke(&self, address: f64, val: f64) -> Result<f64, ICError> {
        let sp = address.round() as i32;
        if !(0..512).contains(&sp) {
            Err(ICError::StackIndexOutOfRange(address))
        } else {
            let mut stack = self.stack.borrow_mut();
            let last = stack[sp as usize];
            stack[sp as usize] = val;
            Ok(last)
        }
    }

    pub fn peek(&self) -> Result<f64, ICError> {
        let sp = (self.registers.borrow()[16] - 1.0).round() as i32;
        if sp < 0 {
            Err(ICError::StackUnderflow)
        } else if sp >= 512 {
            Err(ICError::StackOverflow)
        } else {
            let last = self.stack.borrow()[sp as usize];
            Ok(last)
        }
    }

    pub fn peek_addr(&self, addr: f64) -> Result<f64, ICError> {
        let sp = (addr) as i32;
        if sp < 0 {
            Err(ICError::StackUnderflow)
        } else if sp >= 512 {
            Err(ICError::StackOverflow)
        } else {
            let last = self.stack.borrow()[sp as usize];
            Ok(last)
        }
    }

    pub fn propagate_line_number(&self, vm: &VM) {
        if let Some(device) = vm.devices.get(&self.device) {
            let mut device_ref = device.borrow_mut();
            let _ = device_ref.set_field(LogicType::LineNumber, self.ip.get() as f64, vm, true);
            if let Some(slot) = device_ref
                .slots
                .iter_mut()
                .find(|slot| slot.typ == SlotClass::ProgrammableChip)
            {
                let _ = slot.set_field(LogicSlotType::LineNumber, self.ip.get() as f64, true);
            }
        }
    }

    /// processes one line of the contained program
    pub fn step(&self, vm: &VM, advance_ip_on_err: bool) -> Result<bool, LineError> {
        // TODO: handle sleep
        self.state.replace(ICState::Running);
        let line = self.ip();
        let result = self.internal_step(vm, advance_ip_on_err);
        if let Err(error) = result {
            let error = LineError { error, line };
            self.state.replace(ICState::Error(error.clone()));
            Err(error)
        } else {
            Ok(true)
        }
    }

    pub fn ip(&self) -> u32 {
        self.ip.get()
    }

    pub fn set_ip(&self, val: u32) {
        self.ip.replace(val);
    }

    fn internal_step(&self, vm: &VM, advance_ip_on_err: bool) -> Result<(), ICError> {
        use ICError::*;

        let mut next_ip = self.ip() + 1;
        // XXX: This closure should be replaced with a try block
        // https://github.com/rust-lang/rust/issues/31436
        let mut process_op = |this: &Self| -> Result<(), ICError> {
            use InstructionOp::*;

            // force the program borrow to drop
            let line = {
                let prog = this.program.borrow();
                prog.get_line(this.ip())?.clone()
            };
            let operands = &line.operands;
            let inst = line.instruction;
            match inst {
                Nop => Ok(()),
                Hcf => Ok(()),   // TODO
                Clr => Ok(()),   // TODO
                Clrd => Ok(()),  // TODO
                Label => Ok(()), // NOP

                S => match &operands[..] {
                    [dev, lt, val] => {
                        let (Some(device_id), connection) = dev.as_device(this, inst, 1)? else {
                            return Err(DeviceNotSet);
                        };
                        let lt = lt.as_logic_type(this, inst, 2)?;
                        if CHANNEL_LOGIC_TYPES.contains(&lt) {
                            let channel = lt.as_channel().unwrap();
                            let Some(connection) = connection else {
                                return Err(MissingConnectionSpecifier);
                            };
                            let network_id = vm
                                .get_device_same_network(this.device, device_id)
                                .map(|device| device.borrow().get_network_id(connection))
                                .unwrap_or(Err(UnknownDeviceID(device_id as f64)))?;
                            let val = val.as_value(this, inst, 3)?;
                            vm.set_network_channel(network_id, channel, val)?;
                            return Ok(());
                        }
                        let device = vm.get_device_same_network(this.device, device_id);
                        match device {
                            Some(device) => {
                                let val = val.as_value(this, inst, 1)?;
                                device.borrow_mut().set_field(lt, val, vm, false)?;
                                vm.set_modified(device_id);
                                Ok(())
                            }
                            None => Err(UnknownDeviceID(device_id as f64)),
                        }
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Sd => match &operands[..] {
                    [dev, lt, val] => {
                        let device_id = dev.as_value(this, inst, 1)?;
                        if device_id >= u16::MAX as f64 || device_id < u16::MIN as f64 {
                            return Err(DeviceIndexOutOfRange(device_id));
                        }
                        let device = vm.get_device_same_network(this.device, device_id as u32);
                        match device {
                            Some(device) => {
                                let lt = lt.as_logic_type(this, inst, 2)?;
                                let val = val.as_value(this, inst, 3)?;
                                device.borrow_mut().set_field(lt, val, vm, false)?;
                                vm.set_modified(device_id as u32);
                                Ok(())
                            }
                            None => Err(UnknownDeviceID(device_id)),
                        }
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Ss => match &operands[..] {
                    [dev, index, slt, val] => {
                        let (Some(device_id), _connection) = dev.as_device(this, inst, 1)? else {
                            return Err(DeviceNotSet);
                        };
                        let device = vm.get_device_same_network(this.device, device_id);
                        match device {
                            Some(device) => {
                                let index = index.as_value(this, inst, 2)?;
                                let slt = slt.as_slot_logic_type(this, inst, 3)?;
                                let val = val.as_value(this, inst, 4)?;
                                device
                                    .borrow_mut()
                                    .set_slot_field(index, slt, val, vm, false)?;
                                vm.set_modified(device_id);
                                Ok(())
                            }
                            None => Err(UnknownDeviceID(device_id as f64)),
                        }
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 4)),
                },
                Sb => match &operands[..] {
                    [prefab, lt, val] => {
                        let prefab = prefab.as_value(this, inst, 1)?;
                        let lt = lt.as_logic_type(this, inst, 2)?;
                        let val = val.as_value(this, inst, 3)?;
                        vm.set_batch_device_field(this.device, prefab, lt, val, false)?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Sbs => match &operands[..] {
                    [prefab, index, slt, val] => {
                        let prefab = prefab.as_value(this, inst, 1)?;
                        let index = index.as_value(this, inst, 2)?;
                        let slt = slt.as_slot_logic_type(this, inst, 3)?;
                        let val = val.as_value(this, inst, 4)?;
                        vm.set_batch_device_slot_field(
                            this.device,
                            prefab,
                            index,
                            slt,
                            val,
                            false,
                        )?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 4)),
                },
                Sbn => match &operands[..] {
                    [prefab, name, lt, val] => {
                        let prefab = prefab.as_value(this, inst, 1)?;
                        let name = name.as_value(this, inst, 2)?;
                        let lt = lt.as_logic_type(this, inst, 3)?;
                        let val = val.as_value(this, inst, 4)?;
                        vm.set_batch_name_device_field(this.device, prefab, name, lt, val, false)?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 4)),
                },

                L => match &operands[..] {
                    [reg, dev, lt] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let (Some(device_id), connection) = dev.as_device(this, inst, 2)? else {
                            return Err(DeviceNotSet);
                        };
                        let lt = lt.as_logic_type(this, inst, 3)?;
                        if CHANNEL_LOGIC_TYPES.contains(&lt) {
                            let channel = lt.as_channel().unwrap();
                            let Some(connection) = connection else {
                                return Err(MissingConnectionSpecifier);
                            };
                            let network_id = vm
                                .get_device_same_network(this.device, device_id)
                                .map(|device| device.borrow().get_network_id(connection))
                                .unwrap_or(Err(UnknownDeviceID(device_id as f64)))?;
                            let val = vm.get_network_channel(network_id, channel)?;
                            this.set_register(indirection, target, val)?;
                            return Ok(());
                        }
                        if lt == LogicType::LineNumber && this.device == device_id {
                            // HACK: we can't use device.get_field as that will try to reborrow our
                            // ic which will panic
                            this.set_register(indirection, target, this.ip() as f64)?;
                            Ok(())
                        } else {
                            let device = vm.get_device_same_network(this.device, device_id);
                            match device {
                                Some(device) => {
                                    let val = device.borrow().get_field(lt, vm)?;
                                    this.set_register(indirection, target, val)?;
                                    Ok(())
                                }
                                None => Err(UnknownDeviceID(device_id as f64)),
                            }
                        }
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Ld => match &operands[..] {
                    [reg, dev, lt] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let device_id = dev.as_value(this, inst, 2)?;
                        if device_id >= u16::MAX as f64 || device_id < u16::MIN as f64 {
                            return Err(DeviceIndexOutOfRange(device_id));
                        }
                        let lt = lt.as_logic_type(this, inst, 3)?;
                        if lt == LogicType::LineNumber && this.device == device_id as u32 {
                            // HACK: we can't use device.get_field as that will try to reborrow our
                            // ic which will panic
                            this.set_register(indirection, target, this.ip() as f64)?;
                            Ok(())
                        } else {
                            let device = vm.get_device_same_network(this.device, device_id as u32);
                            match device {
                                Some(device) => {
                                    let val = device.borrow().get_field(lt, vm)?;
                                    this.set_register(indirection, target, val)?;
                                    Ok(())
                                }
                                None => Err(UnknownDeviceID(device_id)),
                            }
                        }
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Ls => match &operands[..] {
                    [reg, dev, index, slt] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let (Some(device_id), _connection) = dev.as_device(this, inst, 2)? else {
                            return Err(DeviceNotSet);
                        };
                        let slt = slt.as_slot_logic_type(this, inst, 4)?;
                        if slt == LogicSlotType::LineNumber && this.device == device_id {
                            // HACK: we can't use device.get_slot_field as that will try to reborrow our
                            // ic which will panic
                            this.set_register(indirection, target, this.ip() as f64)?;
                            Ok(())
                        } else {
                            let device = vm.get_device_same_network(this.device, device_id);
                            match device {
                                Some(device) => {
                                    let index = index.as_value(this, inst, 3)?;
                                    let val = device.borrow().get_slot_field(index, slt, vm)?;
                                    this.set_register(indirection, target, val)?;
                                    Ok(())
                                }
                                None => Err(UnknownDeviceID(device_id as f64)),
                            }
                        }
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 4)),
                },
                Lr => match &operands[..] {
                    [reg, dev, rm, name] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let (Some(device_id), _connection) = dev.as_device(this, inst, 2)? else {
                            return Err(DeviceNotSet);
                        };
                        let device = vm.get_device_same_network(this.device, device_id);
                        match device {
                            Some(device) => {
                                let rm = rm.as_reagent_mode(this, inst, 3)?;
                                let name = name.as_value(this, inst, 4)?;
                                let val = device.borrow().get_reagent(&rm, name);
                                this.set_register(indirection, target, val)?;
                                Ok(())
                            }
                            None => Err(UnknownDeviceID(device_id as f64)),
                        }
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 4)),
                },
                Lb => match &operands[..] {
                    [reg, prefab, lt, bm] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let prefab = prefab.as_value(this, inst, 2)?;
                        let lt = lt.as_logic_type(this, inst, 3)?;
                        let bm = bm.as_batch_mode(this, inst, 4)?;
                        let val = vm.get_batch_device_field(this.device, prefab, lt, bm)?;
                        this.set_register(indirection, target, val)?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 4)),
                },
                Lbn => match &operands[..] {
                    [reg, prefab, name, lt, bm] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let prefab = prefab.as_value(this, inst, 2)?;
                        let name = name.as_value(this, inst, 3)?;
                        let lt = lt.as_logic_type(this, inst, 4)?;
                        let bm = bm.as_batch_mode(this, inst, 5)?;
                        let val =
                            vm.get_batch_name_device_field(this.device, prefab, name, lt, bm)?;
                        this.set_register(indirection, target, val)?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 5)),
                },
                Lbns => match &operands[..] {
                    [reg, prefab, name, index, slt, bm] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let prefab = prefab.as_value(this, inst, 2)?;
                        let name = name.as_value(this, inst, 3)?;
                        let index = index.as_value(this, inst, 4)?;
                        let slt = slt.as_slot_logic_type(this, inst, 5)?;
                        let bm = bm.as_batch_mode(this, inst, 6)?;
                        let val = vm.get_batch_name_device_slot_field(
                            this.device,
                            prefab,
                            name,
                            index,
                            slt,
                            bm,
                        )?;
                        this.set_register(indirection, target, val)?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 6)),
                },
                Lbs => match &operands[..] {
                    [reg, prefab, index, slt, bm] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let prefab = prefab.as_value(this, inst, 2)?;
                        let index = index.as_value(this, inst, 3)?;
                        let slt = slt.as_slot_logic_type(this, inst, 4)?;
                        let bm = bm.as_batch_mode(this, inst, 5)?;
                        let val =
                            vm.get_batch_device_slot_field(this.device, prefab, index, slt, bm)?;
                        this.set_register(indirection, target, val)?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 5)),
                },
            }
        };
        let result = process_op(self);
        if result.is_ok() || advance_ip_on_err {
            self.ic.set(self.ic.get() + 1);
            self.set_ip(next_ip);
            self.propagate_line_number(vm);
        }
        result
    }
}

#[allow(dead_code)]
const CHANNEL_LOGIC_TYPES: [LogicType; 8] = [
    LogicType::Channel0,
    LogicType::Channel1,
    LogicType::Channel2,
    LogicType::Channel3,
    LogicType::Channel4,
    LogicType::Channel5,
    LogicType::Channel6,
    LogicType::Channel7,
];

trait LogicTypeExt {
    fn as_channel(&self) -> Option<usize>;
}
impl LogicTypeExt for LogicType {
    fn as_channel(&self) -> Option<usize> {
        match self {
            LogicType::Channel0 => Some(0),
            LogicType::Channel1 => Some(1),
            LogicType::Channel2 => Some(2),
            LogicType::Channel3 => Some(3),
            LogicType::Channel4 => Some(4),
            LogicType::Channel5 => Some(5),
            LogicType::Channel6 => Some(6),
            LogicType::Channel7 => Some(7),
            _ => None,
        }
    }
}

pub fn f64_to_i64(f: f64, signed: bool) -> i64 {
    let mut num: i64 = (f % (1i64 << 53) as f64) as i64;
    if !signed {
        num &= (1i64 << 54) - 1;
    }
    num
}
pub fn i64_to_f64(i: i64) -> f64 {
    const MASK: i64 = 1 << 53;
    let flag: bool = (i & MASK) != 0;
    let mut i = i & (MASK - 1);
    if flag {
        i |= -MASK;
    }
    i as f64
}

#[cfg(test)]
mod tests {
    use crate::errors::VMError;

    use super::*;

    use color_eyre::eyre::Ok;

    static INIT: std::sync::Once = std::sync::Once::new();

    fn setup() {
        INIT.call_once(|| {
            let _ = color_eyre::install();
        })
    }

    #[test]
    fn batch_modes() -> color_eyre::Result<()> {
        setup();
        let mut vm = VM::new();
        let ic = vm.add_ic(None).unwrap();
        let ic_id = {
            let device = vm.devices.get(&ic).unwrap();
            let device_ref = device.borrow();
            device_ref.ic.unwrap()
        };
        let ic_chip = vm.circuit_holders.get(&ic_id).unwrap().borrow();
        vm.set_code(
            ic,
            r#"lb r0 HASH("ItemActiveVent") On Sum
            lb r1 HASH("ItemActiveVent") On Maximum
            lb r2 HASH("ItemActiveVent") On Minimum"#,
        )?;
        vm.step_ic(ic, false)?;
        let r0 = ic_chip.get_register(0, 0).unwrap();
        assert_eq!(r0, 0.0);
        vm.step_ic(ic, false)?;
        let r1 = ic_chip.get_register(0, 1).unwrap();
        assert_eq!(r1, f64::NEG_INFINITY);
        vm.step_ic(ic, false)?;
        let r2 = ic_chip.get_register(0, 2).unwrap();
        assert_eq!(r2, f64::INFINITY);
        Ok(())
    }

    #[test]
    fn stack() -> color_eyre::Result<()> {
        setup();
        let mut vm = VM::new();
        let ic = vm.add_ic(None).unwrap();
        let ic_id = {
            let device = vm.devices.get(&ic).unwrap();
            let device_ref = device.borrow();
            device_ref.ic.unwrap()
        };
        let ic_chip = vm.circuit_holders.get(&ic_id).unwrap().borrow();
        vm.set_code(
            ic,
            r#"push 100
            push 10
            pop r0
            push 1000
            peek r1
            poke 1 20
            pop r2
            "#,
        )?;
        vm.step_ic(ic, false)?;
        let stack0 = ic_chip.peek_addr(0.0)?;
        assert_eq!(stack0, 100.0);
        vm.step_ic(ic, false)?;
        let stack1 = ic_chip.peek_addr(1.0)?;
        assert_eq!(stack1, 10.0);
        vm.step_ic(ic, false)?;
        let r0 = ic_chip.get_register(0, 0).unwrap();
        assert_eq!(r0, 10.0);
        vm.step_ic(ic, false)?;
        let stack1 = ic_chip.peek_addr(1.0)?;
        assert_eq!(stack1, 1000.0);
        vm.step_ic(ic, false)?;
        let r1 = ic_chip.get_register(0, 1).unwrap();
        assert_eq!(r1, 1000.0);
        vm.step_ic(ic, false)?;
        let stack1 = ic_chip.peek_addr(1.0)?;
        assert_eq!(stack1, 20.0);
        vm.step_ic(ic, false)?;
        let r2 = ic_chip.get_register(0, 2).unwrap();
        assert_eq!(r2, 20.0);
        Ok(())
    }
}
