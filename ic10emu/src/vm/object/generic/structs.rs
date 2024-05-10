use super::{macros::*, traits::*};

use crate::vm::{
    enums::script_enums::LogicType,
    object::{
        macros::ObjectInterface, templates::ItemInfo, traits::*, LogicField, Name, ObjectID, Slot,
    },
};
use macro_rules_attribute::derive;
use std::collections::BTreeMap;

#[derive(ObjectInterface!)]
#[custom(implements(Object { }))]
pub struct Generic {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
}

#[derive(ObjectInterface!, GWStorage!)]
#[custom(implements(Object { Storage }))]
pub struct GenericStorage {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    pub slots: Vec<Slot>,
}

#[derive(ObjectInterface!, GWStorage!, GWLogicable!)]
#[custom(implements(Object { Storage, Logicable }))]
pub struct GenericLogicable {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub slots: Vec<Slot>,
}

#[derive(ObjectInterface!, GWStorage!, GWLogicable!, GWDevice!)]
#[custom(implements(Object { Storage, Logicable, Device }))]
pub struct GenericLogicableDevice {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub slots: Vec<Slot>,
}

#[derive(ObjectInterface!, GWStorage!, GWLogicable!, GWDevice!, GWMemoryReadable!, GWMemoryWritable!)]
#[custom(implements(Object { Storage, Logicable, Device, MemoryReadable }))]
pub struct GenericLogicableDeviceMemoryReadable {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub slots: Vec<Slot>,
    pub memory: Vec<f64>,
}

#[derive(ObjectInterface!, GWStorage!, GWLogicable!, GWDevice!, GWMemoryReadable!, GWMemoryWritable!)]
#[custom(implements(Object { Storage, Logicable, Device, MemoryReadable, MemoryWritable }))]
pub struct GenericLogicableDeviceMemoryReadWriteable {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub slots: Vec<Slot>,
    pub memory: Vec<f64>,
}

#[derive(ObjectInterface!, GWItem!)]
#[custom(implements(Object { Item }))]
pub struct GenericItem {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
}

#[derive(ObjectInterface!, GWItem!, GWStorage! )]
#[custom(implements(Object { Item, Storage }))]
pub struct GenericItemStorage {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
    pub slots: Vec<Slot>,
}

#[derive(ObjectInterface!, GWItem!, GWStorage!, GWLogicable! )]
#[custom(implements(Object { Item, Storage, Logicable }))]
pub struct GenericItemLogicable {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub slots: Vec<Slot>,
}

#[derive(ObjectInterface!, GWItem!, GWStorage!, GWLogicable!, GWMemoryReadable! )]
#[custom(implements(Object { Item, Storage, Logicable, MemoryReadable }))]
pub struct GenericItemLogicableMemoryReadable {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub slots: Vec<Slot>,
    pub memory: Vec<f64>,
}

#[derive(ObjectInterface!, GWItem!, GWStorage!, GWLogicable!, GWMemoryReadable!, GWMemoryWritable! )]
#[custom(implements(Object { Item, Storage, Logicable, MemoryReadable, MemoryWritable }))]
pub struct GenericItemLogicableMemoryReadWriteable {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub slots: Vec<Slot>,
    pub memory: Vec<f64>,
}
