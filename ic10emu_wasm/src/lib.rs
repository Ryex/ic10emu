#[macro_use]
mod utils;

use std::{cell::RefCell, rc::Rc};

use itertools::Itertools;
// use itertools::Itertools;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use serde_with::serde_as;

#[serde_as]
#[derive(Serialize, Deserialize)]
struct Stack(#[serde_as(as = "[_; 512]")] [f64; 512]);

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct DeviceRef {
    device: Rc<RefCell<ic10emu::Device>>,
    vm: Rc<RefCell<ic10emu::VM>>,
}

#[wasm_bindgen]
impl DeviceRef {
    fn from_device(device: Rc<RefCell<ic10emu::Device>>, vm: Rc<RefCell<ic10emu::VM>>) -> Self {
        DeviceRef { device, vm }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u16 {
        self.device.borrow().id
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> Option<String> {
        self.device.borrow().name.clone()
    }

    #[wasm_bindgen(getter, js_name = "nameHash")]
    pub fn name_hash(&self) -> Option<f64> {
        self.device.borrow().name_hash
    }

    #[wasm_bindgen(getter)]
    pub fn fields(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.device.borrow().fields).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn slots(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.device.borrow().slots).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn reagents(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.device.borrow().reagents).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn connections(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.device.borrow().connections).unwrap()
    }

    #[wasm_bindgen(getter, js_name = "ip")]
    pub fn ic_ip(&self) -> Option<u32> {
        self.device
            .borrow()
            .ic
            .as_ref()
            .map(|ic| {
                self.vm
                    .borrow()
                    .ics
                    .get(ic)
                    .map(|ic| ic.as_ref().borrow().ip)
            })
            .flatten()
    }

    #[wasm_bindgen(getter, js_name = "instructionCount")]
    pub fn ic_instruction_count(&self) -> Option<u16> {
        self.device
            .borrow()
            .ic
            .as_ref()
            .map(|ic| {
                self.vm
                    .borrow()
                    .ics
                    .get(ic)
                    .map(|ic| ic.as_ref().borrow().ic)
            })
            .flatten()
    }

    #[wasm_bindgen(getter, js_name = "stack")]
    pub fn ic_stack(&self) -> JsValue {
        serde_wasm_bindgen::to_value(
            &self
                .device
                .borrow()
                .ic
                .as_ref()
                .map(|ic| {
                    self.vm
                        .borrow()
                        .ics
                        .get(ic)
                        .map(|ic| Stack(ic.as_ref().borrow().stack))
                })
                .flatten(),
        )
        .unwrap()
    }

    #[wasm_bindgen(getter, js_name = "registers")]
    pub fn ic_registers(&self) -> JsValue {
        serde_wasm_bindgen::to_value(
            &self
                .device
                .borrow()
                .ic
                .as_ref()
                .map(|ic| {
                    self.vm
                        .borrow()
                        .ics
                        .get(ic)
                        .map(|ic| ic.as_ref().borrow().registers)
                })
                .flatten(),
        )
        .unwrap()
    }

    #[wasm_bindgen(getter, js_name = "aliases")]
    pub fn ic_aliases(&self) -> JsValue {
        serde_wasm_bindgen::to_value(
            &self
                .device
                .borrow()
                .ic
                .as_ref()
                .map(|ic| {
                    self.vm
                        .borrow()
                        .ics
                        .get(ic)
                        .map(|ic| ic.as_ref().borrow().aliases.clone())
                })
                .flatten(),
        )
        .unwrap()
    }

    #[wasm_bindgen(getter, js_name = "defines")]
    pub fn ic_defines(&self) -> JsValue {
        serde_wasm_bindgen::to_value(
            &self
                .device
                .borrow()
                .ic
                .as_ref()
                .map(|ic| {
                    self.vm
                        .borrow()
                        .ics
                        .get(ic)
                        .map(|ic| ic.as_ref().borrow().defines.clone())
                })
                .flatten(),
        )
        .unwrap()
    }

    #[wasm_bindgen(getter, js_name = "pins")]
    pub fn ic_pins(&self) -> JsValue {
        serde_wasm_bindgen::to_value(
            &self
                .device
                .borrow()
                .ic
                .as_ref()
                .map(|ic| {
                    self.vm
                        .borrow()
                        .ics
                        .get(ic)
                        .map(|ic| ic.as_ref().borrow().pins)
                })
                .flatten(),
        )
        .unwrap()
    }

    #[wasm_bindgen(getter, js_name = "state")]
    pub fn ic_state(&self) -> Option<String> {
        self.device
            .borrow()
            .ic
            .as_ref()
            .map(|ic| {
                self.vm
                    .borrow()
                    .ics
                    .get(ic)
                    .map(|ic| ic.borrow().state.clone())
            })
            .flatten()
            .map(|state| state.to_string())
    }

    #[wasm_bindgen(getter, js_name = "program")]
    pub fn ic_program(&self) -> JsValue {
        serde_wasm_bindgen::to_value(
            &self
                .device
                .borrow()
                .ic
                .as_ref()
                .map(|ic| {
                    self.vm
                        .borrow()
                        .ics
                        .get(ic)
                        .map(|ic| ic.borrow().program.clone())
                })
                .flatten(),
        )
        .unwrap()
    }

    #[wasm_bindgen(js_name = "step")]
    pub fn step_ic(&self) -> Result<bool, JsError> {
        let id = self.device.borrow().id;
        Ok(self.vm.borrow().step_ic(id)?)
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
    pub fn set_code(&self, code: &str) -> Result<bool, JsError> {
        let id = self.device.borrow().id;
        Ok(self.vm.borrow().set_code(id, code)?)
    }

    #[wasm_bindgen(js_name = "setRegister")]
    pub fn ic_set_register(&self, index: u32, val: f64) -> Result<f64, JsError> {
        let ic_id = *self
            .device
            .borrow()
            .ic
            .as_ref()
            .ok_or(ic10emu::VMError::NoIC(self.device.borrow().id))?;
        let vm_borrow = self.vm.borrow();
        let ic =  vm_borrow
            .ics
            .get(&ic_id)
            .ok_or(ic10emu::VMError::NoIC(self.device.borrow().id))?;
        let result = ic.borrow_mut().set_register(0, index, val)?;
        Ok(result)
    }

    #[wasm_bindgen(js_name = "setStack")]
    pub fn ic_set_stack(&mut self, address: f64, val: f64) -> Result<f64, JsError> {
        let ic_id = *self
            .device
            .borrow()
            .ic
            .as_ref()
            .ok_or(ic10emu::VMError::NoIC(self.device.borrow().id))?;
        let vm_borrow = self.vm.borrow();
        let ic =  vm_borrow
            .ics
            .get(&ic_id)
            .ok_or(ic10emu::VMError::NoIC(self.device.borrow().id))?;
        let result = ic.borrow_mut().poke(address, val)?;
        Ok(result)
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct VM {
    vm: Rc<RefCell<ic10emu::VM>>,
}
#[wasm_bindgen]
impl VM {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        VM {
            vm: Rc::new(RefCell::new(ic10emu::VM::new())),
        }
    }

    #[wasm_bindgen(js_name = "addDevice")]
    pub fn add_device(&self, network: Option<u16>) -> Result<u16, JsError> {
        Ok(self.vm.borrow_mut().add_device(network)?)
    }

    #[wasm_bindgen(js_name = "getDevice")]
    pub fn get_device(&self, id: u16) -> Option<DeviceRef> {
        let device = self.vm.borrow().get_device(id);
        device.map(|d| DeviceRef::from_device(d.clone(), self.vm.clone()))
    }

    #[wasm_bindgen(js_name = "setCode")]
    pub fn set_code(&self, id: u16, code: &str) -> Result<bool, JsError> {
        Ok(self.vm.borrow().set_code(id, code)?)
    }

    #[wasm_bindgen(js_name = "stepIC")]
    pub fn step_ic(&self, id: u16) -> Result<bool, JsError> {
        Ok(self.vm.borrow().step_ic(id)?)
    }

    #[wasm_bindgen(js_name = "runIC")]
    pub fn run_ic(&self, id: u16, ignore_errors: bool) -> Result<bool, JsError> {
        Ok(self.vm.borrow().run_ic(id, ignore_errors)?)
    }

    #[wasm_bindgen(js_name = "resetIC")]
    pub fn reset_ic(&self, id: u16) -> Result<bool, JsError> {
        Ok(self.vm.borrow().reset_ic(id)?)
    }

    #[wasm_bindgen(getter, js_name = "defaultNetwork")]
    pub fn default_network(&self) -> u16 {
        self.vm.borrow().default_network
    }

    #[wasm_bindgen(getter)]
    pub fn devices(&self) -> Vec<u16> {
        self.vm.borrow().devices.keys().copied().collect_vec()
    }

    #[wasm_bindgen(getter)]
    pub fn networks(&self) -> Vec<u16> {
        self.vm.borrow().networks.keys().copied().collect_vec()
    }

    #[wasm_bindgen(getter)]
    pub fn ics(&self) -> Vec<u16> {
        self.vm.borrow().ics.keys().copied().collect_vec()
    }
}

#[wasm_bindgen]
pub fn init() -> VM {
    utils::set_panic_hook();
    let vm = VM::new();
    log!("Hello from ic10emu!");
    vm
}
