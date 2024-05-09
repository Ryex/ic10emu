use std::str::FromStr;

use crate::vm::enums::prefabs::StationpediaPrefab;
use crate::vm::object::VMObject;

#[allow(unused)]
pub enum PrefabTemplate {
    Hash(i32),
    Name(String),
}

#[allow(unused)]
pub fn object_from_prefab_template(template: &PrefabTemplate) -> Option<VMObject> {
    let prefab = match template {
        PrefabTemplate::Hash(hash) => StationpediaPrefab::from_repr(*hash),
        PrefabTemplate::Name(name) => StationpediaPrefab::from_str(name).ok(),
    };
    match prefab {
        // Some(StationpediaPrefab::ItemIntegratedCircuit10) => Some()
        // Some(StationpediaPrefab::StructureCircuitHousing) => Some()
        // Some(StationpediaPrefab::StructureRocketCircuitHousing) => Some()
        _ => None,
    }
}
