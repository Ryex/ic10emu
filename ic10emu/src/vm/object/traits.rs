use std::fmt::Debug;

use crate::{grammar, vm::{object::{errors::{LogicError, MemoryError}, macros::tag_object_traits, ObjectID, Slot}, VM}};

tag_object_traits! {
    #![object_trait(Object: Debug)]

    pub trait Memory {
        fn memory_size(&self) -> usize;
    }

    pub trait MemoryWritable: Memory {
        fn set_memory(&mut self, index: i32, val: f64) -> Result<(), MemoryError>;
        fn clear_memory(&mut self) -> Result<(), MemoryError>;
    }

    pub trait MemoryReadable: Memory {
        fn get_memory(&self, index: i32) -> Result<f64, MemoryError>;
    }

    pub trait Logicable {
        fn prefab_hash(&self) -> i32;
        /// returns 0 if not set
        fn name_hash(&self) -> i32;
        fn is_logic_readable(&self) -> bool;
        fn is_logic_writeable(&self) -> bool;
        fn can_logic_read(&self, lt: grammar::LogicType) -> bool;
        fn can_logic_write(&self, lt: grammar::LogicType) -> bool;
        fn set_logic(&mut self, lt: grammar::LogicType, value: f64, force: bool) -> Result<(), LogicError>;
        fn get_logic(&self, lt: grammar::LogicType) -> Result<f64, LogicError>;

        fn slots_count(&self) -> usize;
        fn get_slot(&self, index: usize) -> Option<&Slot>;
        fn get_slot_mut(&mut self, index: usize) -> Option<&mut Slot>;
        fn can_slot_logic_read(&self, slt: grammar::SlotLogicType, index: usize) -> bool;
        fn get_slot_logic(&self, slt: grammar::SlotLogicType, index: usize, vm: &VM) -> Result<f64, LogicError>;
    }

    pub trait CircuitHolder: Logicable {
        fn clear_error(&mut self);
        fn set_error(&mut self, state: i32);
        fn get_logicable_from_index(&self, device: usize, vm: &VM) -> Option<LogicableRef<Self>>;
        fn get_logicable_from_index_mut(&self, device: usize, vm: &VM) -> Option<LogicableRefMut<Self>>;
        fn get_logicable_from_id(&self, device: ObjectID, vm: &VM) -> Option<LogicableRef<Self>>;
        fn get_logicable_from_id_mut(&self, device: ObjectID, vm: &VM) -> Option<LogicableRefMut<Self>>;
        fn get_source_code(&self) -> String;
        fn set_source_code(&self, code: String);
        fn get_batch(&self) -> Vec<LogicableRef<Self>>;
        fn get_batch_mut(&self) -> Vec<LogicableRefMut<Self>>;
    }

    pub trait SourceCode {
        fn get_source_code(&self) -> String;
        fn set_source_code(&self, code: String);

    }

    pub trait Instructable: Memory {
        // fn get_instructions(&self) -> Vec<LogicInstruction>
    }

    pub trait LogicStack: Memory {
        // fn logic_stack(&self) -> LogicStack;
    }

    pub trait Device: Logicable {

    }

}

impl<T: Debug> Debug for dyn Object<ID = T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object: (ID = {:?}, Type = {})", self.id(), self.type_name())
    }
}
