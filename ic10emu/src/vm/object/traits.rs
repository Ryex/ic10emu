use crate::{
    errors::ICError,
    vm::{
        enums::script_enums::{LogicSlotType, LogicType},
        instructions::Instruction,
        object::{
            errors::{LogicError, MemoryError},
            macros::tag_object_traits,
            ObjectID, Slot,
        },
        VM,
    },
};

use std::fmt::Debug;

tag_object_traits! {
    #![object_trait(Object: Debug)]

    pub trait MemoryReadable {
        fn memory_size(&self) -> usize;
        fn get_memory(&self, index: i32) -> Result<f64, MemoryError>;
    }

    pub trait MemoryWritable: MemoryReadable {
        fn set_memory(&mut self, index: i32, val: f64) -> Result<(), MemoryError>;
        fn clear_memory(&mut self) -> Result<(), MemoryError>;
    }

    pub trait Logicable {
        fn prefab_hash(&self) -> i32;
        /// returns 0 if not set
        fn name_hash(&self) -> i32;
        fn is_logic_readable(&self) -> bool;
        fn is_logic_writeable(&self) -> bool;
        fn can_logic_read(&self, lt: LogicType) -> bool;
        fn can_logic_write(&self, lt: LogicType) -> bool;
        fn set_logic(&mut self, lt: LogicType, value: f64, force: bool) -> Result<(), LogicError>;
        fn get_logic(&self, lt: LogicType) -> Result<f64, LogicError>;

        fn slots_count(&self) -> usize;
        fn get_slot(&self, index: usize) -> Option<&Slot>;
        fn get_slot_mut(&mut self, index: usize) -> Option<&mut Slot>;
        fn can_slot_logic_read(&self, slt: LogicSlotType, index: usize) -> bool;
        fn get_slot_logic(&self, slt: LogicSlotType, index: usize, vm: &VM) -> Result<f64, LogicError>;
    }

    pub trait SourceCode {
        fn set_source_code(&mut self, code: &str) -> Result<(), ICError>;
        fn set_source_code_with_invalid(&mut self, code: &str);
        fn get_source_code(&self) -> String;
        fn get_line(&self, line: usize) -> Result<&Instruction, ICError>;
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

    pub trait Programmable: crate::vm::instructions::traits::ICInstructable {
        fn get_source_code(&self) -> String;
        fn set_source_code(&self, code: String);
        fn step(&mut self) -> Result<(), crate::errors::ICError>;
    }

    pub trait Instructable: MemoryWritable {
        // fn get_instructions(&self) -> Vec<LogicInstruction>
    }

    pub trait LogicStack: MemoryWritable {
        // fn logic_stack(&self) -> LogicStack;
    }

    pub trait Device: Logicable {

    }



}

impl<T: Debug> Debug for dyn Object<ID = T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Object: (ID = {:?}, Type = {})",
            self.id(),
            self.type_name()
        )
    }
}
