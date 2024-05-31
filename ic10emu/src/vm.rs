pub mod instructions;
pub mod object;

use crate::{
    errors::{ICError, VMError},
    interpreter::ICState,
    network::{CableConnectionType, CableNetwork, Connection, FrozenCableNetwork},
    vm::object::{
        templates::{FrozenObject, FrozenObjectFull, Prefab},
        traits::ParentSlotInfo,
        ObjectID, SlotOccupantInfo, VMObject,
    },
};
use stationeers_data::{
    enums::{
        script::{LogicBatchMethod, LogicSlotType, LogicType},
        ConnectionRole,
    },
    templates::ObjectTemplate,
};
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashSet},
    rc::Rc,
};
#[cfg(feature = "tsify")]
use tsify::Tsify;
#[cfg(feature = "tsify")]
use wasm_bindgen::prelude::*;

use itertools::Itertools;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub struct VM {
    pub objects: RefCell<BTreeMap<ObjectID, VMObject>>,
    pub circuit_holders: RefCell<Vec<ObjectID>>,
    pub program_holders: RefCell<Vec<ObjectID>>,
    pub networks: RefCell<BTreeMap<ObjectID, VMObject>>,
    pub default_network_key: RefCell<ObjectID>,
    pub wireless_transmitters: RefCell<Vec<ObjectID>>,
    pub wireless_receivers: RefCell<Vec<ObjectID>>,
    id_space: RefCell<IdSpace>,
    network_id_space: RefCell<IdSpace>,
    random: Rc<RefCell<crate::rand_mscorlib::Random>>,

    /// list of object id's touched on the last operation
    operation_modified: RefCell<Vec<ObjectID>>,
    template_database: RefCell<Option<BTreeMap<i32, ObjectTemplate>>>,
}

#[derive(Debug, Default)]
pub struct VMTransactionNetwork {
    pub devices: Vec<ObjectID>,
    pub power_only: Vec<ObjectID>,
}

#[allow(dead_code)]
#[derive(Debug)]
/// used as a temp structure to add objects in case
/// there are errors on nested templates
struct VMTransaction {
    pub objects: BTreeMap<ObjectID, VMObject>,
    pub circuit_holders: Vec<ObjectID>,
    pub program_holders: Vec<ObjectID>,
    pub default_network_key: ObjectID,
    pub wireless_transmitters: Vec<ObjectID>,
    pub wireless_receivers: Vec<ObjectID>,
    pub id_space: IdSpace,
    pub networks: BTreeMap<ObjectID, VMTransactionNetwork>,
    object_parents: BTreeMap<ObjectID, (u32, ObjectID)>,
    vm: Rc<VM>,
}

impl VM {
    /// Create a new VM with it's own state and a default network
    pub fn new() -> Rc<Self> {
        let id_space = IdSpace::default();
        let mut network_id_space = IdSpace::default();
        let default_network_key = network_id_space.next();
        let networks = BTreeMap::new();

        let vm = Rc::new(VM {
            objects: RefCell::new(BTreeMap::new()),
            circuit_holders: RefCell::new(Vec::new()),
            program_holders: RefCell::new(Vec::new()),
            networks: RefCell::new(networks),
            default_network_key: RefCell::new(default_network_key),
            wireless_transmitters: RefCell::new(Vec::new()),
            wireless_receivers: RefCell::new(Vec::new()),
            id_space: RefCell::new(id_space),
            network_id_space: RefCell::new(network_id_space),
            random: Rc::new(RefCell::new(crate::rand_mscorlib::Random::new())),
            operation_modified: RefCell::new(Vec::new()),
            template_database: RefCell::new(stationeers_data::build_prefab_database()),
        });

        let default_network = VMObject::new(CableNetwork::new(default_network_key, vm.clone()));
        vm.networks
            .borrow_mut()
            .insert(default_network_key, default_network);

        vm
    }

    /// get a random f64 value using a mscorlib rand PRNG
    /// (Stationeers, being written in .net, using mscorlib's rand)
    pub fn random_f64(self: &Rc<Self>) -> f64 {
        self.random.borrow_mut().next_f64()
    }

    /// Take ownership of an iterable the produces (prefab hash, ObjectTemplate) pairs and build a prefab
    /// database
    pub fn import_template_database(
        self: &Rc<Self>,
        db: impl IntoIterator<Item = (i32, ObjectTemplate)>,
    ) {
        self.template_database
            .borrow_mut()
            .replace(db.into_iter().collect());
    }

    /// Get a Object Template by either prefab name or hash
    pub fn get_template(self: &Rc<Self>, prefab: Prefab) -> Option<ObjectTemplate> {
        let hash = match prefab {
            Prefab::Hash(hash) => hash,
            Prefab::Name(name) => const_crc32::crc32(name.as_bytes()) as i32,
        };
        self.template_database
            .borrow()
            .as_ref()
            .and_then(|db| db.get(&hash).cloned())
    }

    pub fn get_template_database(self: &Rc<Self>) -> BTreeMap<i32, ObjectTemplate> {
        self.template_database
            .borrow()
            .as_ref()
            .cloned()
            .unwrap_or_default()
    }

    /// Add an number of object to the VM state using Frozen Object strusts.
    /// See also `add_objects_frozen`
    /// Returns the built objects' IDs
    pub fn add_objects_frozen(
        self: &Rc<Self>,
        frozen_objects: impl IntoIterator<Item = FrozenObject>,
    ) -> Result<Vec<ObjectID>, VMError> {
        let mut transaction = VMTransaction::new(self);

        let mut obj_ids = Vec::new();
        for frozen in frozen_objects {
            let obj_id = transaction.add_object_from_frozen(frozen)?;
            obj_ids.push(obj_id)
        }

        transaction.finialize()?;

        let transaction_ids = transaction.id_space.in_use_ids();
        self.id_space.borrow_mut().use_new_ids(&transaction_ids);

        self.objects.borrow_mut().extend(transaction.objects);
        self.wireless_transmitters
            .borrow_mut()
            .extend(transaction.wireless_transmitters);
        self.wireless_receivers
            .borrow_mut()
            .extend(transaction.wireless_receivers);
        self.circuit_holders
            .borrow_mut()
            .extend(transaction.circuit_holders);
        self.program_holders
            .borrow_mut()
            .extend(transaction.program_holders);
        for (net_id, trans_net) in transaction.networks.into_iter() {
            let net = self
                .networks
                .borrow()
                .get(&net_id)
                .cloned()
                .unwrap_or_else(|| panic!("desync between vm and transaction networks: {net_id}"));
            let mut net_ref = net.borrow_mut();
            let net_interface = net_ref
                .as_mut_network()
                .ok_or(VMError::NonNetworkNetwork(net_id))?;
            for id in trans_net.devices {
                net_interface.add_data(id);
            }
            for id in trans_net.power_only {
                net_interface.add_power(id);
            }
        }

        Ok(obj_ids)
    }

    /// Add an object to the VM state using a frozen object struct
    /// Errors if the frozen object does not provide a template and the prefab has is not in the
    /// current database.
    /// Errors if the object can not be built do to a template error
    /// Returns the built object's ID
    pub fn add_object_frozen(
        self: &Rc<Self>,
        frozen: FrozenObject,
    ) -> Result<ObjectID, VMError> {
        let mut transaction = VMTransaction::new(self);

        let obj_id = transaction.add_object_from_frozen(frozen)?;

        transaction.finialize()?;

        let transaction_ids = transaction.id_space.in_use_ids();
        self.id_space.borrow_mut().use_new_ids(&transaction_ids);

        self.objects.borrow_mut().extend(transaction.objects);
        self.wireless_transmitters
            .borrow_mut()
            .extend(transaction.wireless_transmitters);
        self.wireless_receivers
            .borrow_mut()
            .extend(transaction.wireless_receivers);
        self.circuit_holders
            .borrow_mut()
            .extend(transaction.circuit_holders);
        self.program_holders
            .borrow_mut()
            .extend(transaction.program_holders);
        for (net_id, trans_net) in transaction.networks.into_iter() {
            let net = self
                .networks
                .borrow()
                .get(&net_id)
                .cloned()
                .unwrap_or_else(|| panic!("desync between vm and transaction networks: {net_id}"));
            let mut net_ref = net.borrow_mut();
            let net_interface = net_ref
                .as_mut_network()
                .ok_or(VMError::NonNetworkNetwork(net_id))?;
            for id in trans_net.devices {
                net_interface.add_data(id);
            }
            for id in trans_net.power_only {
                net_interface.add_power(id);
            }
        }

        Ok(obj_id)
    }

    /// Creates a new network adn return it's ID
    pub fn add_network(self: &Rc<Self>) -> ObjectID {
        let next_id = self.network_id_space.borrow_mut().next();
        self.networks.borrow_mut().insert(
            next_id,
            VMObject::new(CableNetwork::new(next_id, self.clone())),
        );
        next_id
    }

    /// Get Id of default network
    pub fn get_default_network(self: &Rc<Self>) -> VMObject {
        self.networks
            .borrow()
            .get(&*self.default_network_key.borrow())
            .cloned()
            .expect("default network not present")
    }

    /// Get network form Id
    pub fn get_network(self: &Rc<Self>, id: ObjectID) -> Option<VMObject> {
        self.networks.borrow().get(&id).cloned()
    }

    /// Change an object's ID
    ///
    /// Iterates over all objects borrowing them mutably, never call unless VM is not currently
    /// stepping or you'll get reborrow panics
    pub fn change_device_id(
        self: &Rc<Self>,
        old_id: ObjectID,
        new_id: ObjectID,
    ) -> Result<(), VMError> {
        if self.id_space.borrow().has_id(&new_id) {
            return Err(VMError::IdInUse(new_id));
        }
        let obj = self
            .objects
            .borrow_mut()
            .remove(&old_id)
            .ok_or(VMError::UnknownId(old_id))?;
        self.id_space.borrow_mut().use_id(new_id)?;
        obj.borrow_mut().set_id(new_id);
        self.objects.borrow_mut().insert(new_id, obj);

        for obj in self.objects.borrow().values() {
            let mut obj_ref = obj.borrow_mut();
            if let Some(device) = obj_ref.as_mut_device() {
                device.get_slots_mut().iter_mut().for_each(|slot| {
                    if slot.parent == old_id {
                        slot.parent = new_id;
                    }
                    match slot.occupant.as_mut() {
                        Some(info) if info.id == old_id => {
                            info.id = new_id;
                        }
                        _ => (),
                    }
                });
            }
        }

        self.circuit_holders.borrow_mut().iter_mut().for_each(|id| {
            if *id == old_id {
                *id = new_id;
            }
        });
        self.program_holders.borrow_mut().iter_mut().for_each(|id| {
            if *id == old_id {
                *id = new_id;
            }
        });
        self.networks.borrow().iter().for_each(|(_net_id, net)| {
            let mut net_ref = net.borrow_mut();
            let net_interface = net_ref.as_mut_network().expect("non-network network");
            if net_interface.remove_data(old_id) {
                net_interface.add_data(new_id);
            }
            if net_interface.remove_power(old_id) {
                net_interface.add_power(new_id);
            }
        });
        self.id_space.borrow_mut().free_id(old_id);
        Ok(())
    }

    /// Set program code if it's valid
    /// Object Id is the programmable Id or the circuit holder's id
    pub fn set_code(self: &Rc<Self>, id: ObjectID, code: &str) -> Result<bool, VMError> {
        let obj = self
            .objects
            .borrow()
            .get(&id)
            .cloned()
            .ok_or(VMError::UnknownId(id))?;
        {
            let mut obj_ref = obj.borrow_mut();
            if let Some(programmable) = obj_ref.as_mut_source_code() {
                programmable.set_source_code(code)?;
                return Ok(true);
            }
        }
        let ic_obj = {
            let obj_ref = obj.borrow();
            if let Some(circuit_holder) = obj_ref.as_circuit_holder() {
                circuit_holder.get_ic()
            } else {
                return Err(VMError::NotCircuitHolderOrProgrammable(id));
            }
        };
        if let Some(ic_obj) = ic_obj {
            let mut ic_obj_ref = ic_obj.borrow_mut();
            if let Some(programmable) = ic_obj_ref.as_mut_programmable() {
                programmable.set_source_code(code)?;
                return Ok(true);
            }
            return Err(VMError::NotProgrammable(*ic_obj_ref.get_id()));
        }
        Err(VMError::NoIC(id))
    }

    /// Set program code and translate invalid lines to Nop, collecting errors
    /// Object Id is the programmable Id or the circuit holder's id
    pub fn set_code_invalid(self: &Rc<Self>, id: ObjectID, code: &str) -> Result<bool, VMError> {
        let obj = self
            .objects
            .borrow()
            .get(&id)
            .cloned()
            .ok_or(VMError::UnknownId(id))?;
        {
            let mut obj_ref = obj.borrow_mut();
            if let Some(programmable) = obj_ref.as_mut_programmable() {
                programmable.set_source_code_with_invalid(code);
                return Ok(true);
            }
        }
        let ic_obj = {
            let obj_ref = obj.borrow();
            if let Some(circuit_holder) = obj_ref.as_circuit_holder() {
                circuit_holder.get_ic()
            } else {
                return Err(VMError::NotCircuitHolderOrProgrammable(id));
            }
        };
        if let Some(ic_obj) = ic_obj {
            let mut ic_obj_ref = ic_obj.borrow_mut();
            if let Some(programmable) = ic_obj_ref.as_mut_programmable() {
                programmable.set_source_code_with_invalid(code);
                return Ok(true);
            }
            return Err(VMError::NotProgrammable(*ic_obj_ref.get_id()));
        }
        Err(VMError::NoIC(id))
    }

    /// Get program code
    /// Object Id is the programmable Id or the circuit holder's id
    pub fn get_code(self: &Rc<Self>, id: ObjectID) -> Result<String, VMError> {
        let obj = self
            .objects
            .borrow()
            .get(&id)
            .cloned()
            .ok_or(VMError::UnknownId(id))?;
        {
            let obj_ref = obj.borrow();
            if let Some(programmable) = obj_ref.as_source_code() {
                return Ok(programmable.get_source_code());
            }
        }
        let ic_obj = {
            let obj_ref = obj.borrow();
            if let Some(circuit_holder) = obj_ref.as_circuit_holder() {
                circuit_holder.get_ic()
            } else {
                return Err(VMError::NotCircuitHolderOrProgrammable(id));
            }
        };
        if let Some(ic_obj) = ic_obj {
            let ic_obj_ref = ic_obj.borrow();
            if let Some(programmable) = ic_obj_ref.as_source_code() {
                return Ok(programmable.get_source_code());
            }
            return Err(VMError::NotProgrammable(*ic_obj_ref.get_id()));
        }
        Err(VMError::NoIC(id))
    }

    /// Get a vector of any errors compiling the source code
    /// Object Id is the programmable Id or the circuit holder's id
    pub fn get_compile_errors(self: &Rc<Self>, id: ObjectID) -> Result<Vec<ICError>, VMError> {
        let obj = self
            .objects
            .borrow()
            .get(&id)
            .cloned()
            .ok_or(VMError::UnknownId(id))?;
        {
            let obj_ref = obj.borrow();
            if let Some(programmable) = obj_ref.as_source_code() {
                return Ok(programmable.get_compile_errors());
            }
        }
        let ic_obj = {
            let obj_ref = obj.borrow();
            if let Some(circuit_holder) = obj_ref.as_circuit_holder() {
                circuit_holder.get_ic()
            } else {
                return Err(VMError::NotCircuitHolderOrProgrammable(id));
            }
        };
        if let Some(ic_obj) = ic_obj {
            let ic_obj_ref = ic_obj.borrow();
            if let Some(programmable) = ic_obj_ref.as_source_code() {
                return Ok(programmable.get_compile_errors());
            }
            return Err(VMError::NotProgrammable(*ic_obj_ref.get_id()));
        }
        Err(VMError::NoIC(id))
    }

    /// Set register of integrated circuit
    /// Object Id is the circuit Id or the circuit holder's id
    pub fn set_register(
        self: &Rc<Self>,
        id: ObjectID,
        index: u32,
        val: f64,
    ) -> Result<f64, VMError> {
        let obj = self
            .objects
            .borrow()
            .get(&id)
            .cloned()
            .ok_or(VMError::UnknownId(id))?;
        {
            let mut obj_ref = obj.borrow_mut();
            if let Some(circuit) = obj_ref.as_mut_integrated_circuit() {
                let last = circuit.set_register(0, index, val)?;
                return Ok(last);
            }
        }
        let ic_obj = {
            let obj_ref = obj.borrow();
            if let Some(circuit_holder) = obj_ref.as_circuit_holder() {
                circuit_holder.get_ic()
            } else {
                return Err(VMError::NotCircuitHolderOrProgrammable(id));
            }
        };
        if let Some(ic_obj) = ic_obj {
            let mut ic_obj_ref = ic_obj.borrow_mut();
            if let Some(circuit) = ic_obj_ref.as_mut_integrated_circuit() {
                let last = circuit.set_register(0, index, val)?;
                return Ok(last);
            }
            return Err(VMError::NotProgrammable(*ic_obj_ref.get_id()));
        }
        Err(VMError::NoIC(id))
    }

    /// Set memory at address of object with memory
    /// Object Id is the memory writable Id or the circuit holder's id
    pub fn set_memory(
        self: &Rc<Self>,
        id: ObjectID,
        address: u32,
        val: f64,
    ) -> Result<f64, VMError> {
        let obj = self
            .objects
            .borrow()
            .get(&id)
            .cloned()
            .ok_or(VMError::UnknownId(id))?;
        {
            let mut obj_ref = obj.borrow_mut();
            if let Some(circuit) = obj_ref.as_mut_memory_writable() {
                let last = circuit
                    .get_memory(address as i32)
                    .map_err(Into::<ICError>::into)?;
                circuit
                    .set_memory(address as i32, val)
                    .map_err(Into::<ICError>::into)?;
                return Ok(last);
            }
        }
        let ic_obj = {
            let obj_ref = obj.borrow();
            if let Some(circuit_holder) = obj_ref.as_circuit_holder() {
                circuit_holder.get_ic()
            } else {
                None
            }
        };
        if let Some(ic_obj) = ic_obj {
            let mut ic_obj_ref = ic_obj.borrow_mut();
            if let Some(circuit) = ic_obj_ref.as_mut_memory_writable() {
                let last = circuit
                    .get_memory(address as i32)
                    .map_err(Into::<ICError>::into)?;
                circuit
                    .set_memory(address as i32, val)
                    .map_err(Into::<ICError>::into)?;
                return Ok(last);
            }
            return Err(VMError::NotMemoryWritable(*ic_obj_ref.get_id()));
        }
        Err(VMError::NotCircuitHolderOrMemoryWritable(id))
    }

    /// Set logic field on a logicable object
    pub fn set_logic_field(
        self: &Rc<Self>,
        id: ObjectID,
        lt: LogicType,
        val: f64,
        force: bool,
    ) -> Result<(), VMError> {
        let obj = self
            .objects
            .borrow()
            .get(&id)
            .cloned()
            .ok_or(VMError::UnknownId(id))?;
        let mut obj_ref = obj.borrow_mut();
        let logicable = obj_ref
            .as_mut_logicable()
            .ok_or(VMError::NotLogicable(id))?;
        logicable
            .set_logic(lt, val, force)
            .map_err(Into::<ICError>::into)?;
        Ok(())
    }

    /// Set slot logic filed on device object
    pub fn set_slot_logic_field(
        self: &Rc<Self>,
        id: ObjectID,
        slt: LogicSlotType,
        index: u32,
        val: f64,
        force: bool,
    ) -> Result<(), VMError> {
        let obj = self
            .objects
            .borrow()
            .get(&id)
            .cloned()
            .ok_or(VMError::UnknownId(id))?;
        let mut obj_ref = obj.borrow_mut();
        let device = obj_ref.as_mut_device().ok_or(VMError::NotLogicable(id))?;
        device
            .set_slot_logic(slt, index as f64, val, force)
            .map_err(Into::<ICError>::into)?;
        Ok(())
    }

    /// returns a list of device ids modified in the last operations
    pub fn last_operation_modified(self: &Rc<Self>) -> Vec<u32> {
        self.operation_modified.borrow().clone()
    }

    pub fn step_programmable(
        self: &Rc<Self>,
        id: ObjectID,
        advance_ip_on_err: bool,
    ) -> Result<(), VMError> {
        let obj = self
            .objects
            .borrow()
            .get(&id)
            .cloned()
            .ok_or(VMError::UnknownId(id))?;
        {
            let mut obj_ref = obj.borrow_mut();
            if let Some(programmable) = obj_ref.as_mut_programmable() {
                self.operation_modified.borrow_mut().clear();
                self.set_modified(id);
                programmable.step(advance_ip_on_err)?;
                return Ok(());
            }
        }
        let ic_obj = {
            let obj_ref = obj.borrow();
            if let Some(circuit_holder) = obj_ref.as_circuit_holder() {
                circuit_holder.get_ic()
            } else {
                return Err(VMError::NotCircuitHolderOrProgrammable(id));
            }
        };

        if let Some(ic_obj) = ic_obj {
            let mut ic_obj_ref = ic_obj.borrow_mut();
            let ic_id = *ic_obj_ref.get_id();
            if let Some(programmable) = ic_obj_ref.as_mut_programmable() {
                self.operation_modified.borrow_mut().clear();
                self.set_modified(ic_id);
                programmable.step(advance_ip_on_err)?;
                return Ok(());
            }
            return Err(VMError::NotProgrammable(ic_id));
        }
        Err(VMError::NoIC(id))
    }

    /// returns true if executed 128 lines, false if returned early.
    pub fn run_programmable(
        self: &Rc<Self>,
        id: ObjectID,
        ignore_errors: bool,
    ) -> Result<bool, VMError> {
        let obj = self
            .objects
            .borrow()
            .get(&id)
            .cloned()
            .ok_or(VMError::UnknownId(id))?;
        {
            let mut obj_ref = obj.borrow_mut();
            if let Some(programmable) = obj_ref.as_mut_programmable() {
                self.operation_modified.borrow_mut().clear();
                self.set_modified(id);
                for _i in 0..128 {
                    if let Err(err) = programmable.step(ignore_errors) {
                        if !ignore_errors {
                            return Err(err.into());
                        }
                    }
                    match programmable.get_state() {
                        ICState::Yield => return Ok(false),
                        ICState::Sleep(_then, _sleep_for) => return Ok(false),
                        ICState::HasCaughtFire => return Ok(false),
                        ICState::Error(_) if !ignore_errors => return Ok(false),
                        _ => {}
                    }
                }
                programmable.set_state(ICState::Yield);
                return Ok(true);
            }
        }
        let ic_obj = {
            let obj_ref = obj.borrow();
            if let Some(circuit_holder) = obj_ref.as_circuit_holder() {
                circuit_holder.get_ic()
            } else {
                return Err(VMError::NotCircuitHolderOrProgrammable(id));
            }
        };
        if let Some(ic_obj) = ic_obj {
            let mut ic_obj_ref = ic_obj.borrow_mut();
            let ic_id = *ic_obj_ref.get_id();
            if let Some(programmable) = ic_obj_ref.as_mut_programmable() {
                self.operation_modified.borrow_mut().clear();
                self.set_modified(ic_id);
                for _i in 0..128 {
                    if let Err(err) = programmable.step(ignore_errors) {
                        if !ignore_errors {
                            return Err(err.into());
                        }
                    }
                    match programmable.get_state() {
                        ICState::Yield => return Ok(false),
                        ICState::Sleep(_then, _sleep_for) => return Ok(false),
                        ICState::HasCaughtFire => return Ok(false),
                        ICState::Error(_) if !ignore_errors => return Ok(false),
                        _ => {}
                    }
                }
                programmable.set_state(ICState::Yield);
                return Ok(true);
            }
            return Err(VMError::NotProgrammable(ic_id));
        }
        Err(VMError::NoIC(id))
    }

    pub fn set_modified(self: &Rc<Self>, id: ObjectID) {
        self.operation_modified.borrow_mut().push(id);
    }

    pub fn reset_programmable(self: &Rc<Self>, id: ObjectID) -> Result<bool, VMError> {
        let obj = self
            .objects
            .borrow()
            .get(&id)
            .cloned()
            .ok_or(VMError::UnknownId(id))?;
        let mut obj_ref = obj.borrow_mut();
        let programmable = obj_ref
            .as_mut_programmable()
            .ok_or(VMError::NotProgrammable(id))?;
        programmable.reset();
        Ok(true)
    }

    pub fn get_object(self: &Rc<Self>, id: ObjectID) -> Option<VMObject> {
        self.objects.borrow().get(&id).cloned()
    }

    pub fn batch_device(
        self: &Rc<Self>,
        source: ObjectID,
        prefab_hash: f64,
        name: Option<f64>,
    ) -> impl Iterator<Item = VMObject> {
        self.objects
            .borrow()
            .iter()
            .filter(move |(id, device)| {
                if **id == source {
                    // FIXME: check to make sure this won't cause issues
                    // if it will pass in a self ref for access
                    false // exclude source to prevent re-borrow panics
                } else {
                    device.borrow().as_device().is_some_and(|device| {
                        device
                            .get_logic(LogicType::PrefabHash)
                            .is_ok_and(|f| f == prefab_hash)
                    }) && (name.is_none()
                        || name.is_some_and(|name| name == device.borrow().get_name().hash as f64))
                        && self.devices_on_same_network(&[source, **id])
                }
            })
            .map(|(_, d)| d)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn get_device_same_network(
        self: &Rc<Self>,
        source: ObjectID,
        other: ObjectID,
    ) -> Option<VMObject> {
        if self.devices_on_same_network(&[source, other]) {
            self.get_object(other)
        } else {
            None
        }
    }

    pub fn get_network_channel(
        self: &Rc<Self>,
        id: ObjectID,
        channel: usize,
    ) -> Result<f64, ICError> {
        let network = self
            .networks
            .borrow()
            .get(&id)
            .cloned()
            .ok_or(ICError::BadNetworkId(id))?;
        if !(0..8).contains(&channel) {
            Err(ICError::ChannelIndexOutOfRange(channel))
        } else {
            let channel_lt = LogicType::from_repr((LogicType::Channel0 as usize + channel) as u16)
                .expect("channel logictype repr out of range");
            let net_ref = network.borrow();
            let val = net_ref
                .as_network()
                .expect("non-network network")
                .get_logic(channel_lt)?;
            Ok(val)
        }
    }

    pub fn set_network_channel(
        self: &Rc<Self>,
        id: ObjectID,
        channel: usize,
        val: f64,
    ) -> Result<(), ICError> {
        let network = self
            .networks
            .borrow()
            .get(&(id))
            .cloned()
            .ok_or(ICError::BadNetworkId(id))?;
        if !(0..8).contains(&channel) {
            Err(ICError::ChannelIndexOutOfRange(channel))
        } else {
            let channel_lt = LogicType::from_repr((LogicType::Channel0 as usize + channel) as u16)
                .expect("channel logictype repr out of range");
            network
                .borrow_mut()
                .as_mut_network()
                .expect("non-network network")
                .set_logic(channel_lt, val, true)?;
            Ok(())
        }
    }

    pub fn devices_on_same_network(self: &Rc<Self>, ids: &[ObjectID]) -> bool {
        for net in self.networks.borrow().values() {
            if net
                .borrow()
                .as_network()
                .expect("non network network")
                .contains_all_data(ids)
            {
                return true;
            }
        }
        false
    }

    /// return a vector with the device ids the source id can see via it's connected networks
    pub fn visible_devices(self: &Rc<Self>, source: ObjectID) -> Vec<ObjectID> {
        self.networks
            .borrow()
            .values()
            .filter_map(|net| {
                let net_ref = net.borrow();
                let net_interface = net_ref.as_network().expect("non-network network");
                if net_interface.contains_data(&source) {
                    Some(net_interface.data_visible(&source))
                } else {
                    None
                }
            })
            .concat()
    }

    pub fn set_pin(
        self: &Rc<Self>,
        id: ObjectID,
        pin: usize,
        val: Option<ObjectID>,
    ) -> Result<bool, VMError> {
        let Some(obj) = self.objects.borrow().get(&id).cloned() else {
            return Err(VMError::UnknownId(id));
        };
        if let Some(other_device) = val {
            if !self.objects.borrow().contains_key(&other_device) {
                return Err(VMError::UnknownId(other_device));
            }
            if !self.devices_on_same_network(&[id, other_device]) {
                return Err(VMError::DeviceNotVisible(other_device, id));
            }
        }
        let mut obj_ref = obj.borrow_mut();
        let Some(device) = obj_ref.as_mut_device() else {
            return Err(VMError::NotADevice(id));
        };
        let Some(pins) = device.device_pins_mut() else {
            return Err(VMError::NoDevicePins(id));
        };
        if !(0..pins.len()).contains(&pin) {
            Err(ICError::PinIndexOutOfRange(pin).into())
        } else {
            pins[pin] = val;
            Ok(true)
        }
    }

    pub fn set_device_connection(
        self: &Rc<Self>,
        id: ObjectID,
        connection: usize,
        target_net: Option<ObjectID>,
    ) -> Result<bool, VMError> {
        let Some(obj) = self.objects.borrow().get(&id).cloned() else {
            return Err(VMError::UnknownId(id));
        };
        let mut obj_ref = obj.borrow_mut();
        let Some(device) = obj_ref.as_mut_device() else {
            return Err(VMError::NotADevice(id));
        };
        let connections = device.connection_list_mut();
        if connection >= connections.len() {
            let conn_len = connections.len();
            return Err(ICError::ConnectionIndexOutOfRange(connection, conn_len).into());
        }

        // scope this borrow
        let Connection::CableNetwork { net, typ, .. } = &connections[connection] else {
            return Err(ICError::NotACableConnection(connection).into());
        };
        // remove from current network
        if let Some(net) = net {
            if let Some(network) = self.networks.borrow().get(net) {
                // if there is no other connection to this network
                if connections
                    .iter()
                    .filter(|conn| {
                        matches!(conn, Connection::CableNetwork {
                            net: Some(other_net),
                            typ: other_typ,
                            ..
                        } if other_net == net && (
                            !matches!(typ,  CableConnectionType::Power) ||
                            matches!(other_typ, CableConnectionType::Data | CableConnectionType::PowerAndData))
                        )
                    })
                    .count()
                    == 1
                {
                    match typ {
                        CableConnectionType::Power => {
                            network
                                .borrow_mut()
                                .as_mut_network()
                                .ok_or(VMError::NonNetworkNetwork(*net))?
                                .remove_power(id);
                        }
                        _ => {
                            network
                                .borrow_mut()
                                .as_mut_network()
                                .ok_or(VMError::NonNetworkNetwork(*net))?
                                .remove_data(id);
                        }
                    }
                }
            }
        }

        let Connection::CableNetwork {
            ref mut net,
            ref typ,
            ..
        } = connections[connection]
        else {
            return Err(ICError::NotACableConnection(connection).into());
        };
        if let Some(target_net) = target_net {
            if let Some(network) = self.networks.borrow().get(&target_net) {
                match typ {
                    CableConnectionType::Power => {
                        network
                            .borrow_mut()
                            .as_mut_network()
                            .ok_or(VMError::NonNetworkNetwork(target_net))?
                            .add_power(id);
                    }
                    _ => {
                        network
                            .borrow_mut()
                            .as_mut_network()
                            .ok_or(VMError::NonNetworkNetwork(target_net))?
                            .add_data(id);
                    }
                }
            } else {
                return Err(VMError::InvalidNetwork(target_net));
            }
        }
        *net = target_net;
        Ok(true)
    }

    pub fn remove_device_from_network(
        self: &Rc<Self>,
        id: ObjectID,
        network_id: ObjectID,
    ) -> Result<bool, VMError> {
        if let Some(network) = self.networks.borrow().get(&network_id) {
            let Some(obj) = self.objects.borrow().get(&id).cloned() else {
                return Err(VMError::UnknownId(id));
            };
            let mut obj_ref = obj.borrow_mut();
            let Some(device) = obj_ref.as_mut_device() else {
                return Err(VMError::NotADevice(id));
            };

            for conn in device.connection_list_mut().iter_mut() {
                if let Connection::CableNetwork { net, .. } = conn {
                    if net.is_some_and(|id| id == network_id) {
                        *net = None;
                    }
                }
            }
            network
                .borrow_mut()
                .as_mut_network()
                .ok_or(VMError::NonNetworkNetwork(network_id))?
                .remove_all(id);
            Ok(true)
        } else {
            Err(VMError::InvalidNetwork(network_id))
        }
    }

    pub fn set_batch_device_field(
        self: &Rc<Self>,
        source: ObjectID,
        prefab: f64,
        typ: LogicType,
        val: f64,
        write_readonly: bool,
    ) -> Result<(), ICError> {
        self.batch_device(source, prefab, None)
            .map(|device| {
                self.set_modified(*device.borrow().get_id());
                device
                    .borrow_mut()
                    .as_mut_device()
                    .expect("batch iter yielded non device")
                    .set_logic(typ, val, write_readonly)
                    .map_err(Into::into)
            })
            .try_collect()
    }

    pub fn set_batch_device_slot_field(
        self: &Rc<Self>,
        source: ObjectID,
        prefab: f64,
        index: f64,
        typ: LogicSlotType,
        val: f64,
        write_readonly: bool,
    ) -> Result<(), ICError> {
        self.batch_device(source, prefab, None)
            .map(|device| {
                self.set_modified(*device.borrow().get_id());
                device
                    .borrow_mut()
                    .as_mut_device()
                    .expect("batch iter yielded non device")
                    .set_slot_logic(typ, index, val, write_readonly)
                    .map_err(Into::into)
            })
            .try_collect()
    }

    pub fn set_batch_name_device_field(
        self: &Rc<Self>,
        source: ObjectID,
        prefab: f64,
        name: f64,
        typ: LogicType,
        val: f64,
        write_readonly: bool,
    ) -> Result<(), ICError> {
        self.batch_device(source, prefab, Some(name))
            .map(|device| {
                self.set_modified(*device.borrow().get_id());
                device
                    .borrow_mut()
                    .as_mut_device()
                    .expect("batch iter yielded non device")
                    .set_logic(typ, val, write_readonly)
                    .map_err(Into::into)
            })
            .try_collect()
    }

    pub fn get_batch_device_field(
        self: &Rc<Self>,
        source: ObjectID,
        prefab: f64,
        typ: LogicType,
        mode: LogicBatchMethod,
    ) -> Result<f64, ICError> {
        let samples = self
            .batch_device(source, prefab, None)
            .map(|device| {
                device
                    .borrow()
                    .as_device()
                    .expect("batch iter yielded non device")
                    .get_logic(typ)
                    .map_err(Into::into)
            })
            .filter_ok(|val| !val.is_nan())
            .collect::<Result<Vec<_>, ICError>>()?;
        Ok(LogicBatchMethodWrapper(mode).apply(&samples))
    }

    pub fn get_batch_name_device_field(
        self: &Rc<Self>,
        source: ObjectID,
        prefab: f64,
        name: f64,
        typ: LogicType,
        mode: LogicBatchMethod,
    ) -> Result<f64, ICError> {
        let samples = self
            .batch_device(source, prefab, Some(name))
            .map(|device| {
                device
                    .borrow()
                    .as_device()
                    .expect("batch iter yielded non device")
                    .get_logic(typ)
                    .map_err(Into::into)
            })
            .filter_ok(|val| !val.is_nan())
            .collect::<Result<Vec<_>, ICError>>()?;
        Ok(LogicBatchMethodWrapper(mode).apply(&samples))
    }

    pub fn get_batch_name_device_slot_field(
        self: &Rc<Self>,
        source: ObjectID,
        prefab: f64,
        name: f64,
        index: f64,
        typ: LogicSlotType,
        mode: LogicBatchMethod,
    ) -> Result<f64, ICError> {
        let samples = self
            .batch_device(source, prefab, Some(name))
            .map(|device| {
                device
                    .borrow()
                    .as_device()
                    .expect("batch iter yielded non device")
                    .get_slot_logic(typ, index)
                    .map_err(Into::into)
            })
            .filter_ok(|val| !val.is_nan())
            .collect::<Result<Vec<_>, ICError>>()?;
        Ok(LogicBatchMethodWrapper(mode).apply(&samples))
    }

    pub fn get_batch_device_slot_field(
        self: &Rc<Self>,
        source: ObjectID,
        prefab: f64,
        index: f64,
        typ: LogicSlotType,
        mode: LogicBatchMethod,
    ) -> Result<f64, ICError> {
        let samples = self
            .batch_device(source, prefab, None)
            .map(|device| {
                device
                    .borrow()
                    .as_device()
                    .expect("batch iter yielded non device")
                    .get_slot_logic(typ, index)
                    .map_err(Into::into)
            })
            .filter_ok(|val| !val.is_nan())
            .collect::<Result<Vec<_>, ICError>>()?;
        Ok(LogicBatchMethodWrapper(mode).apply(&samples))
    }

    pub fn remove_object(self: &Rc<Self>, id: ObjectID) -> Result<(), VMError> {
        let Some(obj) = self.objects.borrow_mut().remove(&id) else {
            return Err(VMError::UnknownId(id));
        };

        if let Some(device) = obj.borrow().as_device() {
            for conn in device.connection_list().iter() {
                if let Connection::CableNetwork { net: Some(net), .. } = conn {
                    if let Some(network) = self.networks.borrow().get(net) {
                        network
                            .borrow_mut()
                            .as_mut_network()
                            .expect("non-network network")
                            .remove_all(id);
                    }
                }
            }
            if device.as_circuit_holder().is_some() {
                self.circuit_holders.borrow_mut().retain(|a| *a != id);
            }
        }
        self.id_space.borrow_mut().free_id(id);
        Ok(())
    }

    /// set a slot to contain some quantity of an object with ID
    /// object must already be added to the VM
    /// does not clean up previous object
    /// returns the id of any former occupant
    pub fn set_slot_occupant(
        self: &Rc<Self>,
        id: ObjectID,
        index: usize,
        target: Option<ObjectID>,
        quantity: u32,
    ) -> Result<Option<ObjectID>, VMError> {
        let Some(obj) = self.objects.borrow().get(&id).cloned() else {
            return Err(VMError::UnknownId(id));
        };
        let mut obj_ref = obj.borrow_mut();
        let Some(storage) = obj_ref.as_mut_storage() else {
            return Err(VMError::NotStorage(id));
        };
        let slot = storage
            .get_slot_mut(index)
            .ok_or(ICError::SlotIndexOutOfRange(index as f64))?;
        if let Some(target) = target {
            match slot.occupant.as_mut() {
                Some(info) if info.id == target => {
                    info.quantity = quantity;
                    Ok(None)
                }
                _ => {
                    let Some(item_obj) = self.objects.borrow().get(&target).cloned() else {
                        return Err(VMError::UnknownId(id));
                    };
                    let mut item_obj_ref = item_obj.borrow_mut();
                    let Some(item) = item_obj_ref.as_mut_item() else {
                        return Err(VMError::NotAnItem(target));
                    };
                    if let Some(parent_slot_info) = item.get_parent_slot() {
                        self.remove_slot_occupant(parent_slot_info.parent, parent_slot_info.slot)?;
                    }
                    item.set_parent_slot(Some(ParentSlotInfo {
                        parent: id,
                        slot: index,
                    }));
                    let last = slot.occupant.as_ref().map(|info| info.id);
                    slot.occupant.replace(SlotOccupantInfo {
                        id: target,
                        quantity,
                    });
                    Ok(last)
                }
            }
        } else {
            let last = slot.occupant.as_ref().map(|info| info.id);
            slot.occupant = None;
            Ok(last)
        }
    }

    /// returns former occupant id if any
    pub fn remove_slot_occupant(
        self: &Rc<Self>,
        id: ObjectID,
        index: usize,
    ) -> Result<Option<ObjectID>, VMError> {
        let Some(obj) = self.objects.borrow().get(&id).cloned() else {
            return Err(VMError::UnknownId(id));
        };
        let mut obj_ref = obj.borrow_mut();
        let Some(storage) = obj_ref.as_mut_storage() else {
            return Err(VMError::NotStorage(id));
        };
        let slot = storage
            .get_slot_mut(index)
            .ok_or(ICError::SlotIndexOutOfRange(index as f64))?;

        let last = slot.occupant.as_ref().map(|info| info.id);
        Ok(last)
    }

    pub fn freeze_object(self: &Rc<Self>, id: ObjectID) -> Result<FrozenObjectFull, VMError> {
        let Some(obj) = self.objects.borrow().get(&id).cloned() else {
            return Err(VMError::UnknownId(id));
        };
        Ok(FrozenObject::freeze_object(&obj, self)?)
    }

    pub fn freeze_objects(
        self: &Rc<Self>,
        ids: impl IntoIterator<Item = ObjectID>,
    ) -> Result<Vec<FrozenObjectFull>, VMError> {
        ids.into_iter()
            .map(|id| {
                let Some(obj) = self.objects.borrow().get(&id).cloned() else {
                    return Err(VMError::UnknownId(id));
                };
                Ok(FrozenObject::freeze_object(&obj, self)?)
            })
            .collect()
    }

    pub fn freeze_network(self: &Rc<Self>, id: ObjectID) -> Result<FrozenCableNetwork, VMError> {
        Ok(self
            .networks
            .borrow()
            .get(&id)
            .ok_or(VMError::UnknownId(id))?
            .borrow()
            .as_network()
            .ok_or(VMError::NonNetworkNetwork(id))?
            .into())
    }

    pub fn freeze_networks(
        self: &Rc<Self>,
        ids: impl IntoIterator<Item = ObjectID>,
    ) -> Result<Vec<FrozenCableNetwork>, VMError> {
        ids.into_iter()
            .map(|id| {
                Ok(self
                    .networks
                    .borrow()
                    .get(&id)
                    .ok_or(VMError::UnknownId(id))?
                    .borrow()
                    .as_network()
                    .ok_or(VMError::NonNetworkNetwork(id))?
                    .into())
            })
            .collect::<Result<Vec<FrozenCableNetwork>, VMError>>()
    }

    pub fn save_vm_state(self: &Rc<Self>) -> Result<FrozenVM, VMError> {
        Ok(FrozenVM {
            objects: self
                .objects
                .borrow()
                .iter()
                .filter_map(|(_obj_id, obj)| {
                    if obj
                        .borrow()
                        .as_item()
                        .is_some_and(|item| item.get_parent_slot().is_some())
                    {
                        None
                    } else {
                        Some(FrozenObject::freeze_object_sparse(obj, self))
                    }
                })
                .collect::<Result<Vec<_>, _>>()?,
            networks: self
                .networks
                .borrow()
                .values()
                .map(|network| {
                    let net_id = network.get_id();
                    Ok(network
                        .borrow()
                        .as_network()
                        .ok_or(VMError::NonNetworkNetwork(net_id))?
                        .into())
                })
                .collect::<Result<Vec<FrozenCableNetwork>, VMError>>()?,
            default_network_key: *self.default_network_key.borrow(),
            circuit_holders: self.circuit_holders.borrow().clone(),
            program_holders: self.program_holders.borrow().clone(),
            wireless_transmitters: self.wireless_transmitters.borrow().clone(),
            wireless_receivers: self.wireless_receivers.borrow().clone(),
        })
    }

    pub fn restore_vm_state(self: &Rc<Self>, state: FrozenVM) -> Result<(), VMError> {
        let mut transaction_network_id_space = IdSpace::new();
        transaction_network_id_space
            .use_ids(&state.networks.iter().map(|net| net.id).collect_vec())?;
        let transaction_networks: BTreeMap<ObjectID, VMObject> = state
            .networks
            .into_iter()
            .map(|network| {
                (
                    network.id,
                    VMObject::new(CableNetwork::from_frozen(network, self.clone())),
                )
            })
            .collect();
        let mut transaction = VMTransaction::from_scratch_with_networks(
            self,
            &transaction_networks,
            state.default_network_key,
        );
        for frozen in state.objects {
            let _ = transaction.add_object_from_frozen(frozen)?;
        }
        transaction.finialize()?;

        self.circuit_holders.borrow_mut().clear();
        self.program_holders.borrow_mut().clear();
        self.objects.borrow_mut().clear();
        self.networks.borrow_mut().clear();
        self.wireless_transmitters.borrow_mut().clear();
        self.wireless_receivers.borrow_mut().clear();
        self.id_space.borrow_mut().reset();
        self.network_id_space.borrow_mut().reset();

        self.network_id_space.replace(transaction_network_id_space);
        self.networks.replace(transaction_networks);

        let transaction_ids = transaction.id_space.in_use_ids();
        self.id_space.borrow_mut().use_ids(&transaction_ids)?;

        self.circuit_holders
            .borrow_mut()
            .extend(transaction.circuit_holders);
        self.program_holders
            .borrow_mut()
            .extend(transaction.program_holders);
        self.wireless_transmitters
            .borrow_mut()
            .extend(transaction.wireless_transmitters);
        self.wireless_receivers
            .borrow_mut()
            .extend(transaction.wireless_receivers);

        for (net_id, trans_net) in transaction.networks.into_iter() {
            let networks_ref = self.networks.borrow();
            let net = networks_ref
                .get(&net_id)
                .unwrap_or_else(|| panic!("desync between vm and transaction networks: {net_id}"));
            let mut net_ref = net.borrow_mut();
            let net_interface = net_ref
                .as_mut_network()
                .ok_or(VMError::NonNetworkNetwork(net_id))?;
            for id in trans_net.devices {
                net_interface.add_data(id);
            }
            for id in trans_net.power_only {
                net_interface.add_power(id);
            }
        }

        Ok(())
    }
}

impl VMTransaction {
    pub fn new(vm: &Rc<VM>) -> Self {
        VMTransaction {
            objects: BTreeMap::new(),
            circuit_holders: Vec::new(),
            program_holders: Vec::new(),
            default_network_key: *vm.default_network_key.borrow(),
            wireless_transmitters: Vec::new(),
            wireless_receivers: Vec::new(),
            id_space: vm.id_space.borrow().clone(),
            networks: vm
                .networks
                .borrow()
                .keys()
                .map(|net_id| (*net_id, VMTransactionNetwork::default()))
                .collect(),
            object_parents: BTreeMap::new(),
            vm: vm.clone(),
        }
    }

    pub fn from_scratch_with_networks(
        vm: &Rc<VM>,
        networks: &BTreeMap<ObjectID, VMObject>,
        default: ObjectID,
    ) -> Self {
        VMTransaction {
            objects: BTreeMap::new(),
            circuit_holders: Vec::new(),
            program_holders: Vec::new(),
            default_network_key: default,
            wireless_transmitters: Vec::new(),
            wireless_receivers: Vec::new(),
            id_space: IdSpace::new(),
            networks: networks
                .keys()
                .map(|net_id| (*net_id, VMTransactionNetwork::default()))
                .collect(),
            object_parents: BTreeMap::new(),
            vm: vm.clone(),
        }
    }

    pub fn add_object_from_frozen(&mut self, frozen: FrozenObject) -> Result<ObjectID, VMError> {
        for net_id in &frozen.connected_networks() {
            if !self.networks.contains_key(net_id) {
                return Err(VMError::InvalidNetwork(*net_id));
            }
        }

        let obj_id = if let Some(obj_id) = frozen.obj_info.id {
            self.id_space.use_id(obj_id)?;
            obj_id
        } else {
            self.id_space.next()
        };

        let obj = frozen.build_vm_obj(obj_id, &self.vm)?;

        for (index, child_id) in frozen.contained_object_slots() {
            self.object_parents.insert(child_id, (index, obj_id));
        }

        if let Some(_w_logicable) = obj.borrow().as_wireless_transmit() {
            self.wireless_transmitters.push(obj_id);
        }
        if let Some(_r_logicable) = obj.borrow().as_wireless_receive() {
            self.wireless_receivers.push(obj_id);
        }
        if let Some(_circuit_holder) = obj.borrow().as_circuit_holder() {
            self.circuit_holders.push(obj_id);
        }
        if let Some(_programmable) = obj.borrow().as_programmable() {
            self.program_holders.push(obj_id);
        }
        if let Some(device) = obj.borrow_mut().as_mut_device() {
            for conn in device.connection_list() {
                if let Connection::CableNetwork {
                    net: Some(net_id),
                    typ,
                    role: ConnectionRole::None,
                } = conn
                {
                    if let Some(net) = self.networks.get_mut(net_id) {
                        match typ {
                            CableConnectionType::Power => net.power_only.push(obj_id),
                            _ => net.devices.push(obj_id),
                        }
                    } else {
                        return Err(VMError::InvalidNetwork(*net_id));
                    }
                }
            }
        }

        self.objects.insert(obj_id, obj);

        Ok(obj_id)
    }

    pub fn finialize(&mut self) -> Result<(), VMError> {
        for (child, (slot, parent)) in &self.object_parents {
            let child_obj = self
                .objects
                .get(child)
                .ok_or(VMError::MissingChild(*child))?;
            let mut child_obj_ref = child_obj.borrow_mut();
            let item = child_obj_ref
                .as_mut_item()
                .ok_or(VMError::NotParentable(*child))?;
            item.set_parent_slot(Some(ParentSlotInfo {
                slot: *slot as usize,
                parent: *parent,
            }));
        }
        Ok(())
    }
}

pub struct LogicBatchMethodWrapper(LogicBatchMethod);

impl LogicBatchMethodWrapper {
    pub fn apply(&self, samples: &[f64]) -> f64 {
        match self.0 {
            LogicBatchMethod::Sum => samples.iter().sum(),
            // Both c-charp and rust return NaN for 0.0/0.0 so we're good here
            LogicBatchMethod::Average => {
                samples.iter().copied().sum::<f64>() / samples.len() as f64
            }
            // Game uses a default of Positive INFINITY for Minimum
            LogicBatchMethod::Minimum => *samples
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(&f64::INFINITY),
            // Game uses default of NEG_INFINITY for Maximum
            LogicBatchMethod::Maximum => *samples
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(&f64::NEG_INFINITY),
        }
    }
}

#[derive(Debug, Clone)]
struct IdSpace {
    next: ObjectID,
    in_use: HashSet<ObjectID>,
}

impl Default for IdSpace {
    fn default() -> Self {
        IdSpace::new()
    }
}

impl IdSpace {
    pub fn new() -> Self {
        IdSpace {
            next: 1,
            in_use: HashSet::new(),
        }
    }

    pub fn next(&mut self) -> ObjectID {
        let val = self.next;
        self.next += 1;
        self.in_use.insert(val);
        val
    }

    pub fn has_id(&self, id: &ObjectID) -> bool {
        self.in_use.contains(id)
    }

    pub fn in_use_ids(&self) -> Vec<ObjectID> {
        self.in_use.iter().copied().collect()
    }

    pub fn use_id(&mut self, id: ObjectID) -> Result<(), VMError> {
        if self.in_use.contains(&id) {
            Err(VMError::IdInUse(id))
        } else {
            self.in_use.insert(id);
            Ok(())
        }
    }

    pub fn use_ids<'a, I>(&mut self, ids: I) -> Result<(), VMError>
    where
        I: IntoIterator<Item = &'a ObjectID> + std::marker::Copy,
    {
        let mut to_use: HashSet<ObjectID> = HashSet::new();
        let mut duplicates: HashSet<ObjectID> = HashSet::new();
        let all_uniq = ids.into_iter().copied().all(|id| {
            if to_use.insert(id) {
                true
            } else {
                duplicates.insert(id);
                false
            }
        });
        if !all_uniq {
            return Err(VMError::DuplicateIds(duplicates.into_iter().collect_vec()));
        }
        let invalid = self.in_use.intersection(&to_use).copied().collect_vec();
        if !invalid.is_empty() {
            return Err(VMError::IdsInUse(invalid));
        }
        self.in_use.extend(ids);
        self.next = self.in_use.iter().max().unwrap_or(&0) + 1;
        Ok(())
    }

    /// use the ids in the iterator that aren't already in use
    pub fn use_new_ids<'a, I>(&mut self, ids: I)
    where
        I: IntoIterator<Item = &'a ObjectID> + std::marker::Copy,
    {
        self.in_use.extend(ids);
        self.next = self.in_use.iter().max().unwrap_or(&0) + 1;
    }

    pub fn free_id(&mut self, id: ObjectID) {
        self.in_use.remove(&id);
    }

    pub fn reset(&mut self) {
        self.in_use.clear();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct FrozenVM {
    pub objects: Vec<FrozenObject>,
    pub circuit_holders: Vec<ObjectID>,
    pub program_holders: Vec<ObjectID>,
    pub default_network_key: ObjectID,
    pub networks: Vec<FrozenCableNetwork>,
    pub wireless_transmitters: Vec<ObjectID>,
    pub wireless_receivers: Vec<ObjectID>,
}
