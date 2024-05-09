use super::{macros::*, traits::*};

use crate::vm::{
    enums::script_enums::LogicType,
    object::{macros::ObjectInterface, traits::*, LogicField, Name, ObjectID, Slot},
};
use macro_rules_attribute::derive;
use std::{collections::BTreeMap, usize};

#[derive(ObjectInterface!)]
#[custom(implements(Object { }))]
pub struct Generic {
    #[custom(object_id)]
    id: ObjectID,
    #[custom(object_prefab)]
    prefab: Name,
}

#[derive(ObjectInterface!, GWLogicable!)]
#[custom(implements(Object { Logicable }))]
pub struct GenericLogicable {
    #[custom(object_id)]
    id: ObjectID,
    #[custom(object_prefab)]
    prefab: Name,
    name: Option<Name>,
    fields: BTreeMap<LogicType, LogicField>,
    slots: Vec<Slot>,
}

#[derive(ObjectInterface!, GWLogicable!, GWDevice!)]
#[custom(implements(Object { Logicable, Device }))]
pub struct GenericLogicableDevice {
    #[custom(object_id)]
    id: ObjectID,
    #[custom(object_prefab)]
    prefab: Name,
    name: Option<Name>,
    fields: BTreeMap<LogicType, LogicField>,
    slots: Vec<Slot>,
}

#[derive(ObjectInterface!, GWLogicable!, GWMemoryReadable!)]
#[custom(implements(Object { Logicable, MemoryReadable }))]
pub struct GenericLogicableMemoryReadable {
    #[custom(object_id)]
    id: ObjectID,
    #[custom(object_prefab)]
    prefab: Name,
    name: Option<Name>,
    fields: BTreeMap<LogicType, LogicField>,
    slots: Vec<Slot>,
    memory: Vec<f64>,
}

#[derive(ObjectInterface!, GWLogicable!, GWMemoryReadable!, GWMemoryWritable!)]
#[custom(implements(Object { Logicable, MemoryReadable, MemoryWritable }))]
pub struct GenericLogicableMemoryReadWritable {
    #[custom(object_id)]
    id: ObjectID,
    #[custom(object_prefab)]
    prefab: Name,
    name: Option<Name>,
    fields: BTreeMap<LogicType, LogicField>,
    slots: Vec<Slot>,
    memory: Vec<f64>,
}

#[derive(ObjectInterface!, GWLogicable!, GWDevice!, GWMemoryReadable!, GWMemoryWritable!)]
#[custom(implements(Object { Logicable, Device, MemoryReadable }))]
pub struct GenericLogicableDeviceMemoryReadable {
    #[custom(object_id)]
    id: ObjectID,
    #[custom(object_prefab)]
    prefab: Name,
    name: Option<Name>,
    fields: BTreeMap<LogicType, LogicField>,
    slots: Vec<Slot>,
    memory: Vec<f64>,
}

#[derive(ObjectInterface!, GWLogicable!, GWDevice!, GWMemoryReadable!, GWMemoryWritable!)]
#[custom(implements(Object { Logicable, Device, MemoryReadable, MemoryWritable }))]
pub struct GenericLogicableDeviceMemoryReadWriteablable {
    #[custom(object_id)]
    id: ObjectID,
    #[custom(object_prefab)]
    prefab: Name,
    name: Option<Name>,
    fields: BTreeMap<LogicType, LogicField>,
    slots: Vec<Slot>,
    memory: Vec<f64>,
}
