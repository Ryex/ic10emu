pub mod enums;
pub mod instructions;
pub mod object;

use crate::{
    device::{Device, DeviceTemplate, SlotOccupant, SlotOccupantTemplate},
    errors::{ICError, VMError},
    interpreter::{self, FrozenIC},
    network::{CableConnectionType, CableNetwork, Connection, FrozenNetwork},
    vm::{
        enums::script_enums::{LogicBatchMethod as BatchMode, LogicSlotType, LogicType},
        object::{templates::ObjectTemplate, traits::*, BoxedObject, ObjectID, VMObject},
    },
};
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashSet},
    rc::Rc,
};

use itertools::Itertools;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub struct VM {
    pub objects: BTreeMap<ObjectID, VMObject>,
    pub ic_holders: RefCell<Vec<ObjectID>>,
    pub networks: BTreeMap<ObjectID, VMObject>,
    pub default_network_key: ObjectID,
    pub wireless_transmitters: RefCell<Vec<ObjectID>>,
    pub wireless_receivers: RefCell<Vec<ObjectID>>,
    id_space: IdSpace,
    network_id_space: IdSpace,
    random: Rc<RefCell<crate::rand_mscorlib::Random>>,

    /// list of object id's touched on the last operation
    operation_modified: RefCell<Vec<ObjectID>>,
}

#[derive(Debug, Default)]
pub struct VMTransationNetwork {
    pub objects: Vec<ObjectID>,
    pub power_only: Vec<ObjectID>,
}

#[derive(Debug)]
/// used as a temp structure to add objects in case
/// there are errors on nested templates
pub struct VMTransation {
    pub objects: BTreeMap<ObjectID, VMObject>,
    pub ic_holders: Vec<ObjectID>,
    pub default_network_key: ObjectID,
    pub wireless_transmitters: Vec<ObjectID>,
    pub wireless_receivers: Vec<ObjectID>,
    pub id_space: IdSpace,
    pub networks: BTreeMap<ObjectID, VMTransationNetwork>,
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

impl VM {
    pub fn new() -> Self {
        let id_space = IdSpace::default();
        let mut network_id_space = IdSpace::default();
        let default_network_key = network_id_space.next();
        let default_network = VMObject::new(CableNetwork::new(default_network_key));
        let mut networks = BTreeMap::new();
        networks.insert(default_network_key, default_network);

        let mut vm = VM {
            objects: BTreeMap::new(),
            ic_holders: RefCell::new(Vec::new()),
            networks,
            default_network_key,
            wireless_transmitters: RefCell::new(Vec::new()),
            wireless_receivers: RefCell::new(Vec::new()),
            id_space,
            network_id_space,
            random: Rc::new(RefCell::new(crate::rand_mscorlib::Random::new())),
            operation_modified: RefCell::new(Vec::new()),
        };
        vm
    }

    pub fn add_device_from_template(&mut self, template: ObjectTemplate) -> Result<u32, VMError> {
        let mut transaction = VMTransation::new(self);

        let obj_id = transaction.add_device_from_template(template)?;

        let transation_ids = transaction.id_space.in_use_ids();
        self.id_space.use_new_ids(&transation_ids);

        self.objects.extend(transaction.objects);
        self.wireless_transmitters
            .borrow_mut()
            .extend(transaction.wireless_transmitters);
        self.wireless_receivers
            .borrow_mut()
            .extend(transaction.wireless_receivers);
        self.ic_holders.borrow_mut().extend(transaction.ic_holders);
        for (net_id, trans_net) in transaction.networks.into_iter() {
            let net = self
                .networks
                .get(&net_id)
                .expect(&format!(
                    "desync between vm and transation networks: {net_id}"
                ))
                .borrow_mut()
                .as_mut_network()
                .expect(&format!("non network network: {net_id}"));
            for id in trans_net.objects {
                net.add_data(id);
            }
            for id in trans_net.power_only {
                net.add_power(id);
            }
        }

        Ok(obj_id)
    }

    pub fn add_network(&mut self) -> u32 {
        let next_id = self.network_id_space.next();
        self.networks
            .insert(next_id, Rc::new(RefCell::new(CableNetwork::new(next_id))));
        next_id
    }

    pub fn get_default_network(&self) -> Rc<RefCell<CableNetwork>> {
        self.networks
            .get(&self.default_network_key)
            .cloned()
            .unwrap()
    }

    pub fn get_network(&self, id: u32) -> Option<Rc<RefCell<CableNetwork>>> {
        self.networks.get(&id).cloned()
    }

    pub fn remove_ic(&mut self, id: u32) {
        if self.ic_holders.remove(&id).is_some() {
            self.devices.remove(&id);
        }
    }

    pub fn change_device_id(&mut self, old_id: u32, new_id: u32) -> Result<(), VMError> {
        self.id_space.use_id(new_id)?;
        let device = self
            .devices
            .remove(&old_id)
            .ok_or(VMError::UnknownId(old_id))?;
        device.borrow_mut().id = new_id;
        self.devices.insert(new_id, device);
        self.ic_holders.iter().for_each(|(_id, ic)| {
            let mut ic_ref = ic.borrow_mut();
            if ic_ref.device == old_id {
                ic_ref.device = new_id;
            }
            ic_ref.pins.borrow_mut().iter_mut().for_each(|pin| {
                if pin.is_some_and(|d| d == old_id) {
                    pin.replace(new_id);
                }
            });
        });
        self.networks.iter().for_each(|(_net_id, net)| {
            if let Ok(mut net_ref) = net.try_borrow_mut() {
                if net_ref.devices.remove(&old_id) {
                    net_ref.devices.insert(new_id);
                }
            }
        });
        self.id_space.free_id(old_id);
        Ok(())
    }

    /// Set program code if it's valid
    pub fn set_code(&self, id: u32, code: &str) -> Result<bool, VMError> {
        let device = self
            .devices
            .get(&id)
            .ok_or(VMError::UnknownId(id))?
            .borrow();
        let ic_id = *device.ic.as_ref().ok_or(VMError::NoIC(id))?;
        let ic = self
            .ic_holders
            .get(&ic_id)
            .ok_or(VMError::UnknownIcId(ic_id))?
            .borrow();
        let new_prog = interpreter::Program::try_from_code(code)?;
        ic.program.replace(new_prog);
        ic.code.replace(code.to_string());
        Ok(true)
    }

    /// Set program code and translate invalid lines to Nop, collecting errors
    pub fn set_code_invalid(&self, id: u32, code: &str) -> Result<bool, VMError> {
        let device = self
            .devices
            .get(&id)
            .ok_or(VMError::UnknownId(id))?
            .borrow();
        let ic_id = *device.ic.as_ref().ok_or(VMError::NoIC(id))?;
        let ic = self
            .ic_holders
            .get(&ic_id)
            .ok_or(VMError::UnknownIcId(ic_id))?
            .borrow_mut();
        let new_prog = interpreter::Program::from_code_with_invalid(code);
        ic.program.replace(new_prog);
        ic.code.replace(code.to_string());
        Ok(true)
    }

    /// returns a list of device ids modified in the last operations
    pub fn last_operation_modified(&self) -> Vec<u32> {
        self.operation_modified.borrow().clone()
    }

    pub fn step_ic(&self, id: u32, advance_ip_on_err: bool) -> Result<bool, VMError> {
        self.operation_modified.borrow_mut().clear();
        let ic_id = {
            let device = self.devices.get(&id).ok_or(VMError::UnknownId(id))?;
            let device_ref = device.borrow();
            let ic_id = device_ref.ic.as_ref().ok_or(VMError::NoIC(id))?;
            *ic_id
        };
        self.set_modified(id);
        let ic = self
            .ic_holders
            .get(&ic_id)
            .ok_or(VMError::UnknownIcId(ic_id))?
            .clone();
        ic.borrow().ic.replace(0);
        let result = ic.borrow().step(self, advance_ip_on_err)?;
        Ok(result)
    }

    /// returns true if executed 128 lines, false if returned early.
    pub fn run_ic(&self, id: u32, ignore_errors: bool) -> Result<bool, VMError> {
        self.operation_modified.borrow_mut().clear();
        let device = self.devices.get(&id).ok_or(VMError::UnknownId(id))?.clone();
        let ic_id = *device.borrow().ic.as_ref().ok_or(VMError::NoIC(id))?;
        let ic = self
            .ic_holders
            .get(&ic_id)
            .ok_or(VMError::UnknownIcId(ic_id))?
            .clone();
        ic.borrow().ic.replace(0);
        self.set_modified(id);
        for _i in 0..128 {
            if let Err(err) = ic.borrow().step(self, ignore_errors) {
                if !ignore_errors {
                    return Err(err.into());
                }
            }
            if let interpreter::ICState::Yield = *ic.borrow().state.borrow() {
                return Ok(false);
            } else if let interpreter::ICState::Sleep(_then, _sleep_for) =
                *ic.borrow().state.borrow()
            {
                return Ok(false);
            }
        }
        ic.borrow().state.replace(interpreter::ICState::Yield);
        Ok(true)
    }

    pub fn set_modified(&self, id: ObjectID) {
        self.operation_modified.borrow_mut().push(id);
    }

    pub fn reset_ic(&self, id: ObjectID) -> Result<bool, VMError> {
        let obj = self.objects.get(&id).ok_or(VMError::UnknownId(id))?.clone();
        let ic_id = obj
            .borrow()
            .as_mut_circuit_holder()
            .map(|holder| holder.get_ic())
            .flatten()
            .ok_or(VMError::NoIC(id))?;
        let ic = self
            .objects
            .get(&ic_id)
            .ok_or(VMError::UnknownIcId(ic_id))?
            .borrow_mut()
            .as_mut_programmable()
            .ok_or(VMError::UnknownIcId(ic_id))?;
        ic.reset();
        Ok(true)
    }

    pub fn get_object(&self, id: ObjectID) -> Option<VMObject> {
        self.objects.get(&id).cloned()
    }

    pub fn batch_device(
        &self,
        source: ObjectID,
        prefab_hash: f64,
        name: Option<f64>,
    ) -> impl Iterator<Item = &VMObject> {
        self.objects
            .iter()
            .filter(move |(id, device)| {
                device.borrow().as_device().is_some_and(|device| {
                    device
                        .get_logic(LogicType::PrefabHash)
                        .is_ok_and(|f| f == prefab_hash)
                }) && (name.is_none()
                    || name.is_some_and(|name| name == device.borrow().name().hash as f64))
                    && self.devices_on_same_network(&[source, **id])
            })
            .map(|(_, d)| d)
    }

    pub fn get_device_same_network(&self, source: ObjectID, other: ObjectID) -> Option<VMObject> {
        if self.devices_on_same_network(&[source, other]) {
            self.get_object(other)
        } else {
            None
        }
    }

    pub fn get_network_channel(&self, id: u32, channel: usize) -> Result<f64, ICError> {
        let network = self.networks.get(&id).ok_or(ICError::BadNetworkId(id))?;
        if !(0..8).contains(&channel) {
            Err(ICError::ChannelIndexOutOfRange(channel))
        } else {
            let channel_lt = LogicType::from_repr((LogicType::Channel0 as usize + channel) as u16)
                .expect("channel logictype repr out of range");
            let val = network
                .borrow_mut()
                .as_network()
                .expect("non-network network")
                .get_logic(channel_lt)?;
            Ok(val)
        }
    }

    pub fn set_network_channel(
        &self,
        id: ObjectID,
        channel: usize,
        val: f64,
    ) -> Result<(), ICError> {
        let network = self.networks.get(&(id)).ok_or(ICError::BadNetworkId(id))?;
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

    pub fn devices_on_same_network(&self, ids: &[ObjectID]) -> bool {
        for net in self.networks.values() {
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
    pub fn visible_devices(&self, source: ObjectID) -> Vec<ObjectID> {
        self.networks
            .values()
            .filter_map(|net| {
                let net_ref = net.borrow().as_network().expect("non-network network");
                if net_ref.contains_data(&source) {
                    Some(net_ref.data_visible(&source))
                } else {
                    None
                }
            })
            .concat()
    }

    pub fn set_pin(&self, id: u32, pin: usize, val: Option<ObjectID>) -> Result<bool, VMError> {
        let Some(obj) = self.objects.get(&id) else {
            return Err(VMError::UnknownId(id));
        };
        if let Some(other_device) = val {
            if !self.objects.contains_key(&other_device) {
                return Err(VMError::UnknownId(other_device));
            }
            if !self.devices_on_same_network(&[id, other_device]) {
                return Err(VMError::DeviceNotVisible(other_device, id));
            }
        }
        let Some(device) = obj.borrow_mut().as_mut_device() else {
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
        &self,
        id: ObjectID,
        connection: usize,
        target_net: Option<ObjectID>,
    ) -> Result<bool, VMError> {
        let Some(obj) = self.objects.get(&id) else {
            return Err(VMError::UnknownId(id));
        };
        let Some(device) = obj.borrow_mut().as_mut_device() else {
            return Err(VMError::NotADevice(id));
        };
        let connections = device.connection_list_mut();
        if connection >= connections.len() {
            let conn_len = connections.len();
            return Err(ICError::ConnectionIndexOutOfRange(connection, conn_len).into());
        }

        // scope this borrow
        let Connection::CableNetwork { net, typ } = &connections[connection] else {
            return Err(ICError::NotACableConnection(connection).into());
        };
        // remove from current network
        if let Some(net) = net {
            if let Some(network) = self.networks.get(net) {
                // if there is no other connection to this network
                if connections
                    .iter()
                    .filter(|conn| {
                        matches!(conn, Connection::CableNetwork {
                            net: Some(other_net),
                            typ: other_typ
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
                                .expect("non-network network")
                                .remove_power(id);
                        }
                        _ => {
                            network
                                .borrow_mut()
                                .as_mut_network()
                                .expect("non-network network")
                                .remove_data(id);
                        }
                    }
                }
            }
        }

        let Connection::CableNetwork {
            ref mut net,
            ref typ,
        } = connections[connection]
        else {
            return Err(ICError::NotACableConnection(connection).into());
        };
        if let Some(target_net) = target_net {
            if let Some(network) = self.networks.get(&target_net) {
                match typ {
                    CableConnectionType::Power => {
                        network
                            .borrow_mut()
                            .as_mut_network()
                            .expect("non-network network")
                            .add_power(id);
                    }
                    _ => {
                        network
                            .borrow_mut()
                            .as_mut_network()
                            .expect("non-network network")
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
        &self,
        id: ObjectID,
        network_id: ObjectID,
    ) -> Result<bool, VMError> {
        if let Some(network) = self.networks.get(&network_id) {
            let Some(obj) = self.objects.get(&id) else {
                return Err(VMError::UnknownId(id));
            };
            let Some(device) = obj.borrow_mut().as_mut_device() else {
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
                .expect("non-network network")
                .remove_all(id);
            Ok(true)
        } else {
            Err(VMError::InvalidNetwork(network_id))
        }
    }

    pub fn set_batch_device_field(
        &self,
        source: ObjectID,
        prefab: f64,
        typ: LogicType,
        val: f64,
        write_readonly: bool,
    ) -> Result<(), ICError> {
        self.batch_device(source, prefab, None)
            .map(|device| {
                self.set_modified(*device.borrow().id());
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
        &self,
        source: ObjectID,
        prefab: f64,
        index: f64,
        typ: LogicSlotType,
        val: f64,
        write_readonly: bool,
    ) -> Result<(), ICError> {
        self.batch_device(source, prefab, None)
            .map(|device| {
                self.set_modified(*device.borrow().id());
                device
                    .borrow_mut()
                    .as_mut_device()
                    .expect("batch iter yielded non device")
                    .set_slot_logic(typ, index, val, self, write_readonly)
                    .map_err(Into::into)
            })
            .try_collect()
    }

    pub fn set_batch_name_device_field(
        &self,
        source: ObjectID,
        prefab: f64,
        name: f64,
        typ: LogicType,
        val: f64,
        write_readonly: bool,
    ) -> Result<(), ICError> {
        self.batch_device(source, prefab, Some(name))
            .map(|device| {
                self.set_modified(*device.borrow().id());
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
        &self,
        source: ObjectID,
        prefab: f64,
        typ: LogicType,
        mode: BatchMode,
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
        Ok(mode.apply(&samples))
    }

    pub fn get_batch_name_device_field(
        &self,
        source: ObjectID,
        prefab: f64,
        name: f64,
        typ: LogicType,
        mode: BatchMode,
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
        Ok(mode.apply(&samples))
    }

    pub fn get_batch_name_device_slot_field(
        &self,
        source: ObjectID,
        prefab: f64,
        name: f64,
        index: f64,
        typ: LogicSlotType,
        mode: BatchMode,
    ) -> Result<f64, ICError> {
        let samples = self
            .batch_device(source, prefab, Some(name))
            .map(|device| {
                device
                    .borrow()
                    .as_device()
                    .expect("batch iter yielded non device")
                    .get_slot_logic(typ, index, self)
                    .map_err(Into::into)
            })
            .filter_ok(|val| !val.is_nan())
            .collect::<Result<Vec<_>, ICError>>()?;
        Ok(mode.apply(&samples))
    }

    pub fn get_batch_device_slot_field(
        &self,
        source: ObjectID,
        prefab: f64,
        index: f64,
        typ: LogicSlotType,
        mode: BatchMode,
    ) -> Result<f64, ICError> {
        let samples = self
            .batch_device(source, prefab, None)
            .map(|device| {
                device
                    .borrow()
                    .as_device()
                    .expect("batch iter yielded non device")
                    .get_slot_logic(typ, index, self)
                    .map_err(Into::into)
            })
            .filter_ok(|val| !val.is_nan())
            .collect::<Result<Vec<_>, ICError>>()?;
        Ok(mode.apply(&samples))
    }

    pub fn remove_object(&mut self, id: ObjectID) -> Result<(), VMError> {
        let Some(obj) = self.objects.remove(&id) else {
            return Err(VMError::UnknownId(id));
        };

        if let Some(device) = obj.borrow().as_device() {
            for conn in device.connection_list().iter() {
                if let Connection::CableNetwork { net: Some(net), .. } = conn {
                    if let Some(network) = self.networks.get(net) {
                        network
                            .borrow_mut()
                            .as_mut_network()
                            .expect("non-network network")
                            .remove_all(id);
                    }
                }
            }
            if let Some(_) = device.as_circuit_holder() {
                self.ic_holders.borrow_mut().retain(|a| *a != id);
            }
        }
        self.id_space.free_id(id);
        Ok(())
    }

    /// set a slot to contain some quantity of an object with ID
    /// object must already be added to the VM
    /// does not clean up previous object
    /// returns the id of any former occupant
    pub fn set_slot_occupant(
        &mut self,
        id: ObjectID,
        index: usize,
        target: Option<ObjectID>,
        quantity: u32,
    ) -> Result<Option<ObjectID>, VMError> {
        let Some(obj) = self.objects.get(&id) else {
            return Err(VMError::UnknownId(id));
        };

        // FIXME: check that object has storage and object to be added is an item
        // need to move parentage and remove object from it former slot if it has one

        let Some(storage) = obj.borrow_mut().as_mut_storage() else {
            return Err(VMError::NotStorage(id));
        };
        let slot = storage
            .get_slot_mut(index)
            .ok_or(ICError::SlotIndexOutOfRange(index as f64))?;

        if


        if let Some(last) = slot.occupant.as_ref() {
            self.id_space.free_id(last.id);
        }
        slot.occupant = Some(occupant);

        Ok(())
    }

    pub fn remove_slot_occupant(&mut self, id: u32, index: usize) -> Result<(), VMError> {
        let Some(device) = self.devices.get(&id) else {
            return Err(VMError::UnknownId(id));
        };

        let mut device_ref = device.borrow_mut();
        let slot = device_ref
            .slots
            .get_mut(index)
            .ok_or(ICError::SlotIndexOutOfRange(index as f64))?;
        if let Some(last) = slot.occupant.as_ref() {
            self.id_space.free_id(last.id);
        }
        slot.occupant = None;
        Ok(())
    }

    pub fn save_vm_state(&self) -> FrozenVM {
        FrozenVM {
            ics: self
                .ic_holders
                .values()
                .map(|ic| ic.borrow().into())
                .collect(),
            devices: self
                .devices
                .values()
                .map(|device| device.borrow().into())
                .collect(),
            networks: self
                .networks
                .values()
                .map(|network| network.borrow().into())
                .collect(),
            default_network: self.default_network_key,
        }
    }

    pub fn restore_vm_state(&mut self, state: FrozenVM) -> Result<(), VMError> {
        self.ic_holders.clear();
        self.devices.clear();
        self.networks.clear();
        self.id_space.reset();
        self.network_id_space.reset();

        // ic ids sould be in slot occupants, don't duplicate
        let to_use_ids = state
            .devices
            .iter()
            .map(|template| {
                let mut ids = template
                    .slots
                    .iter()
                    .filter_map(|slot| slot.occupant.as_ref().and_then(|occupant| occupant.id))
                    .collect_vec();
                if let Some(id) = template.id {
                    ids.push(id);
                }
                ids
            })
            .concat();
        self.id_space.use_ids(&to_use_ids)?;

        self.network_id_space
            .use_ids(&state.networks.iter().map(|net| net.id).collect_vec())?;

        self.ic_holders = state
            .ics
            .into_iter()
            .map(|ic| (ic.id, Rc::new(RefCell::new(ic.into()))))
            .collect();
        self.devices = state
            .devices
            .into_iter()
            .map(|template| {
                let device = Device::from_template(template, || self.id_space.next());
                (device.id, Rc::new(RefCell::new(device)))
            })
            .collect();
        self.networks = state
            .networks
            .into_iter()
            .map(|network| (network.id, Rc::new(RefCell::new(network.into()))))
            .collect();
        self.default_network_key = state.default_network;
        Ok(())
    }
}

impl VMTransation {
    pub fn new(vm: &VM) -> Self {
        VMTransation {
            objects: BTreeMap::new(),
            ic_holders: Vec::new(),
            default_network_key: vm.default_network_key,
            wireless_transmitters: Vec::new(),
            wireless_receivers: Vec::new(),
            id_space: vm.id_space.clone(),
            networks: vm
                .networks
                .keys()
                .map(|net_id| (*net_id, VMTransationNetwork::default()))
                .collect(),
        }
    }

    pub fn add_device_from_template(
        &mut self,
        template: ObjectTemplate,
    ) -> Result<ObjectID, VMError> {
        for net_id in &template.connected_networks() {
            if !self.networks.contains_key(net_id) {
                return Err(VMError::InvalidNetwork(*net_id));
            }
        }

        let obj_id = if let Some(obj_id) = template.object().map(|info| info.id).flatten() {
            self.id_space.use_id(obj_id)?;
            obj_id
        } else {
            self.id_space.next()
        };

        let obj = template.build(obj_id);

        if let Some(storage) = obj.borrow_mut().as_mut_storage() {
            for (slot_index, occupant_template) in
                template.templates_from_slots().into_iter().enumerate()
            {
                if let Some(occupant_template) = occupant_template {
                    let occupant_id = self.add_device_from_template(occupant_template)?;
                    storage
                        .get_slot_mut(slot_index)
                        .expect(&format!("object storage slots out of sync with template which built it: {slot_index}"))
                        .occupant = Some(occupant_id);
                }
            }
        }

        if let Some(w_logicable) = obj.borrow().as_wireless_transmit() {
            self.wireless_transmitters.push(obj_id);
        }
        if let Some(r_logicable) = obj.borrow().as_wireless_receive() {
            self.wireless_receivers.push(obj_id);
        }
        if let Some(circuit_holder) = obj.borrow().as_circuit_holder() {
            self.ic_holders.push(obj_id);
        }
        if let Some(device) = obj.borrow_mut().as_mut_device() {
            for conn in device.connection_list().iter() {
                if let Connection::CableNetwork {
                    net: Some(net_id),
                    typ,
                } = conn
                {
                    if let Some(net) = self.networks.get_mut(net_id) {
                        match typ {
                            CableConnectionType::Power => net.power_only.push(obj_id),
                            _ => net.objects.push(obj_id),
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrozenVM {
    pub ics: Vec<FrozenIC>,
    pub devices: Vec<DeviceTemplate>,
    pub networks: Vec<FrozenNetwork>,
    pub default_network: u32,
}

impl BatchMode {
    pub fn apply(&self, samples: &[f64]) -> f64 {
        match self {
            BatchMode::Sum => samples.iter().sum(),
            // Both c-charp and rust return NaN for 0.0/0.0 so we're good here
            BatchMode::Average => samples.iter().copied().sum::<f64>() / samples.len() as f64,
            // Game uses a default of Positive INFINITY for Minimum
            BatchMode::Minimum => *samples
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(&f64::INFINITY),
            // Game uses default of NEG_INFINITY for Maximum
            BatchMode::Maximum => *samples
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
