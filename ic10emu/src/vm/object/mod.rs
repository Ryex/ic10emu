use macro_rules_attribute::derive;

mod macros;
mod traits;
mod stationpedia;

use macros::ObjectInterface;
use traits::*;

pub type ObjectID = u32;
pub type BoxedObject = Box<dyn Object<ID = ObjectID>>;


#[derive(ObjectInterface!)]
#[custom(implements(Object { Memory }))]
pub struct Generic {
    mem1: Vec<f64>,

    #[custom(object_id)]
    id: ObjectID,

    mem2: Vec<f64>,
}

impl Memory for Generic {
    fn get_memory(&self) -> &Vec<f64> {
        &self.mem1
    }

    fn set_memory(&mut self, index: usize, val: f64) {
        self.mem2[index] = val;
    }

    fn size(&self) -> usize {
        self.mem1.len()
    }
}

