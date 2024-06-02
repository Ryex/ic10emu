#[macro_use]
mod utils;
// mod types;

use ic10emu::{
    errors::{ICError, TemplateError, VMError},
    network::FrozenCableNetwork,
    vm::{
        object::{
            templates::{FrozenObject, FrozenObjectFull},
            ObjectID,
        },
        FrozenVM, VM,
    },
};
use itertools::Itertools;
use serde_derive::{Deserialize, Serialize};

use stationeers_data::{
    enums::script::{LogicSlotType, LogicType},
    templates::ObjectTemplate,
};

use std::{collections::BTreeMap, rc::Rc};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
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
#[derive(Debug)]
pub struct VMRef {
    vm: Rc<VM>,
}

use tsify::Tsify;

#[derive(Clone, Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TemplateDatabase(BTreeMap<i32, ObjectTemplate>);

impl IntoIterator for TemplateDatabase {
    type Item = (i32, ObjectTemplate);
    type IntoIter = std::collections::btree_map::IntoIter<i32, ObjectTemplate>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FrozenObjects(Vec<FrozenObjectFull>);

impl IntoIterator for FrozenObjects {
    type Item = FrozenObjectFull;
    type IntoIter = std::vec::IntoIter<FrozenObjectFull>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FrozenObjectsSparse(Vec<FrozenObject>);

impl IntoIterator for FrozenObjectsSparse {
    type Item = FrozenObject;
    type IntoIter = std::vec::IntoIter<FrozenObject>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FrozenNetworks(Vec<FrozenCableNetwork>);

impl IntoIterator for FrozenNetworks {
    type Item = FrozenCableNetwork;
    type IntoIter = std::vec::IntoIter<FrozenCableNetwork>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CompileErrors(Vec<ICError>);

impl IntoIterator for CompileErrors {
    type Item = ICError;
    type IntoIter = std::vec::IntoIter<ICError>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

use color_eyre::eyre;
pub fn parse_value<'a, T: serde::Deserialize<'a>>(
    jd: impl serde::Deserializer<'a>,
) -> Result<T, color_eyre::Report> {
    let mut track = serde_path_to_error::Track::new();
    let path = serde_path_to_error::Deserializer::new(jd, &mut track);
    let mut fun = |path: serde_ignored::Path| {
        log!("Found ignored key: {path}");
    };
    serde_ignored::deserialize(path, &mut fun).map_err(|e| {
        eyre::eyre!(
            "path: {track} | error = {e}",
            track = track.path().to_string(),
        )
    })
}

#[wasm_bindgen]
impl VMRef {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        VMRef { vm: VM::new() }
    }

    #[wasm_bindgen(js_name = "importTemplateDatabase")]
    pub fn import_template_database(&self, db: TemplateDatabase) {
        self.vm.import_template_database(db);
    }

    #[wasm_bindgen(js_name = "importTemplateDatabaseSerdeWasm")]
    pub fn import_template_database_serde_wasm(&self, db: JsValue) -> Result<(), JsError> {
        let parsed_db: BTreeMap<i32, ObjectTemplate> =
            parse_value(serde_wasm_bindgen::Deserializer::from(db)).map_err(|err| {
                <&dyn std::error::Error as std::convert::Into<JsError>>::into(
                    std::convert::AsRef::<dyn std::error::Error>::as_ref(&err),
                )
            })?;
        self.vm.import_template_database(parsed_db);
        Ok(())
    }
    #[wasm_bindgen(js_name = "importTemplateDatabaseSerdeJson")]
    pub fn import_template_database_serde_json(&self, db: String) -> Result<(), JsError> {
        let parsed_db: BTreeMap<i32, ObjectTemplate> =
            parse_value(&mut serde_json::Deserializer::from_str(&db)).map_err(|err| {
                <&dyn std::error::Error as std::convert::Into<JsError>>::into(
                    std::convert::AsRef::<dyn std::error::Error>::as_ref(&err),
                )
            })?;
        self.vm.import_template_database(parsed_db);
        Ok(())
    }

    #[wasm_bindgen(js_name = "getTemplateDatabase")]
    pub fn get_template_database(&self) -> TemplateDatabase {
        TemplateDatabase(self.vm.get_template_database())
    }

    #[wasm_bindgen(js_name = "addObjectFrozen")]
    pub fn add_object_frozen(&self, frozen: FrozenObject) -> Result<ObjectID, JsError> {
        web_sys::console::log_2(
            &"(wasm) adding device".into(),
            &serde_wasm_bindgen::to_value(&frozen).unwrap(),
        );
        Ok(self.vm.add_object_frozen(frozen)?)
    }

    #[wasm_bindgen(js_name = "addObjectsFrozen")]
    pub fn add_objects_frozen(
        &self,
        frozen_objects: FrozenObjectsSparse,
    ) -> Result<Vec<ObjectID>, JsError> {
        web_sys::console::log_2(
            &"(wasm) adding device".into(),
            &serde_wasm_bindgen::to_value(&frozen_objects).unwrap(),
        );

        Ok(self.vm.add_objects_frozen(frozen_objects)?)
    }

    // #[wasm_bindgen(js_name = "getDevice")]
    // pub fn get_object(&self, id: ObjectID) -> Option<VMObject> {
    //     self.vm.get_object(id)
    // }

    #[wasm_bindgen(js_name = "freezeObject")]
    pub fn freeze_object(&self, id: ObjectID) -> Result<FrozenObjectFull, JsError> {
        Ok(self.vm.freeze_object(id)?)
    }

    #[wasm_bindgen(js_name = "freezeObjects")]
    pub fn freeze_objects(&self, ids: Vec<ObjectID>) -> Result<FrozenObjects, JsError> {
        Ok(FrozenObjects(self.vm.freeze_objects(ids)?))
    }

    #[wasm_bindgen(js_name = "freezeNetwork")]
    pub fn freeze_network(&self, id: ObjectID) -> Result<FrozenCableNetwork, JsError> {
        Ok(self.vm.freeze_network(id)?)
    }

    #[wasm_bindgen(js_name = "freezeNetworks")]
    pub fn freeze_networks(&self, ids: Vec<ObjectID>) -> Result<FrozenNetworks, JsError> {
        Ok(FrozenNetworks(self.vm.freeze_networks(ids)?))
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

    #[wasm_bindgen(js_name = "getCode")]
    /// Set program code if it's valid
    pub fn get_code(&self, id: ObjectID) -> Result<String, JsError> {
        Ok(self.vm.get_code(id)?)
    }

    #[wasm_bindgen(js_name = "getCompileErrors")]
    /// Set program code if it's valid
    pub fn get_compiler_errors(&self, id: ObjectID) -> Result<CompileErrors, JsError> {
        Ok(CompileErrors(self.vm.get_compile_errors(id)?))
    }

    #[wasm_bindgen(js_name = "stepProgrammable")]
    pub fn step_programmable(&self, id: ObjectID, advance_ip_on_err: bool) -> Result<(), JsError> {
        Ok(self.vm.step_programmable(id, advance_ip_on_err)?)
    }

    #[wasm_bindgen(js_name = "runProgrammable")]
    pub fn run_programmable(&self, id: ObjectID, ignore_errors: bool) -> Result<bool, JsError> {
        Ok(self.vm.run_programmable(id, ignore_errors)?)
    }

    #[wasm_bindgen(js_name = "resetProgrammable")]
    pub fn reset_ic(&self, id: ObjectID) -> Result<bool, JsError> {
        Ok(self.vm.reset_programmable(id)?)
    }

    #[wasm_bindgen(js_name = "getDefaultNetwork")]
    pub fn get_default_network(&self) -> ObjectID {
        *self.vm.default_network_key.borrow()
    }

    #[wasm_bindgen(js_name = "getObjects")]
    pub fn get_objects(&self) -> Vec<ObjectID> {
        self.vm.objects.borrow().keys().copied().collect_vec()
    }

    #[wasm_bindgen(js_name = "getNetworks")]
    pub fn get_networks(&self) -> Vec<ObjectID> {
        self.vm.networks.borrow().keys().copied().collect_vec()
    }

    #[wasm_bindgen(js_name = "getCircuitHolders")]
    pub fn get_circuit_holders(&self) -> Vec<ObjectID> {
        self.vm.circuit_holders.borrow().clone()
    }

    #[wasm_bindgen(js_name = "getProgramHolders")]
    pub fn get_program_holders(&self) -> Vec<ObjectID> {
        self.vm.program_holders.borrow().clone()
    }

    #[wasm_bindgen(js_name = "getLastOperationModified")]
    pub fn get_last_operation_modified(&self) -> Vec<ObjectID> {
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
        let Some(prefab) = frozen.obj_info.prefab.as_ref() else {
            return Err(TemplateError::MissingPrefab.into());
        };
        let obj_id = if let Some(obj) = frozen.obj_info.id.and_then(|id| self.vm.get_object(id)) {
            // TODO: we just assume if the ID is found that the frozen object passed is the same object..
            obj.get_id()
        } else {
            // check to see if frozen is using the same prefab as current occupant
            let obj_id = if let Some(occupant_id) = {
                let obj = self.vm.get_object(id).ok_or(VMError::UnknownId(id))?;
                let obj_ref = obj.borrow();
                let storage = obj_ref.as_storage().ok_or(VMError::NotStorage(id))?;
                let slot = storage
                    .get_slot(index)
                    .ok_or(ICError::SlotIndexOutOfRange(index as f64))?;
                slot.occupant.as_ref().map(|info| info.id)
            } {
                let occupant = self
                    .vm
                    .get_object(id)
                    .ok_or(VMError::UnknownId(occupant_id))?;
                let occupant_ref = occupant.borrow();
                let occupant_prefab = occupant_ref.get_prefab();
                if prefab.as_str() == occupant_prefab.value.as_str() {
                    Some(*occupant_ref.get_id())
                } else {
                    None
                }
            } else {
                None
            };
            if let Some(obj_id) = obj_id {
                obj_id
            } else {
                self.vm.add_object_frozen(frozen)?
            }
        };
        Ok(self
            .vm
            .set_slot_occupant(id, index, Some(obj_id), quantity)?)
    }

    #[wasm_bindgen(js_name = "removeSlotOccupant")]
    pub fn remove_slot_occupant(
        &self,
        id: ObjectID,
        index: usize,
    ) -> Result<Option<ObjectID>, JsError> {
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

    #[wasm_bindgen(js_name = "getObjectName")]
    pub fn get_object_name(&self, id: ObjectID) -> Result<String, JsError> {
        let obj = self.vm.get_object(id).ok_or(VMError::UnknownId(id))?;
        let name = obj.borrow().get_name().value.clone();
        Ok(name)
    }

    #[wasm_bindgen(js_name = "setObjectName")]
    pub fn set_object_name(&self, id: ObjectID, name: &str) -> Result<(), JsError> {
        let obj = self.vm.get_object(id).ok_or(VMError::UnknownId(id))?;
        obj.borrow_mut().get_mut_name().value = name.to_string();
        Ok(())
    }

    #[wasm_bindgen(js_name = "getObjectHash")]
    pub fn get_object_hash(&self, id: ObjectID) -> Result<i32, JsError> {
        let obj = self.vm.get_object(id).ok_or(VMError::UnknownId(id))?;
        let hash = obj.borrow().get_name().hash;
        Ok(hash)
    }

    #[wasm_bindgen(js_name = "getObjectPrefabName")]
    pub fn get_object_prefab_name(&self, id: ObjectID) -> Result<String, JsError> {
        let obj = self.vm.get_object(id).ok_or(VMError::UnknownId(id))?;
        let name = obj.borrow().get_prefab().value.clone();
        Ok(name)
    }

    #[wasm_bindgen(js_name = "getObjectPrefabHash")]
    pub fn get_object_prefab_hash(&self, id: ObjectID) -> Result<i32, JsError> {
        let obj = self.vm.get_object(id).ok_or(VMError::UnknownId(id))?;
        let hash = obj.borrow().get_prefab().hash;
        Ok(hash)
    }

    #[wasm_bindgen(js_name = "getObjectSourceCode")]
    pub fn get_object_source_code(&self, id: ObjectID) -> Result<Option<String>, JsError> {
        let obj = self.vm.get_object(id).ok_or(VMError::UnknownId(id))?;
        let code = obj
            .borrow()
            .as_source_code()
            .map(|source| source.get_source_code());
        Ok(code)
    }

    #[wasm_bindgen(js_name = "setRegister")]
    pub fn set_register(&self, id: ObjectID, index: u32, val: f64) -> Result<f64, JsError> {
        Ok(self.vm.set_register(id, index, val)?)
    }

    #[wasm_bindgen(js_name = "setMemory")]
    pub fn set_memory(&self, id: ObjectID, address: u32, val: f64) -> Result<f64, JsError> {
        Ok(self.vm.set_memory(id, address, val)?)
    }

    #[wasm_bindgen(js_name = "setLogicField")]
    pub fn set_logic_field(
        &self,
        id: ObjectID,
        lt: LogicType,
        val: f64,
        force: bool,
    ) -> Result<(), JsError> {
        Ok(self.vm.set_logic_field(id, lt, val, force)?)
    }

    #[wasm_bindgen(js_name = "setSlotLogicField")]
    pub fn set_slot_logic_field(
        &self,
        id: ObjectID,
        slt: LogicSlotType,
        index: u32,
        val: f64,
        force: bool,
    ) -> Result<(), JsError> {
        Ok(self.vm.set_slot_logic_field(id, slt, index, val, force)?)
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
