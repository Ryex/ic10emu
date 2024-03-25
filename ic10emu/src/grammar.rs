use crate::interpreter;
use crate::tokens::SplitConsecutiveIndicesExt;
use itertools::Itertools;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]
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
    pub fn offset(self, offset: usize) -> Self {
        ParseError {
            line: self.line,
            start: self.start + offset,
            end: self.end + offset,
            msg: self.msg,
        }
    }

    /// Offset the ParseError line, adding the passed value to it's `line`
    pub fn offset_line(self, offset: usize) -> Self {
        ParseError {
            line: self.line + offset,
            start: self.start,
            end: self.end,
            msg: self.msg,
        }
    }

    /// Mark the parse error as extending 'length' bytes from `start`
    pub fn span(self, length: usize) -> Self {
        ParseError {
            line: self.line,
            start: self.start,
            end: self.start + length,
            msg: self.msg,
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/instructions.rs"));
include!(concat!(env!("OUT_DIR"), "/logictypes.rs"));
include!(concat!(env!("OUT_DIR"), "/modes.rs"));
include!(concat!(env!("OUT_DIR"), "/constants.rs"));
include!(concat!(env!("OUT_DIR"), "/enums.rs"));

pub fn parse(code: &str) -> Result<Vec<Line>, ParseError> {
    code.lines()
        .enumerate()
        .map(|(n, l)| l.parse::<Line>().map_err(|e| e.offset_line(n)))
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
        let mut parts = s.splitn(2, "#");
        let code = parts
            .next()
            .map(|s| {
                let s = s.trim_end();
                if s.is_empty() {
                    None
                } else {
                    Some(s.parse::<Code>())
                }
            })
            .flatten()
            .transpose()?;
        let comment = parts.next().map(|s| s.parse()).transpose()?;
        Ok(Line { code, comment })
    }
}

#[derive(PartialEq, Debug)]
pub enum Code {
    Instruction(Instruction),
    Label(Label),
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

#[derive(PartialEq, Debug)]
pub struct Comment {
    pub comment: String,
}

impl FromStr for Comment {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Comment {
            comment: s.to_owned(),
        })
    }
}

#[derive(PartialEq, Debug)]
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
                    msg: format!("Missing instruction"),
                })
            }
        }?;
        let mut operand_tokens = Vec::new();
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
        let operands = operand_tokens
            .into_iter()
            .map(|(index, token)| {
                token
                    .parse::<Operand>()
                    .map_err(|e| e.offset(index).span(token.len()))
            })
            .collect::<Result<Vec<_>, ParseError>>()?;
        Ok(Instruction {
            instruction,
            operands,
        })
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Device {
    Db,
    Numbered(u32),
    Indirect { indirection: u32, target: u32 },
}

#[derive(PartialEq, Debug)]
pub enum Operand {
    RegisterSpec {
        indirection: u32,
        target: u32,
    },
    DeviceSpec {
        device: Device,
        channel: Option<u32>,
    },
    Number(Number),
    LogicType(LogicType),
    Identifier(Identifier),
}

impl Operand {
    pub fn get_value(&self, ic: &interpreter::IC) -> Result<f64, interpreter::ICError> {
        match &self {
            &Operand::RegisterSpec {
                indirection,
                target,
            } => ic.get_register(*indirection, *target),
            &Operand::Number(num) => Ok(num.value()),
            &Operand::LogicType(lt) => Ok(lt.value),
            &Operand::Identifier(ident) => ic.get_ident_value(&ident.name),
            &Operand::DeviceSpec { .. } => Err(interpreter::ICError::DeviceNotValue),
        }
    }
    pub fn get_device_id(
        &self,
        ic: &interpreter::IC,
    ) -> Result<(Option<u16>, Option<u32>), interpreter::ICError> {
        match &self {
            &Operand::DeviceSpec { device, channel } => match device {
                Device::Db => Ok((Some(ic.id), *channel)),
                Device::Numbered(p) => {
                    let dp = ic
                        .pins
                        .get(*p as usize)
                        .ok_or(interpreter::ICError::DeviceIndexOutOfRange(*p as f64))
                        .copied()?;
                    Ok((dp, *channel))
                }
                Device::Indirect {
                    indirection,
                    target,
                } => {
                    let val = ic.get_register(*indirection, *target)?;
                    let dp = ic
                        .pins
                        .get(val as usize)
                        .ok_or(interpreter::ICError::DeviceIndexOutOfRange(val))
                        .copied()?;
                    Ok((dp, *channel))
                }
            },
            &Operand::Identifier(id) => ic.get_ident_device_id(&id.name),
            _ => Err(interpreter::ICError::ValueNotDevice),
        }
    }
}

impl FromStr for Operand {
    type Err = ParseError;
    /// Parse a str containing an single instruction operand
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<_>>();
        match &chars[..] {
            ['s', 'p'] => Ok(Operand::RegisterSpec {
                indirection: 0,
                target: 16,
            }),
            ['r', 'a'] => Ok(Operand::RegisterSpec {
                indirection: 0,
                target: 17,
            }),
            ['r', rest @ ..] => {
                let mut rest_iter = rest.iter();
                let indirection = rest_iter.take_while_ref(|c| *c == &'r').count();
                let target = rest_iter
                    .take_while_ref(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse::<u32>()
                    .ok();
                let trailing = rest_iter.count();
                if let Some(target) = target {
                    if trailing == 0 {
                        return Ok(Operand::RegisterSpec {
                            indirection: indirection as u32,
                            target,
                        });
                    }
                }
                Err(ParseError {
                    line: 0,
                    start: 0,
                    end: 0,
                    msg: format!("Invalid register specifier"),
                })
            }
            ['d', rest @ ..] => match rest {
                ['b'] => Ok(Operand::DeviceSpec {
                    device: Device::Db,
                    channel: None,
                }),
                ['b', ':', chan @ ..] => {
                    if chan.into_iter().all(|c| c.is_digit(10)) {
                        Ok(Operand::DeviceSpec {
                            device: Device::Db,
                            channel: Some(String::from_iter(chan).parse().unwrap()),
                        })
                    } else {
                        Err(ParseError {
                            line: 0,
                            start: 3,
                            end: 3,
                            msg: format!("Invalid device channel specifier"),
                        })
                    }
                }
                ['r', rest @ ..] => {
                    let mut rest_iter = rest.into_iter().peekable();
                    let indirection = rest_iter.take_while_ref(|c| *c == &'r').count();
                    let target_str = rest_iter
                        .take_while_ref(|c| c.is_digit(10))
                        .collect::<String>();
                    let target = target_str.parse::<u32>().ok();
                    let channel = {
                        if rest_iter.peek() == Some(&&':') {
                            // take off ':'
                            rest_iter.next();
                            let channel_str = rest_iter
                                .take_while_ref(|c| c.is_digit(10))
                                .collect::<String>();
                            let channel = channel_str.parse::<u32>().unwrap();
                            let trailing = rest_iter.clone().collect::<Vec<_>>();
                            if trailing.len() == 0 {
                                Ok(Some(channel))
                            } else {
                                let start =
                                    2 + indirection + target_str.len() + 1 + channel_str.len();
                                Err(ParseError {
                                    line: 0,
                                    start,
                                    end: start,
                                    msg: format!("Invalid device channel specifier"),
                                })
                            }
                        } else {
                            Ok(None)
                        }
                    }?;
                    let trailing = rest_iter.collect::<Vec<_>>();
                    if let Some(target) = target {
                        if trailing.len() == 0 {
                            Ok(Operand::DeviceSpec {
                                device: Device::Indirect {
                                    indirection: indirection as u32,
                                    target,
                                },
                                channel,
                            })
                        } else {
                            Err(ParseError {
                                line: 0,
                                start: 0,
                                end: 0,
                                msg: format!("Invalid register specifier"),
                            })
                        }
                    } else {
                        Ok(Operand::Identifier(s.parse::<Identifier>()?))
                    }
                }
                [rest @ ..] => {
                    let mut rest_iter = rest.into_iter().peekable();
                    let target_str = rest_iter
                        .take_while_ref(|c| c.is_digit(10))
                        .collect::<String>();
                    let target = target_str.parse::<u32>().ok();
                    let channel = {
                        if rest_iter.peek() == Some(&&':') {
                            // take off ':'
                            rest_iter.next();
                            let channel_str = rest_iter
                                .take_while_ref(|c| c.is_digit(10))
                                .collect::<String>();
                            let channel = channel_str.parse::<u32>().unwrap();
                            let trailing = rest_iter.clone().collect::<Vec<_>>();
                            if trailing.len() == 0 {
                                Ok(Some(channel))
                            } else {
                                let start = 1 + target_str.len() + 1 + channel_str.len();
                                Err(ParseError {
                                    line: 0,
                                    start,
                                    end: start,
                                    msg: format!("Invalid device channel specifier"),
                                })
                            }
                        } else {
                            Ok(None)
                        }
                    }?;
                    let trailing = rest_iter.collect::<Vec<_>>();
                    if let Some(target) = target {
                        if trailing.len() == 0 {
                            Ok(Operand::DeviceSpec {
                                device: Device::Numbered(target),
                                channel,
                            })
                        } else {
                            Err(ParseError {
                                line: 0,
                                start: 0,
                                end: 0,
                                msg: format!("Invalid device specifier"),
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
                        start: 0,
                        end: 0,
                        msg: format!("Invalid hash string: Can not contain '\"'"),
                    })
                }
            }
            ['$', rest @ ..] => {
                let mut rest_iter = rest.into_iter();
                let num_str = rest_iter
                    .take_while_ref(|c| c.is_digit(16))
                    .collect::<String>();
                let num = i64::from_str_radix(&num_str, 16).unwrap() as f64;
                let trailing = rest_iter.count();
                if trailing == 0 {
                    Ok(Operand::Number(Number::Hexadecimal(num)))
                } else {
                    Err(ParseError {
                        line: 0,
                        start: 0,
                        end: 0,
                        msg: format!("Invalid Hexadecimal Number"),
                    })
                }
            }
            ['%', rest @ ..] => {
                let mut rest_iter = rest.into_iter();
                let num_str = rest_iter
                    .take_while_ref(|c| c.is_digit(2))
                    .collect::<String>();
                let num = i64::from_str_radix(&num_str, 2).unwrap() as f64;
                let trailing = rest_iter.count();
                if trailing == 0 {
                    Ok(Operand::Number(Number::Binary(num)))
                } else {
                    Err(ParseError {
                        line: 0,
                        start: 0,
                        end: 0,
                        msg: format!("Invalid Binary Number"),
                    })
                }
            }
            [rest @ ..] => {
                let mut rest_iter = rest.into_iter().peekable();
                let float_str = if rest_iter.peek() == Some(&&'-') {
                    format!("{}", rest_iter.next().unwrap())
                } else {
                    "".to_string()
                } + &rest_iter
                    .take_while_ref(|c| c.is_digit(10))
                    .collect::<String>();
                if !float_str.is_empty() {
                    if rest_iter.peek() == Some(&&'.') {
                        let decimal_str = rest_iter
                            .take_while_ref(|c| c.is_digit(10))
                            .collect::<String>();
                        if !decimal_str.is_empty() {
                            let float_str = float_str + "." + &decimal_str;
                            let num = f64::from_str(&float_str).unwrap();
                            Ok(Operand::Number(Number::Float(num)))
                        } else {
                            let start = float_str.len() + 1;
                            Err(ParseError {
                                line: 0,
                                start,
                                end: start,
                                msg: format!("Invalid Decimal Number"),
                            })
                        }
                    } else {
                        let trailing = rest_iter.count();
                        if trailing == 0 {
                            let num = f64::from_str(&float_str).unwrap();
                            Ok(Operand::Number(Number::Float(num)))
                        } else {
                            let start = float_str.len();
                            Err(ParseError {
                                line: 0,
                                start,
                                end: start,
                                msg: format!("Invalid Integer Number"),
                            })
                        }
                    }
                } else if let Some(val) = CONSTANTS_LOOKUP.get(s) {
                    Ok(Operand::Number(Number::Constant(*val)))
                } else if let Some(val) = ENUM_LOOKUP.get(s) {
                    Ok(Operand::Number(Number::Enum(*val as f64)))
                } else if let Some(val) = LOGIC_TYPE_LOOKUP.get(s) {
                    Ok(Operand::LogicType(LogicType {
                        name: s.to_string(),
                        value: *val as f64,
                    }))
                } else if let Some(val) = BATCH_MODE_LOOKUP.get(s) {
                    Ok(Operand::LogicType(LogicType {
                        name: s.to_string(),
                        value: *val as f64,
                    }))
                } else if let Some(val) = REAGENT_MODE_LOOKUP.get(s) {
                    Ok(Operand::LogicType(LogicType {
                        name: s.to_string(),
                        value: *val as f64,
                    }))
                } else {
                    Ok(Operand::Identifier(s.parse::<Identifier>()?))
                }
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct LogicType {
    pub name: String,
    pub value: f64,
}

#[derive(PartialEq, Debug)]
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
                msg: "Missing ':' at end of label".to_string(),
            }),
            None => Err(ParseError {
                line: 0,
                start: 0,
                end: 0,
                msg: "empty string for label? parse miscalled".to_string(),
            }),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Identifier {
    // #[rust_sitter::leaf(pattern = r"[a-zA-Z_.][\w\d.]*", transform = |id| id.to_string())]
    pub name: String,
}

impl FromStr for Identifier {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars();
        if let Some(c) = iter.next() {
            if match c {
                'a'..='z' | 'A'..='Z' | '_' | '.' => true,
                _ => false,
            } {
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
                msg: format!("Empty Identifier"),
            })
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Number {
    Float(f64),
    Binary(f64),
    Hexadecimal(f64),
    Constant(f64),
    String(String),
    Enum(f64),
}

impl Number {
    pub fn value(&self) -> f64 {
        match self {
            Number::Enum(val) => *val,
            Number::Float(val) => *val,
            Number::Binary(val) => *val,
            Number::Constant(val) => *val,
            Number::Hexadecimal(val) => *val,
            Number::String(s) => const_crc32::crc32(s.as_bytes()) as i32 as f64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                        Operand::DeviceSpec {
                            device: Device::Numbered(0),
                            channel: None,
                        },
                        Operand::LogicType(LogicType {
                            name: "Setting".to_string(),
                            value: 12.0,
                        },),
                        Operand::Number(Number::Float(0.0)),
                    ],
                },),),
                comment: Some(Comment {
                    comment: " This is a comment".to_string(),
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
                        Operand::RegisterSpec {
                            indirection: 0,
                            target: 0,
                        },
                        Operand::Number(Number::Hexadecimal(4095.0)),
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
                        comment: " This is a comment".to_string(),
                    },),
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Define,
                        operands: vec![
                            Operand::Identifier(Identifier {
                                name: "a_def".to_string(),
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
                                name: "a_hash".to_string(),
                            },),
                            Operand::Number(Number::String("This is a String".to_string()),),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Alias,
                        operands: vec![
                            Operand::Identifier(Identifier {
                                name: "a_var".to_string(),
                            },),
                            Operand::RegisterSpec {
                                indirection: 0,
                                target: 0,
                            },
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Alias,
                        operands: vec![
                            Operand::Identifier(Identifier {
                                name: "a_device".to_string(),
                            },),
                            Operand::DeviceSpec {
                                device: Device::Numbered(0),
                                channel: None,
                            },
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::S,
                        operands: vec![
                            Operand::DeviceSpec {
                                device: Device::Numbered(0),
                                channel: None,
                            },
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
                            Operand::RegisterSpec {
                                indirection: 0,
                                target: 2,
                            },
                            Operand::Number(Number::Enum(6.0)),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Move,
                        operands: vec![
                            Operand::RegisterSpec {
                                indirection: 0,
                                target: 3,
                            },
                            Operand::Number(Number::Constant(f64::INFINITY)),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Label(Label {
                        id: Identifier {
                            name: "main".to_string(),
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
                            Operand::RegisterSpec {
                                indirection: 0,
                                target: 1,
                            },
                            Operand::DeviceSpec {
                                device: Device::Indirect {
                                    indirection: 0,
                                    target: 15,
                                },
                                channel: None,
                            },
                            Operand::LogicType(LogicType {
                                name: "RatioWater".to_string(),
                                value: 19.0,
                            },),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Move,
                        operands: vec![
                            Operand::RegisterSpec {
                                indirection: 0,
                                target: 0,
                            },
                            Operand::Number(Number::String("AccessCardBlack".to_string()),),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Move,
                        operands: vec![
                            Operand::RegisterSpec {
                                indirection: 0,
                                target: 1,
                            },
                            Operand::Number(Number::Float(-2045627372.0)),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Move,
                        operands: vec![
                            Operand::RegisterSpec {
                                indirection: 0,
                                target: 1,
                            },
                            Operand::Number(Number::Hexadecimal(255.0)),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Move,
                        operands: vec![
                            Operand::RegisterSpec {
                                indirection: 0,
                                target: 1,
                            },
                            Operand::Number(Number::Binary(8.0)),
                        ],
                    },),),
                    comment: None,
                },
                Line {
                    code: Some(Code::Instruction(Instruction {
                        instruction: InstructionOp::Move,
                        operands: vec![
                            Operand::RegisterSpec {
                                indirection: 1,
                                target: 1,
                            },
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
                            name: "main".to_string(),
                        },),],
                    },),),
                    comment: None,
                },
            ],
        );
    }
}
