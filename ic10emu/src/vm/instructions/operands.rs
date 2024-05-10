use crate::errors::ICError;
use crate::interpreter;
use crate::vm::enums::script_enums::{
    LogicBatchMethod as BatchMode, LogicReagentMode as ReagentMode, LogicSlotType, LogicType,
};
use crate::vm::instructions::enums::InstructionOp;
use serde_derive::{Deserialize, Serialize};
use strum::EnumProperty;

use super::traits::IntegratedCircuit;

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

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
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

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Operand {
    RegisterSpec(RegisterSpec),
    DeviceSpec(DeviceSpec),
    Number(Number),
    Type {
        logic_type: Option<LogicType>,
        slot_logic_type: Option<LogicSlotType>,
        batch_mode: Option<BatchMode>,
        reagent_mode: Option<ReagentMode>,
        identifier: Identifier,
    },
    Identifier(Identifier),
}

impl Operand {
    pub fn as_value<IC: IntegratedCircuit>(
        &self,
        ic: &IC,
        inst: InstructionOp,
        index: u32,
    ) -> Result<f64, ICError> {
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
                    Err(ICError::TypeValueNotKnown)
                }
            }
            Operand::Identifier(id) => Err(ICError::UnknownIdentifier(id.name.to_string())),
            Operand::DeviceSpec { .. } => Err(ICError::IncorrectOperandType {
                inst,
                index,
                desired: "Value".to_owned(),
            }),
        }
    }

    pub fn as_value_i64<IC: IntegratedCircuit>(
        &self,
        ic: &IC,
        signed: bool,
        inst: InstructionOp,
        index: u32,
    ) -> Result<i64, ICError> {
        match self {
            Self::Number(num) => Ok(num.value_i64(signed)),
            _ => {
                let val = self.as_value(ic, inst, index)?;
                if val < -9.223_372_036_854_776E18 {
                    Err(ICError::ShiftUnderflowI64)
                } else if val <= 9.223_372_036_854_776E18 {
                    Ok(interpreter::f64_to_i64(val, signed))
                } else {
                    Err(ICError::ShiftOverflowI64)
                }
            }
        }
    }
    pub fn as_value_i32<IC: IntegratedCircuit>(
        &self,
        ic: &IC,
        signed: bool,
        inst: InstructionOp,
        index: u32,
    ) -> Result<i32, ICError> {
        match self {
            Self::Number(num) => Ok(num.value_i64(signed) as i32),
            _ => {
                let val = self.as_value(ic, inst, index)?;
                if val < i32::MIN as f64 {
                    Err(ICError::ShiftUnderflowI32)
                } else if val <= i32::MAX as f64 {
                    Ok(val as i32)
                } else {
                    Err(ICError::ShiftOverflowI32)
                }
            }
        }
    }

    pub fn as_register<IC: IntegratedCircuit>(
        &self,
        ic: &IC,
        inst: InstructionOp,
        index: u32,
    ) -> Result<RegisterSpec, ICError> {
        match self.translate_alias(ic) {
            Operand::RegisterSpec(reg) => Ok(reg),
            Operand::Identifier(id) => Err(ICError::UnknownIdentifier(id.name.to_string())),
            _ => Err(ICError::IncorrectOperandType {
                inst,
                index,
                desired: "Register".to_owned(),
            }),
        }
    }

    pub fn as_device<IC: IntegratedCircuit>(
        &self,
        ic: &IC,
        inst: InstructionOp,
        index: u32,
    ) -> Result<(Option<u32>, Option<usize>), ICError> {
        match self.translate_alias(ic) {
            Operand::DeviceSpec(DeviceSpec { device, connection }) => match device {
                Device::Db => Ok((Some(ic.device), connection)),
                Device::Numbered(p) => {
                    let dp = ic
                        .pins
                        .borrow()
                        .get(p as usize)
                        .ok_or(ICError::DeviceIndexOutOfRange(p as f64))
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
                        .ok_or(ICError::DeviceIndexOutOfRange(val))
                        .copied()?;
                    Ok((dp, connection))
                }
            },
            Operand::Identifier(id) => Err(ICError::UnknownIdentifier(id.name.to_string())),
            _ => Err(ICError::IncorrectOperandType {
                inst,
                index,
                desired: "Value".to_owned(),
            }),
        }
    }

    pub fn as_logic_type<IC: IntegratedCircuit>(
        &self,
        ic: &IC,
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

    pub fn as_slot_logic_type<IC: IntegratedCircuit>(
        &self,
        ic: &IC,
        inst: InstructionOp,
        index: u32,
    ) -> Result<LogicSlotType, ICError> {
        match &self {
            Operand::Type {
                slot_logic_type: Some(slt),
                ..
            } => Ok(*slt),
            _ => LogicSlotType::try_from(self.as_value(ic, inst, index)?),
        }
    }

    pub fn as_batch_mode<IC: IntegratedCircuit>(
        &self,
        ic: &IC,
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

    pub fn as_reagent_mode<IC: IntegratedCircuit>(
        &self,
        ic: &IC,
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

    pub fn translate_alias<IC: IntegratedCircuit>(&self, ic: &IC) -> Self {
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
