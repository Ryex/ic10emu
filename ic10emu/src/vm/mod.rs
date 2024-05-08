pub mod enums;
pub mod instructions;
pub mod object;

use crate::{
    device::{Device, DeviceTemplate, SlotOccupant, SlotOccupantTemplate},
    errors::{ICError, VMError},
    interpreter::{self, FrozenIC},
    network::{CableConnectionType, Connection, FrozenNetwork, Network},
    vm::enums::script_enums::{
        LogicBatchMethod as BatchMode, LogicSlotType as SlotLogicType, LogicType,
    },
};
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashSet},
    rc::Rc,
};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct VM {
    pub ics: BTreeMap<u32, Rc<RefCell<interpreter::IC>>>,
    pub devices: BTreeMap<u32, Rc<RefCell<Device>>>,
    pub networks: BTreeMap<u32, Rc<RefCell<Network>>>,
    pub default_network: u32,
    id_space: IdSpace,
    network_id_space: IdSpace,
    random: Rc<RefCell<crate::rand_mscorlib::Random>>,

    /// list of device id's touched on the last operation
    operation_modified: RefCell<Vec<u32>>,
    #[allow(unused)]
    objects: Vec<object::BoxedObject>,
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
        let default_network = Rc::new(RefCell::new(Network::new(default_network_key)));
        let mut networks = BTreeMap::new();
        networks.insert(default_network_key, default_network);

        let mut vm = VM {
            ics: BTreeMap::new(),
            devices: BTreeMap::new(),
            networks,
            default_network: default_network_key,
            id_space,
            network_id_space,
            random: Rc::new(RefCell::new(crate::rand_mscorlib::Random::new())),
            operation_modified: RefCell::new(Vec::new()),
            objects: Vec::new(),
        };
        let _ = vm.add_ic(None);
        vm
    }

    fn new_device(&mut self) -> Device {
        Device::new(self.id_space.next())
    }

    fn new_ic(&mut self) -> (Device, interpreter::IC) {
        let id = self.id_space.next();
        let ic_id = self.id_space.next();
        let ic = interpreter::IC::new(ic_id, id);
        let device = Device::with_ic(id, ic_id);
        (device, ic)
    }

    pub fn random_f64(&self) -> f64 {
        self.random.borrow_mut().next_f64()
    }

    pub fn add_device(&mut self, network: Option<u32>) -> Result<u32, VMError> {
        if let Some(n) = &network {
            if !self.networks.contains_key(n) {
                return Err(VMError::InvalidNetwork(*n));
            }
        }
        let mut device = self.new_device();
        if let Some(first_network) = device.connections.iter_mut().find_map(|c| {
            if let Connection::CableNetwork {
                net,
                typ: CableConnectionType::Data | CableConnectionType::PowerAndData,
            } = c
            {
                Some(net)
            } else {
                None
            }
        }) {
            first_network.replace(if let Some(network) = network {
                network
            } else {
                self.default_network
            });
        }
        let id = device.id;

        let first_data_network = device
            .connections
            .iter()
            .enumerate()
            .find_map(|(index, conn)| match conn {
                Connection::CableNetwork {
                    typ: CableConnectionType::Data | CableConnectionType::PowerAndData,
                    ..
                } => Some(index),
                _ => None,
            });
        self.devices.insert(id, Rc::new(RefCell::new(device)));
        if let Some(first_data_network) = first_data_network {
            let _ = self.set_device_connection(
                id,
                first_data_network,
                if let Some(network) = network {
                    Some(network)
                } else {
                    Some(self.default_network)
                },
            );
        }
        Ok(id)
    }

    pub fn add_ic(&mut self, network: Option<u32>) -> Result<u32, VMError> {
        if let Some(n) = &network {
            if !self.networks.contains_key(n) {
                return Err(VMError::InvalidNetwork(*n));
            }
        }
        let (mut device, ic) = self.new_ic();
        if let Some(first_network) = device.connections.iter_mut().find_map(|c| {
            if let Connection::CableNetwork {
                net,
                typ: CableConnectionType::Data | CableConnectionType::PowerAndData,
            } = c
            {
                Some(net)
            } else {
                None
            }
        }) {
            first_network.replace(if let Some(network) = network {
                network
            } else {
                self.default_network
            });
        }
        let id = device.id;
        let ic_id = ic.id;
        let first_data_network = device
            .connections
            .iter()
            .enumerate()
            .find_map(|(index, conn)| match conn {
                Connection::CableNetwork {
                    typ: CableConnectionType::Data | CableConnectionType::PowerAndData,
                    ..
                } => Some(index),
                _ => None,
            });
        self.devices.insert(id, Rc::new(RefCell::new(device)));
        self.ics.insert(ic_id, Rc::new(RefCell::new(ic)));
        if let Some(first_data_network) = first_data_network {
            let _ = self.set_device_connection(
                id,
                first_data_network,
                if let Some(network) = network {
                    Some(network)
                } else {
                    Some(self.default_network)
                },
            );
        }
        Ok(id)
    }

    pub fn add_device_from_template(&mut self, template: DeviceTemplate) -> Result<u32, VMError> {
        for conn in &template.connections {
            if let Connection::CableNetwork { net: Some(net), .. } = conn {
                if !self.networks.contains_key(net) {
                    return Err(VMError::InvalidNetwork(*net));
                }
            }
        }

        // collect the id's this template wants to use
        let to_use_ids = template
            .slots
            .iter()
            .filter_map(|slot| slot.occupant.as_ref().and_then(|occupant| occupant.id))
            .collect_vec();

        // use those ids or fail
        self.id_space.use_ids(&to_use_ids)?;

        let device = Device::from_template(template, || self.id_space.next());
        let device_id: u32 = device.id;

        // if this device says it has an IC make it so.
        if let Some(ic_id) = &device.ic {
            let chip = interpreter::IC::new(*ic_id, device_id);
            self.ics.insert(*ic_id, Rc::new(RefCell::new(chip)));
        }

        device.connections.iter().for_each(|conn| {
            if let Connection::CableNetwork {
                net: Some(net),
                typ,
            } = conn
            {
                if let Some(network) = self.networks.get(net) {
                    match typ {
                        CableConnectionType::Power => {
                            network.borrow_mut().add_power(device_id);
                        }
                        _ => {
                            network.borrow_mut().add_data(device_id);
                        }
                    }
                }
            }
        });

        self.devices
            .insert(device_id, Rc::new(RefCell::new(device)));

        Ok(device_id)
    }

    pub fn add_network(&mut self) -> u32 {
        let next_id = self.network_id_space.next();
        self.networks
            .insert(next_id, Rc::new(RefCell::new(Network::new(next_id))));
        next_id
    }

    pub fn get_default_network(&self) -> Rc<RefCell<Network>> {
        self.networks.get(&self.default_network).cloned().unwrap()
    }

    pub fn get_network(&self, id: u32) -> Option<Rc<RefCell<Network>>> {
        self.networks.get(&id).cloned()
    }

    pub fn remove_ic(&mut self, id: u32) {
        if self.ics.remove(&id).is_some() {
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
        self.ics.iter().for_each(|(_id, ic)| {
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
            .ics
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
            .ics
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
            .ics
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
            .ics
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

    pub fn set_modified(&self, id: u32) {
        self.operation_modified.borrow_mut().push(id);
    }

    pub fn reset_ic(&self, id: u32) -> Result<bool, VMError> {
        let device = self.devices.get(&id).ok_or(VMError::UnknownId(id))?.clone();
        let ic_id = *device.borrow().ic.as_ref().ok_or(VMError::NoIC(id))?;
        let ic = self
            .ics
            .get(&ic_id)
            .ok_or(VMError::UnknownIcId(ic_id))?
            .clone();
        ic.borrow().ic.replace(0);
        ic.borrow().reset();
        Ok(true)
    }

    pub fn get_device(&self, id: u32) -> Option<Rc<RefCell<Device>>> {
        self.devices.get(&id).cloned()
    }

    pub fn batch_device(
        &self,
        source: u32,
        prefab_hash: f64,
        name: Option<f64>,
    ) -> impl Iterator<Item = &Rc<RefCell<Device>>> {
        self.devices
            .iter()
            .filter(move |(id, device)| {
                device
                    .borrow()
                    .get_fields(self)
                    .get(&LogicType::PrefabHash)
                    .is_some_and(|f| f.value == prefab_hash)
                    && (name.is_none()
                        || name == device.borrow().name_hash.as_ref().map(|hash| *hash as f64))
                    && self.devices_on_same_network(&[source, **id])
            })
            .map(|(_, d)| d)
    }

    pub fn get_device_same_network(&self, source: u32, other: u32) -> Option<Rc<RefCell<Device>>> {
        if self.devices_on_same_network(&[source, other]) {
            self.get_device(other)
        } else {
            None
        }
    }

    pub fn get_network_channel(&self, id: u32, channel: usize) -> Result<f64, ICError> {
        let network = self.networks.get(&id).ok_or(ICError::BadNetworkId(id))?;
        if !(0..8).contains(&channel) {
            Err(ICError::ChannelIndexOutOfRange(channel))
        } else {
            Ok(network.borrow().channels[channel])
        }
    }

    pub fn set_network_channel(&self, id: u32, channel: usize, val: f64) -> Result<(), ICError> {
        let network = self.networks.get(&(id)).ok_or(ICError::BadNetworkId(id))?;
        if !(0..8).contains(&channel) {
            Err(ICError::ChannelIndexOutOfRange(channel))
        } else {
            network.borrow_mut().channels[channel] = val;
            Ok(())
        }
    }

    pub fn devices_on_same_network(&self, ids: &[u32]) -> bool {
        for net in self.networks.values() {
            if net.borrow().contains_all_data(ids) {
                return true;
            }
        }
        false
    }

    /// return a vecter with the device ids the source id can see via it's connected networks
    pub fn visible_devices(&self, source: u32) -> Vec<u32> {
        self.networks
            .values()
            .filter_map(|net| {
                if net.borrow().contains_data(&source) {
                    Some(net.borrow().data_visible(&source))
                } else {
                    None
                }
            })
            .concat()
    }

    pub fn set_pin(&self, id: u32, pin: usize, val: Option<u32>) -> Result<bool, VMError> {
        let Some(device) = self.devices.get(&id) else {
            return Err(VMError::UnknownId(id));
        };
        if let Some(other_device) = val {
            if !self.devices.contains_key(&other_device) {
                return Err(VMError::UnknownId(other_device));
            }
            if !self.devices_on_same_network(&[id, other_device]) {
                return Err(VMError::DeviceNotVisible(other_device, id));
            }
        }
        if !(0..6).contains(&pin) {
            Err(ICError::PinIndexOutOfRange(pin).into())
        } else {
            let Some(ic_id) = device.borrow().ic else {
                return Err(VMError::NoIC(id));
            };
            self.ics.get(&ic_id).unwrap().borrow().pins.borrow_mut()[pin] = val;
            Ok(true)
        }
    }

    pub fn set_device_connection(
        &self,
        id: u32,
        connection: usize,
        target_net: Option<u32>,
    ) -> Result<bool, VMError> {
        let Some(device) = self.devices.get(&id) else {
            return Err(VMError::UnknownId(id));
        };
        if connection >= device.borrow().connections.len() {
            let conn_len = device.borrow().connections.len();
            return Err(ICError::ConnectionIndexOutOfRange(connection, conn_len).into());
        }

        {
            // scope this borrow
            let connections = &device.borrow().connections;
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
                                network.borrow_mut().remove_power(id);
                            }
                            _ => {
                                network.borrow_mut().remove_data(id);

                            }
                        }
                    }
                }
            }
        }
        let mut device_ref = device.borrow_mut();
        let connections = &mut device_ref.connections;
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
                        network.borrow_mut().add_power(id);
                    }
                    _ => {
                        network.borrow_mut().add_data(id);
                    }
                }
            } else {
                return Err(VMError::InvalidNetwork(target_net));
            }
        }
        *net = target_net;
        Ok(true)
    }

    pub fn remove_device_from_network(&self, id: u32, network_id: u32) -> Result<bool, VMError> {
        if let Some(network) = self.networks.get(&network_id) {
            let Some(device) = self.devices.get(&id) else {
                return Err(VMError::UnknownId(id));
            };
            let mut device_ref = device.borrow_mut();

            for conn in device_ref.connections.iter_mut() {
                if let Connection::CableNetwork { net, .. } = conn {
                    if net.is_some_and(|id| id == network_id) {
                        *net = None;
                    }
                }
            }
            network.borrow_mut().remove_all(id);
            Ok(true)
        } else {
            Err(VMError::InvalidNetwork(network_id))
        }
    }

    pub fn set_batch_device_field(
        &self,
        source: u32,
        prefab: f64,
        typ: LogicType,
        val: f64,
        write_readonly: bool,
    ) -> Result<(), ICError> {
        self.batch_device(source, prefab, None)
            .map(|device| {
                self.set_modified(device.borrow().id);
                device
                    .borrow_mut()
                    .set_field(typ, val, self, write_readonly)
            })
            .try_collect()
    }

    pub fn set_batch_device_slot_field(
        &self,
        source: u32,
        prefab: f64,
        index: f64,
        typ: SlotLogicType,
        val: f64,
        write_readonly: bool,
    ) -> Result<(), ICError> {
        self.batch_device(source, prefab, None)
            .map(|device| {
                self.set_modified(device.borrow().id);
                device
                    .borrow_mut()
                    .set_slot_field(index, typ, val, self, write_readonly)
            })
            .try_collect()
    }

    pub fn set_batch_name_device_field(
        &self,
        source: u32,
        prefab: f64,
        name: f64,
        typ: LogicType,
        val: f64,
        write_readonly: bool,
    ) -> Result<(), ICError> {
        self.batch_device(source, prefab, Some(name))
            .map(|device| {
                self.set_modified(device.borrow().id);
                device
                    .borrow_mut()
                    .set_field(typ, val, self, write_readonly)
            })
            .try_collect()
    }

    pub fn get_batch_device_field(
        &self,
        source: u32,
        prefab: f64,
        typ: LogicType,
        mode: BatchMode,
    ) -> Result<f64, ICError> {
        let samples = self
            .batch_device(source, prefab, None)
            .map(|device| device.borrow_mut().get_field(typ, self))
            .filter_ok(|val| !val.is_nan())
            .collect::<Result<Vec<_>, ICError>>()?;
        Ok(mode.apply(&samples))
    }

    pub fn get_batch_name_device_field(
        &self,
        source: u32,
        prefab: f64,
        name: f64,
        typ: LogicType,
        mode: BatchMode,
    ) -> Result<f64, ICError> {
        let samples = self
            .batch_device(source, prefab, Some(name))
            .map(|device| device.borrow_mut().get_field(typ, self))
            .filter_ok(|val| !val.is_nan())
            .collect::<Result<Vec<_>, ICError>>()?;
        Ok(mode.apply(&samples))
    }

    pub fn get_batch_name_device_slot_field(
        &self,
        source: u32,
        prefab: f64,
        name: f64,
        index: f64,
        typ: SlotLogicType,
        mode: BatchMode,
    ) -> Result<f64, ICError> {
        let samples = self
            .batch_device(source, prefab, Some(name))
            .map(|device| device.borrow().get_slot_field(index, typ, self))
            .filter_ok(|val| !val.is_nan())
            .collect::<Result<Vec<_>, ICError>>()?;
        Ok(mode.apply(&samples))
    }

    pub fn get_batch_device_slot_field(
        &self,
        source: u32,
        prefab: f64,
        index: f64,
        typ: SlotLogicType,
        mode: BatchMode,
    ) -> Result<f64, ICError> {
        let samples = self
            .batch_device(source, prefab, None)
            .map(|device| device.borrow().get_slot_field(index, typ, self))
            .filter_ok(|val| !val.is_nan())
            .collect::<Result<Vec<_>, ICError>>()?;
        Ok(mode.apply(&samples))
    }

    pub fn remove_device(&mut self, id: u32) -> Result<(), VMError> {
        let Some(device) = self.devices.remove(&id) else {
            return Err(VMError::UnknownId(id));
        };

        for conn in device.borrow().connections.iter() {
            if let Connection::CableNetwork { net: Some(net), .. } = conn {
                if let Some(network) = self.networks.get(net) {
                    network.borrow_mut().remove_all(id);
                }
            }
        }
        if let Some(ic_id) = device.borrow().ic {
            let _ = self.ics.remove(&ic_id);
        }
        self.id_space.free_id(id);
        Ok(())
    }

    pub fn set_slot_occupant(
        &mut self,
        id: u32,
        index: usize,
        template: SlotOccupantTemplate,
    ) -> Result<(), VMError> {
        let Some(device) = self.devices.get(&id) else {
            return Err(VMError::UnknownId(id));
        };

        let mut device_ref = device.borrow_mut();
        let slot = device_ref
            .slots
            .get_mut(index)
            .ok_or(ICError::SlotIndexOutOfRange(index as f64))?;

        if let Some(id) = template.id.as_ref() {
            self.id_space.use_id(*id)?;
        }

        let occupant = SlotOccupant::from_template(template, || self.id_space.next());
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
            ics: self.ics.values().map(|ic| ic.borrow().into()).collect(),
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
            default_network: self.default_network,
        }
    }

    pub fn restore_vm_state(&mut self, state: FrozenVM) -> Result<(), VMError> {
        self.ics.clear();
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

        self.ics = state
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
        self.default_network = state.default_network;
        Ok(())
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

#[derive(Debug)]
struct IdSpace {
    next: u32,
    in_use: HashSet<u32>,
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

    pub fn next(&mut self) -> u32 {
        let val = self.next;
        self.next += 1;
        self.in_use.insert(val);
        val
    }

    pub fn use_id(&mut self, id: u32) -> Result<(), VMError> {
        if self.in_use.contains(&id) {
            Err(VMError::IdInUse(id))
        } else {
            self.in_use.insert(id);
            Ok(())
        }
    }

    pub fn use_ids<'a, I>(&mut self, ids: I) -> Result<(), VMError>
    where
        I: IntoIterator<Item = &'a u32> + std::marker::Copy,
    {
        let mut to_use: HashSet<u32> = HashSet::new();
        let mut duplicates: HashSet<u32> = HashSet::new();
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

    pub fn free_id(&mut self, id: u32) {
        self.in_use.remove(&id);
    }

    pub fn reset(&mut self) {
        self.in_use.clear();
    }
}
