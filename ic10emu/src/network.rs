use std::{collections::HashSet, ops::Deref};

use crate::vm::{
    enums::script_enums::LogicType,
    object::{errors::LogicError, macros::ObjectInterface, traits::*, Name, ObjectID},
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
        net: Option<u32>,
        typ: CableConnectionType,
    },
    #[default]
    Other,
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

#[derive(ObjectInterface!, Debug, Serialize, Deserialize)]
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
        index: usize,
    ) -> bool {
        false
    }
    fn get_slot_logic(
        &self,
        slt: crate::vm::enums::script_enums::LogicSlotType,
        index: usize,
        vm: &crate::vm::VM,
    ) -> Result<f64, LogicError> {
        Err(LogicError::CantSlotRead(slt, index))
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrozenNetwork {
    pub id: u32,
    pub devices: Vec<u32>,
    pub power_only: Vec<u32>,
    pub channels: [f64; 8],
}

impl<T> From<T> for FrozenNetwork
where
    T: Deref<Target = CableNetwork>,
{
    fn from(value: T) -> Self {
        FrozenNetwork {
            id: value.id,
            devices: value.devices.iter().copied().collect_vec(),
            power_only: value.power_only.iter().copied().collect_vec(),
            channels: value.channels,
        }
    }
}

impl From<FrozenNetwork> for CableNetwork {
    fn from(value: FrozenNetwork) -> Self {
        CableNetwork {
            id: value.id,
            prefab: Name::new(""),
            name: Name::new(""),
            devices: value.devices.into_iter().collect(),
            power_only: value.power_only.into_iter().collect(),
            channels: value.channels,
        }
    }
}

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("")]
    ChannelIndexOutOfRange,
}

impl CableNetwork {
    pub fn new(id: u32) -> Self {
        CableNetwork {
            id,
            prefab: Name::new(""),
            name: Name::new(""),
            devices: HashSet::new(),
            power_only: HashSet::new(),
            channels: [f64::NAN; 8],
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
