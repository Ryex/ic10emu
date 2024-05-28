use core::f64;
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashSet},
    fmt::Display,
    u32,
};

use itertools::Itertools;
#[cfg(feature = "tsify")]
use tsify::Tsify;
#[cfg(feature = "tsify")]
use wasm_bindgen::prelude::*;

use time::format_description;

use crate::{
    errors::{ICError, LineError},
    grammar,
    vm::instructions::{enums::InstructionOp, operands::Operand, Instruction},
};

pub mod instructions;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub enum ICState {
    Start,
    Running,
    Yield,
    Sleep(
        #[cfg_attr(feature = "tsify", tsify(type = "Date"))] time::OffsetDateTime,
        f64,
    ),
    Error(LineError),
    HasCaughtFire,
    Ended,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct ICInfo {
    pub instruction_pointer: u32,
    pub registers: Vec<f64>,
    pub aliases: BTreeMap<String, Operand>,
    pub defines: BTreeMap<String, f64>,
    pub labels: BTreeMap<String, u32>,
    pub state: ICState,
    pub yield_instruciton_count: u16,
}

impl Display for ICState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            ICState::Start => "Not Run".to_owned(),
            ICState::Running => "Running".to_owned(),
            ICState::Yield => "IC has yielded, Resume on next tick".to_owned(),
            ICState::Sleep(then, sleep_for) => {
                let format = format_description::parse("[hour]:[minute]:[second]").unwrap();
                let resume = *then + time::Duration::new(*sleep_for as i64, 0);
                format!(
                    "Sleeping for {sleep_for} seconds, will resume at {}",
                    resume.format(&format).unwrap()
                )
            }
            ICState::Error(err) => format!("{err}"),
            ICState::HasCaughtFire => "IC has caught fire! this is not a joke!".to_owned(),
            ICState::Ended => "Program has reached the end of exacution".to_owned(),
        };
        write!(f, "{out}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub errors: Vec<ICError>,
    pub labels: BTreeMap<String, u32>,
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
            errors: Vec::new(),
            labels: BTreeMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn try_from_code(code: &str) -> Result<Self, ICError> {
        let parse_tree = grammar::parse(code)?;
        let mut labels_set = HashSet::new();
        let mut labels = BTreeMap::new();
        let errors = Vec::new();
        let instructions = parse_tree
            .into_iter()
            .enumerate()
            .map(|(line_number, line)| match line.code {
                None => Ok(Instruction {
                    instruction: InstructionOp::Nop,
                    operands: vec![],
                }),
                Some(code) => match code {
                    grammar::Code::Label(label) => {
                        if labels_set.contains(&label.id.name) {
                            Err(ICError::DuplicateLabel(label.id.name))
                        } else {
                            labels_set.insert(label.id.name.clone());
                            labels.insert(label.id.name, line_number as u32);
                            Ok(Instruction {
                                instruction: InstructionOp::Nop,
                                operands: vec![],
                            })
                        }
                    }
                    grammar::Code::Instruction(instruction) => Ok(instruction),
                    grammar::Code::Invalid(err) => Err(err.into()),
                },
            })
            .try_collect()?;
        Ok(Program {
            instructions,
            errors,
            labels,
        })
    }

    pub fn from_code_with_invalid(code: &str) -> Self {
        let parse_tree = grammar::parse_with_invalid(code);
        let mut labels_set = HashSet::new();
        let mut labels = BTreeMap::new();
        let mut errors = Vec::new();
        let instructions = parse_tree
            .into_iter()
            .enumerate()
            .map(|(line_number, line)| match line.code {
                None => Instruction {
                    instruction: InstructionOp::Nop,
                    operands: vec![],
                },
                Some(code) => match code {
                    grammar::Code::Label(label) => {
                        if labels_set.contains(&label.id.name) {
                            errors.push(ICError::DuplicateLabel(label.id.name));
                        } else {
                            labels_set.insert(label.id.name.clone());
                            labels.insert(label.id.name, line_number as u32);
                        }
                        Instruction {
                            instruction: InstructionOp::Nop,
                            operands: vec![],
                        }
                    }
                    grammar::Code::Instruction(instruction) => instruction,
                    grammar::Code::Invalid(err) => {
                        errors.push(err.into());
                        Instruction {
                            instruction: InstructionOp::Nop,
                            operands: vec![],
                        }
                    }
                },
            })
            .collect_vec();
        Program {
            instructions,
            errors,
            labels,
        }
    }

    pub fn get_line(&self, line: usize) -> Result<&Instruction, ICError> {
        self.instructions
            .get(line)
            .ok_or(ICError::InstructionPointerOutOfRange(line))
    }
}

pub fn f64_to_i64(f: f64, signed: bool) -> i64 {
    let mut num: i64 = (f % (1i64 << 53) as f64) as i64;
    if !signed {
        num &= (1i64 << 54) - 1;
    }
    num
}
pub fn i64_to_f64(i: i64) -> f64 {
    const MASK: i64 = 1 << 53;
    let flag: bool = (i & MASK) != 0;
    let mut i = i & (MASK - 1);
    if flag {
        i |= -MASK;
    }
    i as f64
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use stationeers_data::enums::prefabs::StationpediaPrefab;

    use crate::vm::{
        object::{
            templates::{FrozenObject, ObjectInfo, Prefab},
            ObjectID,
        },
        VM,
    };

    static INIT: std::sync::Once = std::sync::Once::new();

    fn setup() -> color_eyre::Result<(Rc<VM>, ObjectID, ObjectID)> {
        INIT.call_once(|| {
            let _ = color_eyre::install();
        });

        println!("building VM");
        let vm = VM::new();

        println!("VM built");

        let frozen_ic = FrozenObject {
            obj_info: ObjectInfo::with_prefab(Prefab::Hash(
                StationpediaPrefab::ItemIntegratedCircuit10 as i32,
            )),
            database_template: true,
            template: None,
        };

        println!("Adding IC");
        let ic = vm.add_object_from_frozen(frozen_ic)?;
        let frozen_circuit_holder = FrozenObject {
            obj_info: ObjectInfo::with_prefab(Prefab::Hash(
                StationpediaPrefab::StructureCircuitHousing as i32,
            )),
            database_template: true,
            template: None,
        };
        println!("Adding circuit holder");
        let ch = vm.add_object_from_frozen(frozen_circuit_holder)?;
        println!("socketing ic into circuit holder");
        vm.set_slot_occupant(ch, 0, Some(ic), 1)?;
        Ok((vm, ch, ic))
    }

    #[test]
    fn batch_modes() -> color_eyre::Result<()> {
        let (vm, ch, ic) = setup()?;
        eprintln!("Beginning batch mode test");
        let ic_chip = vm.get_object(ic).unwrap();
        let circuit_holder = vm.get_object(ch).unwrap();
        eprintln!("IC Chip: {ic_chip:?}");
        eprintln!("Circuit Holder: {circuit_holder:?}");
        vm.set_code(
            ic,
            r#"lb r0 HASH("ItemActiveVent") On Sum
            lb r1 HASH("ItemActiveVent") On Maximum
            lb r2 HASH("ItemActiveVent") On Minimum"#,
        )?;
        vm.step_programmable(ic, false)?;
        let r0 = ic_chip
            .borrow()
            .as_integrated_circuit()
            .unwrap()
            .get_register(0, 0)
            .unwrap();
        assert_eq!(r0, 0.0);
        vm.step_programmable(ic, false)?;
        let r1 = ic_chip
            .borrow()
            .as_integrated_circuit()
            .unwrap()
            .get_register(0, 1)
            .unwrap();
        assert_eq!(r1, f64::NEG_INFINITY);
        vm.step_programmable(ic, false)?;
        let r2 = ic_chip
            .borrow()
            .as_integrated_circuit()
            .unwrap()
            .get_register(0, 2)
            .unwrap();
        assert_eq!(r2, f64::INFINITY);
        Ok(())
    }

    #[test]
    fn stack() -> color_eyre::Result<()> {
        let (vm, ch, ic) = setup()?;
        eprintln!("Beginning stack test");
        let ic_chip = vm.get_object(ic).unwrap();
        let circuit_holder = vm.get_object(ch).unwrap();
        eprintln!("IC Chip: {ic_chip:?}");
        eprintln!("Circuit Holder: {circuit_holder:?}");
        vm.set_code(
            ic,
            r#"push 100
            push 10
            pop r0
            push 1000
            peek r1
            poke 1 20
            pop r2
            "#,
        )?;
        vm.step_programmable(ic, false)?;
        let stack0 = ic_chip
            .borrow()
            .as_integrated_circuit()
            .unwrap()
            .get_stack(0.0)?;
        assert_eq!(stack0, 100.0);
        vm.step_programmable(ic, false)?;
        let stack1 = ic_chip
            .borrow()
            .as_integrated_circuit()
            .unwrap()
            .get_stack(1.0)?;
        assert_eq!(stack1, 10.0);
        vm.step_programmable(ic, false)?;
        let r0 = ic_chip
            .borrow()
            .as_integrated_circuit()
            .unwrap()
            .get_register(0, 0)
            .unwrap();
        assert_eq!(r0, 10.0);
        vm.step_programmable(ic, false)?;
        let stack1 = ic_chip
            .borrow()
            .as_integrated_circuit()
            .unwrap()
            .get_stack(1.0)?;
        assert_eq!(stack1, 1000.0);
        vm.step_programmable(ch, false)?;
        let r1 = ic_chip
            .borrow()
            .as_integrated_circuit()
            .unwrap()
            .get_register(0, 1)
            .unwrap();
        assert_eq!(r1, 1000.0);
        vm.step_programmable(ch, false)?;
        let stack1 = ic_chip
            .borrow()
            .as_integrated_circuit()
            .unwrap()
            .get_stack(1.0)?;
        assert_eq!(stack1, 20.0);
        vm.step_programmable(ch, false)?;
        let r2 = ic_chip
            .borrow()
            .as_integrated_circuit()
            .unwrap()
            .get_register(0, 2)
            .unwrap();
        assert_eq!(r2, 20.0);
        Ok(())
    }
}
