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
    enums::script::LogicType,
    templates::{
        ConsumerInfo, DeviceInfo, FabricatorInfo, InternalAtmoInfo, ItemInfo, SuitInfo, ThermalInfo,
    },
};
use std::{collections::BTreeMap, rc::Rc};

#[derive(ObjectInterface!, GWThermal!, GWInternalAtmo!, GWStructure!)]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Structure
}))]
pub struct Generic {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub small_grid: bool,
}

#[derive(ObjectInterface!, GWThermal!, GWInternalAtmo!, GWStructure!, GWStorage!)]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Structure, Storage
}))]
pub struct GenericStorage {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub small_grid: bool,
    pub slots: Vec<Slot>,
}

#[derive(
    ObjectInterface!,
    GWThermal!, GWInternalAtmo!,
    GWStructure!, GWStorage!, GWLogicable!
)]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Structure, Storage, Logicable
}))]
pub struct GenericLogicable {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub small_grid: bool,
    pub slots: Vec<Slot>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub modes: Option<BTreeMap<u32, String>>,
}

#[derive(
    ObjectInterface!,
    GWThermal!, GWInternalAtmo!,
    GWStructure!, GWStorage!, GWLogicable!,
    GWDevice!
)]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Structure, Storage, Logicable, Device
}))]
pub struct GenericLogicableDevice {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub small_grid: bool,
    pub slots: Vec<Slot>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub modes: Option<BTreeMap<u32, String>>,
    pub device_info: DeviceInfo,
    pub connections: Vec<Connection>,
    pub pins: Option<Vec<Option<ObjectID>>>,
    pub reagents: Option<BTreeMap<i32, f64>>,
}

#[derive(
    ObjectInterface!,
    GWThermal!, GWInternalAtmo!,
    GWStructure!, GWStorage!, GWLogicable!,
    GWDevice!, GWCircuitHolderDevice!
)]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Structure, Storage, Logicable, Device,
    CircuitHolder
}))]
pub struct GenericCircuitHolder {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub small_grid: bool,
    pub slots: Vec<Slot>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub modes: Option<BTreeMap<u32, String>>,
    pub device_info: DeviceInfo,
    pub connections: Vec<Connection>,
    pub pins: Option<Vec<Option<ObjectID>>>,
    pub reagents: Option<BTreeMap<i32, f64>>,
    pub error: i32,
}

#[derive(
    ObjectInterface!,
    GWThermal!, GWInternalAtmo!,
    GWStructure!, GWStorage!, GWLogicable!,
    GWDevice!
)]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Structure, Storage, Logicable, Device
}))]
pub struct GenericLogicableDeviceConsumer {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub small_grid: bool,
    pub slots: Vec<Slot>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub modes: Option<BTreeMap<u32, String>>,
    pub device_info: DeviceInfo,
    pub connections: Vec<Connection>,
    pub pins: Option<Vec<Option<ObjectID>>>,
    pub reagents: Option<BTreeMap<i32, f64>>,
    pub consumer_info: ConsumerInfo,
}

#[derive(
    ObjectInterface!,
    GWThermal!, GWInternalAtmo!,
    GWStructure!, GWStorage!,
    GWLogicable!, GWDevice!,
    GWMemoryReadable!, GWMemoryWritable!
)]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Structure, Storage, Logicable, Device, MemoryReadable
}))]
pub struct GenericLogicableDeviceMemoryReadable {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
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

#[derive(
    ObjectInterface!,
    GWThermal!, GWInternalAtmo!,
    GWStructure!, GWStorage!, GWLogicable!,
    GWDevice!, GWMemoryReadable!, GWMemoryWritable!
)]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Structure, Storage, Logicable, Device, MemoryReadable
}))]
pub struct GenericLogicableDeviceConsumerMemoryReadable {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub small_grid: bool,
    pub slots: Vec<Slot>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub modes: Option<BTreeMap<u32, String>>,
    pub device_info: DeviceInfo,
    pub connections: Vec<Connection>,
    pub pins: Option<Vec<Option<ObjectID>>>,
    pub reagents: Option<BTreeMap<i32, f64>>,
    pub consumer_info: ConsumerInfo,
    pub fabricator_info: Option<FabricatorInfo>,
    pub memory: Vec<f64>,
}

#[derive(
    ObjectInterface!,
    GWThermal!, GWInternalAtmo!,
    GWStructure!, GWStorage!,
    GWLogicable!, GWDevice!, GWMemoryReadable!, GWMemoryWritable!
)]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Structure, Storage, Logicable, Device, MemoryReadable, MemoryWritable
}))]
pub struct GenericLogicableDeviceMemoryReadWriteable {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
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

#[derive(
    ObjectInterface!,
    GWThermal!, GWInternalAtmo!,
    GWStructure!, GWStorage!, GWLogicable!,
    GWDevice!, GWMemoryReadable!, GWMemoryWritable!
)]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Structure, Storage, Logicable, Device, MemoryReadable, MemoryWritable
}))]
pub struct GenericLogicableDeviceConsumerMemoryReadWriteable {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub small_grid: bool,
    pub slots: Vec<Slot>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub modes: Option<BTreeMap<u32, String>>,
    pub device_info: DeviceInfo,
    pub connections: Vec<Connection>,
    pub pins: Option<Vec<Option<ObjectID>>>,
    pub reagents: Option<BTreeMap<i32, f64>>,
    pub consumer_info: ConsumerInfo,
    pub fabricator_info: Option<FabricatorInfo>,
    pub memory: Vec<f64>,
}

#[derive(ObjectInterface!, GWThermal!, GWInternalAtmo!, GWItem!)]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Item
}))]
pub struct GenericItem {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
    pub damage: Option<f32>,
}

#[derive(ObjectInterface!, GWThermal!, GWInternalAtmo!, GWItem!, GWStorage! )]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Item, Storage
}))]
pub struct GenericItemStorage {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
    pub damage: Option<f32>,
    pub slots: Vec<Slot>,
}

#[derive(ObjectInterface!, GWThermal!, GWInternalAtmo!, GWItem!, GWStorage! )]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Item, Storage
}))]
pub struct GenericItemConsumer {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
    pub damage: Option<f32>,
    pub slots: Vec<Slot>,
    pub consumer_info: ConsumerInfo,
}

#[derive(
    ObjectInterface!,
    GWThermal!, GWInternalAtmo!,
    GWItem!, GWStorage!, GWLogicable!
)]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Item, Storage, Logicable
}))]
pub struct GenericItemLogicable {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
    pub damage: Option<f32>,
    pub slots: Vec<Slot>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub modes: Option<BTreeMap<u32, String>>,
}

#[derive(
    ObjectInterface!,
    GWThermal!, GWInternalAtmo!,
    GWItem!, GWStorage!, GWLogicable!,
    GWMemoryReadable!
)]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Item, Storage, Logicable, MemoryReadable
}))]
pub struct GenericItemLogicableMemoryReadable {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
    pub damage: Option<f32>,
    pub slots: Vec<Slot>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub modes: Option<BTreeMap<u32, String>>,
    pub memory: Vec<f64>,
}

#[derive(
    ObjectInterface!,
    GWThermal!, GWInternalAtmo!,
    GWItem!, GWStorage!, GWLogicable!,
    GWMemoryReadable!, GWMemoryWritable!
)]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Item, Storage, Logicable, MemoryReadable, MemoryWritable
}))]
pub struct GenericItemLogicableMemoryReadWriteable {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
    pub damage: Option<f32>,
    pub slots: Vec<Slot>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub modes: Option<BTreeMap<u32, String>>,
    pub memory: Vec<f64>,
}

#[derive(
    ObjectInterface!,
    GWThermal!, GWInternalAtmo!,
    GWItem!, GWStorage!, GWLogicable!,
    GWCircuitHolderItem!
)]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Item, Storage, Logicable,
    CircuitHolder
}))]
pub struct GenericItemCircuitHolder {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
    pub damage: Option<f32>,
    pub slots: Vec<Slot>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub modes: Option<BTreeMap<u32, String>>,
    pub error: i32,
}

#[derive(
    ObjectInterface!,
    GWThermal!, GWInternalAtmo!,
    GWItem!, GWStorage!, GWLogicable!,
    GWSuit!
)]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Item, Storage, Suit, Logicable
}))]
pub struct GenericItemSuitLogic {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
    pub damage: Option<f32>,
    pub slots: Vec<Slot>,
    pub suit_info: SuitInfo,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub modes: Option<BTreeMap<u32, String>>,
}

#[derive(
    ObjectInterface!,
    GWThermal!, GWInternalAtmo!,
    GWItem!, GWStorage!, GWLogicable!,
    GWMemoryReadable!, GWMemoryWritable!,
    GWSuit!, GWCircuitHolderSuit!
)]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Item, Storage, Suit, Logicable, MemoryReadable, MemoryWritable,
    CircuitHolder
}))]
pub struct GenericItemSuitCircuitHolder {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
    pub damage: Option<f32>,
    pub slots: Vec<Slot>,
    pub suit_info: SuitInfo,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub modes: Option<BTreeMap<u32, String>>,
    pub memory: Vec<f64>,
    pub error: i32,
}

#[derive(
    ObjectInterface!,
    GWThermal!, GWInternalAtmo!,
    GWItem!, GWStorage!, GWSuit!
)]
#[custom(implements(Object {
    Thermal[GWThermal::is_thermal],
    InternalAtmosphere[GWInternalAtmo::is_internal_atmo],
    Item, Storage, Suit
}))]
pub struct GenericItemSuit {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub thermal_info: Option<ThermalInfo>,
    pub internal_atmo_info: Option<InternalAtmoInfo>,
    pub item_info: ItemInfo,
    pub parent_slot: Option<ParentSlotInfo>,
    pub damage: Option<f32>,
    pub slots: Vec<Slot>,
    pub suit_info: SuitInfo,
}
