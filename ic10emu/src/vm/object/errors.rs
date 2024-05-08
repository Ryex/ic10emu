use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::vm::enums::script_enums::{LogicSlotType as SlotLogicType, LogicType};

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum LogicError {
    #[error("can't read LogicType {0}")]
    CantRead(LogicType),
    #[error("can't read slot {1} SlotLogicType {0}")]
    CantSlotRead(SlotLogicType, usize),
    #[error("can't write LogicType {0}")]
    CantWrite(LogicType),
    #[error("can't write slot {1} SlotLogicType {0}")]
    CantSlotWrite(SlotLogicType, usize),
    #[error("slot id {0} is out of range 0..{1}")]
    SlotIndexOutOfRange(usize, usize),
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum MemoryError {
    #[error("stack underflow: {0} < range [0..{1})")]
    StackUnderflow(i32, usize),
    #[error("stack overflow: {0} > range [0..{1})")]
    StackOverflow(i32, usize),
    #[error("memory unit not present")]
    NotPresent,
}
