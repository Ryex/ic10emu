use std::collections::HashMap;

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

    pub fn get_ident_value(&self, ident: &String) -> Result<f64, ICError> {
        if let Some(operand) = self.aliases.get(ident) {
            operand.get_value(self)
        } else {
            Err(ICError::UnknownIdentifier(ident.clone()))
        }
    }

    /// processes one line of the contained program
    pub fn step(&mut self, housing: &mut crate::Device, vm: &mut crate::VM) -> Result<(), ICError> {
        use grammar::InstructionOp::*;
        use grammar::*;
        use ICError::*;

        let line = self.program.get_line(self.ip)?;
        let result: Result<(), ICError> = if let Some(code) = &line.code {
            match code {
                grammar::Code::Label(label) => {
                    self.labels.insert(label.id.name.clone(), self.ip);
                    Ok(())
                }
                grammar::Code::Instruction(inst) => 'inst: {
                    match inst.instruction {
                        Nop => Ok(()),
                        Yield => {
                            self.should_yield = true;
                            Ok(())
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
                        Breqz => Ok(()),
                        Sb => Ok(()),
                        Bgtz => Ok(()),
                        Beqz => Ok(()),
                        Blez => Ok(()),
                        Bapzal => Ok(()),
                        Tan => Ok(()),
                        Bapz => Ok(()),
                        Ble => Ok(()),
                        Bnez => Ok(()),
                        J => Ok(()),
                        Snez => Ok(()),
                        Snanz => Ok(()),
                        Xor => Ok(()),
                        Sdse => Ok(()),
                        Srl => Ok(()),
                        Brna => Ok(()),
                        Sbs => Ok(()),
                        Bnezal => Ok(()),
                        Sge => Ok(()),
                        Bgtal => Ok(()),
                        Nor => Ok(()),
                        Bneal => Ok(()),
                        Label => Ok(()),
                        Blezal => Ok(()),
                        Brap => Ok(()),
                        Log => Ok(()),
                        Sgez => Ok(()),
                        Sin => Ok(()),
                        Seq => Ok(()),
                        Putd => Ok(()),
                        Slt => Ok(()),
                        Snan => Ok(()),
                        Beqzal => Ok(()),
                        Bltal => Ok(()),
                        Lbns => Ok(()),
                        Poke => Ok(()),
                        Brlez => Ok(()),
                        Bdse => Ok(()),
                        Sleep => Ok(()),
                        Lb => Ok(()),
                        Lr => Ok(()),
                        Slez => Ok(()),
                        Beqal => Ok(()),
                        Sdns => Ok(()),
                        Blt => Ok(()),
                        Add => Ok(()),
                        Bdseal => Ok(()),
                        Beq => Ok(()),
                        Atan => Ok(()),
                        Bgtzal => Ok(()),
                        Mul => Ok(()),
                        Sra => Ok(()),
                        Bdns => Ok(()),
                        Peek => Ok(()),
                        Sne => Ok(()),
                        Jr => Ok(()),
                        Sgt => Ok(()),
                        Brgt => Ok(()),
                        Sd => Ok(()),
                        Brapz => Ok(()),
                        Breq => Ok(()),
                        Or => Ok(()),
                        Bdnsal => Ok(()),
                        Bna => Ok(()),
                        Sbn => Ok(()),
                        Mod => Ok(()),
                        Asin => Ok(()),
                        Atan2 => Ok(()),
                        Bgeal => Ok(()),
                        Put => Ok(()),
                        Bgez => Ok(()),
                        Sapz => Ok(()),
                        Bleal => Ok(()),
                        Bltz => Ok(()),
                        Brlt => Ok(()),
                        Brltz => Ok(()),
                        Rand => Ok(()),
                        Trunc => Ok(()),
                        Lbn => Ok(()),
                        Bnazal => Ok(()),
                        Bne => Ok(()),
                        Sltz => Ok(()),
                        Brge => Ok(()),
                        Div => Ok(()),
                        Max => Ok(()),
                        Round => Ok(()),
                        Sgtz => Ok(()),
                        Brdns => Ok(()),
                        Bapal => Ok(()),
                        Lbs => Ok(()),
                        Move => Ok(()),
                        Sla => Ok(()),
                        And => Ok(()),
                        Pop => Ok(()),
                        Brdse => Ok(()),
                        Sll => Ok(()),
                        Bap => Ok(()),
                        Push => Ok(()),
                        Seqz => Ok(()),
                        Sub => Ok(()),
                        Select => Ok(()),
                        Bge => Ok(()),
                        Abs => Ok(()),
                        Brle => Ok(()),
                        Get => Ok(()),
                        Brnez => Ok(()),
                        Snaz => Ok(()),
                        Bnaal => Ok(()),
                        Ss => Ok(()),
                        Exp => Ok(()),
                        Bgezal => Ok(()),
                        Bgt => Ok(()),
                        Brnaz => Ok(()),
                        Brgtz => Ok(()),
                        Brnan => Ok(()),
                        Bltzal => Ok(()),
                        Floor => Ok(()),
                        Ceil => Ok(()),
                        Jal => Ok(()),
                        L => Ok(()),
                        Ld => Ok(()),
                        Bnaz => Ok(()),
                        Sle => Ok(()),
                        Sna => Ok(()),
                        Brne => Ok(()),
                        Acos => Ok(()),
                        Bnan => Ok(()),
                        Cos => Ok(()),
                        Getd => Ok(()),
                        Min => Ok(()),
                        Brgez => Ok(()),
                        S => Ok(()),
                        Sap => Ok(()),
                        Sqrt => Ok(()),
                        Ls => Ok(()),
                        Not => Ok(()),
                        Hcf => Ok(()),
                    }
                }
            }
        } else {
            Ok(())
        };
        // let ip = housing.i
        self.ip += 1;
        result
    }
}
