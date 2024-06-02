use crate::{
    network::Connection,
    vm::object::{
        errors::{LogicError, MemoryError},
        traits::*,
        LogicField, MemoryAccess, ObjectID, Slot, VMObject,
    },
};

use stationeers_data::{
    enums::{
        basic::{Class, GasType, SortingClass},
        script::{LogicSlotType, LogicType},
    },
    templates::{DeviceInfo, InternalAtmoInfo, ItemInfo, SuitInfo, ThermalInfo},
};
use std::{collections::BTreeMap, usize};
use strum::IntoEnumIterator;

pub trait GWThermal {
    fn is_thermal(&self) -> bool;
    fn thermal_info(&self) -> &ThermalInfo;
}

impl<T: GWThermal + Object> Thermal for T {
    fn get_radiation_factor(&self) -> f32 {
        self.thermal_info().radiation_factor
    }
    fn get_convection_factor(&self) -> f32 {
        self.thermal_info().convection_factor
    }
}

pub trait GWInternalAtmo {
    fn is_internal_atmo(&self) -> bool;
    fn internal_atmo_info(&self) -> &InternalAtmoInfo;
}

impl<T: GWInternalAtmo + Object> InternalAtmosphere for T {
    fn get_volume(&self) -> f64 {
        self.internal_atmo_info().volume as f64
    }
}

pub trait GWStructure {
    fn small_grid(&self) -> bool;
}

impl<T: GWStructure + Object> Structure for T {
    fn is_small_grid(&self) -> bool {
        self.small_grid()
    }
}

pub trait GWStorage {
    fn slots(&self) -> &Vec<Slot>;
    fn slots_mut(&mut self) -> &mut Vec<Slot>;
}

impl<T: GWStorage + Object> Storage for T {
    fn slots_count(&self) -> usize {
        self.slots().len()
    }
    fn get_slot(&self, index: usize) -> Option<&Slot> {
        self.slots().get(index)
    }
    fn get_slot_mut(&mut self, index: usize) -> Option<&mut Slot> {
        self.slots_mut().get_mut(index)
    }
    fn get_slots(&self) -> Vec<&Slot> {
        self.slots().iter().collect()
    }
    fn get_slots_mut(&mut self) -> Vec<&mut Slot> {
        self.slots_mut().iter_mut().collect()
    }
}

pub trait GWLogicable: Storage {
    fn fields(&self) -> &BTreeMap<LogicType, LogicField>;
    fn fields_mut(&mut self) -> &mut BTreeMap<LogicType, LogicField>;
    fn known_modes(&self) -> Option<&BTreeMap<u32, String>>;
}

impl<T: GWLogicable + Object> Logicable for T {
    fn prefab_hash(&self) -> i32 {
        self.get_prefab().hash
    }
    fn name_hash(&self) -> i32 {
        self.get_name().hash
    }
    fn is_logic_readable(&self) -> bool {
        true
    }
    fn is_logic_writeable(&self) -> bool {
        LogicType::iter().any(|lt| self.can_logic_write(lt))
    }
    fn can_logic_read(&self, lt: LogicType) -> bool {
        match lt {
            LogicType::PrefabHash | LogicType::NameHash | LogicType::ReferenceId => true,
            _ => self
                .fields()
                .get(&lt)
                .map(|field| {
                    matches!(
                        field.field_type,
                        MemoryAccess::Read | MemoryAccess::ReadWrite
                    )
                })
                .unwrap_or(false),
        }
    }
    fn can_logic_write(&self, lt: LogicType) -> bool {
        match lt {
            LogicType::PrefabHash | LogicType::NameHash | LogicType::ReferenceId => false,
            _ => self
                .fields()
                .get(&lt)
                .map(|field| {
                    matches!(
                        field.field_type,
                        MemoryAccess::Write | MemoryAccess::ReadWrite
                    )
                })
                .unwrap_or(false),
        }
    }
    fn get_logic(&self, lt: LogicType) -> Result<f64, LogicError> {
        match lt {
            LogicType::PrefabHash => Ok(self.get_prefab().hash as f64),
            LogicType::NameHash => Ok(self.get_name().hash as f64),
            LogicType::ReferenceId => Ok(*self.get_id() as f64),
            _ => self
                .fields()
                .get(&lt)
                .and_then(|field| match field.field_type {
                    MemoryAccess::Read | MemoryAccess::ReadWrite => Some(field.value),
                    _ => None,
                })
                .ok_or(LogicError::CantRead(lt)),
        }
    }
    fn set_logic(&mut self, lt: LogicType, value: f64, force: bool) -> Result<(), LogicError> {
        match lt {
            LogicType::PrefabHash | LogicType::NameHash | LogicType::ReferenceId => {
                Err(LogicError::CantWrite(lt))
            }
            _ => self
                .fields_mut()
                .get_mut(&lt)
                .ok_or(LogicError::CantWrite(lt))
                .and_then(|field| match field.field_type {
                    MemoryAccess::Write | MemoryAccess::ReadWrite => {
                        field.value = value;
                        Ok(())
                    }
                    _ if force => {
                        field.value = value;
                        Ok(())
                    }
                    _ => Err(LogicError::CantWrite(lt)),
                }),
        }
    }
    fn can_slot_logic_read(&self, slt: LogicSlotType, index: f64) -> bool {
        if index < 0.0 {
            false
        } else {
            use LogicSlotType::*;
            if matches!(
                slt,
                Occupied
                    | OccupantHash
                    | Quantity
                    | Class
                    | MaxQuantity
                    | PrefabHash
                    | SortingClass
                    | ReferenceId
            ) {
                return true;
            }
            self.get_slot(index as usize)
                .map(|slot| slot.readable_logic.contains(&slt))
                .unwrap_or(false)
        }
    }
    fn get_slot_logic(&self, slt: LogicSlotType, index: f64) -> Result<f64, LogicError> {
        if index < 0.0 {
            return Err(LogicError::SlotIndexOutOfRange(index, self.slots_count()));
        }
        self.get_slot(index as usize)
            .ok_or_else(|| LogicError::SlotIndexOutOfRange(index, self.slots_count()))
            .and_then(|slot| {
                use LogicSlotType::*;
                let occupant = slot
                    .occupant
                    .as_ref()
                    .and_then(|info| self.get_vm().get_object(info.id));
                match slt {
                    Occupied => {
                        if slot.occupant.is_some() {
                            Ok(1.0)
                        } else {
                            Ok(0.0)
                        }
                    }
                    Quantity => {
                        if let Some(info) = &slot.occupant {
                            Ok(info.quantity as f64)
                        } else {
                            Ok(0.0)
                        }
                    }
                    Class => {
                        if slot.occupant.is_some() {
                            Ok(slot.typ as i32 as f64)
                        } else {
                            Ok(0.0)
                        }
                    }
                    OccupantHash | PrefabHash => {
                        if let Some(occupant) = occupant {
                            Ok(occupant.borrow().get_prefab().hash as f64)
                        } else {
                            Ok(0.0)
                        }
                    }
                    MaxQuantity => {
                        if let Some(occupant) = occupant {
                            Ok(occupant
                                .borrow()
                                .as_item()
                                .map(|item| item.max_quantity() as f64)
                                .ok_or(LogicError::CantSlotRead(slt, index))?)
                        } else {
                            Ok(0.0)
                        }
                    }
                    SortingClass => {
                        if let Some(occupant) = occupant {
                            Ok(occupant
                                .borrow()
                                .as_item()
                                .map(|item| item.sorting_class() as i32 as f64)
                                .ok_or(LogicError::CantSlotRead(slt, index))?)
                        } else {
                            Ok(0.0)
                        }
                    }
                    ReferenceId => {
                        if let Some(occupant) = occupant {
                            Ok(*occupant.borrow().get_id() as f64)
                        } else {
                            Ok(0.0)
                        }
                    }
                    slt => {
                        if slot.readable_logic.contains(&slt) {
                            if let Some(occupant) = occupant {
                                let occupant_ref = occupant.borrow();
                                let logicable = occupant_ref
                                    .as_logicable()
                                    .ok_or(LogicError::CantSlotRead(slt, index))?;

                                match slt {
                                    Occupied | Quantity | Class | OccupantHash | PrefabHash
                                    | MaxQuantity | SortingClass | ReferenceId => Ok(0.0), // covered above
                                    LineNumber => logicable.get_logic(LogicType::LineNumber),

                                    Charge => logicable.get_logic(LogicType::Charge),
                                    ChargeRatio => logicable
                                        .as_chargeable()
                                        .map(|chargeable| chargeable.get_charge() as f64)
                                        .ok_or(LogicError::CantSlotRead(slt, index)),
                                    Open => logicable.get_logic(LogicType::Open),
                                    On => logicable.get_logic(LogicType::Open),
                                    Lock => logicable.get_logic(LogicType::Lock),
                                    FilterType => Ok(logicable
                                        .as_item()
                                        .and_then(|item| item.filter_type())
                                        .ok_or(LogicError::CantSlotRead(slt, index))?
                                        as i32
                                        as f64),
                                    Damage => logicable
                                        .as_item()
                                        .map(|item| item.get_damage() as f64)
                                        .ok_or(LogicError::CantSlotRead(slt, index)),
                                    Volume => logicable.get_logic(LogicType::Volume),
                                    Pressure => logicable.get_logic(LogicType::Pressure),
                                    PressureAir => logicable
                                        .as_suit()
                                        .map(|suit| suit.pressure_air() as f64)
                                        .ok_or(LogicError::CantSlotRead(slt, index)),
                                    PressureWaste => logicable
                                        .as_suit()
                                        .map(|suit| suit.pressure_waste() as f64)
                                        .ok_or(LogicError::CantSlotRead(slt, index)),
                                    Temperature => logicable.get_logic(LogicType::Temperature),
                                    Seeding => logicable
                                        .as_plant()
                                        .map(|plant| plant.is_seeding() as i32 as f64)
                                        .ok_or(LogicError::CantSlotRead(slt, index)),
                                    Mature => logicable
                                        .as_plant()
                                        .map(|plant| plant.is_mature() as i32 as f64)
                                        .ok_or(LogicError::CantSlotRead(slt, index)),
                                    Growth => logicable
                                        .as_plant()
                                        .map(|plant| plant.get_growth())
                                        .ok_or(LogicError::CantSlotRead(slt, index)),
                                    Health => logicable
                                        .as_plant()
                                        .map(|plant| plant.get_health())
                                        .ok_or(LogicError::CantSlotRead(slt, index)),
                                    Efficiency => logicable
                                        .as_plant()
                                        .map(|plant| plant.get_health())
                                        .ok_or(LogicError::CantSlotRead(slt, index)),

                                    // defaults
                                    None => Ok(0.0),
                                }
                            } else {
                                Ok(0.0)
                            }
                        } else {
                            Err(LogicError::CantSlotRead(slt, index))
                        }
                    }
                }
            })
    }
    fn valid_logic_types(&self) -> Vec<LogicType> {
        self.fields().keys().copied().collect()
    }
    fn known_modes(&self) -> Option<Vec<(u32, String)>> {
        self.known_modes().map(|modes| {
            modes
                .iter()
                .map(|(mode, name)| (*mode, name.clone()))
                .collect()
        })
    }
}

pub trait GWMemoryReadable {
    fn memory_size(&self) -> usize;
    fn memory(&self) -> &Vec<f64>;
}

impl<T: GWMemoryReadable + Object> MemoryReadable for T {
    fn memory_size(&self) -> usize {
        self.memory_size()
    }
    fn get_memory(&self, index: i32) -> Result<f64, MemoryError> {
        if index < 0 {
            Err(MemoryError::StackUnderflow(index, self.memory().len()))
        } else if index as usize >= self.memory().len() {
            Err(MemoryError::StackOverflow(index, self.memory().len()))
        } else {
            Ok(self.memory()[index as usize])
        }
    }
    fn get_memory_slice(&self) -> &[f64] {
        self.memory()
    }
}

pub trait GWMemoryWritable: MemoryReadable {
    fn memory_mut(&mut self) -> &mut Vec<f64>;
}

impl<T: GWMemoryWritable + MemoryReadable + Object> MemoryWritable for T {
    fn set_memory(&mut self, index: i32, val: f64) -> Result<(), MemoryError> {
        if index < 0 {
            Err(MemoryError::StackUnderflow(index, self.memory_size()))
        } else if index as usize >= self.memory_size() {
            Err(MemoryError::StackOverflow(index, self.memory_size()))
        } else {
            self.memory_mut()[index as usize] = val;
            Ok(())
        }
    }
    fn clear_memory(&mut self) {
        self.memory_mut().fill(0.0);
    }
}

pub trait GWDevice: GWLogicable + Logicable {
    fn device_info(&self) -> &DeviceInfo;
    fn connections(&self) -> &[Connection];
    fn connections_mut(&mut self) -> &mut [Connection];
    fn pins(&self) -> Option<&[Option<ObjectID>]>;
    fn pins_mut(&mut self) -> Option<&mut [Option<ObjectID>]>;
    fn reagents(&self) -> Option<&BTreeMap<i32, f64>>;
    fn reagents_mut(&mut self) -> &mut Option<BTreeMap<i32, f64>>;
}

impl<T: GWDevice + GWStorage + Object> Device for T {
    fn can_slot_logic_write(&self, slt: LogicSlotType, index: f64) -> bool {
        if index < 0.0 {
            false
        } else {
            self.get_slot(index as usize)
                .map(|slot| slot.writeable_logic.contains(&slt))
                .unwrap_or(false)
        }
    }
    fn set_slot_logic(
        &mut self,
        slt: LogicSlotType,
        index: f64,
        value: f64,
        force: bool,
    ) -> Result<(), LogicError> {
        let slots_count = self.slots_count();
        if index < 0.0 {
            return Err(LogicError::SlotIndexOutOfRange(index, slots_count));
        }
        use LogicSlotType::*;
        let vm = self.get_vm().clone();

        self.get_slot_mut(index as usize)
            .ok_or(LogicError::SlotIndexOutOfRange(index, slots_count))
            .and_then(|slot| {
                // special case, update slot quantity if >= 1
                if slt == Quantity && force && value >= 1.0 {
                    if let Some(occupant) = slot.occupant.as_mut() {
                        occupant.quantity = value as u32;
                    }
                    return Ok(());
                }
                if slot.writeable_logic.contains(&slt) {
                    let occupant = slot
                        .occupant
                        .as_ref()
                        .and_then(|info| vm.get_object(info.id));
                    if let Some(occupant) = occupant {
                        let mut occupant_ref = occupant.borrow_mut();
                        let logicable = occupant_ref
                            .as_mut_logicable()
                            .ok_or(LogicError::CantSlotWrite(slt, index))?;
                        match slt {
                            Open => logicable.set_logic(LogicType::Open, value, force),
                            On => logicable.set_logic(LogicType::On, value, force),
                            Lock => logicable.set_logic(LogicType::On, value, force),
                            // no other values are known to be writeable
                            Damage if force => {
                                logicable
                                    .as_mut_item()
                                    .map(|item| item.set_damage(value as f32))
                                    .ok_or(LogicError::CantSlotWrite(slt, index))?;
                                Ok(())
                            }

                            _ => Ok(()),
                        }
                    } else {
                        Ok(())
                    }
                } else {
                    Err(LogicError::CantSlotWrite(slt, index))
                }
            })
    }
    fn connection_list(&self) -> &[crate::network::Connection] {
        self.connections()
    }
    fn connection_list_mut(&mut self) -> &mut [Connection] {
        self.connections_mut()
    }
    fn device_pins(&self) -> Option<&[Option<ObjectID>]> {
        self.pins()
    }
    fn device_pins_mut(&mut self) -> Option<&mut [Option<ObjectID>]> {
        self.pins_mut()
    }
    fn has_reagents(&self) -> bool {
        self.device_info().has_reagents
    }
    fn has_lock_state(&self) -> bool {
        self.device_info().has_lock_state
    }
    fn has_mode_state(&self) -> bool {
        self.device_info().has_mode_state
    }
    fn has_open_state(&self) -> bool {
        self.device_info().has_open_state
    }
    fn has_on_off_state(&self) -> bool {
        self.device_info().has_on_off_state
    }
    fn has_color_state(&self) -> bool {
        self.device_info().has_color_state
    }
    fn has_activate_state(&self) -> bool {
        self.device_info().has_activate_state
    }
    fn has_atmosphere(&self) -> bool {
        self.device_info().has_atmosphere
    }
    fn get_reagents(&self) -> Vec<(i32, f64)> {
        self.reagents()
            .map(|reagents| {
                reagents
                    .iter()
                    .map(|(hash, quant)| (*hash, *quant))
                    .collect()
            })
            .unwrap_or_default()
    }
    fn set_reagents(&mut self, reagents: &[(i32, f64)]) {
        let reagents_ref = self.reagents_mut();
        *reagents_ref = Some(reagents.iter().copied().collect());
    }
    fn add_reagents(&mut self, reagents: &[(i32, f64)]) {
        let reagents_ref = self.reagents_mut();
        if let Some(ref mut reagents_ref) = reagents_ref {
            reagents_ref.extend(reagents.iter().map(|(hash, quant)| (hash, quant)));
        } else {
            *reagents_ref = Some(reagents.iter().copied().collect());
        }
    }
}

pub trait GWItem {
    fn item_info(&self) -> &ItemInfo;
    fn parent_slot(&self) -> Option<ParentSlotInfo>;
    fn set_parent_slot(&mut self, info: Option<ParentSlotInfo>);
    fn damage(&self) -> &Option<f32>;
    fn damage_mut(&mut self) -> &mut Option<f32>;
}

impl<T: GWItem + Object> Item for T {
    fn consumable(&self) -> bool {
        self.item_info().consumable
    }
    fn filter_type(&self) -> Option<GasType> {
        self.item_info().filter_type
    }
    fn ingredient(&self) -> bool {
        self.item_info().ingredient
    }
    fn max_quantity(&self) -> u32 {
        self.item_info().max_quantity
    }
    fn reagents(&self) -> Option<&BTreeMap<String, f64>> {
        self.item_info().reagents.as_ref()
    }
    fn slot_class(&self) -> Class {
        self.item_info().slot_class
    }
    fn sorting_class(&self) -> SortingClass {
        self.item_info().sorting_class
    }
    fn get_parent_slot(&self) -> Option<ParentSlotInfo> {
        self.parent_slot()
    }
    fn set_parent_slot(&mut self, info: Option<ParentSlotInfo>) {
        self.set_parent_slot(info);
    }
    fn get_damage(&self) -> f32 {
        self.damage().unwrap_or(0.0)
    }
    fn set_damage(&mut self, damage: f32) {
        self.damage_mut().replace(damage.clamp(0.0, 1.0));
    }
}

pub trait GWSuit: Storage {
    fn suit_info(&self) -> &SuitInfo;
}

impl<T: GWSuit + Item + Object> Suit for T {
    fn pressure_waste_max(&self) -> f32 {
        self.suit_info().waste_max_pressure
    }
    fn pressure_air(&self) -> f32 {
        // Game "hard" codes air tank to first slot of suits
        let result = self.get_slot(0).and_then(|slot| {
            let canister = slot
                .occupant
                .as_ref()
                .and_then(|info| self.get_vm().get_object(info.id));
            let pressure = canister.and_then(|canister| {
                canister
                    .borrow()
                    .as_logicable()
                    .and_then(|logicable| logicable.get_logic(LogicType::Pressure).ok())
            });

            pressure
        });
        result.unwrap_or(0.0) as f32
    }
    fn pressure_waste(&self) -> f32 {
        // game hard codes waste tank to second slot of suits
        let result = self.get_slot(1).and_then(|slot| {
            let canister = slot
                .occupant
                .as_ref()
                .and_then(|info| self.get_vm().get_object(info.id));
            let pressure = canister.and_then(|canister| {
                canister
                    .borrow()
                    .as_logicable()
                    .and_then(|logicable| logicable.get_logic(LogicType::Pressure).ok())
            });

            pressure
        });
        result.unwrap_or(0.0) as f32
    }
}

pub trait CircuitHolderType {}

pub struct ItemCircuitHolder;
pub struct SuitCircuitHolder;
pub struct DeviceCircuitHolder;
impl CircuitHolderType for ItemCircuitHolder {}
impl CircuitHolderType for SuitCircuitHolder {}
impl CircuitHolderType for DeviceCircuitHolder {}

pub trait GWCircuitHolder: Logicable {
    type Holder: CircuitHolderType;
    fn gw_get_error(&self) -> i32;
    fn gw_set_error(&mut self, state: i32);
}

pub trait GWCircuitHolderWrapper<T: GWCircuitHolder, H = <T as GWCircuitHolder>::Holder> {
    fn clear_error_gw(&mut self);
    fn set_error_gw(&mut self, state: i32);
    /// i32::MAX is db
    fn get_logicable_from_index_gw(
        &self,
        device: i32,
        connection: Option<usize>,
    ) -> Option<ObjectRef>;
    /// i32::MAX is db
    fn get_logicable_from_index_mut_gw(
        &mut self,
        device: i32,
        connection: Option<usize>,
    ) -> Option<ObjectRefMut>;
    fn get_logicable_from_id_gw(
        &self,
        device: ObjectID,
        connection: Option<usize>,
    ) -> Option<ObjectRef>;
    fn get_logicable_from_id_mut_gw(
        &mut self,
        device: ObjectID,
        connection: Option<usize>,
    ) -> Option<ObjectRefMut>;
    fn get_ic_gw(&self) -> Option<VMObject>;
    fn hault_and_catch_fire_gw(&mut self);
}

impl<T> CircuitHolder for T
where
    T: GWCircuitHolder,
    Self: GWCircuitHolderWrapper<T>,
{
    fn clear_error(&mut self) {
        self.clear_error_gw()
    }
    fn set_error(&mut self, state: i32) {
        self.set_error_gw(state)
    }
    /// i32::MAX is db
    fn get_logicable_from_index(
        &self,
        device: i32,
        connection: Option<usize>,
    ) -> Option<ObjectRef> {
        self.get_logicable_from_index_gw(device, connection)
    }
    /// i32::MAX is db
    fn get_logicable_from_index_mut(
        &mut self,
        device: i32,
        connection: Option<usize>,
    ) -> Option<ObjectRefMut> {
        self.get_logicable_from_index_mut_gw(device, connection)
    }
    fn get_logicable_from_id(
        &self,
        device: ObjectID,
        connection: Option<usize>,
    ) -> Option<ObjectRef> {
        self.get_logicable_from_id_gw(device, connection)
    }
    fn get_logicable_from_id_mut(
        &mut self,
        device: ObjectID,
        connection: Option<usize>,
    ) -> Option<ObjectRefMut> {
        self.get_logicable_from_id_mut_gw(device, connection)
    }
    fn get_ic(&self) -> Option<VMObject> {
        self.get_ic_gw()
    }
    fn hault_and_catch_fire(&mut self) {
        self.hault_and_catch_fire_gw()
    }
}

impl<T> GWCircuitHolderWrapper<T, DeviceCircuitHolder> for T
where
    T: GWCircuitHolder<Holder = DeviceCircuitHolder> + Device + Object,
{
    fn clear_error_gw(&mut self) {
        self.gw_set_error(0);
    }
    fn set_error_gw(&mut self, state: i32) {
        self.gw_set_error(state);
    }

    /// i32::MAX is db
    fn get_logicable_from_index_gw(
        &self,
        device: i32,
        connection: Option<usize>,
    ) -> Option<ObjectRef> {
        if device == i32::MAX {
            // self
            if let Some(connection) = connection {
                self.connection_list().get(connection).and_then(|conn| {
                    if let Connection::CableNetwork { net: Some(net), .. } = conn {
                        self.get_vm()
                            .get_network(*net)
                            .map(ObjectRef::from_vm_object)
                    } else {
                        None
                    }
                })
            } else {
                Some(ObjectRef::from_ref(self.as_object()))
            }
        } else {
            if device < 0 {
                return None;
            }
            self.device_pins().and_then(|pins| {
                pins.get(device as usize).and_then(|pin| {
                    pin.and_then(|id| self.get_vm().get_object(id).map(ObjectRef::from_vm_object))
                })
            })
        }
    }

    /// i32::MAX is db
    fn get_logicable_from_index_mut_gw(
        &mut self,
        device: i32,
        connection: Option<usize>,
    ) -> Option<ObjectRefMut> {
        if device == i32::MAX {
            // self
            if let Some(connection) = connection {
                self.connection_list().get(connection).and_then(|conn| {
                    if let Connection::CableNetwork { net: Some(net), .. } = conn {
                        self.get_vm()
                            .get_network(*net)
                            .map(ObjectRefMut::from_vm_object)
                    } else {
                        None
                    }
                })
            } else {
                Some(ObjectRefMut::from_ref(self.as_mut_object()))
            }
        } else {
            if device < 0 {
                return None;
            }
            self.device_pins().and_then(|pins| {
                pins.get(device as usize).and_then(|pin| {
                    pin.and_then(|id| {
                        self.get_vm()
                            .get_object(id)
                            .map(ObjectRefMut::from_vm_object)
                    })
                })
            })
        }
    }

    fn get_logicable_from_id_gw(
        &self,
        device: ObjectID,
        connection: Option<usize>,
    ) -> Option<ObjectRef> {
        if connection.is_some() {
            return None; // this functionality is disabled in the game, no network access via ReferenceId
        }
        if device == *self.get_id() {
            return Some(ObjectRef::from_ref(self.as_object()));
        }
        self.get_vm()
            .get_object(device)
            .map(ObjectRef::from_vm_object)
    }

    fn get_logicable_from_id_mut_gw(
        &mut self,
        device: ObjectID,
        connection: Option<usize>,
    ) -> Option<ObjectRefMut> {
        if connection.is_some() {
            return None; // this functionality is disabled in the game, no network access via ReferenceId
        }
        if device == *self.get_id() {
            return Some(ObjectRefMut::from_ref(self.as_mut_object()));
        }
        self.get_vm()
            .get_object(device)
            .map(ObjectRefMut::from_vm_object)
    }

    fn get_ic_gw(&self) -> Option<crate::vm::object::VMObject> {
        self.get_slots()
            .into_iter()
            .find(|slot| slot.typ == Class::ProgrammableChip)
            .and_then(|slot| {
                slot.occupant
                    .as_ref()
                    .and_then(|info| self.get_vm().get_object(info.id))
            })
    }

    fn hault_and_catch_fire_gw(&mut self) {
        // TODO: do something here??
    }
}

impl<T> GWCircuitHolderWrapper<T, SuitCircuitHolder> for T
where
    T: GWCircuitHolder<Holder = SuitCircuitHolder> + Suit + Object,
{
    fn clear_error_gw(&mut self) {
        self.gw_set_error(0);
    }
    fn set_error_gw(&mut self, state: i32) {
        self.gw_set_error(state);
    }

    /// 0 -> Helmet
    /// 1 -> BackPack
    /// 2 -> ToolBelt
    fn get_logicable_from_index_gw(
        &self,
        device: i32,
        _connection: Option<usize>,
    ) -> Option<ObjectRef> {
        match device {
            i32::MAX => Some(ObjectRef::from_ref(self.as_object())),
            0 => self.root_parent_human().and_then(|obj| {
                obj.borrow().as_human().and_then(|human| {
                    human.helmet_slot().occupant.as_ref().and_then(|info| {
                        self.get_vm()
                            .get_object(info.id)
                            .map(ObjectRef::from_vm_object)
                    })
                })
            }),
            1 => self.root_parent_human().and_then(|obj| {
                obj.borrow().as_human().and_then(|human| {
                    human.backpack_slot().occupant.as_ref().and_then(|info| {
                        self.get_vm()
                            .get_object(info.id)
                            .map(ObjectRef::from_vm_object)
                    })
                })
            }),
            2 => self.root_parent_human().and_then(|obj| {
                obj.borrow().as_human().and_then(|human| {
                    human.toolbelt_slot().occupant.as_ref().and_then(|info| {
                        self.get_vm()
                            .get_object(info.id)
                            .map(ObjectRef::from_vm_object)
                    })
                })
            }),
            _ => None,
        }
    }

    /// i32::MAX is db
    fn get_logicable_from_index_mut_gw(
        &mut self,
        device: i32,
        _connection: Option<usize>,
    ) -> Option<ObjectRefMut> {
        match device {
            i32::MAX => Some(ObjectRefMut::from_ref(self.as_mut_object())),
            0 => self.root_parent_human().and_then(|obj| {
                obj.borrow().as_human().and_then(|human| {
                    human.helmet_slot().occupant.as_ref().and_then(|info| {
                        self.get_vm()
                            .get_object(info.id)
                            .map(ObjectRefMut::from_vm_object)
                    })
                })
            }),
            1 => self.root_parent_human().and_then(|obj| {
                obj.borrow().as_human().and_then(|human| {
                    human.backpack_slot().occupant.as_ref().and_then(|info| {
                        self.get_vm()
                            .get_object(info.id)
                            .map(ObjectRefMut::from_vm_object)
                    })
                })
            }),
            2 => self.root_parent_human().and_then(|obj| {
                obj.borrow().as_human().and_then(|human| {
                    human.toolbelt_slot().occupant.as_ref().and_then(|info| {
                        self.get_vm()
                            .get_object(info.id)
                            .map(ObjectRefMut::from_vm_object)
                    })
                })
            }),
            _ => None,
        }
    }

    fn get_logicable_from_id_gw(
        &self,
        device: ObjectID,
        _connection: Option<usize>,
    ) -> Option<ObjectRef> {
        if device == *self.get_id() {
            return Some(ObjectRef::from_ref(self.as_object()));
        }
        let contained_ids: Vec<ObjectID> = self
            .get_slots()
            .into_iter()
            .filter_map(|slot| slot.occupant.as_ref().map(|info| info.id))
            .collect();
        if contained_ids.contains(&device) {
            self.get_vm()
                .get_object(device)
                .map(ObjectRef::from_vm_object)
        } else {
            None
        }
    }

    fn get_logicable_from_id_mut_gw(
        &mut self,
        device: ObjectID,
        _connection: Option<usize>,
    ) -> Option<ObjectRefMut> {
        if device == *self.get_id() {
            return Some(ObjectRefMut::from_ref(self.as_mut_object()));
        }
        let contained_ids: Vec<ObjectID> = self
            .get_slots()
            .into_iter()
            .filter_map(|slot| slot.occupant.as_ref().map(|info| info.id))
            .collect();
        if contained_ids.contains(&device) {
            self.get_vm()
                .get_object(device)
                .map(ObjectRefMut::from_vm_object)
        } else {
            None
        }
    }

    fn get_ic_gw(&self) -> Option<crate::vm::object::VMObject> {
        self.get_slots()
            .into_iter()
            .find(|slot| slot.typ == Class::ProgrammableChip)
            .and_then(|slot| {
                slot.occupant
                    .as_ref()
                    .and_then(|info| self.get_vm().get_object(info.id))
            })
    }

    fn hault_and_catch_fire_gw(&mut self) {
        // TODO: do something here??
    }
}

impl<T> GWCircuitHolderWrapper<T, ItemCircuitHolder> for T
where
    T: GWCircuitHolder<Holder = ItemCircuitHolder> + Item + Object,
{
    fn clear_error_gw(&mut self) {
        self.gw_set_error(0);
    }
    fn set_error_gw(&mut self, state: i32) {
        self.gw_set_error(state);
    }

    /// i32::MAX is db
    /// 0 -> Helmet
    /// 1 -> Suit
    /// 2 -> BackPack
    /// 3 -> ToolBelt
    fn get_logicable_from_index_gw(
        &self,
        device: i32,
        _connection: Option<usize>,
    ) -> Option<ObjectRef> {
        match device {
            i32::MAX => Some(ObjectRef::from_ref(self.as_object())),
            0 => self.root_parent_human().and_then(|obj| {
                obj.borrow().as_human().and_then(|human| {
                    human.helmet_slot().occupant.as_ref().and_then(|info| {
                        self.get_vm()
                            .get_object(info.id)
                            .map(ObjectRef::from_vm_object)
                    })
                })
            }),
            1 => self.root_parent_human().and_then(|obj| {
                obj.borrow().as_human().and_then(|human| {
                    human.suit_slot().occupant.as_ref().and_then(|info| {
                        self.get_vm()
                            .get_object(info.id)
                            .map(ObjectRef::from_vm_object)
                    })
                })
            }),
            2 => self.root_parent_human().and_then(|obj| {
                obj.borrow().as_human().and_then(|human| {
                    human.backpack_slot().occupant.as_ref().and_then(|info| {
                        self.get_vm()
                            .get_object(info.id)
                            .map(ObjectRef::from_vm_object)
                    })
                })
            }),
            3 => self.root_parent_human().and_then(|obj| {
                obj.borrow().as_human().and_then(|human| {
                    human.toolbelt_slot().occupant.as_ref().and_then(|info| {
                        self.get_vm()
                            .get_object(info.id)
                            .map(ObjectRef::from_vm_object)
                    })
                })
            }),
            _ => None,
        }
    }

    /// i32::MAX is db
    /// 0 -> Helmet
    /// 1 -> Suit
    /// 2 -> BackPack
    /// 3 -> ToolBelt
    fn get_logicable_from_index_mut_gw(
        &mut self,
        device: i32,
        _connection: Option<usize>,
    ) -> Option<ObjectRefMut> {
        match device {
            i32::MAX => Some(ObjectRefMut::from_ref(self.as_mut_object())),
            0 => self.root_parent_human().and_then(|obj| {
                obj.borrow().as_human().and_then(|human| {
                    human.helmet_slot().occupant.as_ref().and_then(|info| {
                        self.get_vm()
                            .get_object(info.id)
                            .map(ObjectRefMut::from_vm_object)
                    })
                })
            }),
            1 => self.root_parent_human().and_then(|obj| {
                obj.borrow().as_human().and_then(|human| {
                    human.suit_slot().occupant.as_ref().and_then(|info| {
                        self.get_vm()
                            .get_object(info.id)
                            .map(ObjectRefMut::from_vm_object)
                    })
                })
            }),
            2 => self.root_parent_human().and_then(|obj| {
                obj.borrow().as_human().and_then(|human| {
                    human.backpack_slot().occupant.as_ref().and_then(|info| {
                        self.get_vm()
                            .get_object(info.id)
                            .map(ObjectRefMut::from_vm_object)
                    })
                })
            }),
            3 => self.root_parent_human().and_then(|obj| {
                obj.borrow().as_human().and_then(|human| {
                    human.toolbelt_slot().occupant.as_ref().and_then(|info| {
                        self.get_vm()
                            .get_object(info.id)
                            .map(ObjectRefMut::from_vm_object)
                    })
                })
            }),
            _ => None,
        }
    }

    fn get_logicable_from_id_gw(
        &self,
        device: ObjectID,
        _connection: Option<usize>,
    ) -> Option<ObjectRef> {
        if device == *self.get_id() {
            return Some(ObjectRef::from_ref(self.as_object()));
        }
        let contained_ids: Vec<ObjectID> = self
            .get_slots()
            .into_iter()
            .filter_map(|slot| slot.occupant.as_ref().map(|info| info.id))
            .collect();
        if contained_ids.contains(&device) {
            self.get_vm()
                .get_object(device)
                .map(ObjectRef::from_vm_object)
        } else {
            None
        }
    }

    fn get_logicable_from_id_mut_gw(
        &mut self,
        device: ObjectID,
        _connection: Option<usize>,
    ) -> Option<ObjectRefMut> {
        if device == *self.get_id() {
            return Some(ObjectRefMut::from_ref(self.as_mut_object()));
        }
        let contained_ids: Vec<ObjectID> = self
            .get_slots()
            .into_iter()
            .filter_map(|slot| slot.occupant.as_ref().map(|info| info.id))
            .collect();
        if contained_ids.contains(&device) {
            self.get_vm()
                .get_object(device)
                .map(ObjectRefMut::from_vm_object)
        } else {
            None
        }
    }

    fn get_ic_gw(&self) -> Option<crate::vm::object::VMObject> {
        self.get_slots()
            .into_iter()
            .find(|slot| slot.typ == Class::ProgrammableChip)
            .and_then(|slot| {
                slot.occupant
                    .as_ref()
                    .and_then(|info| self.get_vm().get_object(info.id))
            })
    }

    fn hault_and_catch_fire_gw(&mut self) {
        // TODO: do something here??
    }
}
