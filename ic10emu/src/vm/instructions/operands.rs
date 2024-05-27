use crate::errors::ICError;
use crate::interpreter;
use crate::vm::{instructions::enums::InstructionOp, object::traits::IntegratedCircuit};
use serde_derive::{Deserialize, Serialize};
use stationeers_data::enums::script::{
    LogicBatchMethod, LogicReagentMode, LogicSlotType, LogicType,
};
use strum::EnumProperty;
#[cfg(feature = "tsify")]
use tsify::Tsify;
#[cfg(feature = "tsify")]
use wasm_bindgen::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub enum Device {
    Db,
    Numbered(u32),
    Indirect { indirection: u32, target: u32 },
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct RegisterSpec {
    pub indirection: u32,
    pub target: u32,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct DeviceSpec {
    pub device: Device,
    pub connection: Option<usize>,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Identifier {
    pub name: String,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub enum Number {
    Float(f64),
    Binary(i64),
    Hexadecimal(i64),
    Constant(f64),
    String(String),
    Enum(f64),
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub enum Operand {
    RegisterSpec(RegisterSpec),
    DeviceSpec(DeviceSpec),
    Number(Number),
    Type {
        logic_type: Option<LogicType>,
        slot_logic_type: Option<LogicSlotType>,
        batch_mode: Option<LogicBatchMethod>,
        reagent_mode: Option<LogicReagentMode>,
        identifier: Identifier,
    },
    Identifier(Identifier),
}

pub struct InstOperand {
    pub operand: Operand,
    pub inst: InstructionOp,
    pub index: usize,
}

impl InstOperand {
    pub fn new(operand: &Operand, inst: InstructionOp, index: usize) -> Self {
        InstOperand {
            operand: operand.clone(),
            inst,
            index,
        }
    }

    pub fn as_ident(&self) -> Result<Identifier, ICError> {
        let Operand::Identifier(ident) = &self.operand else {
            return Err(ICError::IncorrectOperandType {
                inst: self.inst,
                index: self.index,
                desired: "Name".to_owned(),
            });
        };
        Ok(ident.clone())
    }

    pub fn as_number(&self) -> Result<Number, ICError> {
        let Operand::Number(num) = &self.operand else {
            return Err(ICError::IncorrectOperandType {
                inst: self.inst,
                index: self.index,
                desired: "Number".to_owned(),
            });
        };
        Ok(num.clone())
    }

    pub fn as_aliasable(&self) -> Result<Operand, ICError> {
        match &self.operand {
            Operand::RegisterSpec { .. } | Operand::DeviceSpec { .. } => Ok(self.operand.clone()),
            _ => Err(ICError::IncorrectOperandType {
                inst: self.inst,
                index: self.index,
                desired: "Device Or Register".to_owned(),
            }),
        }
    }

    pub fn as_value<IC: IntegratedCircuit>(&self, ic: &IC) -> Result<f64, ICError> {
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
                inst: self.inst,
                index: self.index,
                desired: "Value".to_owned(),
            }),
        }
    }

    pub fn as_value_i64<IC: IntegratedCircuit>(
        &self,
        ic: &IC,
        signed: bool,
    ) -> Result<i64, ICError> {
        match &self.operand {
            Operand::Number(num) => Ok(num.value_i64(signed)),
            _ => {
                let val = self.as_value(ic)?;
                if val < i64::MIN as f64 {
                    Err(ICError::ShiftUnderflowI64)
                } else if val <= i64::MAX as f64 {
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
    ) -> Result<i32, ICError> {
        match &self.operand {
            Operand::Number(num) => Ok(num.value_i64(signed) as i32),
            _ => {
                let val = self.as_value(ic)?;
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

    pub fn as_register<IC: IntegratedCircuit>(&self, ic: &IC) -> Result<RegisterSpec, ICError> {
        match self.translate_alias(ic) {
            Operand::RegisterSpec(reg) => Ok(reg),
            Operand::Identifier(id) => Err(ICError::UnknownIdentifier(id.name.to_string())),
            _ => Err(ICError::IncorrectOperandType {
                inst: self.inst,
                index: self.index,
                desired: "Register".to_owned(),
            }),
        }
    }

    /// interpret the operand as a device index, i32::MAX is db
    pub fn as_device<IC: IntegratedCircuit>(
        &self,
        ic: &IC,
    ) -> Result<(i32, Option<usize>), ICError> {
        match self.translate_alias(ic) {
            Operand::DeviceSpec(DeviceSpec { device, connection }) => match device {
                Device::Db => Ok((i32::MAX, connection)),
                Device::Numbered(p) => Ok((p as i32, connection)),
                Device::Indirect {
                    indirection,
                    target,
                } => {
                    let val = ic.get_register(indirection, target)?;
                    Ok((val as i32, connection))
                }
            },
            Operand::Identifier(id) => Err(ICError::UnknownIdentifier(id.name.to_string())),
            _ => Err(ICError::IncorrectOperandType {
                inst: self.inst,
                index: self.index,
                desired: "Value".to_owned(),
            }),
        }
    }

    pub fn as_logic_type<IC: IntegratedCircuit>(&self, ic: &IC) -> Result<LogicType, ICError> {
        match &self.operand {
            Operand::Type {
                logic_type: Some(lt),
                ..
            } => Ok(*lt),
            _ => {
                let val = self.as_value(ic)?;
                LogicType::try_from(val).map_err(|_| ICError::UnknownLogicType(val))
            }
        }
    }

    pub fn as_slot_logic_type<IC: IntegratedCircuit>(
        &self,
        ic: &IC,
    ) -> Result<LogicSlotType, ICError> {
        match &self.operand {
            Operand::Type {
                slot_logic_type: Some(slt),
                ..
            } => Ok(*slt),
            _ => {
                let val = self.as_value(ic)?;
                LogicSlotType::try_from(val).map_err(|_| ICError::UnknownLogicSlotType(val))
            }
        }
    }

    pub fn as_batch_mode<IC: IntegratedCircuit>(
        &self,
        ic: &IC,
    ) -> Result<LogicBatchMethod, ICError> {
        match &self.operand {
            Operand::Type {
                batch_mode: Some(bm),
                ..
            } => Ok(*bm),
            _ => {
                let val = self.as_value(ic)?;
                LogicBatchMethod::try_from(val).map_err(|_| ICError::UnknownBatchMode(val))
            }
        }
    }

    pub fn as_reagent_mode<IC: IntegratedCircuit>(
        &self,
        ic: &IC,
    ) -> Result<LogicReagentMode, ICError> {
        match &self.operand {
            Operand::Type {
                reagent_mode: Some(rm),
                ..
            } => Ok(*rm),
            _ => {
                let val = self.as_value(ic)?;
                LogicReagentMode::try_from(val).map_err(|_| ICError::UnknownReagentMode(val))
            }
        }
    }

    pub fn translate_alias<IC: IntegratedCircuit>(&self, ic: &IC) -> Operand {
        match &self.operand {
            Operand::Identifier(id) | Operand::Type { identifier: id, .. } => {
                if let Some(alias) = ic.get_aliases().get(&id.name) {
                    alias.clone()
                } else if let Some(define) = ic.get_defines().get(&id.name) {
                    Operand::Number(Number::Float(*define))
                } else if let Some(label) = ic.get_labels().get(&id.name) {
                    Operand::Number(Number::Float(*label as f64))
                } else {
                    self.operand.clone()
                }
            }
            _ => self.operand.clone(),
        }
    }
}
