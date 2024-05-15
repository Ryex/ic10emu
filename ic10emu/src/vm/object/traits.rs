use serde_derive::{Deserialize, Serialize};

use crate::{
    errors::ICError,
    interpreter::ICState,
    network::Connection,
    vm::{
        enums::{
            basic_enums::{Class as SlotClass, GasType, SortingClass},
            script_enums::{LogicSlotType, LogicType},
        },
        instructions::{traits::ICInstructable, Instruction},
        object::{
            errors::{LogicError, MemoryError},
            macros::tag_object_traits,
            ObjectID, Slot,
        },
    },
};
use std::{collections::BTreeMap, fmt::Debug};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ParentSlotInfo {
    pub parent: ObjectID,
    pub slot: usize,
}

tag_object_traits! {
    #![object_trait(Object: Debug)]

    pub trait Structure {
        fn is_small_grid(&self) -> bool;
    }

    pub trait Storage {
        fn slots_count(&self) -> usize;
        fn get_slot(&self, index: usize) -> Option<&Slot>;
        fn get_slot_mut(&mut self, index: usize) -> Option<&mut Slot>;
        fn get_slots(&self) -> &[Slot];
        fn get_slots_mut(&mut self) -> &mut [Slot];
    }

    pub trait MemoryReadable {
        fn memory_size(&self) -> usize;
        fn get_memory(&self, index: i32) -> Result<f64, MemoryError>;
        fn get_memory_slice(&self) -> &[f64];
    }

    pub trait MemoryWritable: MemoryReadable {
        fn set_memory(&mut self, index: i32, val: f64) -> Result<(), MemoryError>;
        fn clear_memory(&mut self) -> Result<(), MemoryError>;
    }

    pub trait Logicable: Storage {
        fn prefab_hash(&self) -> i32;
        /// returns 0 if not set
        fn name_hash(&self) -> i32;
        fn is_logic_readable(&self) -> bool;
        fn is_logic_writeable(&self) -> bool;
        fn can_logic_read(&self, lt: LogicType) -> bool;
        fn can_logic_write(&self, lt: LogicType) -> bool;
        fn set_logic(&mut self, lt: LogicType, value: f64, force: bool) -> Result<(), LogicError>;
        fn get_logic(&self, lt: LogicType) -> Result<f64, LogicError>;
        fn can_slot_logic_read(&self, slt: LogicSlotType, index: f64) -> bool;
        fn get_slot_logic(&self, slt: LogicSlotType, index: f64) -> Result<f64, LogicError>;
        fn valid_logic_types(&self) -> Vec<LogicType>;
        fn known_modes(&self) -> Option<Vec<(u32, String)>>;
    }

    pub trait SourceCode {
        fn set_source_code(&mut self, code: &str) -> Result<(), ICError>;
        fn set_source_code_with_invalid(&mut self, code: &str);
        fn get_source_code(&self) -> String;
        fn get_line(&self, line: usize) -> Result<&Instruction, ICError>;
    }

    pub trait CircuitHolder: Logicable + Storage {
        fn clear_error(&mut self);
        fn set_error(&mut self, state: i32);
        /// i32::MAX is db
        fn get_logicable_from_index(
            &self,
            device: i32,
            connection: Option<usize>,
        ) -> Option<LogicableRef>;
        /// i32::MAX is db
        fn get_logicable_from_index_mut(
            &self,
            device: i32,
            connection: Option<usize>,
        ) -> Option<LogicableRefMut>;
        fn get_logicable_from_id(
            &self,
            device: ObjectID,
            connection: Option<usize>,
        ) -> Option<LogicableRef>;
        fn get_logicable_from_id_mut(
            &self,
            device: ObjectID,
            connection: Option<usize>,
        ) -> Option<LogicableRefMut>;
        fn get_source_code(&self) -> String;
        fn set_source_code(&self, code: String);
        fn get_batch(&self) -> Vec<LogicableRef>;
        fn get_batch_mut(&self) -> Vec<LogicableRefMut>;
        fn get_ic(&self) -> Option<ObjectID>;
    }

    pub trait Item {
        fn consumable(&self) -> bool;
        fn filter_type(&self) -> Option<GasType>;
        fn ingredient(&self) -> bool;
        fn max_quantity(&self) -> u32;
        fn reagents(&self) -> Option<&BTreeMap<String, f64>>;
        fn slot_class(&self) -> SlotClass;
        fn sorting_class(&self) -> SortingClass;
        fn get_parent_slot(&self) -> Option<ParentSlotInfo>;
        fn set_parent_slot(&mut self, info: Option<ParentSlotInfo>);
    }

    pub trait IntegratedCircuit: Logicable + MemoryWritable + SourceCode + Item {
        fn get_circuit_holder(&self) -> Option<CircuitHolderRef>;
        fn get_instruction_pointer(&self) -> f64;
        fn set_next_instruction(&mut self, next_instruction: f64);
        fn set_next_instruction_relative(&mut self, offset: f64) {
            self.set_next_instruction(self.get_instruction_pointer() + offset);
        }
        fn reset(&mut self);
        fn get_real_target(&self, indirection: u32, target: u32) -> Result<f64, ICError>;
        fn get_register(&self, indirection: u32, target: u32) -> Result<f64, ICError>;
        fn set_register(&mut self, indirection: u32, target: u32, val: f64) -> Result<f64, ICError>;
        fn set_return_address(&mut self, addr: f64);
        fn al(&mut self) {
            self.set_return_address(self.get_instruction_pointer() + 1.0);
        }
        fn push_stack(&mut self, val: f64) -> Result<f64, ICError>;
        fn pop_stack(&mut self) -> Result<f64, ICError>;
        fn peek_stack(&self) -> Result<f64, ICError>;
        fn get_stack(&self, addr: f64) -> Result<f64, ICError>;
        fn put_stack(&self, addr: f64, val: f64) -> Result<f64, ICError>;
        fn get_aliases(&self) -> &BTreeMap<String, crate::vm::instructions::operands::Operand>;
        fn get_aliases_mut(&mut self) -> &mut BTreeMap<String, crate::vm::instructions::operands::Operand>;
        fn get_defines(&self) -> &BTreeMap<String, f64>;
        fn get_defines_mut(&mut self) -> &mut BTreeMap<String, f64>;
        fn get_labels(&self) -> &BTreeMap<String, u32>;
        fn get_state(&self) -> ICState;
        fn set_state(&mut self, state: ICState);
    }

    pub trait Programmable: ICInstructable {
        fn get_source_code(&self) -> String;
        fn set_source_code(&self, code: String);
        fn step(&mut self, advance_ip_on_err: bool) -> Result<(), crate::errors::ICError>;
    }

    pub trait Instructable: MemoryWritable {
        // fn get_instructions(&self) -> Vec<LogicInstruction>
    }

    pub trait LogicStack: MemoryWritable {
        // fn logic_stack(&self) -> LogicStack;
    }

    pub trait Device: Logicable {
        fn can_slot_logic_write(&self, slt: LogicSlotType, index: f64) -> bool;
        fn set_slot_logic(
            &mut self,
            slt: LogicSlotType,
            index: f64,
            value: f64,
            force: bool,
        ) -> Result<(), LogicError>;
        fn connection_list(&self) -> &[Connection];
        fn connection_list_mut(&mut self) -> &mut [Connection];
        fn device_pins(&self) -> Option<&[Option<ObjectID>]>;
        fn device_pins_mut(&self) -> Option<&mut [Option<ObjectID>]>;
        fn has_activate_state(&self) -> bool;
        fn has_atmosphere(&self) -> bool;
        fn has_color_state(&self) -> bool;
        fn has_lock_state(&self) -> bool;
        fn has_mode_state(&self) -> bool;
        fn has_on_off_state(&self) -> bool;
        fn has_open_state(&self) -> bool;
        fn has_reagents(&self) -> bool;
    }

    pub trait WirelessTransmit: Logicable {}

    pub trait WirelessReceive: Logicable {}

    pub trait Network: Logicable {
        fn contains(&self, id: &ObjectID) -> bool;
        fn contains_all(&self, ids: &[ObjectID]) -> bool;
        fn contains_data(&self, id: &ObjectID) -> bool;
        fn contains_all_data(&self, ids: &[ObjectID]) -> bool;
        fn contains_power(&self, id: &ObjectID) -> bool;
        fn contains_all_power(&self, ids: &[ObjectID]) -> bool;
        fn data_visible(&self, source: &ObjectID) -> Vec<u32>;
        fn add_data(&mut self, id: ObjectID) -> bool;
        fn add_power(&mut self, id: ObjectID) -> bool;
        fn remove_all(&mut self, id: ObjectID) -> bool;
        fn remove_data(&mut self, id: ObjectID) -> bool;
        fn remove_power(&mut self, id: ObjectID) -> bool;
        fn get_devices(&self) -> Vec<ObjectID>;
        fn get_power_only(&self) -> Vec<ObjectID>;
        fn get_channel_data(&self) -> &[f64; 8];
    }

}

impl Debug for dyn Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Object: (ID = {:?}, Type = {})",
            self.get_id(),
            self.type_name()
        )
    }
}
