use crate::vm::{
    enums::script_enums::{LogicSlotType as SlotLogicType, LogicType},
    object::{
        errors::{LogicError, MemoryError},
        macros::ObjectInterface,
        traits::*,
        FieldType, LogicField, Name, ObjectID, Slot,
    },
    VM,
};
use macro_rules_attribute::derive;
use std::{collections::BTreeMap, usize};
use strum::IntoEnumIterator;

pub trait GWLogicable {
    fn name(&self) -> &Option<Name>;
    fn fields(&self) -> &BTreeMap<LogicType, LogicField>;
    fn fields_mut(&mut self) -> &mut BTreeMap<LogicType, LogicField>;
    fn slots(&self) -> &Vec<Slot>;
    fn slots_mut(&mut self) -> &mut Vec<Slot>;
}
macro_rules! GWLogicable {
    (
        $( #[$attr:meta] )*
        $viz:vis struct $struct:ident {
            $($body:tt)*
        }
    ) => {
        impl GWLogicable for $struct {
            fn name(&self) -> &Option<Name> {
                &self.name
            }
            fn fields(&self) -> &BTreeMap<LogicType, LogicField> {
                &self.fields
            }
            fn fields_mut(&mut self) -> &mut BTreeMap<LogicType, LogicField> {
                &mut self.fields
            }
            fn slots(&self) -> &Vec<Slot> {
                &self.slots
            }
            fn slots_mut(&mut self) -> &mut Vec<Slot> {
                &mut self.slots
            }
        }
    };
}

pub trait GWMemoryReadable {
    fn memory_size(&self) -> usize;
    fn memory(&self) -> &Vec<f64>;
}
macro_rules! GWMemoryReadable {
    (
        $( #[$attr:meta] )*
        $viz:vis struct $struct:ident {
            $($body:tt)*
        }
    ) => {
        impl GWMemoryReadable for $struct {
            fn memory_size(&self) -> usize {
                self.memory.len()
            }
            fn memory(&self) -> &Vec<f64> {
                &self.memory
            }
        }
    };
}
pub trait GWMemoryWritable: GWMemoryReadable {
    fn memory_mut(&mut self) -> &mut Vec<f64>;
}
macro_rules! GWMemoryWritable {
    (
        $( #[$attr:meta] )*
        $viz:vis struct $struct:ident {
            $($body:tt)*
        }
    ) => {
        impl GWMemoryWritable for $struct {
            fn memory_mut(&mut self) -> &mut Vec<f64> {
                &mut self.memory
            }
        }
    };
}

pub trait GWDevice: GWLogicable {}
macro_rules! GWDevice {
    (
        $( #[$attr:meta] )*
        $viz:vis struct $struct:ident {
            $($body:tt)*
        }
    ) => {
        impl GWDevice for $struct {}
    };
}

pub trait GWCircuitHolder: GWLogicable {}

impl<T: GWLogicable + Object> Logicable for T {
    fn prefab_hash(&self) -> i32 {
        self.prefab().hash
    }
    fn name_hash(&self) -> i32 {
        self.name().as_ref().map(|name| name.hash).unwrap_or(0)
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
            .map(|field| matches!(field.field_type, FieldType::Read | FieldType::ReadWrite))
            .unwrap_or(false)
    }
    fn can_logic_write(&self, lt: LogicType) -> bool {
        self.fields()
            .get(&lt)
            .map(|field| matches!(field.field_type, FieldType::Write | FieldType::ReadWrite))
            .unwrap_or(false)
    }
    fn get_logic(&self, lt: LogicType) -> Result<f64, LogicError> {
        self.fields()
            .get(&lt)
            .and_then(|field| match field.field_type {
                FieldType::Read | FieldType::ReadWrite => Some(field.value),
                _ => None,
            })
            .ok_or(LogicError::CantRead(lt))
    }
    fn set_logic(&mut self, lt: LogicType, value: f64, force: bool) -> Result<(), LogicError> {
        self.fields_mut()
            .get_mut(&lt)
            .ok_or(LogicError::CantWrite(lt))
            .and_then(|field| match field.field_type {
                FieldType::Write | FieldType::ReadWrite => {
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
    fn slots_count(&self) -> usize {
        self.slots().len()
    }
    fn get_slot(&self, index: usize) -> Option<&Slot> {
        self.slots().get(index)
    }
    fn get_slot_mut(&mut self, index: usize) -> Option<&mut Slot> {
        self.slots_mut().get_mut(index)
    }
    fn can_slot_logic_read(&self, slt: SlotLogicType, index: usize) -> bool {
        self.get_slot(index)
            .map(|slot| slot.enabled_logic.contains(&slt))
            .unwrap_or(false)
    }
    fn get_slot_logic(
        &self,
        slt: SlotLogicType,
        index: usize,
        _vm: &VM,
    ) -> Result<f64, LogicError> {
        self.get_slot(index)
            .ok_or_else(|| LogicError::SlotIndexOutOfRange(index, self.slots().len()))
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
impl<T: GWMemoryWritable + MemoryReadable + Object> MemoryWritable for T {
    fn set_memory(&mut self, index: i32, val: f64) -> Result<(), MemoryError> {
        if index < 0 {
            Err(MemoryError::StackUnderflow(index, self.memory().len()))
        } else if index as usize >= self.memory().len() {
            Err(MemoryError::StackOverflow(index, self.memory().len()))
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

impl<T: GWDevice + Object> Device for T {}

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
