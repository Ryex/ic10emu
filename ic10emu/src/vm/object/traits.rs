use std::fmt::Debug;

use crate::{grammar, vm::{object::{macros::tag_object_traits, ObjectID}, VM}};

tag_object_traits! {
    #![object_trait(Object: Debug)]

    pub trait Memory {
        fn size(&self) -> usize;
    }

    pub trait MemoryWritable: Memory {
        fn set_memory(&mut self, index: usize, val: f64);
        fn clear_memory(&mut self);
    }

    pub trait MemoryReadable: Memory {
        fn get_memory(&self) -> &Vec<f64>;
    }

    pub trait Logicable {
        fn is_logic_readable(&self) -> bool;
        fn is_logic_writeable(&self) -> bool;
        fn set_logic(&mut self, lt: grammar::LogicType, value: f64);
        fn get_logic(&self, lt: grammar::LogicType) -> Option<f64>;

        fn slots_count(&self) -> usize;
        // fn get_slot(&self, index: usize) -> Slot;
        fn set_slot_logic(&mut self, slt: grammar::SlotLogicType, value: f64);
        fn get_slot_logic(&self, slt: grammar::SlotLogicType) -> Option<f64>;
    }

    pub trait CircuitHolder: Logicable {
        fn clear_error(&mut self);
        fn set_error(&mut self, state: i32);
        fn get_logicable_from_index(&self, device: usize, vm: &VM) -> Option<LogicableRef<Self>>;
        fn get_logicable_from_id(&self, device: ObjectID, vm: &VM) -> Option<LogicableRef<Self>>;
        fn get_source_code(&self) -> String;
        fn set_source_code(&self, code: String);
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

}

impl<T: Debug> Debug for dyn Object<ID = T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object: (ID = {:?}, Type = {})", self.id(), self.type_name())
    }
}
