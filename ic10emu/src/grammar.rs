use crate::interpreter::{self, ICError};
use crate::tokens::{SplitConsecutiveIndicesExt, SplitConsecutiveWithIndices};
use itertools::Itertools;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;
use strum::EnumProperty;

pub mod generated {
    use super::ParseError;
    use crate::interpreter::ICError;
    use serde::{Deserialize, Serialize};
    use std::str::FromStr;
    use strum::AsRefStr;
    use strum::Display;
    use strum::EnumIter;
    use strum::EnumProperty;
    use strum::EnumString;
    use strum::IntoEnumIterator;

    include!(concat!(env!("OUT_DIR"), "/instructions.rs"));
    include!(concat!(env!("OUT_DIR"), "/logictypes.rs"));
    include!(concat!(env!("OUT_DIR"), "/modes.rs"));
    include!(concat!(env!("OUT_DIR"), "/constants.rs"));
    include!(concat!(env!("OUT_DIR"), "/enums.rs"));

    impl TryFrom<f64> for LogicType {
        type Error = ICError;
        fn try_from(value: f64) -> Result<Self, <LogicType as TryFrom<f64>>::Error> {
            if let Some(lt) = LogicType::iter().find(|lt| {
                lt.get_str("value")
                    .map(|val| val.parse::<u16>().unwrap() as f64 == value)
                    .unwrap_or(false)
            }) {
                Ok(lt)
            } else {
                Err(crate::interpreter::ICError::UnknownLogicType(value))
            }
        }
    }

    impl TryFrom<f64> for SlotLogicType {
        type Error = ICError;
        fn try_from(value: f64) -> Result<Self, <SlotLogicType as TryFrom<f64>>::Error> {
            if let Some(slt) = SlotLogicType::iter().find(|lt| {
                lt.get_str("value")
                    .map(|val| val.parse::<u8>().unwrap() as f64 == value)
                    .unwrap_or(false)
            }) {
                Ok(slt)
            } else {
                Err(crate::interpreter::ICError::UnknownSlotLogicType(value))
            }
        }
    }

    impl TryFrom<f64> for BatchMode {
        type Error = ICError;
        fn try_from(value: f64) -> Result<Self, <BatchMode as TryFrom<f64>>::Error> {
            if let Some(bm) = BatchMode::iter().find(|lt| {
                lt.get_str("value")
                    .map(|val| val.parse::<u8>().unwrap() as f64 == value)
                    .unwrap_or(false)
            }) {
                Ok(bm)
            } else {
                Err(crate::interpreter::ICError::UnknownBatchMode(value))
            }
        }
    }

    impl TryFrom<f64> for ReagentMode {
        type Error = ICError;
        fn try_from(value: f64) -> Result<Self, <ReagentMode as TryFrom<f64>>::Error> {
            if let Some(rm) = ReagentMode::iter().find(|lt| {
                lt.get_str("value")
                    .map(|val| val.parse::<u8>().unwrap() as f64 == value)
                    .unwrap_or(false)
            }) {
                Ok(rm)
            } else {
                Err(crate::interpreter::ICError::UnknownReagentMode(value))
            }
        }
    }
}

pub use generated::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ParseError {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub msg: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at line {} {}:{}",
            self.msg, self.line, self.start, self.end
        )
    }
}

impl Error for ParseError {}

impl ParseError {
    /// Offset the ParseError in it's line, adding the passed values to it's `start` and `end`
    #[must_use]
    pub fn offset(self, offset: usize) -> Self {
        ParseError {
            start: self.start + offset,
            end: self.end + offset,
            ..self
        }
    }

    /// Offset the ParseError line, adding the passed value to it's `line`
    #[must_use]
    pub fn offset_line(self, offset: usize) -> Self {
        ParseError {
            line: self.line + offset,
            start: self.start,
            ..self
        }
    }

    /// Mark the parse error as extending 'length' bytes from `start`
    #[must_use]
    pub fn span(self, length: usize) -> Self {
        ParseError {
            start: self.start,
            end: self.start + length,
            ..self
        }
    }
}

pub fn parse(code: &str) -> Result<Vec<Line>, ParseError> {
    code.lines()
        .enumerate()
        .map(|(n, l)| l.parse::<Line>().map_err(|e| e.offset_line(n)))
        .collect()
}

/// Like `parse` but can return Code::Invalid for some lines
pub fn parse_with_invlaid(code: &str) -> Vec<Line> {
    code.lines()
        .enumerate()
        .map(|(n, l)| Line::from_str_with_invalid(n, l))
        .collect()
}

#[derive(PartialEq, Debug)]
pub struct Line {
    pub code: Option<Code>,
    pub comment: Option<Comment>,
}

impl FromStr for Line {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, '#');
        let code = parts
            .next()
            .and_then(|s| {
                let s = s.trim_end();
                if s.is_empty() {
                    None
                } else {
                    Some(s.parse::<Code>())
                }
            })
            .transpose()?;
        let comment = parts
            .next()
            .map(|s| s.parse())
            .transpose()
            .expect("infallible");
        Ok(Line { code, comment })
    }
}

impl Line {
    fn from_str_with_invalid(line: usize, s: &str) -> Self {
        let mut parts = s.splitn(2, '#');
        let code_part = parts
            .next()
            .and_then(|s| {
                let s = s.trim_end();
                if s.is_empty() {
                    None
                } else {
                    Some(s.parse::<Code>().map_err(|e| e.offset_line(line)))
                }
            })
            .transpose();
        let code = match code_part {
            Ok(c) => c,
            Err(e) => Some(Code::Invalid(e)),
        };
        let comment = parts
            .next()
            .map(|s| s.parse())
            .transpose()
            .expect("infallible");
        Line { code, comment }
    }
}

#[derive(PartialEq, Debug)]
pub enum Code {
    Instruction(Instruction),
    Label(Label),
    Invalid(ParseError),
}

impl FromStr for Code {
    type Err = ParseError;

    /// Parse a non empty Code line from a &str with no comment in it
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_end();
        if let Some((index, ':')) = s.chars().enumerate().last() {
            Ok(Code::Label(
                s.parse::<Label>().map_err(|e| e.offset(index))?,
            ))
        } else {
            Ok(Code::Instruction(s.parse()?))
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Comment {
    pub comment: String,
}

impl FromStr for Comment {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Comment {
            comment: s.to_owned(),
        })
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub instruction: InstructionOp,
    pub operands: Vec<Operand>,
}

impl FromStr for Instruction {
    type Err = ParseError;
    /// parse a non-empty string for  an instruction and it's operands
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens_iter = s.split_consecutive_with_indices(&[' ', '\t'][..]);
        let instruction: InstructionOp = {
            if let Some((index, token)) = tokens_iter.next() {
                token
                    .parse::<InstructionOp>()
                    .map_err(|e| e.offset(index).span(token.len()))
            } else {
                Err(ParseError {
                    line: 0,
                    start: 0,
                    end: 0,
                    msg: "Missing instruction".to_owned(),
                })
            }
        }?;

        let operands = get_operand_tokens(s, tokens_iter)
            .iter()
            .map(|(index, token)| {
                token
                    .parse::<Operand>()
                    .map_err(|e| e.offset(*index).span(token.len()))
            })
            .try_collect()?;
        Ok(Instruction {
            instruction,
            operands,
        })
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i = self.instruction.to_string().to_lowercase();
        write!(f, "{}", i)?;
        for operand in &self.operands {
            write!(f, " {}", operand)?;
        }
        Ok(())
    }
}

fn get_operand_tokens<'a>(
    s: &'a str,
    tokens_iter: SplitConsecutiveWithIndices<'a>,
) -> Vec<(usize, &'a str)> {
    let mut operand_tokens = Vec::with_capacity(8);
    let mut string_start = None;
    for (index, token) in tokens_iter {
        if token.starts_with("HASH(\"") {
            string_start = Some(index);
        }
        if let Some(start) = string_start {
            if token.ends_with("\")") {
                operand_tokens.push((start, &s[start..(index + token.len())]));
                string_start = None;
            }
        } else {
            operand_tokens.push((index, token));
        }
    }
    operand_tokens
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Device {
    Db,
    Numbered(u32),
    Indirect { indirection: u32, target: u32 },
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct RegisterSpec {
    pub indirection: u32,
    pub target: u32,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSpec {
    pub device: Device,
    pub connection: Option<usize>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Operand {
    RegisterSpec(RegisterSpec),
    DeviceSpec(DeviceSpec),
    Number(Number),
    Type {
        logic_type: Option<LogicType>,
        slot_logic_type: Option<SlotLogicType>,
        batch_mode: Option<BatchMode>,
        reagent_mode: Option<ReagentMode>,
        identifier: Identifier,
    },
    Identifier(Identifier),
}

impl Operand {
    pub fn as_value(
        &self,
        ic: &interpreter::IC,
        inst: InstructionOp,
        index: u32,
    ) -> Result<f64, interpreter::ICError> {
        match self.translate_alias(ic) {
            Operand::RegisterSpec(RegisterSpec {
                indirection,
                target,
            }) => ic.get_register(indirection, target),
            Operand::Number(num) => Ok(num.value()),
            Operand::Type {
                logic_type,
                slot_logic_type,
                batch_mode,
                reagent_mode,
                identifier: _,
            } => {
                if let Some(lt) = logic_type {
                    Ok(lt
                        .get_str("value")
                        .ok_or_else(|| ICError::NoGeneratedValue(lt.to_string()))?
                        .parse::<u16>()
                        .map_err(|_| {
                            ICError::BadGeneratedValueParse(lt.to_string(), "u16".to_owned())
                        })? as f64)
                } else if let Some(slt) = slot_logic_type {
                    Ok(slt
                        .get_str("value")
                        .ok_or_else(|| ICError::NoGeneratedValue(slt.to_string()))?
                        .parse::<u8>()
                        .map_err(|_| {
                            ICError::BadGeneratedValueParse(slt.to_string(), "u8".to_owned())
                        })? as f64)
                } else if let Some(bm) = batch_mode {
                    Ok(bm
                        .get_str("value")
                        .ok_or_else(|| ICError::NoGeneratedValue(bm.to_string()))?
                        .parse::<u8>()
                        .map_err(|_| {
                            ICError::BadGeneratedValueParse(bm.to_string(), "u8".to_owned())
                        })? as f64)
                } else if let Some(rm) = reagent_mode {
                    Ok(rm
                        .get_str("value")
                        .ok_or_else(|| ICError::NoGeneratedValue(rm.to_string()))?
                        .parse::<u8>()
                        .map_err(|_| {
                            ICError::BadGeneratedValueParse(rm.to_string(), "u8".to_owned())
                        })? as f64)
                } else {
                    Err(interpreter::ICError::TypeValueNotKnown)
                }
            }
            Operand::Identifier(id) => {
                Err(interpreter::ICError::UnknownIdentifier(id.name.to_string()))
            }
            Operand::DeviceSpec { .. } => Err(interpreter::ICError::IncorrectOperandType {
                inst,
                index,
                desired: "Value".to_owned(),
            }),
        }
    }

    pub fn as_value_i64(
        &self,
        ic: &interpreter::IC,
        signed: bool,
        inst: InstructionOp,
        index: u32,
    ) -> Result<i64, interpreter::ICError> {
        match self {
            Self::Number(num) => Ok(num.value_i64(signed)),
            _ => {
                let val = self.as_value(ic, inst, index)?;
                if val < -9.223_372_036_854_776E18 {
                    Err(interpreter::ICError::ShiftUnderflowI64)
                } else if val <= 9.223_372_036_854_776E18 {
                    Ok(interpreter::f64_to_i64(val, signed))
                } else {
                    Err(interpreter::ICError::ShiftOverflowI64)
                }
            }
        }
    }
    pub fn as_value_i32(
        &self,
        ic: &interpreter::IC,
        signed: bool,
        inst: InstructionOp,
        index: u32,
    ) -> Result<i32, interpreter::ICError> {
        match self {
            Self::Number(num) => Ok(num.value_i64(signed) as i32),
            _ => {
                let val = self.as_value(ic, inst, index)?;
                if val < -2147483648.0 {
                    Err(interpreter::ICError::ShiftUnderflowI32)
                } else if val <= 2147483647.0 {
                    Ok(val as i32)
                } else {
                    Err(interpreter::ICError::ShiftOverflowI32)
                }
            }
        }
    }

    pub fn as_register(
        &self,
        ic: &interpreter::IC,
        inst: InstructionOp,
        index: u32,
    ) -> Result<RegisterSpec, interpreter::ICError> {
        match self.translate_alias(ic) {
            Operand::RegisterSpec(reg) => Ok(reg),
            Operand::Identifier(id) => {
                Err(interpreter::ICError::UnknownIdentifier(id.name.to_string()))
            }
            _ => Err(interpreter::ICError::IncorrectOperandType {
                inst,
                index,
                desired: "Register".to_owned(),
            }),
        }
    }

    pub fn as_device(
        &self,
        ic: &interpreter::IC,
        inst: InstructionOp,
        index: u32,
    ) -> Result<(Option<u32>, Option<usize>), interpreter::ICError> {
        match self.translate_alias(ic) {
            Operand::DeviceSpec(DeviceSpec { device, connection }) => match device {
                Device::Db => Ok((Some(ic.device), connection)),
                Device::Numbered(p) => {
                    let dp = ic
                        .pins
                        .borrow()
                        .get(p as usize)
                        .ok_or(interpreter::ICError::DeviceIndexOutOfRange(p as f64))
                        .copied()?;
                    Ok((dp, connection))
                }
                Device::Indirect {
                    indirection,
                    target,
                } => {
                    let val = ic.get_register(indirection, target)?;
                    let dp = ic
                        .pins
                        .borrow()
                        .get(val as usize)
                        .ok_or(interpreter::ICError::DeviceIndexOutOfRange(val))
                        .copied()?;
                    Ok((dp, connection))
                }
            },
            Operand::Identifier(id) => {
                Err(interpreter::ICError::UnknownIdentifier(id.name.to_string()))
            }
            _ => Err(interpreter::ICError::IncorrectOperandType {
                inst,
                index,
                desired: "Value".to_owned(),
            }),
        }
    }

    pub fn as_logic_type(
        &self,
        ic: &interpreter::IC,
        inst: InstructionOp,
        index: u32,
    ) -> Result<LogicType, ICError> {
        match &self {
            Operand::Type {
                logic_type: Some(lt),
                ..
            } => Ok(*lt),
            _ => LogicType::try_from(self.as_value(ic, inst, index)?),
        }
    }

    pub fn as_slot_logic_type(
        &self,
        ic: &interpreter::IC,
        inst: InstructionOp,
        index: u32,
    ) -> Result<SlotLogicType, ICError> {
        match &self {
            Operand::Type {
                slot_logic_type: Some(slt),
                ..
            } => Ok(*slt),
            _ => SlotLogicType::try_from(self.as_value(ic, inst, index)?),
        }
    }

    pub fn as_batch_mode(
        &self,
        ic: &interpreter::IC,
        inst: InstructionOp,
        index: u32,
    ) -> Result<BatchMode, ICError> {
        match &self {
            Operand::Type {
                batch_mode: Some(bm),
                ..
            } => Ok(*bm),
            _ => BatchMode::try_from(self.as_value(ic, inst, index)?),
        }
    }

    pub fn as_reagent_mode(
        &self,
        ic: &interpreter::IC,
        inst: InstructionOp,
        index: u32,
    ) -> Result<ReagentMode, ICError> {
        match &self {
            Operand::Type {
                reagent_mode: Some(rm),
                ..
            } => Ok(*rm),
            _ => ReagentMode::try_from(self.as_value(ic, inst, index)?),
        }
    }

    pub fn translate_alias(&self, ic: &interpreter::IC) -> Self {
        match &self {
            Operand::Identifier(id) | Operand::Type { identifier: id, .. } => {
                if let Some(alias) = ic.aliases.borrow().get(&id.name) {
                    alias.clone()
                } else if let Some(define) = ic.defines.borrow().get(&id.name) {
                    Operand::Number(Number::Float(*define))
                } else if let Some(label) = ic.program.borrow().labels.get(&id.name) {
                    Operand::Number(Number::Float(*label as f64))
                } else {
                    self.clone()
                }
            }
            _ => self.clone(),
        }
    }
}

impl FromStr for Operand {
    type Err = ParseError;
    /// Parse a str containing an single instruction operand
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<_>>();
        match &chars[..] {
            ['s', 'p'] => Ok(Operand::RegisterSpec(RegisterSpec {
                indirection: 0,
                target: 16,
            })),
            ['r', 'a'] => Ok(Operand::RegisterSpec(RegisterSpec {
                indirection: 0,
                target: 17,
            })),
            ['r', rest @ ..] => {
                let mut rest_iter = rest.iter();
                let indirection = rest_iter.take_while_ref(|c| *c == &'r').count();
                let target_str = rest_iter
                    .take_while_ref(|c| c.is_ascii_digit())
                    .collect::<String>();
                if !target_str.is_empty() {
                    let target = target_str.parse::<u32>().ok();
                    if let Some(target) = target {
                        if rest_iter.next().is_none() {
                            return Ok(Operand::RegisterSpec(RegisterSpec {
                                indirection: indirection as u32,
                                target,
                            }));
                        } else {
                            return Err(ParseError {
                                line: 0,
                                start: 0,
                                end: 0,
                                msg: "Invalid register specifier".to_owned(),
                            });
                        }
                    }
                }
                Ok(Operand::Identifier(s.parse::<Identifier>()?))
            }
            ['d', rest @ ..] => match rest {
                ['b'] => Ok(Operand::DeviceSpec(DeviceSpec {
                    device: Device::Db,
                    connection: None,
                })),
                ['b', ':', chan @ ..] => {
                    if chan.iter().all(|c| c.is_ascii_digit()) {
                        Ok(Operand::DeviceSpec(DeviceSpec {
                            device: Device::Db,
                            connection: Some(String::from_iter(chan).parse().unwrap()),
                        }))
                    } else {
                        Err(ParseError {
                            line: 0,
                            start: 3,
                            end: 3,
                            msg: "Invalid device connection specifier".to_owned(),
                        })
                    }
                }
                ['r', rest @ ..] => {
                    let mut rest_iter = rest.iter().peekable();
                    let indirection = rest_iter.take_while_ref(|c| *c == &'r').count();
                    let target_str = rest_iter
                        .take_while_ref(|c| c.is_ascii_digit())
                        .collect::<String>();
                    if target_str.is_empty() {
                        Ok(Operand::Identifier(s.parse::<Identifier>()?))
                    } else {
                        let target = target_str.parse::<u32>().ok();
                        if let Some(target) = target {
                            let connection = {
                                if rest_iter.peek() == Some(&&':') {
                                    // take off ':'
                                    rest_iter.next();
                                    let connection_str = rest_iter
                                        .take_while_ref(|c| c.is_ascii_digit())
                                        .collect::<String>();
                                    let connection = connection_str.parse::<usize>().unwrap();
                                    if rest_iter.next().is_none() {
                                        Ok(Some(connection))
                                    } else {
                                        let start = 2
                                            + indirection
                                            + target_str.len()
                                            + 1
                                            + connection_str.len();
                                        Err(ParseError {
                                            line: 0,
                                            start,
                                            end: start,
                                            msg: "Invalid device connection specifier".to_owned(),
                                        })
                                    }
                                } else {
                                    Ok(None)
                                }
                            }?;
                            if rest_iter.next().is_none() {
                                Ok(Operand::DeviceSpec(DeviceSpec {
                                    device: Device::Indirect {
                                        indirection: indirection as u32,
                                        target,
                                    },
                                    connection,
                                }))
                            } else {
                                Err(ParseError {
                                    line: 0,
                                    start: 0,
                                    end: 0,
                                    msg: "Invalid register specifier".to_owned(),
                                })
                            }
                        } else {
                            Ok(Operand::Identifier(s.parse::<Identifier>()?))
                        }
                    }
                }
                rest => {
                    let mut rest_iter = rest.iter().peekable();
                    let target_str = rest_iter
                        .take_while_ref(|c| c.is_ascii_digit())
                        .collect::<String>();
                    let target = target_str.parse::<u32>().ok();
                    let connection = {
                        if rest_iter.peek() == Some(&&':') {
                            // take off ':'
                            rest_iter.next();
                            let connection_str = rest_iter
                                .take_while_ref(|c| c.is_ascii_digit())
                                .collect::<String>();
                            let connection = connection_str.parse::<usize>().unwrap();
                            if rest_iter.next().is_none() {
                                Ok(Some(connection))
                            } else {
                                let end = 1 + target_str.len() + 1 + connection_str.len();
                                Err(ParseError {
                                    line: 0,
                                    start: end - connection_str.len(),
                                    end,
                                    msg: "Invalid device connection specifier".to_owned(),
                                })
                            }
                        } else {
                            Ok(None)
                        }
                    }?;
                    if let Some(target) = target {
                        if rest_iter.next().is_none() {
                            Ok(Operand::DeviceSpec(DeviceSpec {
                                device: Device::Numbered(target),
                                connection,
                            }))
                        } else {
                            let end = 1 + target_str.len();
                            Err(ParseError {
                                line: 0,
                                start: 1,
                                end,
                                msg: "Invalid device specifier".to_owned(),
                            })
                        }
                    } else {
                        Ok(Operand::Identifier(s.parse::<Identifier>()?))
                    }
                }
            },
            ['H', 'A', 'S', 'H', '(', '"', hash_str @ .., '"', ')'] => {
                if hash_str.iter().all(|c| c != &'"' && c != &'\n') {
                    Ok(Operand::Number(Number::String(String::from_iter(hash_str))))
                } else {
                    Err(ParseError {
                        line: 0,
                        start: 6,
                        end: hash_str.len(),
                        msg: "Invalid hash string: Can not contain '\"'".to_owned(),
                    })
                }
            }
            ['$', rest @ ..] => {
                let mut rest_iter = rest.iter();
                let num_str = rest_iter
                    .take_while_ref(|c| c.is_ascii_hexdigit())
                    .collect::<String>();
                let num = i64::from_str_radix(&num_str, 16).unwrap();
                if rest_iter.next().is_none() {
                    Ok(Operand::Number(Number::Hexadecimal(num)))
                } else {
                    Err(ParseError {
                        line: 0,
                        start: 0,
                        end: 0,
                        msg: "Invalid Hexadecimal Number".to_owned(),
                    })
                }
            }
            ['%', rest @ ..] => {
                let mut rest_iter = rest.iter();
                let num_str = rest_iter
                    .take_while_ref(|c| c.is_digit(2))
                    .collect::<String>();
                let num = i64::from_str_radix(&num_str, 2).unwrap();
                if rest_iter.next().is_none() {
                    Ok(Operand::Number(Number::Binary(num)))
                } else {
                    Err(ParseError {
                        line: 0,
                        start: 0,
                        end: 0,
                        msg: "Invalid Binary Number".to_owned(),
                    })
                }
            }
            rest => {
                let mut rest_iter = rest.iter().peekable();
                let float_str = if rest_iter.peek() == Some(&&'-') {
                    format!("{}", rest_iter.next().unwrap())
                } else {
                    "".to_owned()
                } + &rest_iter
                    .take_while_ref(|c| c.is_ascii_digit())
                    .collect::<String>();
                if !float_str.is_empty() {
                    if rest_iter.peek() == Some(&&'.') {
                        rest_iter.next();
                        let decimal_str = rest_iter
                            .take_while_ref(|c| c.is_ascii_digit())
                            .collect::<String>();
                        if !decimal_str.is_empty() {
                            let float_str = float_str + "." + &decimal_str;
                            if let Ok(num) = f64::from_str(&float_str) {
                                Ok(Operand::Number(Number::Float(num)))
                            } else {
                                Err(ParseError {
                                    line: 0,
                                    start: 0,
                                    end: 0,
                                    msg: "Invalid Number".to_owned(),
                                })
                            }
                        } else {
                            Err(ParseError {
                                line: 0,
                                start: 0,
                                end: float_str.len(),
                                msg: "Invalid Decimal Number".to_owned(),
                            })
                        }
                    } else if rest_iter.next().is_none() {
                        if let Ok(num) = f64::from_str(&float_str) {
                            Ok(Operand::Number(Number::Float(num)))
                        } else {
                            Err(ParseError {
                                line: 0,
                                start: 0,
                                end: float_str.len(),
                                msg: "Invalid Number".to_owned(),
                            })
                        }
                    } else {
                        Err(ParseError {
                            line: 0,
                            start: 0,
                            end: float_str.len(),
                            msg: "Invalid Integer Number".to_owned(),
                        })
                    }
                } else if let Some(val) = CONSTANTS_LOOKUP.get(s) {
                    Ok(Operand::Number(Number::Constant(*val)))
                } else if let Ok(val) = LogicEnums::from_str(s) {
                    Ok(Operand::Number(Number::Enum(
                        val.get_str("value").unwrap().parse().unwrap(),
                    )))
                } else {
                    let lt = LogicType::from_str(s).ok();
                    let slt = SlotLogicType::from_str(s).ok();
                    let bm = BatchMode::from_str(s).ok();
                    let rm = ReagentMode::from_str(s).ok();
                    let identifier = Identifier::from_str(s)?;
                    if lt.is_some() || slt.is_some() || bm.is_some() || rm.is_some() {
                        Ok(Operand::Type {
                            logic_type: lt,
                            slot_logic_type: slt,
                            batch_mode: bm,
                            reagent_mode: rm,
                            identifier,
                        })
                    } else {
                        Ok(Operand::Identifier(identifier))
                    }
                }
            }
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::RegisterSpec(RegisterSpec {
                indirection,
                target,
            }) => {
                for _ in 0..*indirection {
                    write!(f, "r")?;
                }
                if *indirection == 0 {
                    match target {
                        17 => write!(f, "ra"),
                        16 => write!(f, "sp"),
                        _ => write!(f, "r{}", target),
                    }
                } else {
                    write!(f, "r{}", target)
                }
            }
            Operand::DeviceSpec(DeviceSpec { device, connection }) => {
                match device {
                    Device::Db => write!(f, "db"),
                    Device::Numbered(number) => write!(f, "d{}", number),
                    Device::Indirect {
                        indirection,
                        target,
                    } => {
                        write!(f, "d")?;
                        for _ in 0..=*indirection {
                            write!(f, "r")?;
                        }
                        write!(f, "{}", target)
                    }
                }?;
                if let Some(connection) = connection {
                    write!(f, ":{connection}")?;
                }
                Ok(())
            }
            Operand::Number(number) => match number {
                Number::Float(_) => Display::fmt(&number.value(), f),
                Number::Hexadecimal(n) => {
                    write!(f, "${:x}", *n)
                }
                Number::Binary(n) => {
                    write!(f, "%{:b}", *n)
                }
                Number::Constant(c) => {
                    dbg!(c);
                    let (name, _) = CONSTANTS_LOOKUP
                        .entries()
                        .find(|(_, &value)| {
                            *c == value
                                || (c.is_nan() && value.is_nan())
                                || (c.is_infinite()
                                    && value.is_infinite()
                                    && c.is_sign_positive() == value.is_sign_positive())
                        })
                        .expect("constant should be in lookup table");
                    Display::fmt(name, f)
                }
                Number::String(s) => {
                    write!(f, r#"HASH("{s}")"#)
                }

                Number::Enum(_) => {
                    // TODO: handle better
                    Display::fmt(&number.value(), f)
                }
            },
            Operand::Type { identifier, .. } => Display::fmt(&identifier, f),
            Operand::Identifier(ident) => Display::fmt(&ident, f),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Label {
    pub id: Identifier,
    // #[rust_sitter::leaf(text = r":")] pub ());
}

impl FromStr for Label {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().enumerate().last() {
            Some((index, ':')) => Ok(Label {
                id: s[..index].parse()?,
            }),
            Some((index, _)) => Err(ParseError {
                line: 0,
                start: index,
                end: index,
                msg: "Missing ':' at end of label".to_owned(),
            }),
            None => Err(ParseError {
                line: 0,
                start: 0,
                end: 0,
                msg: "empty string for label? parse miscalled".to_owned(),
            }),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
}

impl FromStr for Identifier {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars();
        if let Some(c) = iter.next() {
            if matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '.') {
                for (index, cc) in iter.enumerate() {
                    match cc {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '.' => continue,
                        cc => {
                            return Err(ParseError {
                                line: 0,
                                start: index,
                                end: index,
                                msg: format!("Invalid character in identifier '{}'", cc),
                            })
                        }
                    }
                }
                Ok(Identifier { name: s.to_owned() })
            } else {
                Err(ParseError {
                    line: 0,
                    start: 0,
                    end: 0,
                    msg: format!("Invalid character to start an identifier '{}'", c),
                })
            }
        } else {
            Err(ParseError {
                line: 0,
                start: 0,
                end: 0,
                msg: "Empty Identifier".to_owned(),
            })
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.name, f)
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Number {
    Float(f64),
    Binary(i64),
    Hexadecimal(i64),
    Constant(f64),
    String(String),
    Enum(f64),
}

impl Number {
    pub fn value(&self) -> f64 {
        match self {
            Number::Enum(val) | Number::Float(val) | Number::Constant(val) => *val,

            Number::Binary(val) | Number::Hexadecimal(val) => *val as f64,
            Number::String(s) => const_crc32::crc32(s.as_bytes()) as i32 as f64,
        }
    }
    pub fn value_i64(&self, signed: bool) -> i64 {
        match self {
            Number::Enum(val) | Number::Float(val) | Number::Constant(val) => interpreter::f64_to_i64(*val, signed),
            Number::Binary(val) | Number::Hexadecimal(val) => *val,
            Number::String(s) => const_crc32::crc32(s.as_bytes()) as i32 as i64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_register() {
        let op = "requestingot".parse::<Operand>();
        assert_eq!(
            op.unwrap(),
            Operand::Identifier(Identifier {
                name: "requestingot".to_owned()
            })
        );
    }

    #[test]
    fn successful_parse() {
        let parsed = parse("s d0 Setting 0 # This is a comment\n");
        dbg!(&parsed);
        assert_eq!(
            parsed.unwrap(),
            vec![Line {
                code: Some(Code::Instruction(Instruction {
                    instruction: InstructionOp::S,
                    operands: vec![
                        Operand::DeviceSpec(DeviceSpec {
                            device: Device::Numbered(0),
                            connection: None,
                        }),
                        Operand::Type {
                            logic_type: Some(LogicType::Setting),
                            slot_logic_type: None,
                            batch_mode: None,
                            reagent_mode: None,
                            identifier: Identifier {
                                name: "Setting".to_owned(),
                            },
                        },
                        Operand::Number(Number::Float(0.0)),
                    ],
                },),),
                comment: Some(Comment {
                    comment: " This is a comment".to_owned(),
                },),
            },],
        );
        let parsed = parse("move r0 $fff\n");
        dbg!(&parsed);
        assert_eq!(
            parsed.unwrap(),
            vec![Line {
                code: Some(Code::Instruction(Instruction {
                    instruction: InstructionOp::Move,
                    operands: vec![
                        Operand::RegisterSpec(RegisterSpec {
                            indirection: 0,
                            target: 0,
                        }),
                        Operand::Number(Number::Hexadecimal(4095)),
                    ],
                },),),
                comment: None,
            },],
        );
    }

    #[test]
    fn parse_code_chunk() {
        let code = "# This is a comment\n\
        define a_def 10\n\
        define a_hash HASH(\"This is a String\")\n\
        alias a_var r0\n\
        alias a_device d0\n\
        s d0 On 1\n\
        s d0 12 0 \n\
        move r2 LogicType.Temperature\n\
        move r3 pinf\n\
        main:\n\
        \n\
        l r1 dr15 RatioWater\n\
        move r0 HASH(\"AccessCardBlack\")\n\
        move r1 -2045627372 \n\
        move r1 $FF\n\
        move r1 %1000\n\
        move rr1 0
        yield\n\
        j main\n";
        let parsed = parse(code);
        dbg!(&parsed);
        assert_eq!(
            parsed.unwrap(),
            vec![
                Line {
                    code: None,
                    comment: Some(Comment {
                        comment: " This is a comment".to_owned(),
                    },),
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Define,
                        operands: vec![
                            Operand::Identifier(Identifier {
                                name: "a_def".to_owned(),
                            },),
                            Operand::Number(Number::Float(10.0,),),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Define,
                        operands: vec![
                            Operand::Identifier(Identifier {
                                name: "a_hash".to_owned(),
                            },),
                            Operand::Number(Number::String("This is a String".to_owned()),),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Alias,
                        operands: vec![
                            Operand::Identifier(Identifier {
                                name: "a_var".to_owned(),
                            },),
                            Operand::RegisterSpec(RegisterSpec {
                                indirection: 0,
                                target: 0,
                            }),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Alias,
                        operands: vec![
                            Operand::Identifier(Identifier {
                                name: "a_device".to_owned(),
                            },),
                            Operand::DeviceSpec(DeviceSpec {
                                device: Device::Numbered(0),
                                connection: None,
                            }),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::S,
                        operands: vec![
                            Operand::DeviceSpec(DeviceSpec {
                                device: Device::Numbered(0),
                                connection: None
                            }),
                            Operand::Type {
                                logic_type: Some(LogicType::On),
                                slot_logic_type: Some(SlotLogicType::On),
                                batch_mode: None,
                                reagent_mode: None,
                                identifier: Identifier {
                                    name: "On".to_owned(),
                                },
                            },
                            Operand::Number(Number::Float(1.0))
                        ]
                    })),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::S,
                        operands: vec![
                            Operand::DeviceSpec(DeviceSpec {
                                device: Device::Numbered(0),
                                connection: None,
                            }),
                            Operand::Number(Number::Float(12.0)),
                            Operand::Number(Number::Float(0.0)),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Move,
                        operands: vec![
                            Operand::RegisterSpec(RegisterSpec {
                                indirection: 0,
                                target: 2,
                            }),
                            Operand::Number(Number::Enum(6.0)),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Move,
                        operands: vec![
                            Operand::RegisterSpec(RegisterSpec {
                                indirection: 0,
                                target: 3,
                            }),
                            Operand::Number(Number::Constant(f64::INFINITY)),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Label(Label {
                        id: Identifier {
                            name: "main".to_owned(),
                        },
                    },),),
                    comment: None,
                },
                Line {
                    code: None,
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::L,
                        operands: vec![
                            Operand::RegisterSpec(RegisterSpec {
                                indirection: 0,
                                target: 1,
                            }),
                            Operand::DeviceSpec(DeviceSpec {
                                device: Device::Indirect {
                                    indirection: 0,
                                    target: 15,
                                },
                                connection: None,
                            }),
                            Operand::Type {
                                logic_type: Some(LogicType::RatioWater),
                                slot_logic_type: None,
                                batch_mode: None,
                                reagent_mode: None,
                                identifier: Identifier {
                                    name: "RatioWater".to_owned(),
                                },
                            },
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Move,
                        operands: vec![
                            Operand::RegisterSpec(RegisterSpec {
                                indirection: 0,
                                target: 0,
                            }),
                            Operand::Number(Number::String("AccessCardBlack".to_owned()),),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Move,
                        operands: vec![
                            Operand::RegisterSpec(RegisterSpec {
                                indirection: 0,
                                target: 1,
                            }),
                            Operand::Number(Number::Float(-2045627372.0)),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Move,
                        operands: vec![
                            Operand::RegisterSpec(RegisterSpec {
                                indirection: 0,
                                target: 1,
                            }),
                            Operand::Number(Number::Hexadecimal(255)),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Move,
                        operands: vec![
                            Operand::RegisterSpec(RegisterSpec {
                                indirection: 0,
                                target: 1,
                            }),
                            Operand::Number(Number::Binary(8)),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Move,
                        operands: vec![
                            Operand::RegisterSpec(RegisterSpec {
                                indirection: 1,
                                target: 1,
                            }),
                            Operand::Number(Number::Float(0.0)),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Yield,
                        operands: vec![],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::J,
                        operands: vec![Operand::Identifier(Identifier {
                            name: "main".to_owned(),
                        },),],
                    },),),
                    comment: None,
                },
            ],
        );
    }

    #[test]
    fn test_operand_display() {
        #[track_caller]
        fn test_roundtrip(s: &str) {
            let o: Operand = s.parse().expect("test string should parse with FromStr");
            assert_eq!(o.to_string(), s);
        }
        test_roundtrip("r0");
        test_roundtrip("r15");
        test_roundtrip("rr4");
        test_roundtrip("rrrr4");

        test_roundtrip("ra");
        test_roundtrip("sp");
        assert_eq!("r16".parse::<Operand>().unwrap().to_string(), "sp");
        assert_eq!("r17".parse::<Operand>().unwrap().to_string(), "ra");
        // Not registers
        test_roundtrip("rsp");
        test_roundtrip("rra");
        // Indirect only works through number names
        test_roundtrip("rr16");
        test_roundtrip("rr17");

        test_roundtrip("Identifier");
        test_roundtrip("db");
        test_roundtrip("d0");
        test_roundtrip("drr0");
        test_roundtrip("d0:1");
        test_roundtrip("42");
        test_roundtrip("1.2345");
        test_roundtrip("-1.2345");
        test_roundtrip(LogicType::Pressure.as_ref());
        test_roundtrip(SlotLogicType::Occupied.as_ref());
        test_roundtrip(BatchMode::Average.as_ref());
        test_roundtrip(ReagentMode::Recipe.as_ref());
        test_roundtrip("pi");
        test_roundtrip("pinf");
        test_roundtrip("ninf");
        test_roundtrip("nan");
        test_roundtrip(r#"HASH("StructureFurnace")"#);
        test_roundtrip("$abcd");
        test_roundtrip("%1001");
    }

    #[test]
    fn all_generated_enums_have_value() {
        use strum::IntoEnumIterator;
        for lt in LogicType::iter() {
            println!("testing LogicType.{lt}");
            let value = lt.get_str("value");
            assert!(value.is_some());
            assert!(value.unwrap().parse::<u16>().is_ok());
        }
        for slt in SlotLogicType::iter() {
            println!("testing SlotLogicType.{slt}");
            let value = slt.get_str("value");
            assert!(value.is_some());
            assert!(value.unwrap().parse::<u8>().is_ok());
        }
        for bm in BatchMode::iter() {
            println!("testing BatchMode.{bm}");
            let value = bm.get_str("value");
            assert!(value.is_some());
            assert!(value.unwrap().parse::<u8>().is_ok());
        }
        for rm in ReagentMode::iter() {
            println!("testing ReagentMode.{rm}");
            let value = rm.get_str("value");
            assert!(value.is_some());
            assert!(value.unwrap().parse::<u8>().is_ok());
        }
        for le in LogicEnums::iter() {
            println!("testing Enum.{le}");
            let value = le.get_str("value");
            assert!(value.is_some());
            assert!(value.unwrap().parse::<u32>().is_ok());
        }
    }

    #[test]
    fn bad_parse_does_not_panic() {
        let code = "move foo -";
        let parsed = parse(code);
        assert!(parsed.is_err());
        println!("{}", parsed.unwrap_err());
    }
}
