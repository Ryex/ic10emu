use core::f64;
use std::collections::{HashMap, HashSet};

mod grammar;
mod interpreter;
mod rand_mscorlib;
mod tokens;

use grammar::{BatchMode, LogicType, ReagentMode, SlotLogicType};
use interpreter::ICError;
use itertools::Itertools;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VMError {
    #[error("Device with id '{0}' does not exist")]
    UnknownId(u16),
    #[error("Device with id '{0}' does not have a IC Slot")]
    NoIC(u16),
    #[error("IC encoutered an error: {0}")]
    ICError(#[from] ICError),
    #[error("Invalid network ID {0}")]
    InvalidNetwork(u16),
}

#[derive(Debug, PartialEq, Eq)]
pub enum FieldType {
    Read,
    Write,
    ReadWrite,
}

#[derive(Debug)]
pub struct LogicField {
    pub field_type: FieldType,
    pub value: f64,
}

#[derive(Debug, Default)]
pub struct Slot {
    pub fields: HashMap<grammar::SlotLogicType, LogicField>,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Connection {
    CableNetwork(Option<u16>),
    #[default]
    Other,
}

#[derive(Debug, Default)]
pub struct Device {
    pub id: u16,
    pub name: Option<String>,
    pub name_hash: Option<f64>,
    pub fields: HashMap<grammar::LogicType, LogicField>,
    pub slots: Vec<Slot>,
    pub reagents: HashMap<ReagentMode, HashMap<i32, f64>>,
    pub ic: Option<interpreter::IC>,
    pub connections: [Connection; 8],
    pub prefab_hash: Option<i32>,
}

#[derive(Debug)]
pub struct Network {
    pub devices: HashSet<u16>,
    pub channels: [f64; 8],
}

#[derive(Debug)]
struct IdSequenceGenerator {
    next: u16,
}

impl Default for IdSequenceGenerator {
    fn default() -> Self {
        IdSequenceGenerator { next: 1 }
    }
}

impl IdSequenceGenerator {
    pub fn next(&mut self) -> u16 {
        let val = self.next;
        self.next += 1;
        val
    }
}

#[derive(Debug)]
pub struct VM {
    pub ics: HashSet<u16>,
    pub devices: HashMap<u16, Device>,
    pub networks: HashMap<u16, Network>,
    pub default_network: u16,
    id_gen: IdSequenceGenerator,
    network_id_gen: IdSequenceGenerator,
    random: crate::rand_mscorlib::Random,
}

impl Default for Network {
    fn default() -> Self {
        Network {
            devices: HashSet::new(),
            channels: [f64::NAN; 8],
        }
    }
}

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("")]
    ChannelIndexOutOfRange,
}

impl Network {
    pub fn contains(&self, ids: &[u16]) -> bool {
        ids.iter().all(|id| self.devices.contains(id))
    }

    pub fn add(&mut self, id: u16) -> bool {
        self.devices.insert(id)
    }

    pub fn remove(&mut self, id: u16) -> bool {
        self.devices.remove(&id)
    }

    pub fn set_channel(&mut self, chan: usize, val: f64) -> Result<f64, NetworkError> {
        if chan > 7 {
            Err(NetworkError::ChannelIndexOutOfRange)
        } else {
            let last = self.channels[chan];
            self.channels[chan] = val;
            Ok(last)
        }
    }

    pub fn get_channel(&self, chan: usize) -> Result<f64, NetworkError> {
        if chan > 7 {
            Err(NetworkError::ChannelIndexOutOfRange)
        } else {
            Ok(self.channels[chan])
        }
    }
}

impl Device {
    pub fn new(id: u16) -> Self {
        Device {
            id,
            name: None,
            name_hash: None,
            fields: HashMap::new(),
            slots: Vec::new(),
            reagents: HashMap::new(),
            ic: None,
            connections: [Connection::default(); 8],
            prefab_hash: None,
        }
    }

    pub fn with_ic(id: u16) -> Self {
        let mut device = Device::new(id);
        device.ic = Some(interpreter::IC::new(id));
        device
    }

    pub fn get_network_id(&self, connection: usize) -> Result<u16, ICError> {
        if connection >= 8 {
            Err(ICError::ConnecitonIndexOutOFRange(connection as u32))
        } else {
            if let Connection::CableNetwork(network_id) = self.connections[connection] {
                if let Some(network_id) = network_id {
                    Ok(network_id)
                } else {
                    Err(ICError::NetworkNotConnected(connection as u32))
                }
            } else {
                Err(ICError::NotDataConnection(connection as u32))
            }
        }
    }

    pub fn get_field(&self, typ: grammar::LogicType) -> Result<f64, ICError> {
        if let Some(field) = self.fields.get(&typ) {
            if field.field_type == FieldType::Read || field.field_type == FieldType::ReadWrite {
                Ok(field.value)
            } else {
                Err(ICError::WriteOnlyField(typ.to_string()))
            }
        } else {
            Err(ICError::DeviceHasNoField(typ.to_string()))
        }
    }

    pub fn set_field(&mut self, typ: grammar::LogicType, val: f64) -> Result<(), ICError> {
        if let Some(field) = self.fields.get_mut(&typ) {
            if field.field_type == FieldType::Write || field.field_type == FieldType::ReadWrite {
                field.value = val;
                Ok(())
            } else {
                Err(ICError::ReadOnlyField(typ.to_string()))
            }
        } else {
            Err(ICError::DeviceHasNoField(typ.to_string()))
        }
    }

    pub fn get_slot_field(&self, index: f64, typ: grammar::SlotLogicType) -> Result<f64, ICError> {
        if let Some(field) = self
            .slots
            .get(index as usize)
            .ok_or(ICError::SlotIndexOutOfRange(index))?
            .fields
            .get(&typ)
        {
            if field.field_type == FieldType::Read || field.field_type == FieldType::ReadWrite {
                Ok(field.value)
            } else {
                Err(ICError::WriteOnlyField(typ.to_string()))
            }
        } else {
            Err(ICError::DeviceHasNoField(typ.to_string()))
        }
    }

    pub fn set_slot_field(
        &mut self,
        index: f64,
        typ: grammar::SlotLogicType,
        val: f64,
    ) -> Result<(), ICError> {
        if let Some(field) = self
            .slots
            .get_mut(index as usize)
            .ok_or(ICError::SlotIndexOutOfRange(index))?
            .fields
            .get_mut(&typ)
        {
            if field.field_type == FieldType::Write || field.field_type == FieldType::ReadWrite {
                field.value = val;
                Ok(())
            } else {
                Err(ICError::ReadOnlyField(typ.to_string()))
            }
        } else {
            Err(ICError::DeviceHasNoField(typ.to_string()))
        }
    }

    pub fn get_reagent(&self, rm: &ReagentMode, reagent: f64) -> f64 {
        if let Some(mode) = self.reagents.get(rm) {
            if let Some(val) = mode.get(&(reagent as i32)) {
                return *val;
            }
        }
        0.0
    }
}

impl VM {
    pub fn new() -> Self {
        let id_gen = IdSequenceGenerator::default();
        let mut network_id_gen = IdSequenceGenerator::default();
        let default_network = Network::default();
        let mut networks = HashMap::new();
        let default_network_key = network_id_gen.next();
        networks.insert(default_network_key, default_network);

        let mut vm = VM {
            ics: HashSet::new(),
            devices: HashMap::new(),
            networks,
            default_network: default_network_key,
            id_gen,
            network_id_gen,
            random: crate::rand_mscorlib::Random::new(),
        };
        let _ = vm.add_ic(None);
        vm
    }

    fn new_device(&mut self) -> Device {
        Device::new(self.id_gen.next())
    }

    fn new_ic(&mut self) -> Device {
        Device::with_ic(self.id_gen.next())
    }

    pub fn add_device(&mut self, network: Option<u16>) -> Result<u16, VMError> {
        if let Some(n) = &network {
            if !self.networks.contains_key(n) {
                return Err(VMError::InvalidNetwork(*n));
            }
        }
        let mut device = self.new_device();
        if let Some(first_network) = device
            .connections
            .iter_mut()
            .filter_map(|c| {
                if let Connection::CableNetwork(c) = c {
                    Some(c)
                } else {
                    None
                }
            })
            .next()
        {
            first_network.replace(if let Some(network) = network {
                network
            } else {
                self.default_network
            });
        }
        let id = device.id;
        self.devices.insert(id, device);
        Ok(id)
    }

    pub fn add_ic(&mut self, network: Option<u16>) -> Result<u16, VMError> {
        if let Some(n) = &network {
            if !self.networks.contains_key(n) {
                return Err(VMError::InvalidNetwork(*n));
            }
        }
        let mut device = self.new_ic();
        if let Some(first_network) = device
            .connections
            .iter_mut()
            .filter_map(|c| {
                if let Connection::CableNetwork(c) = c {
                    Some(c)
                } else {
                    None
                }
            })
            .next()
        {
            first_network.replace(if let Some(network) = network {
                network
            } else {
                self.default_network
            });
        }
        let id = device.id;
        self.devices.insert(id, device);
        self.ics.insert(id);
        Ok(id)
    }

    pub fn add_network(&mut self) -> u16 {
        let next_id = self.network_id_gen.next();
        self.networks.insert(next_id, Network::default());
        next_id
    }

    pub fn get_default_network(&mut self) -> &mut Network {
        self.networks.get_mut(&self.default_network).unwrap()
    }

    pub fn get_network(&mut self, id: u16) -> Option<&mut Network> {
        self.networks.get_mut(&id)
    }

    pub fn remove_ic(&mut self, id: u16) {
        if self.ics.remove(&id) {
            self.devices.remove(&id);
        }
    }

    pub fn set_code(&mut self, id: u16, code: &str) -> Result<bool, VMError> {
        let device = self.devices.get_mut(&id).ok_or(VMError::UnknownId(id))?;
        let ic = device.ic.as_mut().ok_or(VMError::NoIC(id))?;
        let new_prog = interpreter::Program::try_from_code(code)?;
        ic.program = new_prog;
        ic.code = code.to_string();
        Ok(true)
    }

    pub fn get_device(&mut self, id: u16) -> Option<&mut Device> {
        self.devices.get_mut(&id)
    }

    pub fn get_device_same_network(&mut self, source: u16, other: u16) -> Option<&mut Device> {
        if self.devices_on_same_network(&[source, other]) {
            self.get_device(other)
        } else {
            None
        }
    }

    pub fn get_network_channel(&self, id: usize, channel: usize) -> Result<f64, ICError> {
        let network = self
            .networks
            .get(&(id as u16))
            .ok_or(ICError::BadNetworkId(id as u32))?;
        Ok(network.channels[channel])
    }

    pub fn set_network_channel(&mut self, id: usize, channel: usize, val: f64) -> Result<(), ICError> {
        let network = self
            .networks
            .get_mut(&(id as u16))
            .ok_or(ICError::BadNetworkId(id as u32))?;
        network.channels[channel] = val;
        Ok(())
    }

    pub fn devices_on_same_network(&self, ids: &[u16]) -> bool {
        for (_id, net) in self.networks.iter() {
            if net.contains(ids) {
                return true;
            }
        }
        false
    }

    pub fn set_batch_device_field(
        &mut self,
        source: u16,
        prefab: f64,
        typ: LogicType,
        val: f64,
    ) -> Result<(), ICError> {
        let networks = &self.networks;
        self.devices
            .iter_mut()
            .map(|(id, device)| {
                if device.fields.get(&LogicType::PrefabHash).map(|f| f.value) == Some(prefab)
                    && networks
                        .iter()
                        .any(|(_net_id, net)| net.contains(&[source, *id]))
                {
                    device.set_field(typ, val)
                } else {
                    Ok(())
                }
            })
            .try_collect()
    }

    pub fn set_batch_device_slot_field(
        &mut self,
        source: u16,
        prefab: f64,
        index: f64,
        typ: SlotLogicType,
        val: f64,
    ) -> Result<(), ICError> {
        let networks = &self.networks;
        self.devices
            .iter_mut()
            .map(|(id, device)| {
                if device.fields.get(&LogicType::PrefabHash).map(|f| f.value) == Some(prefab)
                    && networks
                        .iter()
                        .any(|(_net_id, net)| net.contains(&[source, *id]))
                {
                    device.set_slot_field(index, typ, val)
                } else {
                    Ok(())
                }
            })
            .try_collect()
    }

    pub fn set_batch_name_device_field(
        &mut self,
        source: u16,
        prefab: f64,
        name: f64,
        typ: LogicType,
        val: f64,
    ) -> Result<(), ICError> {
        let networks = &self.networks;
        self.devices
            .iter_mut()
            .map(|(id, device)| {
                if device.fields.get(&LogicType::PrefabHash).map(|f| f.value) == Some(prefab)
                    && Some(name) == device.name_hash
                    && networks
                        .iter()
                        .any(|(_net_id, net)| net.contains(&[source, *id]))
                {
                    device.set_field(typ, val)
                } else {
                    Ok(())
                }
            })
            .try_collect()
    }

    pub fn get_batch_device_field(
        &mut self,
        source: u16,
        prefab: f64,
        typ: LogicType,
        mode: BatchMode,
    ) -> Result<f64, ICError> {
        let networks = &self.networks;
        let samples = self
            .devices
            .iter_mut()
            .map(|(id, device)| {
                if device.fields.get(&LogicType::PrefabHash).map(|f| f.value) == Some(prefab)
                    && networks
                        .iter()
                        .any(|(_net_id, net)| net.contains(&[source, *id]))
                {
                    device.get_field(typ).map(|val| Some(val))
                } else {
                    Ok(None)
                }
            })
            .collect::<Result<Vec<_>, ICError>>()?
            .into_iter()
            .filter_map(|val| {
                val.map(|val| if val.is_nan() { None } else { Some(val) })
                    .flatten()
            })
            .collect_vec();
        match mode {
            BatchMode::Sum => Ok(samples.iter().sum()),
            BatchMode::Average => Ok(samples.iter().copied().sum::<f64>() / samples.len() as f64),
            BatchMode::Minimum => Ok(*samples
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(&0.0)),
            BatchMode::Maximum => Ok(*samples
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(&0.0)),
        }
    }

    pub fn get_batch_name_device_field(
        &mut self,
        source: u16,
        prefab: f64,
        name: f64,
        typ: LogicType,
        mode: BatchMode,
    ) -> Result<f64, ICError> {
        let networks = &self.networks;
        let samples = self
            .devices
            .iter_mut()
            .map(|(id, device)| {
                if device.fields.get(&LogicType::PrefabHash).map(|f| f.value) == Some(prefab)
                    && Some(name) == device.name_hash
                    && networks
                        .iter()
                        .any(|(_net_id, net)| net.contains(&[source, *id]))
                {
                    device.get_field(typ).map(|val| Some(val))
                } else {
                    Ok(None)
                }
            })
            .collect::<Result<Vec<_>, ICError>>()?
            .into_iter()
            .filter_map(|val| {
                val.map(|val| if val.is_nan() { None } else { Some(val) })
                    .flatten()
            })
            .collect_vec();
        match mode {
            BatchMode::Sum => Ok(samples.iter().sum()),
            BatchMode::Average => Ok(samples.iter().copied().sum::<f64>() / samples.len() as f64),
            BatchMode::Minimum => Ok(*samples
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(&0.0)),
            BatchMode::Maximum => Ok(*samples
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(&0.0)),
        }
    }

    pub fn get_batch_name_device_slot_field(
        &mut self,
        source: u16,
        prefab: f64,
        name: f64,
        index: f64,
        typ: SlotLogicType,
        mode: BatchMode,
    ) -> Result<f64, ICError> {
        let networks = &self.networks;
        let samples = self
            .devices
            .iter_mut()
            .map(|(id, device)| {
                if device.fields.get(&LogicType::PrefabHash).map(|f| f.value) == Some(prefab)
                    && Some(name) == device.name_hash
                    && networks
                        .iter()
                        .any(|(_net_id, net)| net.contains(&[source, *id]))
                {
                    device.get_slot_field(index, typ).map(|val| Some(val))
                } else {
                    Ok(None)
                }
            })
            .collect::<Result<Vec<_>, ICError>>()?
            .into_iter()
            .filter_map(|val| {
                val.map(|val| if val.is_nan() { None } else { Some(val) })
                    .flatten()
            })
            .collect_vec();
        match mode {
            BatchMode::Sum => Ok(samples.iter().sum()),
            BatchMode::Average => Ok(samples.iter().copied().sum::<f64>() / samples.len() as f64),
            BatchMode::Minimum => Ok(*samples
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(&0.0)),
            BatchMode::Maximum => Ok(*samples
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(&0.0)),
        }
    }

    pub fn get_batch_device_slot_field(
        &mut self,
        source: u16,
        prefab: f64,
        index: f64,
        typ: SlotLogicType,
        mode: BatchMode,
    ) -> Result<f64, ICError> {
        let networks = &self.networks;
        let samples = self
            .devices
            .iter_mut()
            .map(|(id, device)| {
                if device.fields.get(&LogicType::PrefabHash).map(|f| f.value) == Some(prefab)
                    && networks
                        .iter()
                        .any(|(_net_id, net)| net.contains(&[source, *id]))
                {
                    device.get_slot_field(index, typ).map(|val| Some(val))
                } else {
                    Ok(None)
                }
            })
            .collect::<Result<Vec<_>, ICError>>()?
            .into_iter()
            .filter_map(|val| {
                val.map(|val| if val.is_nan() { None } else { Some(val) })
                    .flatten()
            })
            .collect_vec();
        match mode {
            BatchMode::Sum => Ok(samples.iter().sum()),
            BatchMode::Average => Ok(samples.iter().copied().sum::<f64>() / samples.len() as f64),
            BatchMode::Minimum => Ok(*samples
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(&0.0)),
            BatchMode::Maximum => Ok(*samples
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(&0.0)),
        }
    }
}
