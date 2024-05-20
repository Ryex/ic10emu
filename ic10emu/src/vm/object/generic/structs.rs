use super::{macros::*, traits::*};

use crate::{
    network::Connection,
    vm::{
        object::{macros::ObjectInterface, traits::*, LogicField, Name, ObjectID, Slot},
        VM,
    },
};
use macro_rules_attribute::derive;
use stationeers_data::{
    enums::script_enums::LogicType,
    templates::{DeviceInfo, ItemInfo},
};
use std::{collections::BTreeMap, rc::Rc};

#[derive(ObjectInterface!, GWStructure!)]
#[custom(implements(Object { Structure }))]
pub struct Generic {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub small_grid: bool,
}

#[derive(ObjectInterface!, GWStructure!, GWStorage!)]
#[custom(implements(Object { Structure, Storage }))]
pub struct GenericStorage {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub small_grid: bool,
    pub slots: Vec<Slot>,
}

#[derive(ObjectInterface!, GWStructure!, GWStorage!, GWLogicable!)]
#[custom(implements(Object { Structure, Storage, Logicable }))]
pub struct GenericLogicable {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub small_grid: bool,
    pub slots: Vec<Slot>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub modes: Option<BTreeMap<u32, String>>,
}

#[derive(ObjectInterface!, GWStructure!, GWStorage!, GWLogicable!, GWDevice!)]
#[custom(implements(Object { Structure, Storage, Logicable, Device }))]
pub struct GenericLogicableDevice {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub small_grid: bool,
    pub slots: Vec<Slot>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub modes: Option<BTreeMap<u32, String>>,
    pub device_info: DeviceInfo,
    pub connections: Vec<Connection>,
    pub pins: Option<Vec<Option<ObjectID>>>,
    pub reagents: Option<BTreeMap<i32, f64>>,
}

#[derive(ObjectInterface!, GWStructure!, GWStorage!, GWLogicable!, GWDevice!, GWMemoryReadable!, GWMemoryWritable!)]
#[custom(implements(Object { Structure, Storage, Logicable, Device, MemoryReadable }))]
pub struct GenericLogicableDeviceMemoryReadable {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub small_grid: bool,
    pub slots: Vec<Slot>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub modes: Option<BTreeMap<u32, String>>,
    pub device_info: DeviceInfo,
    pub connections: Vec<Connection>,
    pub pins: Option<Vec<Option<ObjectID>>>,
    pub reagents: Option<BTreeMap<i32, f64>>,
    pub memory: Vec<f64>,
}

#[derive(ObjectInterface!, GWStructure!, GWStorage!, GWLogicable!, GWDevice!, GWMemoryReadable!, GWMemoryWritable!)]
#[custom(implements(Object { Structure, Storage, Logicable, Device, MemoryReadable, MemoryWritable }))]
pub struct GenericLogicableDeviceMemoryReadWriteable {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub small_grid: bool,
    pub slots: Vec<Slot>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub modes: Option<BTreeMap<u32, String>>,
    pub device_info: DeviceInfo,
    pub connections: Vec<Connection>,
    pub pins: Option<Vec<Option<ObjectID>>>,
    pub reagents: Option<BTreeMap<i32, f64>>,
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
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
    pub damage: Option<f32>,
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
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
    pub damage: Option<f32>,
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
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
    pub damage: Option<f32>,
    pub slots: Vec<Slot>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub modes: Option<BTreeMap<u32, String>>,
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
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
    pub damage: Option<f32>,
    pub slots: Vec<Slot>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub modes: Option<BTreeMap<u32, String>>,
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
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
    pub damage: Option<f32>,
    pub slots: Vec<Slot>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub modes: Option<BTreeMap<u32, String>>,
    pub memory: Vec<f64>,
}
