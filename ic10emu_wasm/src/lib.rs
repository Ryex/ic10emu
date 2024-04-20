#[macro_use]
mod utils;
mod types;

use ic10emu::{
    device::{Device, DeviceTemplate},
    grammar::{LogicType, SlotLogicType},
    vm::{FrozenVM, VMError, VM},
};
use serde::{Deserialize, Serialize};
use types::{Registers, Stack};

use std::{cell::RefCell, rc::Rc, str::FromStr};

use itertools::Itertools;
// use itertools::Itertools;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct DeviceRef {
    device: Rc<RefCell<Device>>,
    vm: Rc<RefCell<VM>>,
}

use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum BindingError {
    #[error("{0} is not a valid variant")]
    InvalidEnumVariant(String),
    #[error("Index {0} is out of range {1}")]
    OutOfBounds(usize, usize),
}

#[wasm_bindgen]
impl DeviceRef {
    fn from_device(device: Rc<RefCell<Device>>, vm: Rc<RefCell<VM>>) -> Self {
        DeviceRef { device, vm }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u32 {
        self.device.borrow().id
    }

    #[wasm_bindgen(getter)]
    pub fn ic(&self) -> Option<u32> {
        self.device.borrow().ic
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> Option<String> {
        self.device.borrow().name.clone()
    }

    #[wasm_bindgen(getter, js_name = "nameHash")]
    pub fn name_hash(&self) -> Option<i32> {
        self.device.borrow().name_hash
    }

    #[wasm_bindgen(getter, js_name = "prefabName")]
    pub fn prefab_name(&self) -> Option<String> {
        self.device
            .borrow()
            .prefab
            .as_ref()
            .map(|prefab| prefab.name.clone())
    }

    #[wasm_bindgen(getter, js_name = "prefabHash")]
    pub fn prefab_hash(&self) -> Option<i32> {
        self.device
            .borrow()
            .prefab
            .as_ref()
            .map(|prefab| prefab.hash)
    }

    #[wasm_bindgen(getter, skip_typescript)]
    pub fn fields(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.device.borrow().get_fields(&self.vm.borrow())).unwrap()
    }

    #[wasm_bindgen(getter, skip_typescript)]
    pub fn slots(&self) -> Vec<JsValue> {
        self.device
            .borrow()
            .slots
            .iter()
            .map(|slot| {
                let flat_slot: types::Slot = slot.into();
                serde_wasm_bindgen::to_value(&flat_slot).unwrap()
            })
            .collect_vec()
    }

    #[wasm_bindgen(getter, skip_typescript)]
    pub fn reagents(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.device.borrow().reagents).unwrap()
    }

    #[wasm_bindgen(getter, skip_typescript)]
    pub fn connections(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.device.borrow().connections).unwrap()
    }

    #[wasm_bindgen(getter, js_name = "ip")]
    pub fn ic_ip(&self) -> Option<u32> {
        self.device.borrow().ic.as_ref().and_then(|ic| {
            self.vm
                .borrow()
                .ics
                .get(ic)
                .map(|ic| ic.as_ref().borrow().ip)
        })
    }

    #[wasm_bindgen(getter, js_name = "instructionCount")]
    pub fn ic_instruction_count(&self) -> Option<u16> {
        self.device.borrow().ic.as_ref().and_then(|ic| {
            self.vm
                .borrow()
                .ics
                .get(ic)
                .map(|ic| ic.as_ref().borrow().ic)
        })
    }

    #[wasm_bindgen(getter, js_name = "stack")]
    pub fn ic_stack(&self) -> Option<Stack> {
        self.device.borrow().ic.as_ref().and_then(|ic| {
            self.vm
                .borrow()
                .ics
                .get(ic)
                .map(|ic| Stack(ic.as_ref().borrow().stack))
        })
    }

    #[wasm_bindgen(getter, js_name = "registers")]
    pub fn ic_registers(&self) -> Option<Registers> {
        self.device.borrow().ic.as_ref().and_then(|ic| {
            self.vm
                .borrow()
                .ics
                .get(ic)
                .map(|ic| Registers(ic.as_ref().borrow().registers))
        })
    }

    #[wasm_bindgen(getter, js_name = "aliases", skip_typescript)]
    pub fn ic_aliases(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.device.borrow().ic.as_ref().and_then(|ic| {
            self.vm
                .borrow()
                .ics
                .get(ic)
                .map(|ic| ic.as_ref().borrow().aliases.clone())
        }))
        .unwrap()
    }

    #[wasm_bindgen(getter, js_name = "defines", skip_typescript)]
    pub fn ic_defines(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.device.borrow().ic.as_ref().and_then(|ic| {
            self.vm
                .borrow()
                .ics
                .get(ic)
                .map(|ic| ic.as_ref().borrow().defines.clone())
        }))
        .unwrap()
    }

    #[wasm_bindgen(getter, js_name = "pins", skip_typescript)]
    pub fn ic_pins(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.device.borrow().ic.as_ref().and_then(|ic| {
            self.vm
                .borrow()
                .ics
                .get(ic)
                .map(|ic| ic.as_ref().borrow().pins)
        }))
        .unwrap()
    }

    #[wasm_bindgen(getter, js_name = "state")]
    pub fn ic_state(&self) -> Option<String> {
        self.device
            .borrow()
            .ic
            .as_ref()
            .and_then(|ic| {
                self.vm
                    .borrow()
                    .ics
                    .get(ic)
                    .map(|ic| ic.borrow().state.clone())
            })
            .map(|state| state.to_string())
    }

    #[wasm_bindgen(getter, js_name = "program", skip_typescript)]
    pub fn ic_program(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.device.borrow().ic.as_ref().and_then(|ic| {
            self.vm
                .borrow()
                .ics
                .get(ic)
                .map(|ic| ic.borrow().program.clone())
        }))
        .unwrap()
    }

    #[wasm_bindgen(getter, js_name = "code")]
    pub fn get_code(&self) -> Option<String> {
        self.device.borrow().ic.as_ref().and_then(|ic| {
            self.vm
                .borrow()
                .ics
                .get(ic)
                .map(|ic| ic.borrow().code.clone())
        })
    }

    #[wasm_bindgen(js_name = "step")]
    pub fn step_ic(&self, advance_ip_on_err: bool) -> Result<bool, JsError> {
        let id = self.device.borrow().id;
        Ok(self.vm.borrow().step_ic(id, advance_ip_on_err)?)
    }

    #[wasm_bindgen(js_name = "run")]
    pub fn run_ic(&self, ignore_errors: bool) -> Result<bool, JsError> {
        let id = self.device.borrow().id;
        Ok(self.vm.borrow().run_ic(id, ignore_errors)?)
    }

    #[wasm_bindgen(js_name = "reset")]
    pub fn reset_ic(&self) -> Result<bool, JsError> {
        let id = self.device.borrow().id;
        Ok(self.vm.borrow().reset_ic(id)?)
    }

    #[wasm_bindgen(js_name = "setCode")]
    /// Set program code if it's valid
    pub fn set_code(&self, code: &str) -> Result<bool, JsError> {
        let id = self.device.borrow().id;
        Ok(self.vm.borrow().set_code(id, code)?)
    }

    #[wasm_bindgen(js_name = "setCodeInvalid")]
    /// Set program code and translate invalid lines to Nop, collecting errors
    pub fn set_code_invlaid(&self, code: &str) -> Result<bool, JsError> {
        let id = self.device.borrow().id;
        Ok(self.vm.borrow().set_code_invalid(id, code)?)
    }

    #[wasm_bindgen(js_name = "setRegister")]
    pub fn ic_set_register(&self, index: u32, val: f64) -> Result<f64, JsError> {
        let ic_id = *self
            .device
            .borrow()
            .ic
            .as_ref()
            .ok_or(VMError::NoIC(self.device.borrow().id))?;
        let vm_borrow = self.vm.borrow();
        let ic = vm_borrow
            .ics
            .get(&ic_id)
            .ok_or(VMError::NoIC(self.device.borrow().id))?;
        let result = ic.borrow_mut().set_register(0, index, val)?;
        Ok(result)
    }

    #[wasm_bindgen(js_name = "setStack")]
    pub fn ic_set_stack(&self, address: f64, val: f64) -> Result<f64, JsError> {
        let ic_id = *self
            .device
            .borrow()
            .ic
            .as_ref()
            .ok_or(VMError::NoIC(self.device.borrow().id))?;
        let vm_borrow = self.vm.borrow();
        let ic = vm_borrow
            .ics
            .get(&ic_id)
            .ok_or(VMError::NoIC(self.device.borrow().id))?;
        let result = ic.borrow_mut().poke(address, val)?;
        Ok(result)
    }

    #[wasm_bindgen(js_name = "setName")]
    pub fn set_name(&self, name: &str) {
        self.device.borrow_mut().set_name(name)
    }

    #[wasm_bindgen(js_name = "setField", skip_typescript)]
    pub fn set_field(&self, field: &str, value: f64, force: bool) -> Result<(), JsError> {
        let logic_typ = LogicType::from_str(field)?;
        let mut device_ref = self.device.borrow_mut();
        device_ref.set_field(logic_typ, value, &self.vm.borrow(), force)?;
        Ok(())
    }

    #[wasm_bindgen(js_name = "setSlotField", skip_typescript)]
    pub fn set_slot_field(
        &self,
        slot: f64,
        field: &str,
        value: f64,
        force: bool,
    ) -> Result<(), JsError> {
        let logic_typ = SlotLogicType::from_str(field)?;
        let mut device_ref = self.device.borrow_mut();
        device_ref.set_slot_field(slot, logic_typ, value, &self.vm.borrow(), force)?;
        Ok(())
    }

    #[wasm_bindgen(js_name = "getSlotField", skip_typescript)]
    pub fn get_slot_field(&self, slot: f64, field: &str) -> Result<f64, JsError> {
        let logic_typ = SlotLogicType::from_str(field)?;
        let device_ref = self.device.borrow_mut();
        Ok(device_ref.get_slot_field(slot, logic_typ, &self.vm.borrow())?)
    }

    #[wasm_bindgen(js_name = "getSlotFields", skip_typescript)]
    pub fn get_slot_fields(&self, slot: f64) -> Result<JsValue, JsError> {
        let device_ref = self.device.borrow_mut();
        let fields = device_ref.get_slot_fields(slot, &self.vm.borrow())?;
        Ok(serde_wasm_bindgen::to_value(&fields).unwrap())
    }

    #[wasm_bindgen(js_name = "setConnection")]
    pub fn set_connection(&self, conn: usize, net: Option<u32>) -> Result<(), JsError> {
        let device_id = self.device.borrow().id;
        self.vm
            .borrow()
            .set_device_connection(device_id, conn, net)?;
        Ok(())
    }

    #[wasm_bindgen(js_name = "removeDeviceFromNetwork")]
    pub fn remove_device_from_network(&self, network_id: u32) -> Result<bool, JsError> {
        let id = self.device.borrow().id;
        Ok(self
            .vm
            .borrow()
            .remove_device_from_network(id, network_id)?)
    }

    #[wasm_bindgen(js_name = "setPin")]
    pub fn set_pin(&self, pin: usize, val: Option<u32>) -> Result<bool, JsError> {
        let id = self.device.borrow().id;
        Ok(self.vm.borrow().set_pin(id, pin, val)?)
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct VMRef {
    vm: Rc<RefCell<VM>>,
}

#[wasm_bindgen]
impl VMRef {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        VMRef {
            vm: Rc::new(RefCell::new(VM::new())),
        }
    }

    #[wasm_bindgen(js_name = "addDevice")]
    pub fn add_device(&self, network: Option<u32>) -> Result<u32, JsError> {
        Ok(self.vm.borrow_mut().add_device(network)?)
    }

    #[wasm_bindgen(js_name = "addDeviceFromTemplate", skip_typescript)]
    pub fn add_device_from_template(&self, template: JsValue) -> Result<u32, JsError> {
        let template: DeviceTemplate = serde_wasm_bindgen::from_value(template)?;
        web_sys::console::log_2(
            &"(wasm) adding device".into(),
            &serde_wasm_bindgen::to_value(&template).unwrap(),
        );
        Ok(self.vm.borrow_mut().add_device_from_template(template)?)
    }

    #[wasm_bindgen(js_name = "getDevice")]
    pub fn get_device(&self, id: u32) -> Option<DeviceRef> {
        let device = self.vm.borrow().get_device(id);
        device.map(|d| DeviceRef::from_device(d.clone(), self.vm.clone()))
    }

    #[wasm_bindgen(js_name = "setCode")]
    /// Set program code if it's valid
    pub fn set_code(&self, id: u32, code: &str) -> Result<bool, JsError> {
        Ok(self.vm.borrow().set_code(id, code)?)
    }

    #[wasm_bindgen(js_name = "setCodeInvalid")]
    /// Set program code and translate invalid lines to Nop, collecting errors
    pub fn set_code_invalid(&self, id: u32, code: &str) -> Result<bool, JsError> {
        Ok(self.vm.borrow().set_code_invalid(id, code)?)
    }

    #[wasm_bindgen(js_name = "stepIC")]
    pub fn step_ic(&self, id: u32, advance_ip_on_err: bool) -> Result<bool, JsError> {
        Ok(self.vm.borrow().step_ic(id, advance_ip_on_err)?)
    }

    #[wasm_bindgen(js_name = "runIC")]
    pub fn run_ic(&self, id: u32, ignore_errors: bool) -> Result<bool, JsError> {
        Ok(self.vm.borrow().run_ic(id, ignore_errors)?)
    }

    #[wasm_bindgen(js_name = "resetIC")]
    pub fn reset_ic(&self, id: u32) -> Result<bool, JsError> {
        Ok(self.vm.borrow().reset_ic(id)?)
    }

    #[wasm_bindgen(getter, js_name = "defaultNetwork")]
    pub fn default_network(&self) -> u32 {
        self.vm.borrow().default_network
    }

    #[wasm_bindgen(getter)]
    pub fn devices(&self) -> Vec<u32> {
        self.vm.borrow().devices.keys().copied().collect_vec()
    }

    #[wasm_bindgen(getter)]
    pub fn networks(&self) -> Vec<u32> {
        self.vm.borrow().networks.keys().copied().collect_vec()
    }

    #[wasm_bindgen(getter)]
    pub fn ics(&self) -> Vec<u32> {
        self.vm.borrow().ics.keys().copied().collect_vec()
    }

    #[wasm_bindgen(getter, js_name = "lastOperationModified")]
    pub fn last_operation_modified(&self) -> Vec<u32> {
        self.vm.borrow().last_operation_modified()
    }

    #[wasm_bindgen(js_name = "visibleDevices")]
    pub fn visible_devices(&self, source: u32) -> Vec<u32> {
        self.vm.borrow().visible_devices(source)
    }

    #[wasm_bindgen(js_name = "setDeviceConnection")]
    pub fn set_device_connection(
        &self,
        id: u32,
        connection: usize,
        network_id: Option<u32>,
    ) -> Result<bool, JsError> {
        Ok(self
            .vm
            .borrow()
            .set_device_connection(id, connection, network_id)?)
    }

    #[wasm_bindgen(js_name = "removeDeviceFromNetwork")]
    pub fn remove_device_from_network(&self, id: u32, network_id: u32) -> Result<bool, JsError> {
        Ok(self
            .vm
            .borrow()
            .remove_device_from_network(id, network_id)?)
    }

    #[wasm_bindgen(js_name = "setPin")]
    pub fn set_pin(&self, id: u32, pin: usize, val: Option<u32>) -> Result<bool, JsError> {
        Ok(self.vm.borrow().set_pin(id, pin, val)?)
    }

    #[wasm_bindgen(js_name = "changeDeviceId")]
    pub fn change_device_id(&self, old_id: u32, new_id: u32) -> Result<(), JsError> {
        Ok(self.vm.borrow_mut().change_device_id(old_id, new_id)?)
    }

    #[wasm_bindgen(js_name = "removeDevice")]
    pub fn remove_device(&self, id: u32) -> Result<(), JsError> {
        Ok(self.vm.borrow_mut().remove_device(id)?)
    }

    #[wasm_bindgen(js_name = "saveVMState", skip_typescript)]
    pub fn save_vm_state(&self) -> JsValue {
        let state = self.vm.borrow().save_vm_state();
        serde_wasm_bindgen::to_value(&state).unwrap()
    }

    #[wasm_bindgen(js_name = "restoreVMState", skip_typescript)]
    pub fn restore_vm_state(&self, state: JsValue) -> Result<(), JsError> {
        let state: FrozenVM = serde_wasm_bindgen::from_value(state)?;
        self.vm.borrow_mut().restore_vm_state(state)?;
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
