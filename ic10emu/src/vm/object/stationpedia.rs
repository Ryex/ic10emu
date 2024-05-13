use crate::vm::enums::prefabs::StationpediaPrefab;
use crate::vm::object::VMObject;

use super::templates::ObjectTemplate;
use super::ObjectID;

pub mod structs;

#[allow(unused)]
pub fn object_from_prefab_template(template: &ObjectTemplate, id: ObjectID) -> Option<VMObject> {
    let prefab = StationpediaPrefab::from_repr(template.prefab_info().prefab_hash);
    match prefab {
        Some(StationpediaPrefab::ItemIntegratedCircuit10) => {
            Some(VMObject::new(structs::ItemIntegratedCircuit10))
        }
        // Some(StationpediaPrefab::StructureCircuitHousing) => Some()
        // Some(StationpediaPrefab::StructureRocketCircuitHousing) => Some()
        _ => None,
    }
}
