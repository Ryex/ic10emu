#[macro_use]
mod utils;
// mod types;

use ic10emu::{
    errors::{TemplateError, VMError},
    vm::{
        object::{templates::FrozenObject, ObjectID, VMObject},
        FrozenVM, VM,
    },
};
use serde_derive::{Deserialize, Serialize};
// use types::{Registers, Stack};

use std::{cell::RefCell, rc::Rc, str::FromStr};

use itertools::Itertools;
// use std::iter::FromIterator;
// use itertools::Itertools;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// #[wasm_bindgen]
// pub struct DeviceRef {
//     device: Rc<RefCell<Device>>,
//     vm: Rc<RefCell<VM>>,
// }

use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum BindingError {
    #[error("{0} is not a valid variant")]
    InvalidEnumVariant(String),
    #[error("Index {0} is out of range {1}")]
    OutOfBounds(usize, usize),
}

// #[wasm_bindgen]
// impl DeviceRef {
//     fn from_device(device: Rc<RefCell<Device>>, vm: Rc<RefCell<VM>>) -> Self {
//         DeviceRef { device, vm }
//     }
//
//     #[wasm_bindgen(getter)]
//     pub fn id(&self) -> ObjectID {
//         self.device.id
//     }
//
//     #[wasm_bindgen(getter)]
//     pub fn ic(&self) -> Option<ObjectID> {
//         self.device.ic
//     }
//
//     #[wasm_bindgen(getter)]
//     pub fn name(&self) -> Option<String> {
//         self.device.name.clone()
//     }
//
//     #[wasm_bindgen(getter, js_name = "nameHash")]
//     pub fn name_hash(&self) -> Option<i32> {
//         self.device.name_hash
//     }
//
//     #[wasm_bindgen(getter, js_name = "prefabName")]
//     pub fn prefab_name(&self) -> Option<String> {
//         self.device
//
//             .prefab
//             .as_ref()
//             .map(|prefab| prefab.name.clone())
//     }
//
//     #[wasm_bindgen(getter, js_name = "prefabHash")]
//     pub fn prefab_hash(&self) -> Option<i32> {
//         self.device
//
//             .prefab
//             .as_ref()
//             .map(|prefab| prefab.hash)
//     }
//
//     #[wasm_bindgen(getter, skip_typescript)]
//     pub fn fields(&self) -> JsValue {
//         serde_wasm_bindgen::to_value(&self.device.get_fields(&self.vm.borrow())).unwrap()
//     }
//
//     #[wasm_bindgen(getter)]
//     pub fn slots(&self) -> types::Slots {
//         types::Slots::from_iter(self.device.slots.iter())
//     }
//
//     #[wasm_bindgen(getter, skip_typescript)]
//     pub fn reagents(&self) -> JsValue {
//         serde_wasm_bindgen::to_value(&self.device.reagents).unwrap()
//     }
//
//     #[wasm_bindgen(getter, skip_typescript)]
//     pub fn connections(&self) -> JsValue {
//         serde_wasm_bindgen::to_value(&self.device.connections).unwrap()
//     }
//
//     #[wasm_bindgen(getter, js_name = "ip")]
//     pub fn ic_ip(&self) -> Option<ObjectID> {
//         self.device.ic.as_ref().and_then(|ic| {
//             self.vm
//
//                 .circuit_holders
//                 .get(ic)
//                 .map(|ic| ic.as_ref().ip())
//         })
//     }
//
//     #[wasm_bindgen(getter, js_name = "instructionCount")]
//     pub fn ic_instruction_count(&self) -> Option<u16> {
//         self.device.ic.as_ref().and_then(|ic| {
//             self.vm
//
//                 .circuit_holders
//                 .get(ic)
//                 .map(|ic| ic.as_ref().ic.get())
//         })
//     }
//
//     #[wasm_bindgen(getter, js_name = "stack")]
//     pub fn ic_stack(&self) -> Option<Stack> {
//         self.device.ic.as_ref().and_then(|ic| {
//             self.vm
//
//                 .circuit_holders
//                 .get(ic)
//                 .map(|ic| Stack(*ic.as_ref().stack.borrow()))
//         })
//     }
//
//     #[wasm_bindgen(getter, js_name = "registers")]
//     pub fn ic_registers(&self) -> Option<Registers> {
//         self.device.ic.as_ref().and_then(|ic| {
//             self.vm
//
//                 .circuit_holders
//                 .get(ic)
//                 .map(|ic| Registers(*ic.as_ref().registers.borrow()))
//         })
//     }
//
//     #[wasm_bindgen(getter, js_name = "aliases", skip_typescript)]
//     pub fn ic_aliases(&self) -> JsValue {
//         let aliases = &self.device.ic.as_ref().and_then(|ic| {
//             self.vm
//
//                 .circuit_holders
//                 .get(ic)
//                 .map(|ic| ic.as_ref().aliases.borrow().clone())
//         });
//         serde_wasm_bindgen::to_value(aliases).unwrap()
//     }
//
//     #[wasm_bindgen(getter, js_name = "defines", skip_typescript)]
//     pub fn ic_defines(&self) -> JsValue {
//         let defines = &self.device.ic.as_ref().and_then(|ic| {
//             self.vm
//
//                 .circuit_holders
//                 .get(ic)
//                 .map(|ic| ic.as_ref().defines.borrow().clone())
//         });
//         serde_wasm_bindgen::to_value(defines).unwrap()
//     }
//
//     #[wasm_bindgen(getter, js_name = "pins", skip_typescript)]
//     pub fn ic_pins(&self) -> JsValue {
//         let pins = &self.device.ic.as_ref().and_then(|ic| {
//             self.vm
//
//                 .circuit_holders
//                 .get(ic)
//                 .map(|ic| *ic.as_ref().pins.borrow())
//         });
//         serde_wasm_bindgen::to_value(pins).unwrap()
//     }
//
//     #[wasm_bindgen(getter, js_name = "state")]
//     pub fn ic_state(&self) -> Option<String> {
//         self.device
//
//             .ic
//             .as_ref()
//             .and_then(|ic| {
//                 self.vm
//
//                     .circuit_holders
//                     .get(ic)
//                     .map(|ic| ic.state.clone())
//             })
//             .map(|state| state.to_string())
//     }
//
//     #[wasm_bindgen(getter, js_name = "program", skip_typescript)]
//     pub fn ic_program(&self) -> JsValue {
//         let prog = &self.device.ic.as_ref().and_then(|ic| {
//             self.vm
//
//                 .circuit_holders
//                 .get(ic)
//                 .map(|ic| ic.program.borrow().clone())
//         });
//         serde_wasm_bindgen::to_value(prog).unwrap()
//     }
//
//     #[wasm_bindgen(getter, js_name = "code")]
//     pub fn get_code(&self) -> Option<String> {
//         self.device.ic.as_ref().and_then(|ic| {
//             self.vm
//
//                 .circuit_holders
//                 .get(ic)
//                 .map(|ic| ic.code.borrow().clone())
//         })
//     }
//
//     #[wasm_bindgen(js_name = "step")]
//     pub fn step_ic(&self, advance_ip_on_err: bool) -> Result<bool, JsError> {
//         let id = self.device.id;
//         Ok(self.vm.step_ic(id, advance_ip_on_err)?)
//     }
//
//     #[wasm_bindgen(js_name = "run")]
//     pub fn run_ic(&self, ignore_errors: bool) -> Result<bool, JsError> {
//         let id = self.device.id;
//         Ok(self.vm.run_ic(id, ignore_errors)?)
//     }
//
//     #[wasm_bindgen(js_name = "reset")]
//     pub fn reset_ic(&self) -> Result<bool, JsError> {
//         let id = self.device.id;
//         Ok(self.vm.reset_ic(id)?)
//     }
//
//     #[wasm_bindgen(js_name = "setCode")]
//     /// Set program code if it's valid
//     pub fn set_code(&self, code: &str) -> Result<bool, JsError> {
//         let id = self.device.id;
//         Ok(self.vm.set_code(id, code)?)
//     }
//
//     #[wasm_bindgen(js_name = "setCodeInvalid")]
//     /// Set program code and translate invalid lines to Nop, collecting errors
//     pub fn set_code_invlaid(&self, code: &str) -> Result<bool, JsError> {
//         let id = self.device.id;
//         Ok(self.vm.set_code_invalid(id, code)?)
//     }
//
//     #[wasm_bindgen(js_name = "setRegister")]
//     pub fn ic_set_register(&self, index: ObjectID, val: f64) -> Result<f64, JsError> {
//         let ic_id = *self
//             .device
//
//             .ic
//             .as_ref()
//             .ok_or(VMError::NoIC(self.device.id))?;
//         let vm_borrow = self.vm;
//         let ic = vm_borrow
//             .circuit_holders
//             .get(&ic_id)
//             .ok_or(VMError::NoIC(self.device.id))?;
//         let result = ic.set_register(0, index, val)?;
//         Ok(result)
//     }
//
//     #[wasm_bindgen(js_name = "setStack")]
//     pub fn ic_set_stack(&self, address: f64, val: f64) -> Result<f64, JsError> {
//         let ic_id = *self
//             .device
//
//             .ic
//             .as_ref()
//             .ok_or(VMError::NoIC(self.device.id))?;
//         let vm_borrow = self.vm;
//         let ic = vm_borrow
//             .circuit_holders
//             .get(&ic_id)
//             .ok_or(VMError::NoIC(self.device.id))?;
//         let result = ic.poke(address, val)?;
//         Ok(result)
//     }
//
//     #[wasm_bindgen(js_name = "setName")]
//     pub fn set_name(&self, name: &str) {
//         self.device.set_name(name)
//     }
//
//     #[wasm_bindgen(js_name = "setField", skip_typescript)]
//     pub fn set_field(&self, field: &str, value: f64, force: bool) -> Result<(), JsError> {
//         let logic_typ = LogicType::from_str(field)?;
//         let mut device_ref = self.device;
//         device_ref.set_field(logic_typ, value, &self.vm, force)?;
//         Ok(())
//     }
//
//     #[wasm_bindgen(js_name = "setSlotField", skip_typescript)]
//     pub fn set_slot_field(
//         &self,
//         slot: f64,
//         field: &str,
//         value: f64,
//         force: bool,
//     ) -> Result<(), JsError> {
//         let logic_typ = LogicSlotType::from_str(field)?;
//         let mut device_ref = self.device;
//         device_ref.set_slot_field(slot, logic_typ, value, &self.vm, force)?;
//         Ok(())
//     }
//
//     #[wasm_bindgen(js_name = "getSlotField", skip_typescript)]
//     pub fn get_slot_field(&self, slot: f64, field: &str) -> Result<f64, JsError> {
//         let logic_typ = LogicSlotType::from_str(field)?;
//         let device_ref = self.device;
//         Ok(device_ref.get_slot_field(slot, logic_typ, &self.vm)?)
//     }
//
//     #[wasm_bindgen(js_name = "getSlotFields", skip_typescript)]
//     pub fn get_slot_fields(&self, slot: f64) -> Result<JsValue, JsError> {
//         let device_ref = self.device;
//         let fields = device_ref.get_slot_fields(slot, &self.vm)?;
//         Ok(serde_wasm_bindgen::to_value(&fields).unwrap())
//     }
//
//     #[wasm_bindgen(js_name = "setConnection")]
//     pub fn set_connection(&self, conn: usize, net: Option<ObjectID>) -> Result<(), JsError> {
//         let device_id = self.device.id;
//         self.vm
//
//             .set_device_connection(device_id, conn, net)?;
//         Ok(())
//     }
//
//     #[wasm_bindgen(js_name = "removeDeviceFromNetwork")]
//     pub fn remove_device_from_network(&self, network_id: ObjectID) -> Result<bool, JsError> {
//         let id = self.device.id;
//         Ok(self
//             .vm
//
//             .remove_device_from_network(id, network_id)?)
//     }
//
//     #[wasm_bindgen(js_name = "setPin")]
//     pub fn set_pin(&self, pin: usize, val: Option<ObjectID>) -> Result<bool, JsError> {
//         let id = self.device.id;
//         Ok(self.vm.set_pin(id, pin, val)?)
//     }
// }

#[wasm_bindgen]
#[derive(Debug)]
pub struct VMRef {
    vm: Rc<VM>,
}

// #[wasm_bindgen]
// pub struct ObjectRef(VMObject);

#[wasm_bindgen]
impl VMRef {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        VMRef { vm: VM::new() }
    }

    #[wasm_bindgen(js_name = "addDeviceFromTemplate")]
    pub fn add_device_from_template(&self, frozen: FrozenObject) -> Result<ObjectID, JsError> {
        web_sys::console::log_2(
            &"(wasm) adding device".into(),
            &serde_wasm_bindgen::to_value(&frozen).unwrap(),
        );
        Ok(self.vm.add_object_from_frozen(frozen)?)
    }

    #[wasm_bindgen(js_name = "getDevice")]
    pub fn get_object(&self, id: ObjectID) -> Option<VMObject> {
        let obj = self.vm.get_object(id);
        // device.map(|d| DeviceRef::from_device(d.clone(), self.vm.clone()))
        obj
    }

    #[wasm_bindgen(js_name = "setCode")]
    /// Set program code if it's valid
    pub fn set_code(&self, id: ObjectID, code: &str) -> Result<bool, JsError> {
        Ok(self.vm.set_code(id, code)?)
    }

    #[wasm_bindgen(js_name = "setCodeInvalid")]
    /// Set program code and translate invalid lines to Nop, collecting errors
    pub fn set_code_invalid(&self, id: ObjectID, code: &str) -> Result<bool, JsError> {
        Ok(self.vm.set_code_invalid(id, code)?)
    }

    #[wasm_bindgen(js_name = "stepIC")]
    pub fn step_ic(&self, id: ObjectID, advance_ip_on_err: bool) -> Result<(), JsError> {
        Ok(self.vm.step_programmable(id, advance_ip_on_err)?)
    }

    #[wasm_bindgen(js_name = "runIC")]
    pub fn run_ic(&self, id: ObjectID, ignore_errors: bool) -> Result<bool, JsError> {
        Ok(self.vm.run_programmable(id, ignore_errors)?)
    }

    #[wasm_bindgen(js_name = "resetIC")]
    pub fn reset_ic(&self, id: ObjectID) -> Result<bool, JsError> {
        Ok(self.vm.reset_programmable(id)?)
    }

    #[wasm_bindgen(getter, js_name = "defaultNetwork")]
    pub fn default_network(&self) -> ObjectID {
        *self.vm.default_network_key.borrow()
    }

    // #[wasm_bindgen(getter)]
    // pub fn devices(&self) -> Vec<ObjectID> {
    //     self.vm.devices.keys().copied().collect_vec()
    // }
    //
    // #[wasm_bindgen(getter)]
    // pub fn networks(&self) -> Vec<ObjectID> {
    //     self.vm.networks.keys().copied().collect_vec()
    // }
    //
    // #[wasm_bindgen(getter)]
    // pub fn ics(&self) -> Vec<ObjectID> {
    //     self.vm.circuit_holders.keys().copied().collect_vec()
    // }

    #[wasm_bindgen(getter, js_name = "lastOperationModified")]
    pub fn last_operation_modified(&self) -> Vec<ObjectID> {
        self.vm.last_operation_modified()
    }

    #[wasm_bindgen(js_name = "visibleDevices")]
    pub fn visible_devices(&self, source: ObjectID) -> Vec<u32> {
        self.vm.visible_devices(source)
    }

    #[wasm_bindgen(js_name = "setDeviceConnection")]
    pub fn set_device_connection(
        &self,
        id: ObjectID,
        connection: usize,
        network_id: Option<ObjectID>,
    ) -> Result<bool, JsError> {
        Ok(self.vm.set_device_connection(id, connection, network_id)?)
    }

    #[wasm_bindgen(js_name = "removeDeviceFromNetwork")]
    pub fn remove_device_from_network(
        &self,
        id: ObjectID,
        network_id: u32,
    ) -> Result<bool, JsError> {
        Ok(self.vm.remove_device_from_network(id, network_id)?)
    }

    #[wasm_bindgen(js_name = "setPin")]
    pub fn set_pin(&self, id: ObjectID, pin: usize, val: Option<u32>) -> Result<bool, JsError> {
        Ok(self.vm.set_pin(id, pin, val)?)
    }

    #[wasm_bindgen(js_name = "changeDeviceId")]
    pub fn change_device_id(&self, old_id: ObjectID, new_id: u32) -> Result<(), JsError> {
        Ok(self.vm.change_device_id(old_id, new_id)?)
    }

    #[wasm_bindgen(js_name = "removeDevice")]
    pub fn remove_device(&self, id: ObjectID) -> Result<(), JsError> {
        Ok(self.vm.remove_object(id)?)
    }

    #[wasm_bindgen(js_name = "setSlotOccupant")]
    pub fn set_slot_occupant(
        &self,
        id: ObjectID,
        index: usize,
        frozen: FrozenObject,
        quantity: u32,
    ) -> Result<Option<ObjectID>, JsError> {
        let obj_id = if let Some(obj) = frozen.obj_info.id.and_then(|id| self.vm.get_object(id)) {
            // TODO: we just assume if the ID is found that the frozen object passed is the same object..
            obj.get_id()
        } else {
            self.vm.add_object_from_frozen(frozen)?
        };
        Ok(self
            .vm
            .set_slot_occupant(id, index, Some(obj_id), quantity)?)
    }

    #[wasm_bindgen(js_name = "removeSlotOccupant")]
    pub fn remove_slot_occupant(&self, id: ObjectID, index: usize) -> Result<Option<ObjectID>, JsError> {
        Ok(self.vm.remove_slot_occupant(id, index)?)
    }

    #[wasm_bindgen(js_name = "saveVMState")]
    pub fn save_vm_state(&self) -> Result<FrozenVM, JsError> {
        Ok(self.vm.save_vm_state()?)
    }

    #[wasm_bindgen(js_name = "restoreVMState")]
    pub fn restore_vm_state(&self, state: FrozenVM) -> Result<(), JsError> {
        self.vm.restore_vm_state(state)?;
        Ok(())
    }
}

impl Default for VMRef {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
pub fn init() -> VMRef {
    utils::set_panic_hook();
    let vm = VMRef::new();
    log!("Hello from ic10emu!");
    vm
}
