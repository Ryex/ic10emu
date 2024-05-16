use crate::{
    network::Connection,
    vm::{
        enums::{
            basic_enums::{Class as SlotClass, GasType, SortingClass},
            script_enums::{LogicSlotType, LogicType},
        },
        object::{
            errors::{LogicError, MemoryError},
            templates::{DeviceInfo, ItemInfo},
            traits::*,
            LogicField, MemoryAccess, ObjectID, Slot,
        },
    },
};
use std::{collections::BTreeMap, usize};
use strum::IntoEnumIterator;

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
    fn get_slots(&self) -> &[Slot] {
        self.slots()
    }
    fn get_slots_mut(&mut self) -> &mut [Slot] {
        self.slots_mut()
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
        LogicType::iter().any(|lt| self.can_logic_read(lt))
    }
    fn is_logic_writeable(&self) -> bool {
        LogicType::iter().any(|lt| self.can_logic_write(lt))
    }
    fn can_logic_read(&self, lt: LogicType) -> bool {
        self.fields()
            .get(&lt)
            .map(|field| {
                matches!(
                    field.field_type,
                    MemoryAccess::Read | MemoryAccess::ReadWrite
                )
            })
            .unwrap_or(false)
    }
    fn can_logic_write(&self, lt: LogicType) -> bool {
        self.fields()
            .get(&lt)
            .map(|field| {
                matches!(
                    field.field_type,
                    MemoryAccess::Write | MemoryAccess::ReadWrite
                )
            })
            .unwrap_or(false)
    }
    fn get_logic(&self, lt: LogicType) -> Result<f64, LogicError> {
        self.fields()
            .get(&lt)
            .and_then(|field| match field.field_type {
                MemoryAccess::Read | MemoryAccess::ReadWrite => Some(field.value),
                _ => None,
            })
            .ok_or(LogicError::CantRead(lt))
    }
    fn set_logic(&mut self, lt: LogicType, value: f64, force: bool) -> Result<(), LogicError> {
        self.fields_mut()
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
            })
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
                let occupant = slot.occupant.and_then(|id| self.get_vm().get_object(id));
                match slt {
                    Occupied => {
                        if slot.occupant.is_some() {
                            Ok(1.0)
                        } else {
                            Ok(0.0)
                        }
                    }
                    Quantity => {
                        if slot.occupant.is_some() {
                            Ok(slot.quantity as f64)
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
                            Ok(occupant.borrow().get_id() as f64)
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
                                        .map(|suit| suit.pressure_air())
                                        .ok_or(LogicError::CantSlotRead(slt, index)),
                                    PressureWaste => logicable
                                        .as_suit()
                                        .map(|suit| suit.pressure_waste())
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
    fn clear_memory(&mut self) -> Result<(), MemoryError> {
        self.memory_mut().fill(0.0);
        Ok(())
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
                    slot.quantity = value as u32;
                    return Ok(());
                }
                if slot.writeable_logic.contains(&slt) {
                    let occupant = slot.occupant.and_then(|id| vm.get_object(id));
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
    fn slot_class(&self) -> SlotClass {
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
        self.damage_mut().replace(damage);
    }
}

pub trait GWCircuitHolder: Logicable {}
