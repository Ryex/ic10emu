use core::f64;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
    u32,
};

use itertools::Itertools;

use time::format_description;

use crate::grammar::{self, ParseError};

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
    #[error("Error Compileing Code: {0}")]
    ParseError(#[from] ParseError),
    #[error("Duplicate label {0}")]
    DuplicateLabel(String),
    #[error("Instruction Pointer out of range: '{0}'")]
    InstructionPointerOutOfRange(u32),
    #[error("Register Pointer out of range: '{0}'")]
    RegisterIndexOutOfRange(f64),
    #[error("Device Pointer out of range: '{0}'")]
    DeviceIndexOutOfRange(f64),
    #[error("Stack index out of range: '{0}'")]
    StackIndexOutOfRange(f64),
    #[error("slot index out of range: '{0}'")]
    SlotIndexOutOfRange(f64),
    #[error("Unknown device ID '{0}'")]
    UnknownDeviceID(f64),
    #[error("Too few operands!: provide: '{provided}', desired: '{desired}'")]
    TooFewOperands { provided: u32, desired: u32 },
    #[error("Too many operands!: provide: '{provided}', desired: '{desired}'")]
    TooManyOperands { provided: u32, desired: u32 },
    #[error("Incorrect Operand Type for operand {index}, not a {desired} ")]
    IncorrectOperandType { index: u32, desired: String },
    #[error("Unknown identifier '{0}")]
    UnknownIdentifier(String),
    #[error("A Device is not a Value")]
    DeviceNotValue,
    #[error("A Value is not a Device")]
    ValueNotDevice,
    #[error("Device Not Set")]
    DeviceNotSet,
    #[error("Shift Underflow i64(signed long)")]
    ShiftUnderflowI64,
    #[error("Shift Overflow i64(signed long)")]
    ShiftOverflowI64,
    #[error("Shift Underflow i32(signed int)")]
    ShiftUnderflowI32,
    #[error("Shift Overflow i32(signed int)")]
    ShiftOverflowI32,
    #[error("Stack Underflow")]
    StackUnderflow,
    #[error("Stack Overflow")]
    StackOverflow,
    #[error("Duplicate Define '{0}'")]
    DuplicateDefine(String),
    #[error("Read Only field '{0}'")]
    ReadOnlyField(String),
    #[error("Write Only field '{0}'")]
    WriteOnlyField(String),
    #[error("Device Has No Field '{0}'")]
    DeviceHasNoField(String),
    #[error("Device has not IC")]
    DeviceHasNoIC,
    #[error("Unknown Device '{0}'")]
    UnknownDeviceId(f64),
    #[error("Unknown Logic Type '{0}'")]
    UnknownLogicType(f64),
    #[error("Unknown Slot Logic Type '{0}'")]
    UnknownSlotLogicType(f64),
    #[error("Unknown Batch Mode '{0}'")]
    UnknownBatchMode(f64),
    #[error("Unknown Reagent Mode '{0}'")]
    UnknownReagentMode(f64),
    #[error("Type Value Not Known")]
    TypeValueNotKnown,
    #[error("Empty Device List")]
    EmptyDeviceList,
    #[error("Connection index out of range: '{0}'")]
    ConnecitonIndexOutOFRange(u32),
    #[error("Connection specifier missing")]
    MissingConnecitonSpecifier,
    #[error("No data network on connection '{0}'")]
    NotDataConnection(u32),
    #[error("Network not connected on connection '{0}'")]
    NetworkNotConnected(u32),
    #[error("Bad Network Id '{0}'")]
    BadNetworkId(u32),
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
            ICState::Start => "Not Run".to_string(),
            ICState::Running => "Running".to_string(),
            ICState::Yield => "Ic has yielded, Resume on next tick".to_string(),
            ICState::Sleep(then, sleep_for) => {
                let format = format_description::parse("[hour]:[minute]:[second]").unwrap();
                let resume = then.clone() + time::Duration::new(*sleep_for as i64, 0);
                format!(
                    "Sleeping for {sleep_for} seconds, will resume at {}",
                    resume.format(&format).unwrap()
                )
            }
            ICState::Error(err) => format!("{err}"),
            ICState::HasCaughtFire => "IC has caught fire! this is not a joke!".to_string(),
        };
        write!(f, "{out}")
    }
}

#[derive(Debug)]
pub struct IC {
    pub device: u16,
    pub id: u16,
    pub registers: [f64; 18],
    pub ip: u32,
    pub ic: u16,
    pub stack: [f64; 512],
    pub aliases: HashMap<String, grammar::Operand>,
    pub defines: HashMap<String, f64>,
    pub pins: [Option<u16>; 6],
    pub code: String,
    pub program: Program,
    pub state: ICState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub instructions: Vec<grammar::Instruction>,
    pub labels: HashMap<String, u32>,
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
            labels: HashMap::new(),
        }
    }

    pub fn try_from_code(code: &str) -> Result<Self, ICError> {
        let parse_tree = grammar::parse(&code)?;
        let mut labels_set = HashSet::new();
        let mut labels = HashMap::new();
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
                            Err(ICError::DuplicateLabel(label.id.name.clone()))
                        } else {
                            labels_set.insert(label.id.name.clone());
                            labels.insert(label.id.name.clone(), line_number as u32);
                            Ok(grammar::Instruction {
                                instruction: grammar::InstructionOp::Nop,
                                operands: vec![],
                            })
                        }
                    }
                    grammar::Code::Instruction(instruction) => Ok(instruction),
                },
            })
            .try_collect()?;
        Ok(Program {
            instructions,
            labels,
        })
    }

    pub fn get_line(&self, line: u32) -> Result<&grammar::Instruction, ICError> {
        self.instructions
            .get(line as usize)
            .ok_or(ICError::InstructionPointerOutOfRange(line))
    }
}

impl IC {

    pub fn new(id: u16, device: u16) -> Self {
        IC {
            device,
            id,
            ip: 0,
            ic: 0,
            registers: [0.0; 18],
            stack: [0.0; 512],
            pins: [None; 6],
            program: Program::new(),
            code: String::new(),
            aliases: HashMap::new(),
            defines: HashMap::new(),
            state: ICState::Start,
        }
    }

    pub fn reset(&mut self) {
        self.ip = 0;
        self.ic = 0;
        self.registers = [0.0; 18];
        self.stack = [0.0; 512];
        self.aliases = HashMap::new();
        self.defines = HashMap::new();
        self.state = ICState::Start;
    }

    pub fn set_code(&mut self, code: &str) -> Result<(), ICError> {
        let prog = Program::try_from_code(code)?;
        self.ip = 0;
        self.ic = 0;
        self.aliases = HashMap::new();
        self.defines = HashMap::new();
        self.program = prog;
        self.code = code.to_string();
        Ok(())
    }

    pub fn get_real_target(&self, indirection: u32, target: u32) -> Result<f64, ICError> {
        let mut i = indirection;
        let mut t = target as f64;
        while i > 0 {
            if let Some(new_t) = self.registers.get(t as usize) {
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
            .get(t as usize)
            .ok_or(ICError::RegisterIndexOutOfRange(t))
            .copied()
    }

    /// sets a register thorough, recursing through provided indirection, returns value previously
    pub fn set_register(
        &mut self,
        indirection: u32,
        target: u32,
        val: f64,
    ) -> Result<f64, ICError> {
        let t = self.get_real_target(indirection, target)?;
        let old_val = self
            .registers
            .get(t as usize)
            .ok_or(ICError::RegisterIndexOutOfRange(t))
            .copied()?;
        self.registers[t as usize] = val;
        Ok(old_val)
    }

    pub fn get_ident_value(&self, ident: &str) -> Result<f64, ICError> {
        if let Some(operand) = self.aliases.get(ident) {
            operand.get_value(self)
        } else if let Some(val) = self.defines.get(ident) {
            Ok(*val)
        } else if let Some(label) = self.program.labels.get(ident) {
            Ok(*label as f64)
        } else {
            Err(ICError::UnknownIdentifier(ident.to_string()))
        }
    }

    pub fn get_ident_device_id(&self, ident: &str) -> Result<(Option<u16>, Option<u32>), ICError> {
        if let Some(operand) = self.aliases.get(ident) {
            operand.get_device_id(self)
        } else {
            Err(ICError::UnknownIdentifier(ident.to_string()))
        }
    }

    /// save ip to 'ra' or register 18
    fn al(&mut self) {
        self.registers[17] = self.ip as f64 + 1.0;
    }

    pub fn push(&mut self, val: f64) -> Result<f64, ICError> {
        let sp = (self.registers[16]) as i32;
        if sp < 0 {
            Err(ICError::StackUnderflow)
        } else if sp >= 512 {
            Err(ICError::StackOverflow)
        } else {
            let last = self.stack[sp as usize];
            self.stack[sp as usize] = val;
            self.registers[16] += 1.0;
            Ok(last)
        }
    }

    pub fn pop(&mut self) -> Result<f64, ICError> {
        let sp = (self.registers[16]) as i32;
        if sp < 0 {
            Err(ICError::StackUnderflow)
        } else if sp >= 512 {
            Err(ICError::StackOverflow)
        } else {
            let last = self.stack[sp as usize];
            self.registers[16] -= 1.0;
            Ok(last)
        }
    }

    pub fn poke(&mut self, address: f64, val: f64) -> Result<f64, ICError> {
        let sp = address as i32;
        if sp < 0 || sp >= 512 {
            Err(ICError::StackIndexOutOfRange(address))
        } else {
            let last = self.stack[sp as usize];
            self.stack[sp as usize] = val;
            Ok(last)
        }
    }

    pub fn peek(&self) -> Result<f64, ICError> {
        let sp = (self.registers[16]) as i32;
        if sp < 0 {
            Err(ICError::StackUnderflow)
        } else if sp >= 512 {
            Err(ICError::StackOverflow)
        } else {
            let last = self.stack[sp as usize];
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
            let last = self.stack[sp as usize];
            Ok(last)
        }
    }

    /// processes one line of the contained program
    pub fn step(&mut self, vm: &crate::VM) -> Result<bool, LineError> {
        // TODO: handle sleep
        self.state = ICState::Running;
        let line = self.ip;
        let result = self.internal_step(vm);
        if let Err(error) = result {
            let error = LineError { error, line };
            self.state = ICState::Error(error.clone());
            Err(error)
        } else {
            Ok(true)
        }
    }

    fn internal_step(&mut self, vm: &crate::VM) -> Result<(), ICError> {
        use grammar::*;
        use ICError::*;

        let line = self.program.get_line(self.ip)?;
        let mut next_ip = self.ip + 1;
        let result: Result<(), ICError> = 'inst: {
            use grammar::InstructionOp::*;
            let operands = &line.operands;
            match line.instruction {
                Nop => Ok(()),
                Hcf => Ok(()), // TODO
                Sleep => match &operands[..] {
                    [a] => {
                        let a = a.get_value(self)?;
                        let now = time::OffsetDateTime::now_local()
                            .unwrap_or(time::OffsetDateTime::now_utc());
                        self.state = ICState::Sleep(now, a);
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 1,
                    }),
                }, // TODO
                Yield => {
                    if operands.len() != 0 {
                        Err(TooManyOperands {
                            provided: operands.len() as u32,
                            desired: 0,
                        })
                    } else {
                        self.state = ICState::Yield;
                        Ok(())
                    }
                }
                Define => match &operands[..] {
                    [_op1] => Err(TooFewOperands {
                        provided: 1,
                        desired: 2,
                    }),
                    [name, number] => {
                        let &Operand::Identifier(ident) = &name else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Name".to_string(),
                            });
                        };
                        let &Operand::Number(num) = &number else {
                            break 'inst Err(IncorrectOperandType {
                                index: 2,
                                desired: "Number".to_string(),
                            });
                        };
                        if self.defines.contains_key(&ident.name) {
                            Err(DuplicateDefine(ident.name.clone()))
                        } else {
                            self.defines.insert(ident.name.clone(), num.value());
                            Ok(())
                        }
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Alias => match &operands[..] {
                    [_op1] => Err(TooFewOperands {
                        provided: 1,
                        desired: 2,
                    }),
                    [name, device_reg] => {
                        let &Operand::Identifier(ident) = &name else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Name".to_string(),
                            });
                        };
                        let alias = match &device_reg {
                            &Operand::RegisterSpec {
                                indirection,
                                target,
                            } => Operand::RegisterSpec {
                                indirection: *indirection,
                                target: *target,
                            },
                            &Operand::DeviceSpec { device, connection } => Operand::DeviceSpec {
                                device: *device,
                                connection: *connection,
                            },
                            _ => {
                                break 'inst Err(IncorrectOperandType {
                                    index: 2,
                                    desired: "Device Or Register".to_string(),
                                })
                            }
                        };
                        self.aliases.insert(ident.name.clone(), alias);
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Move => match &operands[..] {
                    [_op1] => Err(TooFewOperands {
                        provided: 1,
                        desired: 2,
                    }),
                    [reg, val] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = &reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };

                        let val = val.get_value(self)?;
                        self.set_register(*indirection, *target, val)?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },

                Beq => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a == b { c as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Beqal => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a == b { c as u32 } else { next_ip };
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Breq => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a == b {
                            (self.ip as f64 + c) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Beqz => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a == 0.0 { b as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Beqzal => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a == 0.0 { b as u32 } else { next_ip };
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Breqz => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a == 0.0 {
                            (self.ip as f64 + b) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Bne => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a != b { c as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Bneal => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a != b { c as u32 } else { next_ip };
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Brne => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a != b {
                            (self.ip as f64 + c) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Bnez => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a != 0.0 { b as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Bnezal => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a != 0.0 { b as u32 } else { next_ip };
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Brnez => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a != 0.0 {
                            (self.ip as f64 + b) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Blt => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a < b { c as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Bltal => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a < b { c as u32 } else { next_ip };
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Brlt => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a < b {
                            (self.ip as f64 + c) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Ble => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a <= b { c as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Bleal => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a <= b { c as u32 } else { next_ip };
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Brle => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a <= b {
                            (self.ip as f64 + c) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Blez => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a <= 0.0 { b as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Blezal => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a <= 0.0 { b as u32 } else { next_ip };
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Brlez => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a <= 0.0 {
                            (self.ip as f64 + b) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Bltz => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a < 0.0 { b as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Bltzal => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a < 0.0 { b as u32 } else { next_ip };
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Brltz => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a < 0.0 {
                            (self.ip as f64 + b) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Bgt => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a > b { c as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Bgtal => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a > b { c as u32 } else { next_ip };
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Brgt => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a > b {
                            (self.ip as f64 + c) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Bgtz => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a > 0.0 { b as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Bgtzal => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a > 0.0 { b as u32 } else { next_ip };
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Brgtz => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a > 0.0 {
                            (self.ip as f64 + b) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Bge => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a >= b { c as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Bgeal => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a >= b { c as u32 } else { next_ip };
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Brge => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a >= b {
                            (self.ip as f64 + c) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Bgez => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a >= 0.0 { b as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Bgezal => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a >= 0.0 { b as u32 } else { next_ip };
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Brgez => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a >= 0.0 {
                            (self.ip as f64 + b) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Bap => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                    [a, b, c, d] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        let d = d.get_value(self)?;
                        next_ip = if f64::abs(a - b)
                            <= f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0)
                        {
                            d as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                },
                Bapal => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                    [a, b, c, d] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        let d = d.get_value(self)?;
                        next_ip = if f64::abs(a - b)
                            <= f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0)
                        {
                            d as u32
                        } else {
                            next_ip
                        };
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                },
                Brap => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                    [a, b, c, d] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        let d = d.get_value(self)?;
                        next_ip = if f64::abs(a - b)
                            <= f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0)
                        {
                            (self.ip as f64 + d) as u32
                        } else {
                            next_ip
                        };
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                },
                Bapz => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a.abs() <= f64::max(b * a.abs(), f64::EPSILON * 8.0) {
                            c as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Bapzal => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a.abs() <= f64::max(b * a.abs(), f64::EPSILON * 8.0) {
                            c as u32
                        } else {
                            next_ip
                        };
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Brapz => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a.abs() <= f64::max(b * a.abs(), f64::EPSILON * 8.0) {
                            (self.ip as f64 + c) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Bna => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                    [a, b, c, d] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        let d = d.get_value(self)?;
                        next_ip = if f64::abs(a - b)
                            > f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0)
                        {
                            d as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                },
                Bnaal => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                    [a, b, c, d] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        let d = d.get_value(self)?;
                        next_ip = if f64::abs(a - b)
                            > f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0)
                        {
                            d as u32
                        } else {
                            next_ip
                        };
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                },
                Brna => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                    [a, b, c, d] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        let d = d.get_value(self)?;
                        next_ip = if f64::abs(a - b)
                            > f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0)
                        {
                            (self.ip as f64 + d) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                },
                Bnaz => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a.abs() > f64::max(b * a.abs(), f64::EPSILON * 8.0) {
                            c as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Bnazal => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a.abs() > f64::max(b * a.abs(), f64::EPSILON * 8.0) {
                            c as u32
                        } else {
                            next_ip
                        };
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Brnaz => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [a, b, c] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        next_ip = if a.abs() > f64::max(b * a.abs(), f64::EPSILON * 8.0) {
                            (self.ip as f64 + c) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Bdse => match &operands[..] {
                    [_] => Err(TooFewOperands {
                        provided: 1,
                        desired: 2,
                    }),
                    [d, a] => {
                        let (device, _connection) = d.get_device_id(self)?;
                        let a = a.get_value(self)?;
                        next_ip = if device.is_some() { a as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Bdseal => match &operands[..] {
                    [_] => Err(TooFewOperands {
                        provided: 1,
                        desired: 2,
                    }),
                    [d, a] => {
                        let (device, _connection) = d.get_device_id(self)?;
                        let a = a.get_value(self)?;
                        next_ip = if device.is_some() { a as u32 } else { next_ip };
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Brdse => match &operands[..] {
                    [_] => Err(TooFewOperands {
                        provided: 1,
                        desired: 2,
                    }),
                    [d, a] => {
                        let (device, _connection) = d.get_device_id(self)?;
                        let a = a.get_value(self)?;
                        next_ip = if device.is_some() {
                            (self.ip as f64 + a) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Bdns => match &operands[..] {
                    [_] => Err(TooFewOperands {
                        provided: 1,
                        desired: 2,
                    }),
                    [d, a] => {
                        let (device, _connection) = d.get_device_id(self)?;
                        let a = a.get_value(self)?;
                        next_ip = if device.is_none() { a as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Bdnsal => match &operands[..] {
                    [_] => Err(TooFewOperands {
                        provided: 1,
                        desired: 2,
                    }),
                    [d, a] => {
                        let (device, _connection) = d.get_device_id(self)?;
                        let a = a.get_value(self)?;
                        next_ip = if device.is_none() { a as u32 } else { next_ip };
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Brdns => match &operands[..] {
                    [_] => Err(TooFewOperands {
                        provided: 1,
                        desired: 2,
                    }),
                    [d, a] => {
                        let (device, _connection) = d.get_device_id(self)?;
                        let a = a.get_value(self)?;
                        next_ip = if device.is_none() {
                            (self.ip as f64 + a) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Bnan => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a.is_nan() { b as u32 } else { next_ip };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Brnan => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        next_ip = if a.is_nan() {
                            (self.ip as f64 + b) as u32
                        } else {
                            next_ip
                        };
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },

                J => match &operands[..] {
                    [a] => {
                        let a = a.get_value(self)?;
                        next_ip = a as u32;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 1,
                    }),
                },
                Jal => match &operands[..] {
                    [a] => {
                        let a = a.get_value(self)?;
                        next_ip = a as u32;
                        self.al();
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 1,
                    }),
                },
                Jr => match &operands[..] {
                    [a] => {
                        let a = a.get_value(self)?;
                        next_ip = (self.ip as f64 + a) as u32;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 1,
                    }),
                },

                Seq => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        self.set_register(indirection, target, if a == b { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Seqz => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, if a == 0.0 { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Sne => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        self.set_register(indirection, target, if a != b { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Snez => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, if a != 0.0 { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Slt => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        self.set_register(indirection, target, if a < b { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Sltz => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, if a < 0.0 { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Sle => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        self.set_register(indirection, target, if a <= b { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Slez => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, if a <= 0.0 { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Sgt => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        self.set_register(indirection, target, if a > b { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Sgtz => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, if a > 0.0 { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Sge => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        self.set_register(indirection, target, if a >= b { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Sgez => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, if a >= 0.0 { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Sap => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b, c] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        self.set_register(
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
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Sapz => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        self.set_register(
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
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Sna => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                    [reg, a, b, c] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        self.set_register(
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
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                },
                Snaz => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        self.set_register(
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
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Sdse => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [reg, device] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let (device, _connection) = device.get_device_id(self)?;
                        self.set_register(
                            indirection,
                            target,
                            if device.is_some() { 1.0 } else { 0.0 },
                        )?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Sdns => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [reg, device] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let (device, _connection) = device.get_device_id(self)?;
                        self.set_register(
                            indirection,
                            target,
                            if device.is_none() { 1.0 } else { 0.0 },
                        )?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Snan => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, if a.is_nan() { 1.0 } else { 0.0 })?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Snanz => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, if a.is_nan() { 0.0 } else { 1.0 })?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },

                Select => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                    [reg, a, b, c] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        let c = c.get_value(self)?;
                        self.set_register(indirection, target, if a != 0.0 { b } else { c })?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                },

                Add => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        self.set_register(indirection, target, a + b)?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Sub => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        self.set_register(indirection, target, a - b)?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Mul => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        self.set_register(indirection, target, a * b)?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Div => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        self.set_register(indirection, target, a / b)?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Mod => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        self.set_register(indirection, target, ((a % b) + b) % b)?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Exp => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, f64::exp(a))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Log => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, f64::ln(a))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Sqrt => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, f64::sqrt(a))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },

                Max => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        self.set_register(indirection, target, f64::max(a, b))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Min => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        self.set_register(indirection, target, f64::min(a, b))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Ceil => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, f64::ceil(a))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Floor => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, f64::floor(a))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Abs => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, f64::abs(a))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Round => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, f64::round(a))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Trunc => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, f64::trunc(a))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },

                Rand => match &operands[..] {
                    [reg] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let val = vm.random.clone().borrow_mut().next_f64();
                        self.set_register(indirection, target, val)?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 1,
                    }),
                },

                Sin => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, f64::sin(a))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Cos => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, f64::cos(a))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Tan => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, f64::tan(a))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Asin => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, f64::asin(a))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Acos => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, f64::acos(a))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Atan => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        self.set_register(indirection, target, f64::atan(a))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Atan2 => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        self.set_register(indirection, target, f64::atan2(a, b))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },

                Sll | Sla => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value_i64(self, true)?;
                        let b = b.get_value_i32(self)?;
                        self.set_register(indirection, target, i64_to_f64(a << b))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Srl => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value_i64(self, false)?;
                        let b = b.get_value_i32(self)?;
                        self.set_register(indirection, target, i64_to_f64((a as u64 >> b) as i64))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Sra => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value_i64(self, true)?;
                        let b = b.get_value_i32(self)?;
                        self.set_register(indirection, target, i64_to_f64(a >> b))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },

                And => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value_i64(self, true)?;
                        let b = b.get_value_i64(self, true)?;
                        self.set_register(indirection, target, i64_to_f64(a & b))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Or => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value_i64(self, true)?;
                        let b = b.get_value_i64(self, true)?;
                        self.set_register(indirection, target, i64_to_f64(a | b))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Xor => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value_i64(self, true)?;
                        let b = b.get_value_i64(self, true)?;
                        self.set_register(indirection, target, i64_to_f64(a ^ b))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Nor => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, a, b] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value_i64(self, true)?;
                        let b = b.get_value_i64(self, true)?;
                        self.set_register(indirection, target, i64_to_f64(!(a | b)))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Not => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [reg, a] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let a = a.get_value_i64(self, true)?;
                        self.set_register(indirection, target, i64_to_f64(!a))?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },

                Push => match &operands[..] {
                    [a] => {
                        let a = a.get_value(self)?;
                        self.push(a)?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 1,
                    }),
                },
                Pop => match &operands[..] {
                    [reg] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let val = self.pop()?;
                        self.set_register(indirection, target, val)?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 1,
                    }),
                },
                Poke => match &operands[..] {
                    oprs @ [_] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                    [a, b] => {
                        let a = a.get_value(self)?;
                        let b = b.get_value(self)?;
                        self.poke(a, b)?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 2,
                    }),
                },
                Peek => match &operands[..] {
                    [reg] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let val = self.peek()?;
                        self.set_register(indirection, target, val)?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 1,
                    }),
                },

                Get => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, dev_id, addr] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let (Some(device_id), _connection) = dev_id.get_device_id(self)? else {
                            break 'inst Err(DeviceNotSet);
                        };
                        let device = vm.get_device_same_network(self.device, device_id);
                        match device {
                            Some(device) => match device.borrow().ic.as_ref() {
                                Some(ic_id) => {
                                    let addr = addr.get_value(self)?;
                                    let ic = vm.ics.get(&ic_id).unwrap().borrow();
                                    let val = ic.peek_addr(addr)?;
                                    self.set_register(indirection, target, val)?;
                                    Ok(())
                                }
                                None => Err(DeviceHasNoIC),
                            },
                            None => Err(UnknownDeviceID(device_id as f64)),
                        }
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Getd => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, dev_id, addr] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let (Some(device_id), _connection) = dev_id.get_device_id(self)? else {
                            break 'inst Err(DeviceNotSet);
                        };
                        let device = vm.get_device_same_network(self.device, device_id);
                        match device {
                            Some(device) => match device.borrow().ic.as_ref() {
                                Some(ic_id) => {
                                    let addr = addr.get_value(self)?;
                                    let ic = vm.ics.get(&ic_id).unwrap().borrow();
                                    let val = ic.peek_addr(addr)?;
                                    self.set_register(indirection, target, val)?;
                                    Ok(())
                                }
                                None => Err(DeviceHasNoIC),
                            },
                            None => Err(UnknownDeviceID(device_id as f64)),
                        }
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Put => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [dev_id, addr, val] => {
                        let (Some(device_id), _connection) = dev_id.get_device_id(self)? else {
                            break 'inst Err(DeviceNotSet);
                        };
                        let device = vm.get_device_same_network(self.device, device_id as u16);
                        match device {
                            Some(device) => match device.borrow().ic.as_ref() {
                                Some(ic_id) => {
                                    let addr = addr.get_value(self)?;
                                    let val = val.get_value(self)?;
                                    let mut ic = vm.ics.get(&ic_id).unwrap().borrow_mut();
                                    ic.poke(addr, val)?;
                                    Ok(())
                                }
                                None => Err(DeviceHasNoIC),
                            },
                            None => Err(UnknownDeviceID(device_id as f64)),
                        }
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Putd => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [dev_id, addr, val] => {
                        let device_id = dev_id.get_value(self)?;
                        if device_id >= u16::MAX as f64 || device_id < u16::MIN as f64 {
                            break 'inst Err(DeviceIndexOutOfRange(device_id));
                        }
                        let device = vm.get_device_same_network(self.device, device_id as u16);
                        match device {
                            Some(device) => match device.borrow().ic.as_ref() {
                                Some(ic_id) => {
                                    let addr = addr.get_value(self)?;
                                    let val = val.get_value(self)?;
                                    let mut ic = vm.ics.get(&ic_id).unwrap().borrow_mut();
                                    ic.poke(addr, val)?;
                                    Ok(())
                                }
                                None => Err(DeviceHasNoIC),
                            },
                            None => Err(UnknownDeviceID(device_id)),
                        }
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },

                S => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [dev, lt, val] => {
                        let (Some(device_id), connection) = dev.get_device_id(self)? else {
                            break 'inst Err(DeviceNotSet);
                        };
                        let lt = LogicType::try_from(lt.get_value(self)?)?;
                        if CHANNEL_LOGIC_TYPES.contains(&lt) {
                            let channel = channel_logic_to_channel(lt).unwrap();
                            let Some(connection) = connection else {
                                break 'inst Err(MissingConnecitonSpecifier);
                            };
                            let network_id = vm
                                .get_device_same_network(self.device, device_id as u16)
                                .map(|device| device.borrow().get_network_id(connection as usize))
                                .unwrap_or(Err(UnknownDeviceID(device_id as f64)))?;
                            let val = val.get_value(self)?;
                            vm.set_network_channel(network_id as usize, channel, val)?;
                            return Ok(());
                        }
                        let device = vm.get_device_same_network(self.device, device_id as u16);
                        match device {
                            Some(device) => {
                                let val = val.get_value(self)?;
                                device.borrow_mut().set_field(lt, val)?;
                                Ok(())
                            }
                            None => Err(UnknownDeviceID(device_id as f64)),
                        }
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Sd => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [dev, lt, val] => {
                        let device_id = dev.get_value(self)?;
                        if device_id >= u16::MAX as f64 || device_id < u16::MIN as f64 {
                            break 'inst Err(DeviceIndexOutOfRange(device_id));
                        }
                        let device = vm.get_device_same_network(self.device, device_id as u16);
                        match device {
                            Some(device) => {
                                let lt = LogicType::try_from(lt.get_value(self)?)?;
                                let val = val.get_value(self)?;
                                device.borrow_mut().set_field(lt, val)?;
                                Ok(())
                            }
                            None => Err(UnknownDeviceID(device_id as f64)),
                        }
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Ss => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                    [dev, index, lt, val] => {
                        let (Some(device_id), _connection) = dev.get_device_id(self)? else {
                            break 'inst Err(DeviceNotSet);
                        };
                        let device = vm.get_device_same_network(self.device, device_id as u16);
                        match device {
                            Some(device) => {
                                let index = index.get_value(self)?;
                                let lt = SlotLogicType::try_from(lt.get_value(self)?)?;
                                let val = val.get_value(self)?;
                                device.borrow_mut().set_slot_field(index, lt, val)?;
                                Ok(())
                            }
                            None => Err(UnknownDeviceID(device_id as f64)),
                        }
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                },
                Sb => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [prefab, lt, val] => {
                        let prefab = prefab.get_value(self)?;
                        let lt = LogicType::try_from(lt.get_value(self)?)?;
                        let val = val.get_value(self)?;
                        vm.set_batch_device_field(self.device, prefab, lt, val)?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Sbs => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                    [prefab, index, lt, val] => {
                        let prefab = prefab.get_value(self)?;
                        let index = index.get_value(self)?;
                        let lt = SlotLogicType::try_from(lt.get_value(self)?)?;
                        let val = val.get_value(self)?;
                        vm.set_batch_device_slot_field(self.device, prefab, index, lt, val)?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                },
                Sbn => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                    [prefab, name, lt, val] => {
                        let prefab = prefab.get_value(self)?;
                        let name = name.get_value(self)?;
                        let lt = LogicType::try_from(lt.get_value(self)?)?;
                        let val = val.get_value(self)?;
                        vm.set_batch_name_device_field(self.device, prefab, name, lt, val)?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                },

                L => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, dev, lt] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let (Some(device_id), connection) = dev.get_device_id(self)? else {
                            break 'inst Err(DeviceNotSet);
                        };
                        let lt = LogicType::try_from(lt.get_value(self)?)?;
                        if CHANNEL_LOGIC_TYPES.contains(&lt) {
                            let channel = channel_logic_to_channel(lt).unwrap();
                            let Some(connection) = connection else {
                                break 'inst Err(MissingConnecitonSpecifier);
                            };
                            let network_id = vm
                                .get_device_same_network(self.device, device_id as u16)
                                .map(|device| device.borrow().get_network_id(connection as usize))
                                .unwrap_or(Err(UnknownDeviceID(device_id as f64)))?;
                            let val = vm.get_network_channel(network_id as usize, channel)?;
                            self.set_register(indirection, target, val)?;
                            return Ok(());
                        }
                        let device = vm.get_device_same_network(self.device, device_id as u16);
                        match device {
                            Some(device) => {
                                let val = device.borrow().get_field(lt)?;
                                self.set_register(indirection, target, val)?;
                                Ok(())
                            }
                            None => Err(UnknownDeviceID(device_id as f64)),
                        }
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Ld => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                    [reg, dev, lt] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let device_id = dev.get_value(self)?;
                        if device_id >= u16::MAX as f64 || device_id < u16::MIN as f64 {
                            break 'inst Err(DeviceIndexOutOfRange(device_id));
                        }
                        let device = vm.get_device_same_network(self.device, device_id as u16);
                        match device {
                            Some(device) => {
                                let lt = LogicType::try_from(lt.get_value(self)?)?;
                                let val = device.borrow().get_field(lt)?;
                                self.set_register(indirection, target, val)?;
                                Ok(())
                            }
                            None => Err(UnknownDeviceID(device_id as f64)),
                        }
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 3,
                    }),
                },
                Ls => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                    [reg, dev, index, lt] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let (Some(device_id), _connection) = dev.get_device_id(self)? else {
                            break 'inst Err(DeviceNotSet);
                        };
                        let device = vm.get_device_same_network(self.device, device_id as u16);
                        match device {
                            Some(device) => {
                                let index = index.get_value(self)?;
                                let lt = SlotLogicType::try_from(lt.get_value(self)?)?;
                                let val = device.borrow().get_slot_field(index, lt)?;
                                self.set_register(indirection, target, val)?;
                                Ok(())
                            }
                            None => Err(UnknownDeviceID(device_id as f64)),
                        }
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                },
                Lr => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                    [reg, dev, rm, name] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let (Some(device_id), _connection) = dev.get_device_id(self)? else {
                            break 'inst Err(DeviceNotSet);
                        };
                        let device = vm.get_device_same_network(self.device, device_id as u16);
                        match device {
                            Some(device) => {
                                let rm = ReagentMode::try_from(rm.get_value(self)?)?;
                                let name = name.get_value(self)?;
                                let val = device.borrow().get_reagent(&rm, name);
                                self.set_register(indirection, target, val)?;
                                Ok(())
                            }
                            None => Err(UnknownDeviceID(device_id as f64)),
                        }
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                },
                Lb => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                    [reg, prefab, lt, bm] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let prefab = prefab.get_value(self)?;
                        let lt = LogicType::try_from(lt.get_value(self)?)?;
                        let bm = BatchMode::try_from(bm.get_value(self)?)?;
                        let val = vm.get_batch_device_field(self.device, prefab, lt, bm)?;
                        self.set_register(indirection, target, val)?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 4,
                    }),
                },
                Lbn => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] | oprs @ [_, _, _, _] => {
                        Err(TooFewOperands {
                            provided: oprs.len() as u32,
                            desired: 5,
                        })
                    }
                    [reg, prefab, name, lt, bm] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let prefab = prefab.get_value(self)?;
                        let name = name.get_value(self)?;
                        let lt = LogicType::try_from(lt.get_value(self)?)?;
                        let bm = BatchMode::try_from(bm.get_value(self)?)?;
                        let val =
                            vm.get_batch_name_device_field(self.device, prefab, name, lt, bm)?;
                        self.set_register(indirection, target, val)?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 5,
                    }),
                },
                Lbns => match &operands[..] {
                    oprs @ [_]
                    | oprs @ [_, _]
                    | oprs @ [_, _, _]
                    | oprs @ [_, _, _, _]
                    | oprs @ [_, _, _, _, _] => Err(TooFewOperands {
                        provided: oprs.len() as u32,
                        desired: 6,
                    }),
                    [reg, prefab, name, index, slt, bm] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let prefab = prefab.get_value(self)?;
                        let name = name.get_value(self)?;
                        let index = index.get_value(self)?;
                        let slt = SlotLogicType::try_from(slt.get_value(self)?)?;
                        let bm = BatchMode::try_from(bm.get_value(self)?)?;
                        let val = vm.get_batch_name_device_slot_field(
                            self.device,
                            prefab,
                            name,
                            index,
                            slt,
                            bm,
                        )?;
                        self.set_register(indirection, target, val)?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 6,
                    }),
                },
                Lbs => match &operands[..] {
                    oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] | oprs @ [_, _, _, _] => {
                        Err(TooFewOperands {
                            provided: oprs.len() as u32,
                            desired: 5,
                        })
                    }
                    [reg, prefab, index, slt, bm] => {
                        let &Operand::RegisterSpec {
                            indirection,
                            target,
                        } = reg
                        else {
                            break 'inst Err(IncorrectOperandType {
                                index: 1,
                                desired: "Register".to_string(),
                            });
                        };
                        let prefab = prefab.get_value(self)?;
                        let index = index.get_value(self)?;
                        let slt = SlotLogicType::try_from(slt.get_value(self)?)?;
                        let bm = BatchMode::try_from(bm.get_value(self)?)?;
                        let val =
                            vm.get_batch_device_slot_field(self.device, prefab, index, slt, bm)?;
                        self.set_register(indirection, target, val)?;
                        Ok(())
                    }
                    oprs => Err(TooManyOperands {
                        provided: oprs.len() as u32,
                        desired: 5,
                    }),
                },
            }
        };
        self.ic += 1;
        self.ip = next_ip;
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

fn channel_logic_to_channel(lt: grammar::LogicType) -> Option<usize> {
    match lt {
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

pub fn f64_to_i64(f: f64, signed: bool) -> i64 {
    let mut num: i64 = (f % 9007199254740992.0) as i64;
    if !signed {
        num &= 18014398509481983_i64
    }
    num
}

pub fn i64_to_f64(i: i64) -> f64 {
    let flag: bool = (i & 9007199254740992_i64) != 0;
    let mut i = i & 9007199254740991_i64;
    if flag {
        i &= -9007199254740992_i64
    }
    i as f64
}
