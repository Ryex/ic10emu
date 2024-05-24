use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

use stationeers_data::enums::script::{LogicSlotType, LogicType};

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum LogicError {
    #[error("can't read LogicType {0}")]
    CantRead(LogicType),
    #[error("can't read slot {1} LogicSlotType {0}")]
    CantSlotRead(LogicSlotType, f64),
    #[error("can't write LogicType {0}")]
    CantWrite(LogicType),
    #[error("can't write slot {1} LogicSlotType {0}")]
    CantSlotWrite(LogicSlotType, f64),
    #[error("slot id {0} is out of range 0..{1}")]
    SlotIndexOutOfRange(f64, usize),
}

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum MemoryError {
    #[error("stack underflow: {0} < range [0..{1})")]
    StackUnderflow(i32, usize),
    #[error("stack overflow: {0} > range [0..{1})")]
    StackOverflow(i32, usize),
    #[error("memory not readable")]
    NotReadable,
    #[error("memory not writeable")]
    NotWriteable,
}
