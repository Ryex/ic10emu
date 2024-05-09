use crate::{
    errors::{ICError, ParseError},
    interpreter,
    tokens::{SplitConsecutiveIndicesExt, SplitConsecutiveWithIndices},
    vm::{
        enums::{
            basic_enums::BasicEnum,
            script_enums::{LogicBatchMethod, LogicReagentMode, LogicSlotType, LogicType},
        },
        instructions::{
            enums::InstructionOp,
            operands::{Device, DeviceSpec, Identifier, Number, Operand, RegisterSpec},
            Instruction, CONSTANTS_LOOKUP,
        },
    },
};
use itertools::Itertools;
use std::{fmt::Display, str::FromStr};
use strum::IntoEnumIterator;

impl TryFrom<f64> for LogicType {
    type Error = ICError;
    fn try_from(value: f64) -> Result<Self, <LogicType as TryFrom<f64>>::Error> {
        if let Some(lt) = LogicType::iter().find(|lt| *lt as u16 as f64 == value) {
            Ok(lt)
        } else {
            Err(ICError::UnknownLogicType(value))
        }
    }
}

impl TryFrom<f64> for LogicSlotType {
    type Error = ICError;
    fn try_from(value: f64) -> Result<Self, <LogicSlotType as TryFrom<f64>>::Error> {
        if let Some(slt) = LogicSlotType::iter().find(|lt| *lt as u8 as f64 == value) {
            Ok(slt)
        } else {
            Err(ICError::UnknownLogicSlotType(value))
        }
    }
}

impl TryFrom<f64> for LogicBatchMethod {
    type Error = ICError;
    fn try_from(value: f64) -> Result<Self, <LogicBatchMethod as TryFrom<f64>>::Error> {
        if let Some(bm) = LogicBatchMethod::iter().find(|lt| *lt as u8 as f64 == value) {
            Ok(bm)
        } else {
            Err(ICError::UnknownBatchMode(value))
        }
    }
}

impl TryFrom<f64> for LogicReagentMode {
    type Error = ICError;
    fn try_from(value: f64) -> Result<Self, <LogicReagentMode as TryFrom<f64>>::Error> {
        if let Some(rm) = LogicReagentMode::iter().find(|lt| *lt as u8 as f64 == value) {
            Ok(rm)
        } else {
            Err(ICError::UnknownReagentMode(value))
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
pub fn parse_with_invalid(code: &str) -> Vec<Line> {
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

impl FromStr for Instruction {
    type Err = ParseError;
    /// parse a non-empty string for  an instruction and it's operands
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens_iter = s.split_consecutive_with_indices(&[' ', '\t'][..]);
        let instruction: InstructionOp = {
            if let Some((index, token)) = tokens_iter.next() {
                token.parse::<InstructionOp>().map_err(|_e| {
                    ParseError {
                        line: 0,
                        start: 0,
                        end: 0,
                        msg: format!("unknown instruction '{token}'"),
                    }
                    .offset(index)
                    .span(token.len())
                })
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
                } else if let Ok(val) = BasicEnum::from_str(s) {
                    Ok(Operand::Number(Number::Enum(val.get_value() as f64)))
                } else {
                    let lt = LogicType::from_str(s).ok();
                    let slt = LogicSlotType::from_str(s).ok();
                    let bm = LogicBatchMethod::from_str(s).ok();
                    let rm = LogicReagentMode::from_str(s).ok();
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
            Number::Enum(val) | Number::Float(val) | Number::Constant(val) => {
                interpreter::f64_to_i64(*val, signed)
            }
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
                                slot_logic_type: Some(LogicSlotType::On),
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
        test_roundtrip(LogicSlotType::Occupied.as_ref());
        test_roundtrip(LogicBatchMethod::Average.as_ref());
        test_roundtrip(LogicReagentMode::Recipe.as_ref());
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
            assert_eq!(lt as u16, value.unwrap().parse::<u16>().unwrap());
        }
        for slt in LogicSlotType::iter() {
            println!("testing LogicSlotType.{slt}");
            let value = slt.get_str("value");
            assert!(value.is_some());
            assert!(value.unwrap().parse::<u8>().is_ok());
            assert_eq!(slt as u8, value.unwrap().parse::<u8>().unwrap());
        }
        for bm in LogicReagentMode::iter() {
            println!("testing BatchMode.{bm}");
            let value = bm.get_str("value");
            assert!(value.is_some());
            assert!(value.unwrap().parse::<u8>().is_ok());
            assert_eq!(bm as u8, value.unwrap().parse::<u8>().unwrap());
        }
        for rm in LogicReagentMode::iter() {
            println!("testing ReagentMode.{rm}");
            let value = rm.get_str("value");
            assert!(value.is_some());
            assert!(value.unwrap().parse::<u8>().is_ok());
            assert_eq!(rm as u8, value.unwrap().parse::<u8>().unwrap());
        }
        for le in BasicEnum::iter() {
            println!("testing BasicEnum {le}");
            let value = le.get_str("value");
            assert!(value.is_some());
            assert!(value.unwrap().parse::<u32>().is_ok());
            assert_eq!(le.get_value(), value.unwrap().parse::<u32>().unwrap());
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
