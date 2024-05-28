use serde_derive::{Deserialize, Serialize};

use crate::{
    errors::ICError,
    interpreter::ICState,
    network::Connection,
    vm::{
        instructions::{traits::ICInstructable, Instruction},
        object::{
            errors::{LogicError, MemoryError},
            macros::tag_object_traits,
            ObjectID, Slot, VMObject,
        },
    },
};
use stationeers_data::enums::{
    basic::{Class, GasType, SortingClass},
    script::{LogicSlotType, LogicType},
    Species,
};
use std::{collections::BTreeMap, fmt::Debug};
#[cfg(feature = "tsify")]
use tsify::Tsify;
#[cfg(feature = "tsify")]
use wasm_bindgen::prelude::*;

use strum::{AsRefStr, Display, EnumIter, EnumProperty, EnumString, FromRepr};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct ParentSlotInfo {
    pub parent: ObjectID,
    pub slot: usize,
}

#[derive(
    Default,
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub enum StatState {
    #[default]
    Normal,
    Warning,
    Critical,
}

tag_object_traits! {
    #![object_trait(Object: Debug)]

    pub trait Structure {
        fn is_small_grid(&self) -> bool;
    }

    pub trait Storage {
        /// Number of storage slots this object has
        fn slots_count(&self) -> usize;
        /// Get a reference to a indexed slot
        fn get_slot(&self, index: usize) -> Option<&Slot>;
        /// Get a mutable reference to a indexed slot
        fn get_slot_mut(&mut self, index: usize) -> Option<&mut Slot>;
        /// Get a vector of references to all an object's slots
        fn get_slots(&self) -> Vec<&Slot>;
        /// Get a vector a mutable references to all an object's slots
        fn get_slots_mut(&mut self) -> Vec<&mut Slot>;
    }

    pub trait MemoryReadable {
        /// Size of an object memory, the count of f64 elements stored
        fn memory_size(&self) -> usize;
        /// Get the value at the indexed memory.
        /// Errors if the index over or under flows the memory
        fn get_memory(&self, index: i32) -> Result<f64, MemoryError>;
        /// get a slice of the objects' memory
        fn get_memory_slice(&self) -> &[f64];
    }

    pub trait MemoryWritable: MemoryReadable {
        /// Set the value at the indexed memory
        /// Errors if the index over or under flows the memory
        fn set_memory(&mut self, index: i32, val: f64) -> Result<(), MemoryError>;
        /// Reset all an object's memory (typically to all zero values)
        fn clear_memory(&mut self);
    }

    pub trait Logicable: Storage {
        /// The crc32 hash of the object's prefab name
        fn prefab_hash(&self) -> i32;
        /// The crc32 hash of an object's name
        fn name_hash(&self) -> i32;
        /// If the object has *any* readable logic fields
        fn is_logic_readable(&self) -> bool;
        /// If the object has *any* writable logic fields
        fn is_logic_writeable(&self) -> bool;
        /// Can the logic type be read form this object
        fn can_logic_read(&self, lt: LogicType) -> bool;
        /// Can the logic type be written to this object
        fn can_logic_write(&self, lt: LogicType) -> bool;
        /// Write the value of the logic type on this object.
        /// Errors if the type can not be written to.
        /// force will allow special cases for existing values that arn't
        /// normally writable.
        /// This is for use outside of ic10 code but does not guarantee the
        /// value will write or that no error will result.
        /// If a logic type is not present on an object the force write will still error.
        fn set_logic(&mut self, lt: LogicType, value: f64, force: bool) -> Result<(), LogicError>;
        /// Read the value of the logic type on this object.
        /// Errors if a logic type is not readable
        fn get_logic(&self, lt: LogicType) -> Result<f64, LogicError>;
        /// Can a slot logic type be read from the indexed slot
        fn can_slot_logic_read(&self, slt: LogicSlotType, index: f64) -> bool;
        /// Read a slot logic type value from an index slot
        fn get_slot_logic(&self, slt: LogicSlotType, index: f64) -> Result<f64, LogicError>;
        /// Returns a vector of the `LogicType`'s that could be read or written to or form this
        /// object
        fn valid_logic_types(&self) -> Vec<LogicType>;
        /// If this object has modes returns a vector of (value, name) pairs
        fn known_modes(&self) -> Option<Vec<(u32, String)>>;
    }

    pub trait SourceCode {
        /// Set the source code for this object.
        /// Errors if the source code has compilation errors.
        fn set_source_code(&mut self, code: &str) -> Result<(), ICError>;
        /// Set the source code for this object, lines that fail to compile are reduced to nops.
        fn set_source_code_with_invalid(&mut self, code: &str);
        /// Return the source code form this object
        fn get_source_code(&self) -> String;
        /// Return the compiled instruction and it's operands at the indexed line in the source
        /// code.
        fn get_line(&self, line: usize) -> Result<&Instruction, ICError>;
    }

    pub trait CircuitHolder: Logicable + Storage {
        /// Clear any error set on the circuit holder
        fn clear_error(&mut self);
        /// Set an int error value on the circuit holder
        fn set_error(&mut self, state: i32);
        /// Get a reference (which may be self) to a logicable object based on an index
        /// (`db`, `d0`, `d1` etc.).
        /// for a StructureCircuitHolder this would be the set pins
        /// fpr a tablet or suit this would be the parent human's equipment
        /// i32::MAX is db
        fn get_logicable_from_index(
            &self,
            device: i32,
            connection: Option<usize>,
        ) -> Option<ObjectRef>;
        /// Get a mutable reference (which may be self) to a logicable object based on an index
        /// (`db`, `d0`, `d1` etc.).
        /// for a StructureCircuitHolder this would be the set pins
        /// fpr a tablet or suit this would be the parent human's equipment
        /// i32::MAX is db
        fn get_logicable_from_index_mut(
            &mut self,
            device: i32,
            connection: Option<usize>,
        ) -> Option<ObjectRefMut>;
        /// Use an object id to get a reference to an object network visible object.
        /// uses ObjectRef in case the object ID is it's own ID
        fn get_logicable_from_id(
            &self,
            device: ObjectID,
            connection: Option<usize>,
        ) -> Option<ObjectRef>;
        /// Use an object id to get a mutable reference to an object network visible object.
        /// uses ObjectRefMut in case the object ID is it's own ID
        fn get_logicable_from_id_mut(
            &mut self,
            device: ObjectID,
            connection: Option<usize>,
        ) -> Option<ObjectRefMut>;
        /// Get the programmable circuit object slotted into this circuit holder
        fn get_ic(&self) -> Option<VMObject>;
        /// Execute a `hcf` instruction
        fn hault_and_catch_fire(&mut self);
    }

    pub trait Item {
        /// Is an item consumable?
        fn consumable(&self) -> bool;
        /// If an item is a filter what gas is it for?
        fn filter_type(&self) -> Option<GasType>;
        /// Is this item an ingredient ?
        fn ingredient(&self) -> bool;
        /// The max quantity this item stacks to
        fn max_quantity(&self) -> u32;
        /// Map of the reagents to the quantity produces by processing this item
        fn reagents(&self) -> Option<&BTreeMap<String, f64>>;
        /// The class of item this is for storage slots
        fn slot_class(&self) -> Class;
        /// The sorting class of the item
        fn sorting_class(&self) -> SortingClass;
        /// The parent object and slot index this item is stored in
        fn get_parent_slot(&self) -> Option<ParentSlotInfo>;
        /// Set the parent object and slot index this object is stored in
        fn set_parent_slot(&mut self, info: Option<ParentSlotInfo>);
        /// Get the damage 0.0 is no damage, 1.0 is full damage
        fn get_damage(&self) -> f32;
        /// Set the damage of the object, 0.0 is no damage, 1.0 is full damage
        fn set_damage(&mut self, damage: f32);
        /// If this object is stored in a human's inventory or in an inventory down the chain from
        /// a human, return that human
        fn root_parent_human(&self) -> Option<VMObject> {
            self.get_parent_slot().and_then(|info| {
                if let Some(obj) = self.get_vm().get_object(info.parent) {
                    if obj.borrow().as_human().is_some() {
                        return Some(obj);
                    }
                    let obj_ref = obj.borrow();
                    if let Some(item) = obj_ref.as_item() {
                        return item.root_parent_human()
                    }
                }
                None
            })
        }
    }

    pub trait Plant {
        fn get_efficiency(&self) -> f64;
        fn get_health(&self) -> f64;
        fn get_growth(&self) -> f64;
        fn is_mature(&self) -> bool;
        fn is_seeding(&self) -> bool;
    }

    pub trait Suit: Item + Storage {
        fn pressure_waste(&self) -> f32;
        fn pressure_waste_max(&self) -> f32;
        fn pressure_air(&self) -> f32;
    }

    pub trait InternalAtmosphere {
        fn get_volume(&self) -> f64;
    }

    pub trait Thermal {
        fn get_convection_factor(&self) -> f32;
        fn get_radiation_factor(&self) -> f32;
    }

    pub trait IntegratedCircuit: Logicable + MemoryWritable + SourceCode + Item {
        /// Get the object that acts as the circuit holder for this object
        fn get_circuit_holder(&self) -> Option<VMObject>;
        /// Get the current instruction pointer
        fn get_instruction_pointer(&self) -> u32;
        /// Set the next instruction to execute. The instruction pointer is set to this value once
        /// execution of the current instruction is complete.
        fn set_next_instruction(&mut self, next_instruction: f64);
        /// Set the next instruction to execute relative to the current instruction pointer.
        /// The instruction pointer is set to this value once execution of the current
        /// instruction is complete.
        fn set_next_instruction_relative(&mut self, offset: f64) {
            self.set_next_instruction(self.get_instruction_pointer() as f64 + offset);
        }
        /// Reset the circuit. The instruction pointer, instruction count since last yield, all
        /// registers and memory are set to 0; aliases and defines are cleared; state is set back
        /// to start.
        fn reset(&mut self);
        /// When given some indirection level and a first target read registers values as
        /// targets while reducing indirection level by one until it reaches to 0
        /// to find the real target.
        /// Errors if any index along the chain is out of range
        fn get_real_target(&self, indirection: u32, target: u32) -> Result<f64, ICError>;
        /// Return a register value through possible indirection
        /// Errors if any index along the chain is out of range
        fn get_register(&self, indirection: u32, target: u32) -> Result<f64, ICError>;
        /// Get a slice of all registers
        fn get_registers(&self) -> &[f64];
        /// Get a mutable slice of all registers
        fn get_registers_mut(&mut self) -> &mut [f64];
        /// Set a register value through possible indirection
        /// Errors if any index along the chain is out of range
        fn set_register(&mut self, indirection: u32, target: u32, val: f64) -> Result<f64, ICError>;
        /// Set the return address register's value
        fn set_return_address(&mut self, addr: f64);
        /// Set the return address to the instruction after the current instruction pointer
        fn al(&mut self) {
            self.set_return_address(self.get_instruction_pointer() as f64 + 1.0);
        }
        /// Write value to the stack memory at the current stack pointer and advance stack pointer
        /// Errors for stack under or overflow of the stack pointer
        fn push_stack(&mut self, val: f64) -> Result<f64, ICError>;
        /// Read value from the stack memory at the current stack pointer and decrement the stack pointer
        /// Errors for stack under or overflow of the stack pointer
        fn pop_stack(&mut self) -> Result<f64, ICError>;
        /// Read the value form the stack memory at the current stack pointer and leave the stack pointer
        /// at the same location
        /// Errors for stack under or overflow of the stack pointer
        fn peek_stack(&self) -> Result<f64, ICError>;
        /// Read the value from the stack memory at indexed address
        /// Errors for stack under or overflow of the address
        fn get_stack(&self, addr: f64) -> Result<f64, ICError>;
        /// Write the value to the stack memory at the indexed address
        /// Errors for stack under or overflow of the address
        fn put_stack(&mut self, addr: f64, val: f64) -> Result<f64, ICError>;
        /// Get a reference to the alias Map
        fn get_aliases(&self) -> &BTreeMap<String, crate::vm::instructions::operands::Operand>;
        /// Get a mutable reference to the alias Map
        fn get_aliases_mut(&mut self) -> &mut BTreeMap<String, crate::vm::instructions::operands::Operand>;
        /// Get a reference to the define Map
        fn get_defines(&self) -> &BTreeMap<String, f64>;
        /// Get a mutable reference to the define Map
        fn get_defines_mut(&mut self) -> &mut BTreeMap<String, f64>;
        /// Get a reference to the labels Map
        fn get_labels(&self) -> &BTreeMap<String, u32>;
        /// Get the current circuit state. (Start, Yield, Sleep, Error, etc.)
        fn get_state(&self) -> ICState;
        /// Set the current circuit state. (Start, Yield, Sleep, Error, etc.)
        fn set_state(&mut self, state: ICState);
        /// Get the count of instructions executed since the last yield
        fn get_instructions_since_yield(&self) -> u16;
    }

    pub trait Programmable: ICInstructable {
        fn step(&mut self, advance_ip_on_err: bool) -> Result<(), crate::errors::ICError>;
    }

    pub trait Chargeable {
        fn get_charge(&self) -> f32;
        fn set_charge(&mut self, charge: f32);
        fn get_max_charge(&self) -> f32;
        fn get_charge_ratio(&self) -> f32 {
            self.get_charge() / self.get_max_charge()
        }
        fn get_charge_delta(&self) -> f32 {
            self.get_charge() - self.get_max_charge()
        }
        fn is_empty(&self) -> bool {
            self.get_charge() == 0.0
        }
    }

    pub trait Instructable: MemoryWritable {
        // fn get_instructions(&self) -> Vec<LogicInstruction>
    }

    pub trait LogicStack: MemoryWritable {
        // fn logic_stack(&self) -> LogicStack;
    }

    pub trait Device: Logicable {
        /// Can the slot logic type be written to the object at the indexed slot
        fn can_slot_logic_write(&self, slt: LogicSlotType, index: f64) -> bool;
        /// Write to the slot logic type at the indexed slot
        /// Errors if the index is out of range or the slot logic type is not writable
        fn set_slot_logic(
            &mut self,
            slt: LogicSlotType,
            index: f64,
            value: f64,
            force: bool,
        ) -> Result<(), LogicError>;
        /// Get a slice of the Device's network connections
        fn connection_list(&self) -> &[Connection];
        /// Get a mutable slice of the Device's network connections
        fn connection_list_mut(&mut self) -> &mut [Connection];
        /// Get a slice of the devices "pins" (connected object Ids) if the device has pins
        fn device_pins(&self) -> Option<&[Option<ObjectID>]>;
        /// Get a mutable slice of the devices "pins" (connected object Ids) if the device has pins
        fn device_pins_mut(&mut self) -> Option<&mut [Option<ObjectID>]>;
        /// Does the device respond to Activate
        fn has_activate_state(&self) -> bool;
        /// Does the device have an internal atmosphere
        fn has_atmosphere(&self) -> bool;
        /// Does the device have a Color state
        fn has_color_state(&self) -> bool;
        /// Does the device have a Lock state
        fn has_lock_state(&self) -> bool;
        /// Does the device have a mode state
        fn has_mode_state(&self) -> bool;
        /// Does the device have an On / off state
        fn has_on_off_state(&self) -> bool;
        /// Does the device have an Open state
        fn has_open_state(&self) -> bool;
        /// Does the device store reagents
        fn has_reagents(&self) -> bool;
        /// Return vector of (reagent_hash, quantity) pairs
        fn get_reagents(&self) -> Vec<(i32, f64)>;
        /// Overwrite present reagents
        fn set_reagents(&mut self, reagents: &[(i32, f64)]);
        /// Adds the reagents to contents
        fn add_reagents(&mut self, reagents: &[(i32, f64)]);
    }

    pub trait ReagentInterface: Device {
        /// Reagents required by current recipe
        fn get_current_recipie(&self) -> Vec<(i32, f64)>;
        /// Reagents required to complete current recipe
        fn get_current_required(&self) -> Vec<(i32, f64)>;
    }

    pub trait Fabricator: ReagentInterface {}

    pub trait WirelessTransmit: Logicable {}

    pub trait WirelessReceive: Logicable {}

    pub trait Network: Logicable {
        /// Does the network contain the Object id
        fn contains(&self, id: &ObjectID) -> bool;
        /// Does the network contain all the object ids
        fn contains_all(&self, ids: &[ObjectID]) -> bool;
        /// Does the network contain the object id on a data connection
        fn contains_data(&self, id: &ObjectID) -> bool;
        /// Does the network contain all the object ids on a data connection
        fn contains_all_data(&self, ids: &[ObjectID]) -> bool;
        /// Does the network contain the object id on a power connection
        fn contains_power(&self, id: &ObjectID) -> bool;
        /// Does the network contain all the object ids on a power connection
        fn contains_all_power(&self, ids: &[ObjectID]) -> bool;
        /// Return a vector of all object ids visible to the data connection of the source ID object
        fn data_visible(&self, source: &ObjectID) -> Vec<u32>;
        /// Add the object to the network as a data connection
        fn add_data(&mut self, id: ObjectID) -> bool;
        /// Add the object id as a power connection
        fn add_power(&mut self, id: ObjectID) -> bool;
        /// remove the object id for both power and data connections if present in either
        fn remove_all(&mut self, id: ObjectID) -> bool;
        /// remove the object id from data network
        fn remove_data(&mut self, id: ObjectID) -> bool;
        /// remove object id from power network
        fn remove_power(&mut self, id: ObjectID) -> bool;
        /// get all data connected devices
        fn get_devices(&self) -> Vec<ObjectID>;
        /// get all power connected devices
        fn get_power_only(&self) -> Vec<ObjectID>;
        /// get a slice of the channel data values
        fn get_channel_data(&self) -> &[f64; 8];
    }

    pub trait Human : Storage {
        fn get_species(&self) -> Species;
        fn get_damage(&self) -> f32;
        fn set_damage(&mut self, damage: f32);
        fn get_nutrition(&self) -> f32;
        fn set_nutrition(&mut self, nutrition: f32);
        fn nutrition_state(&self) -> StatState;
        fn get_hydration(&self) -> f32;
        fn set_hydration(&mut self, hydration: f32);
        fn hydration_state(&self) -> StatState;
        fn get_oxygenation(&self) -> f32;
        fn set_oxygenation(&mut self, oxygenation: f32);
        fn get_food_quality(&self) -> f32;
        fn set_food_quality(&mut self, quality: f32);
        fn get_mood(&self) -> f32;
        fn set_mood(&mut self, mood: f32);
        fn mood_state(&self) -> StatState;
        fn get_hygiene(&self) -> f32;
        fn set_hygiene(&mut self, hygine: f32);
        fn hygine_state(&self) -> StatState;
        fn is_artificial(&self) -> bool;
        fn robot_battery(&self) -> Option<VMObject>;
        fn suit_slot(&self) -> &Slot;
        fn suit_slot_mut(&mut self) -> &mut Slot;
        fn helmet_slot(&self) -> &Slot;
        fn helmet_slot_mut(&mut self) -> &mut Slot;
        fn glasses_slot(&self) -> &Slot;
        fn glasses_slot_mut(&mut self) -> &mut Slot;
        fn backpack_slot(&self) -> &Slot;
        fn backpack_slot_mut(&mut self) -> &mut Slot;
        fn left_hand_slot(&self) -> &Slot;
        fn left_hand_slot_mut(&mut self) -> &mut Slot;
        fn right_hand_slot(&self) -> &Slot;
        fn right_hand_slot_mut(&mut self) -> &mut Slot;
        fn uniform_slot(&self) -> &Slot;
        fn uniform_slot_mut(&mut self) -> &mut Slot;
        fn toolbelt_slot(&self) -> &Slot;
        fn toolbelt_slot_mut(&mut self) -> &mut Slot;
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
