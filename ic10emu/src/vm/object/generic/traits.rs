use crate::{network::Connection, vm::{
    enums::{
        basic_enums::{Class as SlotClass, GasType, SortingClass},
        script_enums::{LogicSlotType, LogicType},
    },
    object::{
        errors::{LogicError, MemoryError}, templates::{DeviceInfo, ItemInfo}, traits::*, LogicField, MemoryAccess, ObjectID, Slot
    },
    VM,
}};
use std::{collections::BTreeMap, usize};
use strum::IntoEnumIterator;

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
}

pub trait GWLogicable: Storage {
    fn fields(&self) -> &BTreeMap<LogicType, LogicField>;
    fn fields_mut(&mut self) -> &mut BTreeMap<LogicType, LogicField>;
}

impl<T: GWLogicable + Object> Logicable for T {
    fn prefab_hash(&self) -> i32 {
        self.prefab().hash
    }
    fn name_hash(&self) -> i32 {
        self.name().hash
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
    fn can_slot_logic_read(&self, slt: LogicSlotType, index: usize) -> bool {
        self.get_slot(index)
            .map(|slot| slot.enabled_logic.contains(&slt))
            .unwrap_or(false)
    }
    fn get_slot_logic(
        &self,
        slt: LogicSlotType,
        index: usize,
        _vm: &VM,
    ) -> Result<f64, LogicError> {
        self.get_slot(index)
            .ok_or_else(|| LogicError::SlotIndexOutOfRange(index, self.slots_count()))
            .and_then(|slot| {
                if slot.enabled_logic.contains(&slt) {
                    match slot.occupant {
                        Some(_id) => {
                            // FIXME: impliment by accessing VM to get occupant
                            Ok(0.0)
                        }
                        None => Ok(0.0),
                    }
                } else {
                    Err(LogicError::CantSlotRead(slt, index))
                }
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

pub trait GWDevice: Logicable {
    fn device_info(&self) -> &DeviceInfo;
    fn device_connections(&self) -> &[Connection];
    fn device_connections_mut(&mut self) -> &mut [Connection];
    fn device_pins(&self) -> Option<&[Option<ObjectID>]>;
    fn device_pins_mut(&mut self) -> Option<&mut [Option<ObjectID>]>;
}

impl<T: GWDevice + Object> Device for T {
    fn connection_list(&self) ->  &[crate::network::Connection] {
        self.device_connections()
    }
    fn connection_list_mut(&mut self) ->  &mut[Connection] {
        self.device_connections_mut()
    }
    fn device_pins(&self) -> Option<&[Option<ObjectID>]> {
        self.device_pins()
    }
    fn device_pins_mut(&self) -> Option<&mut[Option<ObjectID>]> {
        self.device_pins_mut()
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
    fn has_activate_state(&self) -> bool {
        self.device_info().has_activate_state
    }
}

pub trait GWItem {
    fn item_info(&self) -> &ItemInfo;
    fn parent_slot(&self) -> Option<ParentSlotInfo>;
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
    fn parent_slot(&self) -> Option<ParentSlotInfo> {
        self.parent_slot()
    }
}

pub trait GWCircuitHolder: Logicable {}
