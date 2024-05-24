use std::rc::Rc;

use stationeers_data::{enums::prefabs::StationpediaPrefab, templates::ObjectTemplate};

use crate::vm::object::{
    templates::{FrozenObject, ObjectInfo, Prefab},
    VMObject,
};
use crate::vm::VM;

use super::ObjectID;

pub mod structs;

#[allow(unused)]
pub fn object_from_frozen(obj: &ObjectInfo, id: ObjectID, vm: &Rc<VM>) -> Option<VMObject> {
    let hash = match obj.prefab {
        Some(Prefab::Hash(hash)) => hash,
        Some(Prefab::Name(name)) => const_crc32::crc32(name.as_bytes()) as i32,
        None => return None,
    };

    let prefab = StationpediaPrefab::from_repr(hash);
    match prefab {
        // Some(StationpediaPrefab::ItemIntegratedCircuit10) => {
        //     Some(VMObject::new(structs::ItemIntegratedCircuit10))
        // }
        // Some(StationpediaPrefab::StructureCircuitHousing) => Some()
        // Some(StationpediaPrefab::StructureRocketCircuitHousing) => Some()
        _ => None,
    }
}
