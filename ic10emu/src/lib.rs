use core::f64;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub mod grammar;
pub mod interpreter;
mod rand_mscorlib;
pub mod tokens;

use grammar::{BatchMode, LogicType, ReagentMode, SlotLogicType};
use interpreter::{ICError, LineError};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::interpreter::ICState;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum VMError {
    #[error("device with id '{0}' does not exist")]
    UnknownId(u32),
    #[error("ic with id '{0}' does not exist")]
    UnknownIcId(u32),
    #[error("device with id '{0}' does not have a ic slot")]
    NoIC(u32),
    #[error("ic encountered an error: {0}")]
    ICError(#[from] ICError),
    #[error("ic encountered an error: {0}")]
    LineError(#[from] LineError),
    #[error("invalid network id {0}")]
    InvalidNetwork(u32),
    #[error("device {0} not visible to device {1} (not on the same networks)")]
    DeviceNotVisible(u32, u32),
    #[error("a device with id {0} already exists")]
    IdInUse(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldType {
    Read,
    Write,
    ReadWrite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicField {
    pub field_type: FieldType,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotOccupant {
    pub id: u32,
    pub prefab_hash: i32,
    pub quantity: u32,
    pub max_quantity: u32,
    pub damage: f64,
    fields: HashMap<SlotLogicType, LogicField>,
}

impl SlotOccupant {
    pub fn new(id: u32, prefab_hash: i32) -> Self {
        SlotOccupant {
            id,
            prefab_hash,
            quantity: 1,
            max_quantity: 1,
            damage: 0.0,
            fields: HashMap::new(),
        }
    }

    /// chainable constructor
    pub fn with_quantity(mut self, quantity: u32) -> Self {
        self.quantity = quantity;
        self
    }

    /// chainable constructor
    pub fn with_max_quantity(mut self, max_quantity: u32) -> Self {
        self.max_quantity = max_quantity;
        self
    }

    /// chainable constructor
    pub fn with_damage(mut self, damage: f64) -> Self {
        self.damage = damage;
        self
    }

    /// chainable constructor
    pub fn with_fields(mut self, fields: HashMap<SlotLogicType, LogicField>) -> Self {
        self.fields.extend(fields);
        self
    }

    /// chainable constructor
    pub fn get_fields(&self) -> HashMap<SlotLogicType, LogicField> {
        self.fields.clone()
    }

    /// slot field operations don't fail
    pub fn set_field(&mut self, field: SlotLogicType, val: f64) {
        if let Some(logic) = self.fields.get_mut(&field) {
            match logic.field_type {
                FieldType::ReadWrite | FieldType::Write => logic.value = val,
                _ => {}
            }
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Slot {
    pub typ: SlotType,
    pub occupant: Option<SlotOccupant>,
}

impl Slot {
    pub fn new(typ: SlotType) -> Self {
        Slot {
            typ,
            occupant: None,
        }
    }
    pub fn with_occupant(typ: SlotType, occupant: SlotOccupant) -> Self {
        Slot {
            typ,
            occupant: Some(occupant),
        }
    }

    pub fn get_fields(&self) -> HashMap<SlotLogicType, LogicField> {
        let mut copy = self
            .occupant
            .as_ref()
            .map(|occupant| occupant.get_fields())
            .unwrap_or_default();
        copy.insert(
            SlotLogicType::ReferenceId,
            LogicField {
                field_type: FieldType::Read,
                value: self
                    .occupant
                    .as_ref()
                    .map(|occupant| occupant.id as f64)
                    .unwrap_or(0.0),
            },
        );
        copy.insert(
            SlotLogicType::Occupied,
            LogicField {
                field_type: FieldType::Read,
                value: if self.occupant.is_some() { 1.0 } else { 0.0 },
            },
        );
        copy.insert(
            SlotLogicType::OccupantHash,
            LogicField {
                field_type: FieldType::Read,
                value: self
                    .occupant
                    .as_ref()
                    .map(|occupant| occupant.prefab_hash as f64)
                    .unwrap_or(0.0),
            },
        );
        copy.insert(
            SlotLogicType::PrefabHash,
            LogicField {
                field_type: FieldType::Read,
                value: self
                    .occupant
                    .as_ref()
                    .map(|occupant| occupant.prefab_hash as f64)
                    .unwrap_or(0.0),
            },
        );
        copy.insert(
            SlotLogicType::Quantity,
            LogicField {
                field_type: FieldType::Read,
                value: self
                    .occupant
                    .as_ref()
                    .map(|occupant| occupant.quantity as f64)
                    .unwrap_or(0.0),
            },
        );
        copy.insert(
            SlotLogicType::MaxQuantity,
            LogicField {
                field_type: FieldType::Read,
                value: self
                    .occupant
                    .as_ref()
                    .map(|occupant| occupant.max_quantity as f64)
                    .unwrap_or(0.0),
            },
        );
        copy.insert(
            SlotLogicType::Class,
            LogicField {
                field_type: FieldType::Read,
                value: self.typ as u32 as f64,
            },
        );
        copy
    }

    /// the game returns 0.0 for all slotlogic types if they are not set
    pub fn get_field(&self, field: SlotLogicType) -> f64 {
        let fields = self.get_fields();
        fields
            .get(&field)
            .map(|field| match field.field_type {
                FieldType::Read | FieldType::ReadWrite => field.value,
                _ => 0.0,
            })
            .unwrap_or(0.0)
    }

    pub fn set_field(&mut self, field: SlotLogicType, val: f64) {
        if let Some(occupant) = self.occupant.as_mut() {
            occupant.set_field(field, val);
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum CableConnectionType {
    Power,
    Data,
    #[default]
    PowerAndData,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum Connection {
    CableNetwork {
        net: Option<u32>,
        typ: CableConnectionType,
    },
    #[default]
    Other,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeviceTemplate {
    pub name: Option<String>,
    pub hash: Option<i32>,
    pub logic: HashMap<grammar::LogicType, LogicField>,
    pub slots: Vec<SlotTemplate>,
    pub slotlogic: HashMap<grammar::LogicType, usize>,
    pub conn: Vec<(ConnectionType, ConnectionRole)>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionType {
    Pipe,
    Power,
    Data,
    Chute,
    Elevator,
    PipeLiquid,
    LandingPad,
    LaunchPad,
    PowerAndData,
    #[serde(other)]
    #[default]
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionRole {
    Input,
    Input2,
    Output,
    Output2,
    Waste,
    #[serde(other)]
    #[default]
    None,
}

impl Connection {
    #[allow(dead_code)]
    fn from(typ: ConnectionType, _role: ConnectionRole) -> Self {
        match typ {
            ConnectionType::None
            | ConnectionType::Chute
            | ConnectionType::Pipe
            | ConnectionType::Elevator
            | ConnectionType::LandingPad
            | ConnectionType::LaunchPad
            | ConnectionType::PipeLiquid => Self::Other,
            ConnectionType::Data => Self::CableNetwork {
                net: None,
                typ: CableConnectionType::Data,
            },
            ConnectionType::Power => Self::CableNetwork {
                net: None,
                typ: CableConnectionType::Power,
            },
            ConnectionType::PowerAndData => Self::CableNetwork {
                net: None,
                typ: CableConnectionType::PowerAndData,
            },
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]

pub enum SlotType {
    Helmet = 1,
    Suit = 2,
    Back,
    GasFilter = 4,
    GasCanister,
    MotherBoard,
    Circuitboard,
    DataDisk = 8,
    Organ,
    Ore,
    Plant,
    Uniform,
    Entity,
    Battery,
    Egg,
    Belt = 16,
    Tool,
    Appliance,
    Ingot,
    Torpedo,
    Cartridge,
    AccessCard,
    Magazine,
    Circuit = 24,
    Bottle,
    ProgramableChip,
    Glasses,
    CreditCard,
    DirtCanister,
    SensorProcessingUnit,
    LiquidCanister,
    LiquidBottle = 32,
    Wreckage,
    SoundCartridge,
    DrillHead,
    ScanningHead,
    Flare,
    Blocked,
    #[default]
    #[serde(other)]
    None = 0,
}

#[derive(Debug, Default)]
pub struct Device {
    pub id: u32,
    pub name: Option<String>,
    pub name_hash: Option<f64>,
    pub prefab_name: Option<String>,
    pub prefab_hash: Option<i32>,
    pub slots: Vec<Slot>,
    pub reagents: HashMap<ReagentMode, HashMap<i32, f64>>,
    pub ic: Option<u32>,
    pub connections: Vec<Connection>,
    pub on: bool,
    fields: HashMap<LogicType, LogicField>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    pub devices: HashSet<u32>,
    pub channels: [f64; 8],
}

#[derive(Debug, Default)]
struct IdSequenceGenerator {
    next: u32,
}

impl IdSequenceGenerator {
    pub fn next(&mut self) -> u32 {
        let val = self.next;
        self.next += 1;
        val
    }

    pub fn next_free<'a, I>(&mut self, in_use: I) -> u32
    where
        I: IntoIterator<Item = &'a u32>,
    {
        let sorted_in_use = in_use.into_iter().sorted();
        let mut last = None;
        for val in sorted_in_use.into_iter() {
            if val > &self.next && last.is_some_and(|last| (val - last) > 1) {
                break;
            }
            last = Some(val);
        }
        if let Some(last) = last {
            self.next = u32::max(*last, self.next) + 1;
            self.next
        } else {
            self.next()
        }
    }
}

#[derive(Debug)]
pub struct VM {
    pub ics: HashMap<u32, Rc<RefCell<interpreter::IC>>>,
    pub devices: HashMap<u32, Rc<RefCell<Device>>>,
    pub networks: HashMap<u32, Rc<RefCell<Network>>>,
    pub default_network: u32,
    id_gen: IdSequenceGenerator,
    network_id_gen: IdSequenceGenerator,
    random: Rc<RefCell<crate::rand_mscorlib::Random>>,

    /// list of device id's touched on the last operation
    operation_modified: RefCell<Vec<u32>>,
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
    pub fn contains(&self, ids: &[u32]) -> bool {
        ids.iter().all(|id| self.devices.contains(id))
    }

    pub fn add(&mut self, id: u32) -> bool {
        self.devices.insert(id)
    }

    pub fn remove(&mut self, id: u32) -> bool {
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
    pub fn new(id: u32) -> Self {
        let mut device = Device {
            id,
            name: None,
            name_hash: None,
            prefab_name: None,
            prefab_hash: None,
            fields: HashMap::new(),
            slots: Vec::new(),
            reagents: HashMap::new(),
            ic: None,
            on: true,
            connections: vec![Connection::CableNetwork {
                net: None,
                typ: CableConnectionType::default(),
            }],
        };
        device.fields.insert(
            LogicType::ReferenceId,
            LogicField {
                field_type: FieldType::Read,
                value: id as f64,
            },
        );
        device
    }

    pub fn with_ic(id: u32, ic: u32) -> Self {
        let mut device = Device::new(id);
        device.ic = Some(ic);
        device.connections = vec![
            Connection::CableNetwork {
                net: None,
                typ: CableConnectionType::Data,
            },
            Connection::CableNetwork {
                net: None,
                typ: CableConnectionType::Power,
            },
        ];
        device.prefab_name = Some("StructureCircuitHousing".to_owned());
        device.prefab_hash = Some(-128473777);
        device.fields.extend(vec![
            (
                LogicType::Setting,
                LogicField {
                    field_type: FieldType::ReadWrite,
                    value: 0.0,
                },
            ),
            (
                LogicType::RequiredPower,
                LogicField {
                    field_type: FieldType::Read,
                    value: 0.0,
                },
            ),
            (
                LogicType::PrefabHash,
                LogicField {
                    field_type: FieldType::Read,
                    value: -128473777.0,
                },
            ),
        ]);
        device.slots.push(Slot::with_occupant(
            SlotType::ProgramableChip,
            // -744098481 = ItemIntegratedCircuit10
            SlotOccupant::new(ic, -744098481),
        ));

        device
    }

    pub fn get_fields(&self, vm: &VM) -> HashMap<LogicType, LogicField> {
        let mut copy = self.fields.clone();
        if let Some(ic_id) = &self.ic {
            let ic = vm.ics.get(ic_id).expect("our own ic to exist").borrow();
            copy.insert(
                LogicType::LineNumber,
                LogicField {
                    field_type: FieldType::ReadWrite,
                    value: ic.ip as f64,
                },
            );
            copy.insert(
                LogicType::Error,
                LogicField {
                    field_type: FieldType::Read,
                    value: match ic.state {
                        ICState::Error(_) => 1.0,
                        _ => 0.0,
                    },
                },
            );
        }
        copy.insert(
            LogicType::On,
            LogicField {
                field_type: FieldType::ReadWrite,
                value: if self.on && self.has_power() {
                    1.0
                } else {
                    0.0
                },
            },
        );
        copy.insert(
            LogicType::Power,
            LogicField {
                field_type: FieldType::Read,
                value: if self.has_power() { 1.0 } else { 0.0 },
            },
        );
        copy
    }

    pub fn get_network_id(&self, connection: usize) -> Result<u32, ICError> {
        if connection >= self.connections.len() {
            Err(ICError::ConnectionIndexOutOfRange(
                connection,
                self.connections.len(),
            ))
        } else if let Connection::CableNetwork {
            net: network_id, ..
        } = self.connections[connection]
        {
            if let Some(network_id) = network_id {
                Ok(network_id)
            } else {
                Err(ICError::NetworkNotConnected(connection))
            }
        } else {
            Err(ICError::NotACableConnection(connection))
        }
    }

    pub fn get_field(&self, typ: LogicType, vm: &VM) -> Result<f64, ICError> {
        if typ == LogicType::LineNumber && self.ic.is_some() {
            if let Ok(ic) = vm
                .ics
                .get(&self.ic.unwrap())
                .ok_or_else(|| ICError::UnknownDeviceID(self.ic.unwrap() as f64))?
                .try_borrow()
            {
                Ok(ic.ip as f64)
            } else {
                // FIXME: the game succeeds in getting the correct line number
                // when reading it's own IC
                Ok(0.0)
            }
        } else if let Some(field) = self.get_fields(vm).get(&typ) {
            if field.field_type == FieldType::Read || field.field_type == FieldType::ReadWrite {
                Ok(field.value)
            } else {
                Err(ICError::WriteOnlyField(typ.to_string()))
            }
        } else {
            Err(ICError::DeviceHasNoField(typ.to_string()))
        }
    }

    pub fn set_field(&mut self, typ: LogicType, val: f64, vm: &VM) -> Result<(), ICError> {
        if typ == LogicType::On {
            self.on = val != 0.0;
            Ok(())
        } else if typ == LogicType::LineNumber && self.ic.is_some() {
            // try borrow to set ip, we shoudl only fail if the ic is in us aka is is *our* ic
            // in game trying to set your own ic's LineNumber appears to be a Nop so this is fine.
            if let Ok(mut ic) = vm
                .ics
                .get(&self.ic.unwrap())
                .ok_or_else(|| ICError::UnknownDeviceID(self.ic.unwrap() as f64))?
                .try_borrow_mut()
            {
                ic.ip = val as u32;
            }
            Ok(())
        } else if let Some(field) = self.fields.get_mut(&typ) {
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

    pub fn get_slot_field(&self, index: f64, typ: SlotLogicType, vm: &VM) -> Result<f64, ICError> {
        let slot = self
            .slots
            .get(index as usize)
            .ok_or(ICError::SlotIndexOutOfRange(index))?;
        if slot.typ == SlotType::ProgramableChip
            && slot.occupant.is_some()
            && self.ic.is_some()
            && typ == SlotLogicType::LineNumber
        {
            // try borrow to get ip, we should only fail if the ic is in us aka is is *our* ic
            if let Ok(ic) = vm
                .ics
                .get(&self.ic.unwrap())
                .ok_or_else(|| ICError::UnknownDeviceID(self.ic.unwrap() as f64))?
                .try_borrow()
            {
                Ok(ic.ip as f64)
            } else {
                // FIXME: the game succeeds in getting the correct line number
                // when reading it's own IC
                Ok(0.0)
            }
        } else {
            Ok(slot.get_field(typ))
        }
    }

    pub fn get_slot_fields(
        &self,
        index: f64,
        vm: &VM,
    ) -> Result<HashMap<SlotLogicType, LogicField>, ICError> {
        let slot = self
            .slots
            .get(index as usize)
            .ok_or(ICError::SlotIndexOutOfRange(index))?;
        let mut fields = slot.get_fields();
        if slot.typ == SlotType::ProgramableChip && slot.occupant.is_some() && self.ic.is_some() {
            // try borrow to get ip, we should only fail if the ic is in us aka is is *our* ic
            if let Ok(ic) = vm
                .ics
                .get(&self.ic.unwrap())
                .ok_or_else(|| ICError::UnknownDeviceID(self.ic.unwrap() as f64))?
                .try_borrow()
            {
                fields.insert(
                    SlotLogicType::LineNumber,
                    LogicField {
                        field_type: FieldType::ReadWrite,
                        value: ic.ip as f64,
                    },
                );
            } else {
                // FIXME: the game succeeds in getting the correct line number
                // when reading it's own IC
                fields.insert(
                    SlotLogicType::LineNumber,
                    LogicField {
                        field_type: FieldType::ReadWrite,
                        value: 0.0,
                    },
                );
            }
        }
        Ok(fields)
    }

    pub fn set_slot_field(
        &mut self,
        index: f64,
        typ: grammar::SlotLogicType,
        val: f64,
        vm: &VM,
    ) -> Result<(), ICError> {
        let slot = self
            .slots
            .get_mut(index as usize)
            .ok_or(ICError::SlotIndexOutOfRange(index))?;
        if slot.typ == SlotType::ProgramableChip
            && slot.occupant.is_some()
            && self.ic.is_some()
            && typ == SlotLogicType::LineNumber
        {
            // try borrow to set ip, we shoudl only fail if the ic is in us aka is is *our* ic
            // in game trying to set your own ic's LineNumber appears to be a Nop so this is fine.
            if let Ok(mut ic) = vm
                .ics
                .get(&self.ic.unwrap())
                .ok_or_else(|| ICError::UnknownDeviceID(self.ic.unwrap() as f64))?
                .try_borrow_mut()
            {
                ic.ip = val as u32;
            }
            Ok(())
        } else {
            slot.set_field(typ, val);
            Ok(())
        }
    }

    pub fn get_slot(&self, index: f64) -> Result<&Slot, ICError> {
        self.slots
            .get(index as usize)
            .ok_or(ICError::SlotIndexOutOfRange(index))
    }

    pub fn get_reagent(&self, rm: &ReagentMode, reagent: f64) -> f64 {
        if let Some(mode) = self.reagents.get(rm) {
            if let Some(val) = mode.get(&(reagent as i32)) {
                return *val;
            }
        }
        0.0
    }

    pub fn set_name(&mut self, name: &str) {
        self.name_hash = Some((const_crc32::crc32(name.as_bytes()) as i32).into());
        self.name = Some(name.to_owned());
    }

    pub fn has_power(&self) -> bool {
        self.connections.iter().any(|conn| {
            if let Connection::CableNetwork { net, typ } = conn {
                net.is_some()
                    && matches!(
                        typ,
                        CableConnectionType::Power | CableConnectionType::PowerAndData
                    )
            } else {
                false
            }
        })
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

impl VM {
    pub fn new() -> Self {
        let id_gen = IdSequenceGenerator::default();
        let mut network_id_gen = IdSequenceGenerator::default();
        let default_network = Rc::new(RefCell::new(Network::default()));
        let mut networks = HashMap::new();
        let default_network_key = network_id_gen.next();
        networks.insert(default_network_key, default_network);

        let mut vm = VM {
            ics: HashMap::new(),
            devices: HashMap::new(),
            networks,
            default_network: default_network_key,
            id_gen,
            network_id_gen,
            random: Rc::new(RefCell::new(crate::rand_mscorlib::Random::new())),
            operation_modified: RefCell::new(Vec::new()),
        };
        let _ = vm.add_ic(None);
        vm
    }

    fn new_device(&mut self) -> Device {
        Device::new(self.id_gen.next())
    }

    fn new_ic(&mut self) -> (Device, interpreter::IC) {
        let id = self
            .id_gen
            .next_free(self.devices.keys().chain(self.ics.keys()));
        let ic_id = self
            .id_gen
            .next_free(self.devices.keys().chain(self.ics.keys()));
        let ic = interpreter::IC::new(ic_id, id);
        let device = Device::with_ic(id, ic_id);
        (device, ic)
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

    pub fn add_network(&mut self) -> u32 {
        let next_id = self.network_id_gen.next();
        self.networks
            .insert(next_id, Rc::new(RefCell::new(Network::default())));
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
        if self.devices.contains_key(&new_id) | self.ics.contains_key(&new_id) {
            return Err(VMError::IdInUse(new_id));
        }
        let device = self
            .devices
            .remove(&old_id)
            .ok_or(VMError::UnknownId(old_id))?;
        device.borrow_mut().id = new_id;
        self.devices.insert(new_id, device);
        self.ics.iter().for_each(|(_id, ic)| {
            if let Ok(mut ic_ref) = ic.try_borrow_mut() {
                ic_ref.pins.iter_mut().for_each(|pin| {
                    if pin.is_some_and(|d| d == old_id) {
                        pin.replace(new_id);
                    }
                })
            }
        });
        self.networks.iter().for_each(|(_net_id, net)| {
            if let Ok(mut net_ref) = net.try_borrow_mut() {
                if net_ref.devices.remove(&old_id) {
                    net_ref.devices.insert(new_id);
                }
            }
        });
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
        let mut ic = self
            .ics
            .get(&ic_id)
            .ok_or(VMError::UnknownIcId(ic_id))?
            .borrow_mut();
        let new_prog = interpreter::Program::try_from_code(code)?;
        ic.program = new_prog;
        ic.ip = 0;
        ic.code = code.to_string();
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
        let mut ic = self
            .ics
            .get(&ic_id)
            .ok_or(VMError::UnknownIcId(ic_id))?
            .borrow_mut();
        let new_prog = interpreter::Program::from_code_with_invalid(code);
        ic.program = new_prog;
        ic.ip = 0;
        ic.code = code.to_string();
        Ok(true)
    }

    /// returns a list of device ids modified in the last operations
    pub fn last_operation_modified(&self) -> Vec<u32> {
        self.operation_modified.borrow().clone()
    }

    pub fn step_ic(&self, id: u32, advance_ip_on_err: bool) -> Result<bool, VMError> {
        self.operation_modified.borrow_mut().clear();
        let device = self.devices.get(&id).ok_or(VMError::UnknownId(id))?.clone();
        let ic_id = *device.borrow().ic.as_ref().ok_or(VMError::NoIC(id))?;
        self.set_modified(id);
        let ic = self
            .ics
            .get(&ic_id)
            .ok_or(VMError::UnknownIcId(ic_id))?
            .clone();
        ic.borrow_mut().ic = 0;
        let result = ic.borrow_mut().step(self, advance_ip_on_err)?;
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
        ic.borrow_mut().ic = 0;
        self.set_modified(id);
        for _i in 0..128 {
            if let Err(err) = ic.borrow_mut().step(self, ignore_errors) {
                if !ignore_errors {
                    return Err(err.into());
                }
            }
            if let interpreter::ICState::Yield = ic.borrow().state {
                return Ok(false);
            } else if let interpreter::ICState::Sleep(_then, _sleep_for) = ic.borrow().state {
                return Ok(false);
            }
        }
        ic.borrow_mut().state = interpreter::ICState::Yield;
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
        ic.borrow_mut().ic = 0;
        ic.borrow_mut().reset();
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
                    .fields
                    .get(&LogicType::PrefabHash)
                    .is_some_and(|f| f.value == prefab_hash)
                    && (name.is_none() || name == device.borrow().name_hash)
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
            if net.borrow().contains(ids) {
                return true;
            }
        }
        false
    }

    /// return a vecter with the device ids the source id can see via it's connected netowrks
    pub fn visible_devices(&self, source: u32) -> Vec<u32> {
        self.networks
            .values()
            .filter_map(|net| {
                if net.borrow().contains(&[source]) {
                    Some(
                        net.borrow()
                            .devices
                            .iter()
                            .filter(|id| id != &&source)
                            .copied()
                            .collect_vec(),
                    )
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
            self.ics.get(&ic_id).unwrap().borrow_mut().pins[pin] = val;
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
            let Connection::CableNetwork { net, .. } = &connections[connection] else {
                return Err(ICError::NotACableConnection(connection).into());
            };
            // remove from current network
            if let Some(net) = net {
                if let Some(network) = self.networks.get(net) {
                    // if there is no other connection to this network
                    if connections.clone().iter().enumerate().all(|(index, conn)| {
                        if let Connection::CableNetwork { net: other_net, .. } = conn {
                            !(other_net.is_some_and(|id| id == *net) && index != connection)
                        } else {
                            true
                        }
                    }) {
                        network.borrow_mut().remove(id);
                    }
                }
            }
        }
        let mut device_ref = device.borrow_mut();
        let connections = &mut device_ref.connections;
        let Connection::CableNetwork { ref mut net, .. } = connections[connection] else {
            return Err(ICError::NotACableConnection(connection).into());
        };
        if let Some(target_net) = target_net {
            if let Some(network) = self.networks.get(&target_net) {
                network.borrow_mut().add(id);
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
            network.borrow_mut().remove(id);
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
    ) -> Result<(), ICError> {
        self.batch_device(source, prefab, None)
            .map(|device| {
                self.set_modified(device.borrow().id);
                device.borrow_mut().set_field(typ, val, self)
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
    ) -> Result<(), ICError> {
        self.batch_device(source, prefab, None)
            .map(|device| {
                self.set_modified(device.borrow().id);
                device.borrow_mut().set_slot_field(index, typ, val, self)
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
    ) -> Result<(), ICError> {
        self.batch_device(source, prefab, Some(name))
            .map(|device| {
                self.set_modified(device.borrow().id);
                device.borrow_mut().set_field(typ, val, self)
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
}

impl BatchMode {
    pub fn apply(&self, samples: &[f64]) -> f64 {
        match self {
            BatchMode::Sum => samples.iter().sum(),
            /// Both c-charp and rust return NaN for 0.0/0.0 so we're good here
            BatchMode::Average => samples.iter().copied().sum::<f64>() / samples.len() as f64,
            /// Game uses a default of Positive INFINITY for Minimum
            BatchMode::Minimum => *samples
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(&f64::INFINITY),
            /// Game uses default of NEG_INFINITY for Maximum
            BatchMode::Maximum => *samples
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(&f64::NEG_INFINITY),
        }
    }
}
