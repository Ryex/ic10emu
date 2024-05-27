use std::rc::Rc;

use stationeers_data::{
    enums::prefabs::StationpediaPrefab,
    templates::{ObjectTemplate},
};

use crate::{
    errors::TemplateError,
    vm::object::{
        templates::{ObjectInfo, Prefab},
        Name, VMObject,
    },
};
use crate::{
    interpreter::Program,
    vm::{object::LogicField, VM},
};
use strum::EnumProperty;

use super::ObjectID;

pub mod structs;

#[allow(unused)]
pub fn object_from_frozen(
    obj: &ObjectInfo,
    id: ObjectID,
    vm: &Rc<VM>,
) -> Result<Option<VMObject>, TemplateError> {
    #[allow(clippy::cast_possible_wrap)]
    let hash = match &obj.prefab {
        Some(Prefab::Hash(hash)) => *hash,
        Some(Prefab::Name(name)) => const_crc32::crc32(name.as_bytes()) as i32,
        None => return Ok(None),
    };

    let prefab = StationpediaPrefab::from_repr(hash);
    #[allow(clippy::match_single_binding)]
    match prefab {
        Some(prefab @ StationpediaPrefab::ItemIntegratedCircuit10) => {
            let template = vm
                .get_template(Prefab::Hash(hash))
                .ok_or(TemplateError::NoTemplateForPrefab(Prefab::Hash(hash)))?;
            let ObjectTemplate::ItemLogicMemory(template) = template else {
                return Err(TemplateError::IncorrectTemplate(
                    "ItemIntegratedCircuit10".to_string(),
                    Prefab::Name("ItemIntegratedCircuit10".to_string()),
                ));
            };

            Ok(Some(VMObject::new(structs::ItemIntegratedCircuit10 {
                id,
                vm: vm.clone(),
                name: Name::new(
                    &(obj
                        .name
                        .clone()
                        .unwrap_or_else(|| prefab.get_str("name").unwrap().to_string())),
                ),
                prefab: Name::from_prefab_name(&prefab.to_string()),
                fields: template
                    .logic
                    .logic_types
                    .iter()
                    .map(|(key, access)| {
                        (
                            *key,
                            LogicField {
                                field_type: *access,
                                value: obj
                                    .logic_values
                                    .as_ref()
                                    .and_then(|values| values.get(key))
                                    .copied()
                                    .unwrap_or(0.0),
                            },
                        )
                    })
                    .collect(),
                memory: obj
                    .memory
                    .clone()
                    .map(TryInto::try_into)
                    .transpose()
                    .map_err(|vec: Vec<f64>| TemplateError::MemorySize(vec.len(), 512))?
                    .unwrap_or_else(|| [0.0f64; 512]),
                parent_slot: None,
                registers: obj
                    .circuit
                    .as_ref()
                    .map(|circuit| circuit.registers.clone().try_into())
                    .transpose()
                    .map_err(|vec: Vec<f64>| TemplateError::MemorySize(vec.len(), 18))?
                    .unwrap_or_else(|| [0.0f64; 18]),
                ip: obj
                    .circuit
                    .as_ref()
                    .map(|circuit| circuit.instruction_pointer as usize)
                    .unwrap_or(0),
                next_ip: 0,
                ic: obj
                    .circuit
                    .as_ref()
                    .map(|circuit| circuit.yield_instruciton_count)
                    .unwrap_or(0),
                aliases: obj
                    .circuit
                    .as_ref()
                    .map(|circuit| circuit.aliases.clone())
                    .unwrap_or_default(),
                defines: obj
                    .circuit
                    .as_ref()
                    .map(|circuit| circuit.defines.clone())
                    .unwrap_or_default(),
                pins: obj
                    .device_pins
                    .as_ref()
                    .map(|pins| {
                        (0..6)
                            .map(|index| pins.get(&index).copied())
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap() // fixed sized iterator into array should not panic
                    })
                    .unwrap_or_default(),
                state: obj
                    .circuit
                    .as_ref()
                    .map(|circuit| circuit.state.clone())
                    .unwrap_or(crate::interpreter::ICState::Start),
                code: obj.source_code.clone().unwrap_or_default(),
                damage: obj.damage.unwrap_or(0.0),
                program: obj
                    .source_code
                    .as_ref()
                    .map(|code| {
                        if code.is_empty() {
                            Program::default()
                        } else {
                            Program::from_code_with_invalid(code)
                        }
                    })
                    .unwrap_or_default(),
            })))
        }
        // Some(StationpediaPrefab::StructureCircuitHousing) => Some()
        // Some(StationpediaPrefab::StructureRocketCircuitHousing) => Some()
        _ => Ok(None),
    }
}
