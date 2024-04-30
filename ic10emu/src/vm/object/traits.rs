use macro_rules_attribute::apply;
use crate::vm::object::macros::ObjectTrait;
use crate::vm::object::VmObject;

#[apply(ObjectTrait!)]
#[custom(object_trait = VmObject)]
pub trait Memory: Test {
    fn get_memory(&self) -> &Vec<u32>;
    fn set_memory(&mut self, index: usize, val: u32);
}


pub trait Test {
    fn test(&self) {
        println!("test!");
    }
}
