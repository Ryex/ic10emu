use crate::vm::{
    instructions::enums::InstructionOp,
    object::{
        errors::{LogicError, MemoryError},
        ObjectID,
    },
};
use serde_derive::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum VMError {
    #[error("device with id '{0}' does not exist")]
    UnknownId(u32),
    #[error("ic with id '{0}' does not exist")]
    UnknownIcId(u32),
    #[error("device with id '{0}' does not have a ic slot")]
    NoIC(u32),
    #[error("ic encountered an error: {0}")]
    ICError(#[from] ICError),
    #[error("ic encountered an error: {0}")]
    LineError(#[from] LineError),
    #[error("invalid network id {0}")]
    InvalidNetwork(ObjectID),
    #[error("device {0} not visible to device {1} (not on the same networks)")]
    DeviceNotVisible(u32, u32),
    #[error("a device with id {0} already exists")]
    IdInUse(u32),
    #[error("device(s) with ids {0:?} already exist")]
    IdsInUse(Vec<u32>),
    #[error("atempt to use a set of id's with duplicates: id(s) {0:?} exsist more than once")]
    DuplicateIds(Vec<u32>),
    #[error("object {0} is not a device")]
    NotADevice(ObjectID),
    #[error("device object {0} has no pins")]
    NoDevicePins(ObjectID),
    #[error("object {0} has no slots")]
    NotStorage(ObjectID),
    #[error("object {0} is not an item")]
    NotAnItem(ObjectID),
    #[error("object {0} is not programmable")]
    NotProgrammable(ObjectID),
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum TemplateError {
    #[error("object id {0} has a non conforming set of interfaces")]
    NonConformingObject(ObjectID),
    #[error("ObjectID {0} is missing fomr the VM")]
    MissingVMObject(ObjectID),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineError {
    pub error: ICError,
    pub line: u32,
}

impl Display for LineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error on line {}: {}", self.line, self.error)
    }
}

impl StdError for LineError {}

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

impl StdError for ParseError {}

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

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum ICError {
    #[error("error compiling code: {0}")]
    ParseError(#[from] ParseError),
    #[error("{0}")]
    LogicError(#[from] LogicError),
    #[error("{0}")]
    MemoryError(#[from] MemoryError),
    #[error("duplicate label {0}")]
    DuplicateLabel(String),
    #[error("instruction pointer out of range: '{0}'")]
    InstructionPointerOutOfRange(usize),
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
        inst: InstructionOp,
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
    UnknownLogicSlotType(f64),
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
