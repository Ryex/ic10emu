use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumIter};
use thiserror::Error;

use itertools::Itertools;

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
    Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter, AsRefStr,
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
    Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter, AsRefStr,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    pub devices: HashSet<u32>,
    pub power_only: HashSet<u32>,
    pub channels: [f64; 8],
}

impl Default for Network {
    fn default() -> Self {
        Network {
            devices: HashSet::new(),
            power_only: HashSet::new(),
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
    pub fn contains(&self, id: &u32) -> bool {
        self.devices.contains(id) || self.power_only.contains(id)
    }

    pub fn contains_all(&self, ids: &[u32]) -> bool {
        ids.iter().all(|id| self.contains(id))
    }

    pub fn contains_data(&self, id: &u32) -> bool {
        self.devices.contains(id)
    }

    pub fn contains_all_data(&self, ids: &[u32]) -> bool {
        ids.iter().all(|id| self.contains_data(id))
    }

    pub fn contains_power(&self, id: &u32) -> bool {
        self.power_only.contains(id)
    }

    pub fn contains_all_power(&self, ids: &[u32]) -> bool {
        ids.iter().all(|id| self.contains_power(id))
    }

    pub fn data_visible(&self, source: &u32) -> Vec<u32> {
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

    pub fn add_data(&mut self, id: u32) -> bool {
        self.devices.insert(id)
    }

    pub fn add_power(&mut self, id: u32) -> bool {
        self.power_only.insert(id)
    }

    pub fn remove_all(&mut self, id: u32) -> bool {
        self.devices.remove(&id) || self.power_only.remove(&id)
    }
    pub fn remove_data(&mut self, id: u32) -> bool {
        self.devices.remove(&id)
    }

    pub fn remove_power(&mut self, id: u32) -> bool {
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
