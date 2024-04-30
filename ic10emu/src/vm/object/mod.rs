use macro_rules_attribute::derive;

mod macros;
mod traits;

use macros::{object_trait, ObjectInterface};
use traits::Memory;

use crate::vm::object::traits::Test;

object_trait!(VmObject { Memory });

#[derive(ObjectInterface!)]
#[custom(implements(VmObject { Memory }))]
pub struct Generic {
    mem1: Vec<u32>,

    #[custom(object_id)]
    id: u32,

    mem2: Vec<u32>,
}

impl Memory for Generic {
    fn get_memory(&self) -> &Vec<u32> {
        &self.mem1
    }

    fn set_memory(&mut self, index: usize, val: u32) {
        self.mem2[index] = val;
    }
}

impl Test for Generic {}
