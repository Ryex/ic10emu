use std::{collections::HashSet, ops::Deref, rc::Rc};

use crate::vm::{
    enums::script_enums::LogicType,
    object::{
        errors::LogicError, macros::ObjectInterface, templates::ConnectionInfo, traits::*, Name,
        ObjectID,
    },
    VM,
};
use itertools::Itertools;
use macro_rules_attribute::derive;
use serde_derive::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumIter};
use thiserror::Error;

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
        net: Option<ObjectID>,
        typ: CableConnectionType,
        role: ConnectionRole,
    },
    Chute {
        role: ConnectionRole,
    },
    Pipe {
        role: ConnectionRole,
    },
    Elevator {
        role: ConnectionRole,
    },
    LandingPad {
        role: ConnectionRole,
    },
    LaunchPad {
        role: ConnectionRole,
    },
    PipeLiquid {
        role: ConnectionRole,
    },
    #[default]
    None,
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    EnumIter,
    AsRefStr,
)]
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

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    EnumIter,
    AsRefStr,
)]
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
    pub fn from_info(typ: ConnectionType, role: ConnectionRole, net: Option<ObjectID>) -> Self {
        match typ {
            ConnectionType::None => Self::None,
            ConnectionType::Data => Self::CableNetwork {
                net,
                typ: CableConnectionType::Data,
                role,
            },
            ConnectionType::Power => Self::CableNetwork {
                net,
                typ: CableConnectionType::Power,
                role,
            },
            ConnectionType::PowerAndData => Self::CableNetwork {
                net,
                typ: CableConnectionType::PowerAndData,
                role,
            },
            ConnectionType::Chute => Self::Chute { role },
            ConnectionType::Pipe => Self::Pipe { role },
            ConnectionType::Elevator => Self::Elevator { role },
            ConnectionType::LandingPad => Self::LandingPad { role },
            ConnectionType::LaunchPad => Self::LaunchPad { role },
            ConnectionType::PipeLiquid => Self::PipeLiquid { role },
        }
    }

    pub fn to_info(&self) -> ConnectionInfo {
        match self {
            Self::None => ConnectionInfo {
                typ: ConnectionType::None,
                role: ConnectionRole::None,
                network: None,
            },
            Self::CableNetwork {
                net,
                typ: CableConnectionType::Data,
                role,
            } => ConnectionInfo {
                typ: ConnectionType::Data,
                role: *role,
                network: *net,
            },
            Self::CableNetwork {
                net,
                typ: CableConnectionType::Power,
                role,
            } => ConnectionInfo {
                typ: ConnectionType::Power,
                role: *role,
                network: *net,
            },
            Self::CableNetwork {
                net,
                typ: CableConnectionType::PowerAndData,
                role,
            } => ConnectionInfo {
                typ: ConnectionType::PowerAndData,
                role: *role,
                network: *net,
            },
            Self::Chute { role } => ConnectionInfo {
                typ: ConnectionType::Chute,
                role: *role,
                network: None,
            },
            Self::Pipe { role } => ConnectionInfo {
                typ: ConnectionType::Pipe,
                role: *role,
                network: None,
            },
            Self::PipeLiquid { role } => ConnectionInfo {
                typ: ConnectionType::PipeLiquid,
                role: *role,
                network: None,
            },
            Self::Elevator { role } => ConnectionInfo {
                typ: ConnectionType::Elevator,
                role: *role,
                network: None,
            },
            Self::LandingPad { role } => ConnectionInfo {
                typ: ConnectionType::LandingPad,
                role: *role,
                network: None,
            },
            Self::LaunchPad { role } => ConnectionInfo {
                typ: ConnectionType::LaunchPad,
                role: *role,
                network: None,
            },
        }
    }
}

#[derive(ObjectInterface!, Debug)]
#[custom(implements(Object { Storage, Logicable, Network}))]
pub struct CableNetwork {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    /// required by object interface but atm unused by network
    pub prefab: Name,
    #[custom(object_name)]
    /// required by object interface but atm unused by network
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    /// data enabled objects (must be devices)
    pub devices: HashSet<ObjectID>,
    /// power only connections
    pub power_only: HashSet<ObjectID>,
    /// channel data
    pub channels: [f64; 8],
}

impl Storage for CableNetwork {
    fn slots_count(&self) -> usize {
        0
    }
    fn get_slot(&self, index: usize) -> Option<&crate::vm::object::Slot> {
        None
    }
    fn get_slot_mut(&mut self, index: usize) -> Option<&mut crate::vm::object::Slot> {
        None
    }
    fn get_slots(&self) -> &[crate::vm::object::Slot] {
        &vec![]
    }
    fn get_slots_mut(&mut self) -> &mut [crate::vm::object::Slot] {
        &mut vec![]
    }
}

impl Logicable for CableNetwork {
    fn prefab_hash(&self) -> i32 {
        0
    }
    fn name_hash(&self) -> i32 {
        0
    }
    fn is_logic_readable(&self) -> bool {
        true
    }
    fn is_logic_writeable(&self) -> bool {
        true
    }
    fn can_logic_read(&self, lt: LogicType) -> bool {
        use LogicType::*;
        match lt {
            Channel0 | Channel1 | Channel2 | Channel3 | Channel4 | Channel5 | Channel6
            | Channel7 => true,
            _ => false,
        }
    }
    fn can_logic_write(&self, lt: LogicType) -> bool {
        use LogicType::*;
        match lt {
            Channel0 | Channel1 | Channel2 | Channel3 | Channel4 | Channel5 | Channel6
            | Channel7 => true,
            _ => false,
        }
    }
    fn get_logic(&self, lt: LogicType) -> Result<f64, crate::vm::object::errors::LogicError> {
        use LogicType::*;
        let index: usize = match lt {
            Channel0 => 0,
            Channel1 => 1,
            Channel2 => 2,
            Channel3 => 3,
            Channel4 => 4,
            Channel5 => 5,
            Channel6 => 6,
            Channel7 => 7,
            _ => return Err(LogicError::CantRead(lt)),
        };
        Ok(self.channels[index])
    }
    fn set_logic(&mut self, lt: LogicType, value: f64, force: bool) -> Result<(), LogicError> {
        use LogicType::*;
        let index: usize = match lt {
            Channel0 => 0,
            Channel1 => 1,
            Channel2 => 2,
            Channel3 => 3,
            Channel4 => 4,
            Channel5 => 5,
            Channel6 => 6,
            Channel7 => 7,
            _ => return Err(LogicError::CantWrite(lt)),
        };
        self.channels[index] = value;
        Ok(())
    }
    fn can_slot_logic_read(
        &self,
        slt: crate::vm::enums::script_enums::LogicSlotType,
        index: f64,
    ) -> bool {
        false
    }
    fn get_slot_logic(
        &self,
        slt: crate::vm::enums::script_enums::LogicSlotType,
        index: f64,
        vm: &crate::vm::VM,
    ) -> Result<f64, LogicError> {
        Err(LogicError::CantSlotRead(slt, index))
    }
    fn valid_logic_types(&self) -> Vec<LogicType> {
        use LogicType::*;
        vec![
            Channel0, Channel1, Channel2, Channel3, Channel4, Channel5, Channel6, Channel7,
        ]
    }
    fn known_modes(&self) -> Option<Vec<(u32, String)>> {
        None
    }
}

impl Network for CableNetwork {
    fn contains(&self, id: &ObjectID) -> bool {
        self.devices.contains(id) || self.power_only.contains(id)
    }

    fn contains_all(&self, ids: &[ObjectID]) -> bool {
        ids.iter().all(|id| self.contains(id))
    }

    fn contains_data(&self, id: &ObjectID) -> bool {
        self.devices.contains(id)
    }

    fn contains_all_data(&self, ids: &[ObjectID]) -> bool {
        ids.iter().all(|id| self.contains_data(id))
    }

    fn contains_power(&self, id: &ObjectID) -> bool {
        self.power_only.contains(id)
    }

    fn contains_all_power(&self, ids: &[ObjectID]) -> bool {
        ids.iter().all(|id| self.contains_power(id))
    }

    fn data_visible(&self, source: &ObjectID) -> Vec<ObjectID> {
        if self.contains_data(source) {
            self.devices
                .iter()
                .filter(|id| id != &source)
                .copied()
                .collect_vec()
        } else {
            Vec::new()
        }
    }

    fn add_data(&mut self, id: ObjectID) -> bool {
        self.devices.insert(id)
    }

    fn add_power(&mut self, id: ObjectID) -> bool {
        self.power_only.insert(id)
    }

    fn remove_all(&mut self, id: ObjectID) -> bool {
        self.devices.remove(&id) || self.power_only.remove(&id)
    }
    fn remove_data(&mut self, id: ObjectID) -> bool {
        self.devices.remove(&id)
    }

    fn remove_power(&mut self, id: ObjectID) -> bool {
        self.devices.remove(&id)
    }

    fn get_devices(&self) -> Vec<ObjectID> {
        self.devices.iter().copied().collect_vec()
    }

    fn get_power_only(&self) -> Vec<ObjectID> {
        self.power_only.iter().copied().collect_vec()
    }

    fn get_channel_data(&self) -> &[f64; 8] {
        &self.channels
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrozenCableNetwork {
    pub id: ObjectID,
    pub devices: Vec<u32>,
    pub power_only: Vec<u32>,
    pub channels: [f64; 8],
}

impl<T> From<T> for FrozenCableNetwork
where
    T: Deref<Target = CableNetwork>,
{
    fn from(value: T) -> Self {
        FrozenCableNetwork {
            id: value.id,
            devices: value.devices.iter().copied().collect_vec(),
            power_only: value.power_only.iter().copied().collect_vec(),
            channels: value.channels,
        }
    }
}

impl From<NetworkRef<'_>> for FrozenCableNetwork {
    fn from(value: NetworkRef) -> Self {
        FrozenCableNetwork {
            id: value.get_id(),
            devices: value.get_devices(),
            power_only: value.get_power_only(),
            channels: *value.get_channel_data(),
        }
    }
}

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("")]
    ChannelIndexOutOfRange,
}

impl CableNetwork {
    pub fn new(id: u32, vm: Rc<VM>) -> Self {
        CableNetwork {
            id,
            prefab: Name::new(""),
            name: Name::new(""),
            vm,
            devices: HashSet::new(),
            power_only: HashSet::new(),
            channels: [f64::NAN; 8],
        }
    }
    pub fn from_frozen(value: FrozenCableNetwork, vm: Rc<VM>) -> Self {
        CableNetwork {
            id: value.id,
            prefab: Name::new(""),
            name: Name::new(""),
            vm,
            devices: value.devices.into_iter().collect(),
            power_only: value.power_only.into_iter().collect(),
            channels: value.channels,
        }
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
