use core::f64;
use std::{collections::HashMap, u32};

use crate::grammar;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ICError {
    #[error("")]
    InstructionPointerOutOfRange(u32),
    #[error("")]
    RegisterIndexOutOfRange(f64),
    #[error("")]
    DeviceIndexOutOfRange(f64),
    #[error("")]
    StackIndexOutOfRange(f64),
    #[error("")]
    UnknownDeviceID(f64),
    #[error("")]
    ToFewOperands { provided: u32, desired: u32 },
    #[error("")]
    TooManyOperands { provided: u32, desired: u32 },
    #[error("")]
    IncorrectOperandType { index: u32, desired: String },
    #[error("")]
    UnknownIdentifier(String),
    #[error("")]
    DeviceNotValue,
    #[error("")]
    ValueNotDevice,
    #[error("")]
    DeviceNotSet,
    #[error("")]
    OperandNotRegister,
}

#[derive(Debug)]
pub struct IC {
    pub id: u16,
    pub registers: [f64; 18],
    pub ip: u32,
    pub stack: [f64; 512],
    pub aliases: HashMap<String, grammar::Operand>,
    pub labels: HashMap<String, u32>,
    pub pins: [Option<u16>; 6],
    pub code: String,
    pub program: Program,
    should_yield: bool,
}

#[derive(Debug)]
pub struct Program {
    pub instructions: Vec<grammar::Line>,
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
        }
    }
    pub fn try_from_code(input: &str) -> Result<Self, grammar::ParseError> {
        let parse_tree = grammar::parse(&input)?;
        Ok(Program {
            instructions: parse_tree,
        })
    }
    pub fn get_line(&self, line: u32) -> Result<&grammar::Line, ICError> {
        self.instructions
            .get(line as usize)
            .ok_or(ICError::InstructionPointerOutOfRange(line))
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
            program: Program::new(),
            code: String::new(),
            aliases: HashMap::new(),
            labels: HashMap::new(),
            should_yield: false,
        }
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
        self.registers[17] = self.ip as f64;
    }

    /// processes one line of the contained program
    pub fn step(&mut self, _housing: &mut crate::Device, _vm: &mut crate::VM) -> Result<(), ICError> {
        use grammar::InstructionOp::*;
        use grammar::*;
        use ICError::*;

        let line = self.program.get_line(self.ip)?;
        let mut next_ip = self.ip + 1;
        let result: Result<(), ICError> = if let Some(code) = &line.code {
            match code {
                Code::Label(label) => {
                    self.labels.insert(label.id.name.clone(), self.ip);
                    Ok(())
                }
                Code::Instruction(inst) => 'inst: {
                    match inst.instruction {
                        Nop => Ok(()),   // not used, empty line
                        Label => Ok(()), // Not used
                        Sleep => Ok(()), // TODO
                        Yield => {
                            if inst.operands.len() != 0 {
                                Err(TooManyOperands {
                                    provided: inst.operands.len() as u32,
                                    desired: 0,
                                })
                            } else {
                                self.should_yield = true;
                                Ok(())
                            }
                        }
                        Define => match &inst.operands[..] {
                            [_op1] => Err(ToFewOperands {
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
                                self.aliases.insert(
                                    ident.name.clone(),
                                    Operand::Number(Number::Float(num.value())),
                                );
                                Ok(())
                            }
                            oprs => Err(TooManyOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                        },
                        Alias => match &inst.operands[..] {
                            [_op1] => Err(ToFewOperands {
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
                                    &Operand::DeviceSpec { device, channel } => {
                                        Operand::DeviceSpec {
                                            device: *device,
                                            channel: *channel,
                                        }
                                    }
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
                        Move => match &inst.operands[..] {
                            [_op1] => Err(ToFewOperands {
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

                        Beq => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Beqal => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Breq => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Beqz => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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
                        Beqzal => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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
                        Breqz => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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
                        Bne => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Bneal => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Brne => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Bnez => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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
                        Bnezal => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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
                        Brnez => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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
                        Blt => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Bltal => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Brlt => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Ble => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Bleal => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Brle => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Blez => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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
                        Blezal => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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
                        Brlez => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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
                        Bltz => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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
                        Bltzal => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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
                        Brltz => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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
                        Bgt => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Bgtal => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Brgt => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Bgtz => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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
                        Bgtzal => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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
                        Brgtz => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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
                        Bge => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Bgeal => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Brge => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Bgez => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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
                        Bgezal => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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
                        Brgez => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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
                        Bap => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(ToFewOperands {
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
                        Bapal => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(ToFewOperands {
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
                        Brap => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(ToFewOperands {
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
                        Bapz => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Bapzal => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Brapz => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Bna => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(ToFewOperands {
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
                        Bnaal => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(ToFewOperands {
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
                        Brna => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(ToFewOperands {
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
                        Bnaz => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Bnazal => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Brnaz => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
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
                        Bdse => match &inst.operands[..] {
                            [_] => Err(ToFewOperands {
                                provided: 1,
                                desired: 2,
                            }),
                            [d, a] => {
                                let (device, _channel) = d.get_device_id(self)?;
                                let a = a.get_value(self)?;
                                next_ip = if device.is_some() { a as u32 } else { next_ip };
                                Ok(())
                            }
                            oprs => Err(TooManyOperands {
                                provided: oprs.len() as u32,
                                desired: 2,
                            }),
                        },
                        Bdseal => match &inst.operands[..] {
                            [_] => Err(ToFewOperands {
                                provided: 1,
                                desired: 2,
                            }),
                            [d, a] => {
                                let (device, _channel) = d.get_device_id(self)?;
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
                        Brdse => match &inst.operands[..] {
                            [_] => Err(ToFewOperands {
                                provided: 1,
                                desired: 2,
                            }),
                            [d, a] => {
                                let (device, _channel) = d.get_device_id(self)?;
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
                        Bdns => match &inst.operands[..] {
                            [_] => Err(ToFewOperands {
                                provided: 1,
                                desired: 2,
                            }),
                            [d, a] => {
                                let (device, _channel) = d.get_device_id(self)?;
                                let a = a.get_value(self)?;
                                next_ip = if device.is_none() { a as u32 } else { next_ip };
                                Ok(())
                            }
                            oprs => Err(TooManyOperands {
                                provided: oprs.len() as u32,
                                desired: 2,
                            }),
                        },
                        Bdnsal => match &inst.operands[..] {
                            [_] => Err(ToFewOperands {
                                provided: 1,
                                desired: 2,
                            }),
                            [d, a] => {
                                let (device, _channel) = d.get_device_id(self)?;
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
                        Brdns => match &inst.operands[..] {
                            [_] => Err(ToFewOperands {
                                provided: 1,
                                desired: 2,
                            }),
                            [d, a] => {
                                let (device, _channel) = d.get_device_id(self)?;
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
                        Bnan => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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
                        Brnan => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
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

                        J => match &inst.operands[..] {
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
                        Jal => match &inst.operands[..] {
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
                        Jr => match &inst.operands[..] {
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

                        Seq => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                            [reg, a, b] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
                                };
                                let a = a.get_value(self)?;
                                let b = b.get_value(self)?;
                                self.set_register(
                                    indirection,
                                    target,
                                    if a == b { 1.0 } else { 0.0 },
                                )?;
                                Ok(())
                            }
                            oprs => Err(TooManyOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                        },
                        Seqz => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                            [reg, a] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
                                };
                                let a = a.get_value(self)?;
                                self.set_register(
                                    indirection,
                                    target,
                                    if a == 0.0 { 1.0 } else { 0.0 },
                                )?;
                                Ok(())
                            }
                            oprs => Err(TooManyOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                        },
                        Sne => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                            [reg, a, b] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
                                };
                                let a = a.get_value(self)?;
                                let b = b.get_value(self)?;
                                self.set_register(
                                    indirection,
                                    target,
                                    if a != b { 1.0 } else { 0.0 },
                                )?;
                                Ok(())
                            }
                            oprs => Err(TooManyOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                        },
                        Snez => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                            [reg, a] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
                                };
                                let a = a.get_value(self)?;
                                self.set_register(
                                    indirection,
                                    target,
                                    if a != 0.0 { 1.0 } else { 0.0 },
                                )?;
                                Ok(())
                            }
                            oprs => Err(TooManyOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                        },
                        Slt => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                            [reg, a, b] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
                                };
                                let a = a.get_value(self)?;
                                let b = b.get_value(self)?;
                                self.set_register(
                                    indirection,
                                    target,
                                    if a < b { 1.0 } else { 0.0 },
                                )?;
                                Ok(())
                            }
                            oprs => Err(TooManyOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                        },
                        Sltz => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                            [reg, a] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
                                };
                                let a = a.get_value(self)?;
                                self.set_register(
                                    indirection,
                                    target,
                                    if a < 0.0 { 1.0 } else { 0.0 },
                                )?;
                                Ok(())
                            }
                            oprs => Err(TooManyOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                        },
                        Sle => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                            [reg, a, b] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
                                };
                                let a = a.get_value(self)?;
                                let b = b.get_value(self)?;
                                self.set_register(
                                    indirection,
                                    target,
                                    if a <= b { 1.0 } else { 0.0 },
                                )?;
                                Ok(())
                            }
                            oprs => Err(TooManyOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                        },
                        Slez => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                            [reg, a] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
                                };
                                let a = a.get_value(self)?;
                                self.set_register(
                                    indirection,
                                    target,
                                    if a <= 0.0 { 1.0 } else { 0.0 },
                                )?;
                                Ok(())
                            }
                            oprs => Err(TooManyOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                        },
                        Sgt => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                            [reg, a, b] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
                                };
                                let a = a.get_value(self)?;
                                let b = b.get_value(self)?;
                                self.set_register(
                                    indirection,
                                    target,
                                    if a > b { 1.0 } else { 0.0 },
                                )?;
                                Ok(())
                            }
                            oprs => Err(TooManyOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                        },
                        Sgtz => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                            [reg, a] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
                                };
                                let a = a.get_value(self)?;
                                self.set_register(
                                    indirection,
                                    target,
                                    if a > 0.0 { 1.0 } else { 0.0 },
                                )?;
                                Ok(())
                            }
                            oprs => Err(TooManyOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                        },
                        Sge => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                            [reg, a, b] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
                                };
                                let a = a.get_value(self)?;
                                let b = b.get_value(self)?;
                                self.set_register(
                                    indirection,
                                    target,
                                    if a >= b { 1.0 } else { 0.0 },
                                )?;
                                Ok(())
                            }
                            oprs => Err(TooManyOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                        },
                        Sgez => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                            [reg, a] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
                                };
                                let a = a.get_value(self)?;
                                self.set_register(
                                    indirection,
                                    target,
                                    if a >= 0.0 { 1.0 } else { 0.0 },
                                )?;
                                Ok(())
                            }
                            oprs => Err(TooManyOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                        },
                        Sap => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                            [reg, a, b, c] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
                                };
                                let a = a.get_value(self)?;
                                let b = b.get_value(self)?;
                                let c = c.get_value(self)?;
                                self.set_register(
                                    indirection,
                                    target,
                                    if f64::abs(a - b)
                                        <= f64::max(
                                            c * f64::max(a.abs(), b.abs()),
                                            f64::EPSILON * 8.0,
                                        )
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
                        Sapz => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                            [reg, a, b] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
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
                        Sna => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 4,
                            }),
                            [reg, a, b, c] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
                                };
                                let a = a.get_value(self)?;
                                let b = b.get_value(self)?;
                                let c = c.get_value(self)?;
                                self.set_register(
                                    indirection,
                                    target,
                                    if f64::abs(a - b)
                                        > f64::max(
                                            c * f64::max(a.abs(), b.abs()),
                                            f64::EPSILON * 8.0,
                                        )
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
                        Snaz => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 3,
                            }),
                            [reg, a, b] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
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
                        Sdse => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 2,
                            }),
                            [reg, device] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
                                };
                                let (device, _channel) = device.get_device_id(self)?;
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
                        Sdns => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 2,
                            }),
                            [reg, device] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
                                };
                                let (device, _channel) = device.get_device_id(self)?;
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
                        Snan => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 2,
                            }),
                            [reg, a] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
                                };
                                let a = a.get_value(self)?;
                                self.set_register(
                                    indirection,
                                    target,
                                    if a.is_nan() { 1.0 } else { 0.0 },
                                )?;
                                Ok(())
                            }
                            oprs => Err(TooManyOperands {
                                provided: oprs.len() as u32,
                                desired: 2,
                            }),
                        },
                        Snanz => match &inst.operands[..] {
                            oprs @ [_] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 2,
                            }),
                            [reg, a] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
                                };
                                let a = a.get_value(self)?;
                                self.set_register(
                                    indirection,
                                    target,
                                    if a.is_nan() { 0.0 } else { 1.0 },
                                )?;
                                Ok(())
                            }
                            oprs => Err(TooManyOperands {
                                provided: oprs.len() as u32,
                                desired: 2,
                            }),
                        },

                        Select => match &inst.operands[..] {
                            oprs @ [_] | oprs @ [_, _] | oprs @ [_, _, _] => Err(ToFewOperands {
                                provided: oprs.len() as u32,
                                desired: 4,
                            }),
                            [reg, a, b, c] => {
                                let &Operand::RegisterSpec {
                                    indirection,
                                    target,
                                } = reg
                                else {
                                    break 'inst Err(OperandNotRegister);
                                };
                                let a = a.get_value(self)?;
                                let b = b.get_value(self)?;
                                let c = c.get_value(self)?;
                                self.set_register(
                                    indirection,
                                    target,
                                    if a != 0.0 { b } else { c },
                                )?;
                                Ok(())
                            }
                            oprs => Err(TooManyOperands {
                                provided: oprs.len() as u32,
                                desired: 4,
                            }),
                        },


                        Push => Ok(()),
                        Pop => Ok(()),
                        Poke => Ok(()),
                        Peek => Ok(()),
                        Get => Ok(()),
                        Getd => Ok(()),
                        Put => Ok(()),
                        Putd => Ok(()),

                        Sll => Ok(()),
                        Srl => Ok(()),
                        Sla => Ok(()),
                        Sra => Ok(()),

                        And => Ok(()),
                        Or => Ok(()),
                        Xor => Ok(()),
                        Nor => Ok(()),
                        Not => Ok(()),

                        Add => Ok(()),
                        Sub => Ok(()),
                        Mul => Ok(()),
                        Div => Ok(()),
                        Mod => Ok(()),
                        Exp => Ok(()),
                        Log => Ok(()),
                        Sqrt => Ok(()),

                        Max => Ok(()),
                        Min => Ok(()),
                        Ceil => Ok(()),
                        Floor => Ok(()),
                        Abs => Ok(()),
                        Round => Ok(()),
                        Trunc => Ok(()),

                        Rand => Ok(()),

                        Sin => Ok(()),
                        Cos => Ok(()),
                        Tan => Ok(()),
                        Asin => Ok(()),
                        Acos => Ok(()),
                        Atan => Ok(()),
                        Atan2 => Ok(()),

                        S => Ok(()),
                        Sd => Ok(()),
                        Ss => Ok(()),
                        Sb => Ok(()),
                        Sbs => Ok(()),
                        Sbn => Ok(()),

                        L => Ok(()),
                        Ld => Ok(()),
                        Ls => Ok(()),
                        Lr => Ok(()),
                        Lb => Ok(()),
                        Lbn => Ok(()),
                        Lbns => Ok(()),
                        Lbs => Ok(()),

                        Hcf => Ok(()),
                    }
                }
            }
        } else {
            Ok(())
        };
        self.ip = next_ip;
        result
    }
}
