use std::convert::AsRef;
use crate::grammar;

// include files built from lang def
include!(concat!(env!("OUT_DIR"), "/logictypes.rs"));
include!(concat!(env!("OUT_DIR"), "/modes.rs"));
include!(concat!(env!("OUT_DIR"), "/constants.rs"));
include!(concat!(env!("OUT_DIR"), "/enums.rs"));
include!(concat!(env!("OUT_DIR"), "/instructions.rs"));

#[derive(Debug)]
pub enum Device {
    Db,
    Numbered(u8),
    Indirect { indirection: u32, target: u8 },
}

impl From<grammar::rich_types::Device> for Device {
    fn from(value: grammar::rich_types::Device) -> Self {
        match value {
            grammar::rich_types::Device::Db => Self::Db,
            grammar::rich_types::Device::Numbered(n) => Self::Numbered(n),
            grammar::rich_types::Device::Indirect(r) => Self::Indirect {
                indirection: r.indirection,
                target: r.target,
            },
        }
    }
}

#[derive(Debug)]
pub enum Operand {
    Register {
        indirection: u32,
        target: u8,
    },
    DeviceSpec {
        device: Device,
        channel: Option<u32>,
    },
    Number(f64),
    Identifier(String),
}

impl TryFrom<grammar::ic10::Operand> for Operand {
    type Error = String;
    fn try_from(value: grammar::ic10::Operand) -> Result<Self, Self::Error> {
        match value {
            grammar::ic10::Operand::RegisterSpec(r) => Ok(Self::Register {
                indirection: r.indirection,
                target: r.target,
            }),
            grammar::ic10::Operand::DeviceSpec(ds) => Ok(Self::DeviceSpec {
                device: ds.device.into(),
                channel: ds.channel,
            }),
            grammar::ic10::Operand::LogicType(t) => Ok(Self::Number(
                (*LOGIC_TYPE_LOOKUP
                    .get(t.as_ref())
                    .ok_or(format!("Unknown Logic Type {}", t.as_ref()))?)
                .into(),
            )),
            grammar::ic10::Operand::Identifier(id) => Ok(Self::Identifier(id.name)),
            grammar::ic10::Operand::Number(n) => match n {
                grammar::ic10::Number::Float(f) => Ok(Self::Number(f)),
                grammar::ic10::Number::Binary(_, f) => Ok(Self::Number(f)),
                grammar::ic10::Number::Hexadecimal(_, f) => Ok(Self::Number(f)),
                grammar::ic10::Number::Constant(c) => Ok(Self::Number(
                    *CONSTANTS_LOOKUP
                        .get(c.as_ref())
                        .ok_or(format!("Unknown constant {}", c.as_ref()))?,
                )),
                grammar::ic10::Number::Enum(e) => Ok(Self::Number(
                    (*ENUM_LOOKUP
                        .get(e.as_ref())
                        .ok_or(format!("Unknown enum {}", e.as_ref()))?)
                    .into(),
                )),
                grammar::ic10::Number::String(s) => Ok(Self::Number(s.string.hash.into())),
            },
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub instruction: InstructionOp,
    pub operands: Vec<Operand>,
}

#[derive(Debug)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

impl Default for Program {
    fn default() -> Self {
        Program::new()
    }
}

impl Program {
    pub fn new() -> Self { Program { instructions: Vec::new() }}
    pub fn try_from_code(input: &str) -> Result<Self, String> {
        let mut code = input.to_string();
        if let Some((i, _)) = code.char_indices().rev().nth(0) {
            let last_char = &code[i..];
            if last_char != "\r" && last_char != "\n" {code.push('\n');}
        }
        let parse_tree = grammar::ic10::parse(&code);


        Ok(Program { instructions: vec![] })
    }
}
