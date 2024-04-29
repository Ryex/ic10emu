use core::f64;
use serde::{Deserialize, Serialize};
use std::{cell::{Cell, RefCell}, ops::Deref, string::ToString};
use std::{
    collections::{BTreeMap, HashSet},
    error::Error,
    fmt::Display,
    u32,
};

use itertools::Itertools;

use time::format_description;

use crate::{
    device::SlotType, grammar::{self, LogicType, ParseError, SlotLogicType}, vm::VM
};

use serde_with::serde_as;

use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineError {
    error: ICError,
    line: u32,
}

impl Display for LineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error on line {}: {}", self.line, self.error)
    }
}

impl Error for LineError {}

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum ICError {
    #[error("error compiling code: {0}")]
    ParseError(#[from] ParseError),
    #[error("duplicate label {0}")]
    DuplicateLabel(String),
    #[error("instruction pointer out of range: '{0}'")]
    InstructionPointerOutOfRange(u32),
    #[error("register pointer out of range: '{0}'")]
    RegisterIndexOutOfRange(f64),
    #[error("device pointer out of range: '{0}'")]
    DeviceIndexOutOfRange(f64),
    #[error("stack index out of range: '{0}'")]
    StackIndexOutOfRange(f64),
    #[error("slot index out of range: '{0}'")]
    SlotIndexOutOfRange(f64),
    #[error("pin index {0} out of range 0-6")]
    PinIndexOutOfRange(usize),
    #[error("connection index {0} out of range {1}")]
    ConnectionIndexOutOfRange(usize, usize),
    #[error("unknown device ID '{0}'")]
    UnknownDeviceID(f64),
    #[error("too few operands!: provide: '{provided}', desired: '{desired}'")]
    TooFewOperands { provided: u32, desired: u32 },
    #[error("too many operands!: provide: '{provided}', desired: '{desired}'")]
    TooManyOperands { provided: u32, desired: u32 },
    #[error("incorrect operand type for instruction `{inst}` operand {index}, not a {desired} ")]
    IncorrectOperandType {
        inst: grammar::InstructionOp,
        index: u32,
        desired: String,
    },
    #[error("unknown identifier {0}")]
    UnknownIdentifier(String),
    #[error("device Not Set")]
    DeviceNotSet,
    #[error("shift Underflow i64(signed long)")]
    ShiftUnderflowI64,
    #[error("shift Overflow i64(signed long)")]
    ShiftOverflowI64,
    #[error("shift underflow i32(signed int)")]
    ShiftUnderflowI32,
    #[error("shift overflow i32(signed int)")]
    ShiftOverflowI32,
    #[error("stack underflow")]
    StackUnderflow,
    #[error("stack overflow")]
    StackOverflow,
    #[error("duplicate define '{0}'")]
    DuplicateDefine(String),
    #[error("read only field '{0}'")]
    ReadOnlyField(String),
    #[error("write only field '{0}'")]
    WriteOnlyField(String),
    #[error("device has no field '{0}'")]
    DeviceHasNoField(String),
    #[error("device has not ic")]
    DeviceHasNoIC,
    #[error("unknown device '{0}'")]
    UnknownDeviceId(f64),
    #[error("unknown logic type '{0}'")]
    UnknownLogicType(f64),
    #[error("unknown slot logic type '{0}'")]
    UnknownSlotLogicType(f64),
    #[error("unknown batch mode '{0}'")]
    UnknownBatchMode(f64),
    #[error("unknown reagent mode '{0}'")]
    UnknownReagentMode(f64),
    #[error("type value not known")]
    TypeValueNotKnown,
    #[error("empty device list")]
    EmptyDeviceList,
    #[error("connection specifier missing")]
    MissingConnectionSpecifier,
    #[error("no data network on connection '{0}'")]
    NotACableConnection(usize),
    #[error("network not connected on connection '{0}'")]
    NetworkNotConnected(usize),
    #[error("bad network Id '{0}'")]
    BadNetworkId(u32),
    #[error("channel index out of range '{0}'")]
    ChannelIndexOutOfRange(usize),
    #[error("slot has no occupant")]
    SlotNotOccupied,
    #[error("generated Enum {0} has no value attached. Report this error.")]
    NoGeneratedValue(String),
    #[error("generated Enum {0}'s value does not parse as {1} . Report this error.")]
    BadGeneratedValueParse(String, String),
}

impl ICError {
    pub const fn too_few_operands(provided: usize, desired: u32) -> Self {
        ICError::TooFewOperands {
            provided: provided as u32,
            desired,
        }
    }

    pub const fn too_many_operands(provided: usize, desired: u32) -> Self {
        ICError::TooManyOperands {
            provided: provided as u32,
            desired,
        }
    }

    pub const fn mismatch_operands(provided: usize, desired: u32) -> Self {
        if provided < desired as usize {
            ICError::too_few_operands(provided, desired)
        } else {
            ICError::too_many_operands(provided, desired)
        }
    }
}

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
    pub aliases: RefCell<BTreeMap<String, grammar::Operand>>,
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
    pub aliases: BTreeMap<String, grammar::Operand>,
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
    pub instructions: Vec<grammar::Instruction>,
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
                None => Ok(grammar::Instruction {
                    instruction: grammar::InstructionOp::Nop,
                    operands: vec![],
                }),
                Some(code) => match code {
                    grammar::Code::Label(label) => {
                        if labels_set.contains(&label.id.name) {
                            Err(ICError::DuplicateLabel(label.id.name))
                        } else {
                            labels_set.insert(label.id.name.clone());
                            labels.insert(label.id.name, line_number as u32);
                            Ok(grammar::Instruction {
                                instruction: grammar::InstructionOp::Nop,
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
        let parse_tree = grammar::parse_with_invlaid(code);
        let mut labels_set = HashSet::new();
        let mut labels = BTreeMap::new();
        let mut errors = Vec::new();
        let instructions = parse_tree
            .into_iter()
            .enumerate()
            .map(|(line_number, line)| match line.code {
                None => grammar::Instruction {
                    instruction: grammar::InstructionOp::Nop,
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
                        grammar::Instruction {
                            instruction: grammar::InstructionOp::Nop,
                            operands: vec![],
                        }
                    }
                    grammar::Code::Instruction(instruction) => instruction,
                    grammar::Code::Invalid(err) => {
                        errors.push(err.into());
                        grammar::Instruction {
                            instruction: grammar::InstructionOp::Nop,
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

    pub fn get_line(&self, line: u32) -> Result<&grammar::Instruction, ICError> {
        self.instructions
            .get(line as usize)
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
    pub fn set_register(
        &self,
        indirection: u32,
        target: u32,
        val: f64,
    ) -> Result<f64, ICError> {
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

    pub fn propgate_line_number(&self, vm: &VM) {
        if let Some(device) = vm.devices.get(&self.device) {
            let mut device_ref = device.borrow_mut();
            let _ = device_ref.set_field(LogicType::LineNumber, self.ip.get() as f64, vm, true);
            if let Some(slot) = device_ref.slots.iter_mut().find(|slot| slot.typ == SlotType::ProgrammableChip) {
                let _ = slot.set_field(SlotLogicType::LineNumber, self.ip.get() as f64, true);
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
        use grammar::*;
        use ICError::*;

        let mut next_ip = self.ip() + 1;
        // XXX: This closure should be replaced with a try block
        // https://github.com/rust-lang/rust/issues/31436
        let mut process_op = |this: &Self| -> Result<(), ICError> {
            use grammar::InstructionOp::*;

            // force the program borrow to drop
            let line = {
                let prog = this.program.borrow();
                prog.get_line(this.ip())?.clone()
            };
            let operands = &line.operands;
            let inst = line.instruction;
            match inst {
                Nop => Ok(()),
                Hcf => Ok(()), // TODO
                Sleep => match &operands[..] {
                    [a] => {
                        let a = a.as_value(this, inst, 1)?;
                        let now = time::OffsetDateTime::now_local()
                            .unwrap_or_else(|_| time::OffsetDateTime::now_utc());
                        this.state.replace(ICState::Sleep(now, a));
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 1)),
                }, // TODO
                Yield => match &operands[..] {
                    [] => {
                        this.state.replace(ICState::Yield);
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 0)),
                },
                Define => match &operands[..] {
                    [name, number] => {
                        let &Operand::Identifier(ident) = &name else {
                            return Err(IncorrectOperandType {
                                inst: line.instruction,
                                index: 1,
                                desired: "Name".to_owned(),
                            });
                        };
                        let &Operand::Number(num) = &number else {
                            return Err(IncorrectOperandType {
                                inst: line.instruction,
                                index: 2,
                                desired: "Number".to_owned(),
                            });
                        };
                        let mut defines = this.defines.borrow_mut();
                        if defines.contains_key(&ident.name) {
                            Err(DuplicateDefine(ident.name.clone()))
                        } else {
                            defines.insert(ident.name.clone(), num.value());
                            Ok(())
                        }
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Alias => match &operands[..] {
                    [name, device_reg] => {
                        let &Operand::Identifier(ident) = &name else {
                            return Err(IncorrectOperandType {
                                inst: line.instruction,
                                index: 1,
                                desired: "Name".to_owned(),
                            });
                        };
                        let alias = match &device_reg {
                            Operand::RegisterSpec(RegisterSpec {
                                indirection,
                                target,
                            }) => Operand::RegisterSpec(RegisterSpec {
                                indirection: *indirection,
                                target: *target,
                            }),
                            Operand::DeviceSpec(DeviceSpec { device, connection }) => {
                                Operand::DeviceSpec(DeviceSpec {
                                    device: *device,
                                    connection: *connection,
                                })
                            }
                            _ => {
                                return Err(IncorrectOperandType {
                                    inst: line.instruction,
                                    index: 2,
                                    desired: "Device Or Register".to_owned(),
                                })
                            }
                        };
                        this.aliases.borrow_mut().insert(ident.name.clone(), alias);
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Move => match &operands[..] {
                    [reg, val] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, line.instruction, 1)?;

                        let val = val.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, val)?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },

                Beq => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a == b { c as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Beqal => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a == b { c as u32 } else { next_ip };
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Breq => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a == b {
                            (this.ip() as f64 + c) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Beqz => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a == 0.0 { b as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Beqzal => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a == 0.0 { b as u32 } else { next_ip };
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Breqz => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a == 0.0 {
                            (this.ip() as f64 + b) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Bne => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a != b { c as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Bneal => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a != b { c as u32 } else { next_ip };
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Brne => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a != b {
                            (this.ip() as f64 + c) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Bnez => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a != 0.0 { b as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Bnezal => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a != 0.0 { b as u32 } else { next_ip };
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Brnez => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a != 0.0 {
                            (this.ip() as f64 + b) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Blt => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a < b { c as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Bltal => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a < b { c as u32 } else { next_ip };
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Brlt => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a < b {
                            (this.ip() as f64 + c) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Ble => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a <= b { c as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Bleal => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a <= b { c as u32 } else { next_ip };
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Brle => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a <= b {
                            (this.ip() as f64 + c) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Blez => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a <= 0.0 { b as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Blezal => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a <= 0.0 { b as u32 } else { next_ip };
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Brlez => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a <= 0.0 {
                            (this.ip() as f64 + b) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Bltz => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a < 0.0 { b as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Bltzal => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a < 0.0 { b as u32 } else { next_ip };
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Brltz => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a < 0.0 {
                            (this.ip() as f64 + b) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Bgt => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a > b { c as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Bgtal => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a > b { c as u32 } else { next_ip };
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Brgt => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a > b {
                            (this.ip() as f64 + c) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Bgtz => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a > 0.0 { b as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Bgtzal => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a > 0.0 { b as u32 } else { next_ip };
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Brgtz => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a > 0.0 {
                            (this.ip() as f64 + b) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Bge => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a >= b { c as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Bgeal => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a >= b { c as u32 } else { next_ip };
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Brge => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a >= b {
                            (this.ip() as f64 + c) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Bgez => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a >= 0.0 { b as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Bgezal => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a >= 0.0 { b as u32 } else { next_ip };
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Brgez => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a >= 0.0 {
                            (this.ip() as f64 + b) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Bap => match &operands[..] {
                    [a, b, c, d] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        let d = d.as_value(this, inst, 4)?;
                        next_ip = if f64::abs(a - b)
                            <= f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0)
                        {
                            d as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 4)),
                },
                Bapal => match &operands[..] {
                    [a, b, c, d] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        let d = d.as_value(this, inst, 4)?;
                        next_ip = if f64::abs(a - b)
                            <= f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0)
                        {
                            d as u32
                        } else {
                            next_ip
                        };
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 4)),
                },
                Brap => match &operands[..] {
                    [a, b, c, d] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        let d = d.as_value(this, inst, 4)?;
                        next_ip = if f64::abs(a - b)
                            <= f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0)
                        {
                            (this.ip() as f64 + d) as u32
                        } else {
                            next_ip
                        };
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 4)),
                },
                Bapz => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a.abs() <= f64::max(b * a.abs(), f64::EPSILON * 8.0) {
                            c as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Bapzal => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a.abs() <= f64::max(b * a.abs(), f64::EPSILON * 8.0) {
                            c as u32
                        } else {
                            next_ip
                        };
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Brapz => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a.abs() <= f64::max(b * a.abs(), f64::EPSILON * 8.0) {
                            (this.ip() as f64 + c) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Bna => match &operands[..] {
                    [a, b, c, d] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        let d = d.as_value(this, inst, 4)?;
                        next_ip = if f64::abs(a - b)
                            > f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0)
                        {
                            d as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 4)),
                },
                Bnaal => match &operands[..] {
                    [a, b, c, d] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        let d = d.as_value(this, inst, 4)?;
                        next_ip = if f64::abs(a - b)
                            > f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0)
                        {
                            d as u32
                        } else {
                            next_ip
                        };
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 4)),
                },
                Brna => match &operands[..] {
                    [a, b, c, d] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        let d = d.as_value(this, inst, 4)?;
                        next_ip = if f64::abs(a - b)
                            > f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0)
                        {
                            (this.ip() as f64 + d) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 4)),
                },
                Bnaz => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a.abs() > f64::max(b * a.abs(), f64::EPSILON * 8.0) {
                            c as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Bnazal => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a.abs() > f64::max(b * a.abs(), f64::EPSILON * 8.0) {
                            c as u32
                        } else {
                            next_ip
                        };
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Brnaz => match &operands[..] {
                    [a, b, c] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        let c = c.as_value(this, inst, 3)?;
                        next_ip = if a.abs() > f64::max(b * a.abs(), f64::EPSILON * 8.0) {
                            (this.ip() as f64 + c) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Bdse => match &operands[..] {
                    [d, a] => {
                        let (device, _connection) = d.as_device(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        next_ip = if device.is_some() { a as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Bdseal => match &operands[..] {
                    [d, a] => {
                        let (device, _connection) = d.as_device(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        next_ip = if device.is_some() { a as u32 } else { next_ip };
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Brdse => match &operands[..] {
                    [d, a] => {
                        let (device, _connection) = d.as_device(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        next_ip = if device.is_some() {
                            (this.ip() as f64 + a) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Bdns => match &operands[..] {
                    [d, a] => {
                        let (device, _connection) = d.as_device(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        next_ip = if device.is_none() { a as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Bdnsal => match &operands[..] {
                    [d, a] => {
                        let (device, _connection) = d.as_device(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        next_ip = if device.is_none() { a as u32 } else { next_ip };
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Brdns => match &operands[..] {
                    [d, a] => {
                        let (device, _connection) = d.as_device(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        next_ip = if device.is_none() {
                            (this.ip() as f64 + a) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Bnan => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a.is_nan() { b as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Brnan => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        next_ip = if a.is_nan() {
                            (this.ip() as f64 + b) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },

                J => match &operands[..] {
                    [a] => {
                        let a = a.as_value(this, inst, 1)?;
                        next_ip = a as u32;
                        Ok(())
                    }
                    oprs => Err(ICError::too_many_operands(oprs.len(), 1)),
                },
                Jal => match &operands[..] {
                    [a] => {
                        let a = a.as_value(this, inst, 1)?;
                        next_ip = a as u32;
                        this.al();
                        Ok(())
                    }
                    oprs => Err(ICError::too_many_operands(oprs.len(), 1)),
                },
                Jr => match &operands[..] {
                    [a] => {
                        let a = a.as_value(this, inst, 1)?;
                        next_ip = (this.ip() as f64 + a) as u32;
                        Ok(())
                    }
                    oprs => Err(ICError::too_many_operands(oprs.len(), 1)),
                },

                Seq => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        let b = b.as_value(this, inst, 3)?;
                        this.set_register(indirection, target, if a == b { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Seqz => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, if a == 0.0 { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Sne => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        let b = b.as_value(this, inst, 3)?;
                        this.set_register(indirection, target, if a != b { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Snez => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, if a != 0.0 { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Slt => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        let b = b.as_value(this, inst, 3)?;
                        this.set_register(indirection, target, if a < b { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Sltz => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, if a < 0.0 { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Sle => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        let b = b.as_value(this, inst, 3)?;
                        this.set_register(indirection, target, if a <= b { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Slez => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, if a <= 0.0 { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Sgt => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        let b = b.as_value(this, inst, 3)?;
                        this.set_register(indirection, target, if a > b { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Sgtz => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, if a > 0.0 { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Sge => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        let b = b.as_value(this, inst, 3)?;
                        this.set_register(indirection, target, if a >= b { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Sgez => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, if a >= 0.0 { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Sap => match &operands[..] {
                    [reg, a, b, c] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        let b = b.as_value(this, inst, 3)?;
                        let c = c.as_value(this, inst, 4)?;
                        this.set_register(
                            indirection,
                            target,
                            if f64::abs(a - b)
                                <= f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0)
                            {
                                1.0
                            } else {
                                0.0
                            },
                        )?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Sapz => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        let b = b.as_value(this, inst, 3)?;
                        this.set_register(
                            indirection,
                            target,
                            if a.abs() <= f64::max(b * a.abs(), f64::EPSILON * 8.0) {
                                1.0
                            } else {
                                0.0
                            },
                        )?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Sna => match &operands[..] {
                    [reg, a, b, c] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        let b = b.as_value(this, inst, 3)?;
                        let c = c.as_value(this, inst, 4)?;
                        this.set_register(
                            indirection,
                            target,
                            if f64::abs(a - b)
                                > f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0)
                            {
                                1.0
                            } else {
                                0.0
                            },
                        )?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 4)),
                },
                Snaz => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        let b = b.as_value(this, inst, 3)?;
                        this.set_register(
                            indirection,
                            target,
                            if a.abs() > f64::max(b * a.abs(), f64::EPSILON * 8.0) {
                                1.0
                            } else {
                                0.0
                            },
                        )?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Sdse => match &operands[..] {
                    [reg, device] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let (device, _connection) = device.as_device(this, inst, 2)?;
                        this.set_register(
                            indirection,
                            target,
                            if device.is_some() { 1.0 } else { 0.0 },
                        )?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Sdns => match &operands[..] {
                    [reg, device] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let (device, _connection) = device.as_device(this, inst, 2)?;
                        this.set_register(
                            indirection,
                            target,
                            if device.is_none() { 1.0 } else { 0.0 },
                        )?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Snan => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, if a.is_nan() { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Snanz => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, if a.is_nan() { 0.0 } else { 1.0 })?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },

                Select => match &operands[..] {
                    [reg, a, b, c] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        let b = b.as_value(this, inst, 3)?;
                        let c = c.as_value(this, inst, 4)?;
                        this.set_register(indirection, target, if a != 0.0 { b } else { c })?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 4)),
                },

                Add => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        let b = b.as_value(this, inst, 3)?;
                        this.set_register(indirection, target, a + b)?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Sub => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        let b = b.as_value(this, inst, 3)?;
                        this.set_register(indirection, target, a - b)?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Mul => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        let b = b.as_value(this, inst, 3)?;
                        this.set_register(indirection, target, a * b)?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Div => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        let b = b.as_value(this, inst, 3)?;
                        this.set_register(indirection, target, a / b)?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Mod => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        let b = b.as_value(this, inst, 1)?;
                        let mut m = (a % b);
                        if m < 0.0 {
                            m += b;
                        }
                        this.set_register(indirection, target, m)?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Exp => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, f64::exp(a))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Log => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, f64::ln(a))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Sqrt => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, f64::sqrt(a))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },

                Max => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        let b = b.as_value(this, inst, 3)?;
                        this.set_register(indirection, target, f64::max(a, b))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Min => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        let b = b.as_value(this, inst, 3)?;
                        this.set_register(indirection, target, f64::min(a, b))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Ceil => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, f64::ceil(a))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Floor => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, f64::floor(a))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Abs => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, f64::abs(a))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Round => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, f64::round(a))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Trunc => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, f64::trunc(a))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },

                Rand => match &operands[..] {
                    [reg] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let val = vm.random_f64();
                        this.set_register(indirection, target, val)?;
                        Ok(())
                    }
                    oprs => Err(ICError::too_many_operands(oprs.len(), 1)),
                },

                Sin => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, f64::sin(a))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Cos => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, f64::cos(a))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Tan => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, f64::tan(a))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Asin => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, f64::asin(a))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Acos => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, f64::acos(a))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Atan => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        this.set_register(indirection, target, f64::atan(a))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Atan2 => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value(this, inst, 2)?;
                        let b = b.as_value(this, inst, 3)?;
                        this.set_register(indirection, target, f64::atan2(a, b))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },

                Sll | Sla => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value_i64(this, true, inst, 2)?;
                        let b = b.as_value_i32(this, inst, 3)?;
                        this.set_register(indirection, target, i64_to_f64(a << b))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Srl => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value_i64(this, false, inst, 2)?;
                        let b = b.as_value_i32(this, inst, 3)?;
                        this.set_register(indirection, target, i64_to_f64((a as u64 >> b) as i64))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Sra => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value_i64(this, true, inst, 2)?;
                        let b = b.as_value_i32(this, inst, 3)?;
                        this.set_register(indirection, target, i64_to_f64(a >> b))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },

                And => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value_i64(this, true, inst, 2)?;
                        let b = b.as_value_i64(this, true, inst, 3)?;
                        this.set_register(indirection, target, i64_to_f64(a & b))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Or => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value_i64(this, true, inst, 2)?;
                        let b = b.as_value_i64(this, true, inst, 3)?;
                        this.set_register(indirection, target, i64_to_f64(a | b))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Xor => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value_i64(this, true, inst, 2)?;
                        let b = b.as_value_i64(this, true, inst, 3)?;
                        this.set_register(indirection, target, i64_to_f64(a ^ b))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Nor => match &operands[..] {
                    [reg, a, b] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value_i64(this, true, inst, 2)?;
                        let b = b.as_value_i64(this, true, inst, 3)?;
                        this.set_register(indirection, target, i64_to_f64(!(a | b)))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Not => match &operands[..] {
                    [reg, a] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let a = a.as_value_i64(this, true, inst, 2)?;
                        this.set_register(indirection, target, i64_to_f64(!a))?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },

                Push => match &operands[..] {
                    [a] => {
                        let a = a.as_value(this, inst, 1)?;
                        this.push(a)?;
                        Ok(())
                    }
                    oprs => Err(ICError::too_many_operands(oprs.len(), 1)),
                },
                Pop => match &operands[..] {
                    [reg] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let val = this.pop()?;
                        this.set_register(indirection, target, val)?;
                        Ok(())
                    }
                    oprs => Err(ICError::too_many_operands(oprs.len(), 1)),
                },
                Poke => match &operands[..] {
                    [a, b] => {
                        let a = a.as_value(this, inst, 1)?;
                        let b = b.as_value(this, inst, 2)?;
                        this.poke(a, b)?;
                        Ok(())
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 2)),
                },
                Peek => match &operands[..] {
                    [reg] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let val = this.peek()?;
                        this.set_register(indirection, target, val)?;
                        Ok(())
                    }
                    oprs => Err(ICError::too_many_operands(oprs.len(), 1)),
                },

                Get => match &operands[..] {
                    [reg, dev_id, addr] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let (Some(device_id), _connection) = dev_id.as_device(this, inst, 2)?
                        else {
                            return Err(DeviceNotSet);
                        };
                        let device = vm.get_device_same_network(this.device, device_id);
                        match device {
                            Some(device) => match device.borrow().ic.as_ref() {
                                Some(ic_id) => {
                                    let addr = addr.as_value(this, inst, 3)?;
                                    let val = {
                                        if ic_id == &this.id {
                                            this.peek_addr(addr)
                                        } else {
                                            let ic = vm.ics.get(ic_id).unwrap().borrow();
                                            ic.peek_addr(addr)
                                        }
                                    }?;
                                    this.set_register(indirection, target, val)?;
                                    Ok(())
                                }
                                None => Err(DeviceHasNoIC),
                            },
                            None => Err(UnknownDeviceID(device_id as f64)),
                        }
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Getd => match &operands[..] {
                    [reg, dev_id, addr] => {
                        let RegisterSpec {
                            indirection,
                            target,
                        } = reg.as_register(this, inst, 1)?;
                        let (Some(device_id), _connection) = dev_id.as_device(this, inst, 2)?
                        else {
                            return Err(DeviceNotSet);
                        };
                        let device = vm.get_device_same_network(this.device, device_id);
                        match device {
                            Some(device) => match device.borrow().ic.as_ref() {
                                Some(ic_id) => {
                                    let addr = addr.as_value(this, inst, 3)?;
                                    let val = {
                                        if ic_id == &this.id {
                                            this.peek_addr(addr)
                                        } else {
                                            let ic = vm.ics.get(ic_id).unwrap().borrow();
                                            ic.peek_addr(addr)
                                        }
                                    }?;
                                    this.set_register(indirection, target, val)?;
                                    Ok(())
                                }
                                None => Err(DeviceHasNoIC),
                            },
                            None => Err(UnknownDeviceID(device_id as f64)),
                        }
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Put => match &operands[..] {
                    [dev_id, addr, val] => {
                        let (Some(device_id), _connection) = dev_id.as_device(this, inst, 1)?
                        else {
                            return Err(DeviceNotSet);
                        };
                        let device = vm.get_device_same_network(this.device, device_id);
                        match device {
                            Some(device) => match device.borrow().ic.as_ref() {
                                Some(ic_id) => {
                                    let addr = addr.as_value(this, inst, 2)?;
                                    let val = val.as_value(this, inst, 3)?;
                                    if ic_id == &this.id {
                                        this.poke(addr, val)?;
                                    } else {
                                        let ic = vm.ics.get(ic_id).unwrap().borrow();
                                        ic.poke(addr, val)?;
                                    }
                                    vm.set_modified(device_id);
                                    Ok(())
                                }
                                None => Err(DeviceHasNoIC),
                            },
                            None => Err(UnknownDeviceID(device_id as f64)),
                        }
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },
                Putd => match &operands[..] {
                    [dev_id, addr, val] => {
                        let device_id = dev_id.as_value(this, inst, 1)?;
                        if device_id >= u16::MAX as f64 || device_id < u16::MIN as f64 {
                            return Err(DeviceIndexOutOfRange(device_id));
                        }
                        let device = vm.get_device_same_network(this.device, device_id as u32);
                        match device {
                            Some(device) => match device.borrow().ic.as_ref() {
                                Some(ic_id) => {
                                    let addr = addr.as_value(this, inst, 2)?;
                                    let val = val.as_value(this, inst, 3)?;
                                    if ic_id == &this.id {
                                        this.poke(addr, val)?;
                                    } else {
                                        let ic = vm.ics.get(ic_id).unwrap().borrow();
                                        ic.poke(addr, val)?;
                                    }
                                    vm.set_modified(device_id as u32);
                                    Ok(())
                                }
                                None => Err(DeviceHasNoIC),
                            },
                            None => Err(UnknownDeviceID(device_id)),
                        }
                    }
                    oprs => Err(ICError::mismatch_operands(oprs.len(), 3)),
                },

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
                        if slt == SlotLogicType::LineNumber && this.device == device_id {
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
            self.propgate_line_number(vm);
        }
        result
    }
}

#[allow(dead_code)]
const CHANNEL_LOGIC_TYPES: [grammar::LogicType; 8] = [
    grammar::LogicType::Channel0,
    grammar::LogicType::Channel1,
    grammar::LogicType::Channel2,
    grammar::LogicType::Channel3,
    grammar::LogicType::Channel4,
    grammar::LogicType::Channel5,
    grammar::LogicType::Channel6,
    grammar::LogicType::Channel7,
];

trait LogicTypeExt {
    fn as_channel(&self) -> Option<usize>;
}
impl LogicTypeExt for grammar::LogicType {
    fn as_channel(&self) -> Option<usize> {
        match self {
            grammar::LogicType::Channel0 => Some(0),
            grammar::LogicType::Channel1 => Some(1),
            grammar::LogicType::Channel2 => Some(2),
            grammar::LogicType::Channel3 => Some(3),
            grammar::LogicType::Channel4 => Some(4),
            grammar::LogicType::Channel5 => Some(5),
            grammar::LogicType::Channel6 => Some(6),
            grammar::LogicType::Channel7 => Some(7),
            _ => None,
        }
    }
}

pub fn f64_to_i64(f: f64, signed: bool) -> i64 {
    let mut num: i64 = (f % 9007199254740992.0) as i64;
    if !signed {
        num &= 18014398509481983_i64;
    }
    num
}

pub fn i64_to_f64(i: i64) -> f64 {
    let flag: bool = (i & 9007199254740992_i64) != 0;
    let mut i = i & 9007199254740991_i64;
    if flag {
        i &= -9007199254740992_i64;
    }
    i as f64
}

#[cfg(test)]
mod tests {
    use crate::vm::VMError;

    use super::*;

    #[test]
    fn batch_modes() -> Result<(), VMError> {
        let mut vm = VM::new();
        let ic = vm.add_ic(None).unwrap();
        let ic_id = {
            let device = vm.devices.get(&ic).unwrap();
            let device_ref = device.borrow();
            device_ref.ic.unwrap()
        };
        let ic_chip = vm.ics.get(&ic_id).unwrap().borrow();
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
    fn stack() -> Result<(), VMError> {
        let mut vm = VM::new();
        let ic = vm.add_ic(None).unwrap();
        let ic_id = {
            let device = vm.devices.get(&ic).unwrap();
            let device_ref = device.borrow();
            device_ref.ic.unwrap()
        };
        let ic_chip = vm.ics.get(&ic_id).unwrap().borrow();
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
