use serde_derive::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumProperty, EnumString, FromRepr};
#[cfg(feature = "tsify")]
use tsify::Tsify;
#[cfg(feature = "tsify")]
use wasm_bindgen::prelude::*;
use super::script::{LogicSlotType, LogicType};
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(u8)]
pub enum AirConditioningMode {
    #[strum(serialize = "Cold")]
    #[strum(props(docs = "", value = "0"))]
    Cold = 0u8,
    #[strum(serialize = "Hot")]
    #[strum(props(docs = "", value = "1"))]
    Hot = 1u8,
}
impl TryFrom<f64> for AirConditioningMode {
    type Error = super::ParseError;
    fn try_from(
        value: f64,
    ) -> Result<Self, <AirConditioningMode as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = AirConditioningMode::iter()
            .find(|enm| (f64::from(*enm as u8) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
#[derive(
    Default,
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(u8)]
pub enum AirControlMode {
    #[strum(serialize = "None")]
    #[strum(props(docs = "", value = "0"))]
    #[default]
    None = 0u8,
    #[strum(serialize = "Offline")]
    #[strum(props(docs = "", value = "1"))]
    Offline = 1u8,
    #[strum(serialize = "Pressure")]
    #[strum(props(docs = "", value = "2"))]
    Pressure = 2u8,
    #[strum(serialize = "Draught")]
    #[strum(props(docs = "", value = "4"))]
    Draught = 4u8,
}
impl TryFrom<f64> for AirControlMode {
    type Error = super::ParseError;
    fn try_from(value: f64) -> Result<Self, <AirControlMode as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = AirControlMode::iter()
            .find(|enm| (f64::from(*enm as u8) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(u8)]
pub enum ColorType {
    #[strum(serialize = "Blue")]
    #[strum(props(docs = "", value = "0"))]
    Blue = 0u8,
    #[strum(serialize = "Gray")]
    #[strum(props(docs = "", value = "1"))]
    Gray = 1u8,
    #[strum(serialize = "Green")]
    #[strum(props(docs = "", value = "2"))]
    Green = 2u8,
    #[strum(serialize = "Orange")]
    #[strum(props(docs = "", value = "3"))]
    Orange = 3u8,
    #[strum(serialize = "Red")]
    #[strum(props(docs = "", value = "4"))]
    Red = 4u8,
    #[strum(serialize = "Yellow")]
    #[strum(props(docs = "", value = "5"))]
    Yellow = 5u8,
    #[strum(serialize = "White")]
    #[strum(props(docs = "", value = "6"))]
    White = 6u8,
    #[strum(serialize = "Black")]
    #[strum(props(docs = "", value = "7"))]
    Black = 7u8,
    #[strum(serialize = "Brown")]
    #[strum(props(docs = "", value = "8"))]
    Brown = 8u8,
    #[strum(serialize = "Khaki")]
    #[strum(props(docs = "", value = "9"))]
    Khaki = 9u8,
    #[strum(serialize = "Pink")]
    #[strum(props(docs = "", value = "10"))]
    Pink = 10u8,
    #[strum(serialize = "Purple")]
    #[strum(props(docs = "", value = "11"))]
    Purple = 11u8,
}
impl TryFrom<f64> for ColorType {
    type Error = super::ParseError;
    fn try_from(value: f64) -> Result<Self, <ColorType as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = ColorType::iter()
            .find(|enm| (f64::from(*enm as u8) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
#[derive(
    Default,
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(u8)]
pub enum DaylightSensorMode {
    #[strum(serialize = "Default")]
    #[strum(props(docs = "", value = "0"))]
    #[default]
    Default = 0u8,
    #[strum(serialize = "Horizontal")]
    #[strum(props(docs = "", value = "1"))]
    Horizontal = 1u8,
    #[strum(serialize = "Vertical")]
    #[strum(props(docs = "", value = "2"))]
    Vertical = 2u8,
}
impl TryFrom<f64> for DaylightSensorMode {
    type Error = super::ParseError;
    fn try_from(
        value: f64,
    ) -> Result<Self, <DaylightSensorMode as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = DaylightSensorMode::iter()
            .find(|enm| (f64::from(*enm as u8) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(u8)]
pub enum ElevatorMode {
    #[strum(serialize = "Stationary")]
    #[strum(props(docs = "", value = "0"))]
    Stationary = 0u8,
    #[strum(serialize = "Upward")]
    #[strum(props(docs = "", value = "1"))]
    Upward = 1u8,
    #[strum(serialize = "Downward")]
    #[strum(props(docs = "", value = "2"))]
    Downward = 2u8,
}
impl TryFrom<f64> for ElevatorMode {
    type Error = super::ParseError;
    fn try_from(value: f64) -> Result<Self, <ElevatorMode as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = ElevatorMode::iter()
            .find(|enm| (f64::from(*enm as u8) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(u8)]
pub enum EntityState {
    #[strum(serialize = "Alive")]
    #[strum(props(docs = "", value = "0"))]
    Alive = 0u8,
    #[strum(serialize = "Dead")]
    #[strum(props(docs = "", value = "1"))]
    Dead = 1u8,
    #[strum(serialize = "Unconscious")]
    #[strum(props(docs = "", value = "2"))]
    Unconscious = 2u8,
    #[strum(serialize = "Decay")]
    #[strum(props(docs = "", value = "3"))]
    Decay = 3u8,
}
impl TryFrom<f64> for EntityState {
    type Error = super::ParseError;
    fn try_from(value: f64) -> Result<Self, <EntityState as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = EntityState::iter()
            .find(|enm| (f64::from(*enm as u8) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(u32)]
pub enum GasType {
    #[strum(serialize = "Undefined")]
    #[strum(props(docs = "", value = "0"))]
    Undefined = 0u32,
    #[strum(serialize = "Oxygen")]
    #[strum(props(docs = "", value = "1"))]
    Oxygen = 1u32,
    #[strum(serialize = "Nitrogen")]
    #[strum(props(docs = "", value = "2"))]
    Nitrogen = 2u32,
    #[strum(serialize = "CarbonDioxide")]
    #[strum(props(docs = "", value = "4"))]
    CarbonDioxide = 4u32,
    #[strum(serialize = "Volatiles")]
    #[strum(props(docs = "", value = "8"))]
    Volatiles = 8u32,
    #[strum(serialize = "Pollutant")]
    #[strum(props(docs = "", value = "16"))]
    Pollutant = 16u32,
    #[strum(serialize = "Water")]
    #[strum(props(docs = "", value = "32"))]
    Water = 32u32,
    #[strum(serialize = "NitrousOxide")]
    #[strum(props(docs = "", value = "64"))]
    NitrousOxide = 64u32,
    #[strum(serialize = "LiquidNitrogen")]
    #[strum(props(docs = "", value = "128"))]
    LiquidNitrogen = 128u32,
    #[strum(serialize = "LiquidOxygen")]
    #[strum(props(docs = "", value = "256"))]
    LiquidOxygen = 256u32,
    #[strum(serialize = "LiquidVolatiles")]
    #[strum(props(docs = "", value = "512"))]
    LiquidVolatiles = 512u32,
    #[strum(serialize = "Steam")]
    #[strum(props(docs = "", value = "1024"))]
    Steam = 1024u32,
    #[strum(serialize = "LiquidCarbonDioxide")]
    #[strum(props(docs = "", value = "2048"))]
    LiquidCarbonDioxide = 2048u32,
    #[strum(serialize = "LiquidPollutant")]
    #[strum(props(docs = "", value = "4096"))]
    LiquidPollutant = 4096u32,
    #[strum(serialize = "LiquidNitrousOxide")]
    #[strum(props(docs = "", value = "8192"))]
    LiquidNitrousOxide = 8192u32,
    #[strum(serialize = "Hydrogen")]
    #[strum(props(docs = "", value = "16384"))]
    Hydrogen = 16384u32,
    #[strum(serialize = "LiquidHydrogen")]
    #[strum(props(docs = "", value = "32768"))]
    LiquidHydrogen = 32768u32,
    #[strum(serialize = "PollutedWater")]
    #[strum(props(docs = "", value = "65536"))]
    PollutedWater = 65536u32,
}
impl TryFrom<f64> for GasType {
    type Error = super::ParseError;
    fn try_from(value: f64) -> Result<Self, <GasType as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = GasType::iter()
            .find(|enm| (f64::from(*enm as u32) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(u8)]
pub enum PowerMode {
    #[strum(serialize = "Idle")]
    #[strum(props(docs = "", value = "0"))]
    Idle = 0u8,
    #[strum(serialize = "Discharged")]
    #[strum(props(docs = "", value = "1"))]
    Discharged = 1u8,
    #[strum(serialize = "Discharging")]
    #[strum(props(docs = "", value = "2"))]
    Discharging = 2u8,
    #[strum(serialize = "Charging")]
    #[strum(props(docs = "", value = "3"))]
    Charging = 3u8,
    #[strum(serialize = "Charged")]
    #[strum(props(docs = "", value = "4"))]
    Charged = 4u8,
}
impl TryFrom<f64> for PowerMode {
    type Error = super::ParseError;
    fn try_from(value: f64) -> Result<Self, <PowerMode as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = PowerMode::iter()
            .find(|enm| (f64::from(*enm as u8) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
#[derive(
    Default,
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(u8)]
pub enum PrinterInstruction {
    #[strum(serialize = "None")]
    #[strum(props(docs = "", value = "0"))]
    #[default]
    None = 0u8,
    #[strum(serialize = "StackPointer")]
    #[strum(props(docs = "", value = "1"))]
    StackPointer = 1u8,
    #[strum(serialize = "ExecuteRecipe")]
    #[strum(props(docs = "", value = "2"))]
    ExecuteRecipe = 2u8,
    #[strum(serialize = "WaitUntilNextValid")]
    #[strum(props(docs = "", value = "3"))]
    WaitUntilNextValid = 3u8,
    #[strum(serialize = "JumpIfNextInvalid")]
    #[strum(props(docs = "", value = "4"))]
    JumpIfNextInvalid = 4u8,
    #[strum(serialize = "JumpToAddress")]
    #[strum(props(docs = "", value = "5"))]
    JumpToAddress = 5u8,
    #[strum(serialize = "DeviceSetLock")]
    #[strum(props(docs = "", value = "6"))]
    DeviceSetLock = 6u8,
    #[strum(serialize = "EjectReagent")]
    #[strum(props(docs = "", value = "7"))]
    EjectReagent = 7u8,
    #[strum(serialize = "EjectAllReagents")]
    #[strum(props(docs = "", value = "8"))]
    EjectAllReagents = 8u8,
    #[strum(serialize = "MissingRecipeReagent")]
    #[strum(props(docs = "", value = "9"))]
    MissingRecipeReagent = 9u8,
}
impl TryFrom<f64> for PrinterInstruction {
    type Error = super::ParseError;
    fn try_from(
        value: f64,
    ) -> Result<Self, <PrinterInstruction as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = PrinterInstruction::iter()
            .find(|enm| (f64::from(*enm as u8) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
#[derive(
    Default,
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(u8)]
pub enum ReEntryProfile {
    #[strum(serialize = "None")]
    #[strum(props(docs = "", value = "0"))]
    #[default]
    None = 0u8,
    #[strum(serialize = "Optimal")]
    #[strum(props(docs = "", value = "1"))]
    Optimal = 1u8,
    #[strum(serialize = "Medium")]
    #[strum(props(docs = "", value = "2"))]
    Medium = 2u8,
    #[strum(serialize = "High")]
    #[strum(props(docs = "", value = "3"))]
    High = 3u8,
    #[strum(serialize = "Max")]
    #[strum(props(docs = "", value = "4"))]
    Max = 4u8,
}
impl TryFrom<f64> for ReEntryProfile {
    type Error = super::ParseError;
    fn try_from(value: f64) -> Result<Self, <ReEntryProfile as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = ReEntryProfile::iter()
            .find(|enm| (f64::from(*enm as u8) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
#[derive(
    Default,
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(u8)]
pub enum RobotMode {
    #[strum(serialize = "None")]
    #[strum(props(docs = "", value = "0"))]
    #[default]
    None = 0u8,
    #[strum(serialize = "Follow")]
    #[strum(props(docs = "", value = "1"))]
    Follow = 1u8,
    #[strum(serialize = "MoveToTarget")]
    #[strum(props(docs = "", value = "2"))]
    MoveToTarget = 2u8,
    #[strum(serialize = "Roam")]
    #[strum(props(docs = "", value = "3"))]
    Roam = 3u8,
    #[strum(serialize = "Unload")]
    #[strum(props(docs = "", value = "4"))]
    Unload = 4u8,
    #[strum(serialize = "PathToTarget")]
    #[strum(props(docs = "", value = "5"))]
    PathToTarget = 5u8,
    #[strum(serialize = "StorageFull")]
    #[strum(props(docs = "", value = "6"))]
    StorageFull = 6u8,
}
impl TryFrom<f64> for RobotMode {
    type Error = super::ParseError;
    fn try_from(value: f64) -> Result<Self, <RobotMode as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = RobotMode::iter()
            .find(|enm| (f64::from(*enm as u8) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
#[derive(
    Default,
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(u8)]
pub enum RocketMode {
    #[strum(serialize = "Invalid")]
    #[strum(props(docs = "", value = "0"))]
    Invalid = 0u8,
    #[strum(serialize = "None")]
    #[strum(props(docs = "", value = "1"))]
    #[default]
    None = 1u8,
    #[strum(serialize = "Mine")]
    #[strum(props(docs = "", value = "2"))]
    Mine = 2u8,
    #[strum(serialize = "Survey")]
    #[strum(props(docs = "", value = "3"))]
    Survey = 3u8,
    #[strum(serialize = "Discover")]
    #[strum(props(docs = "", value = "4"))]
    Discover = 4u8,
    #[strum(serialize = "Chart")]
    #[strum(props(docs = "", value = "5"))]
    Chart = 5u8,
}
impl TryFrom<f64> for RocketMode {
    type Error = super::ParseError;
    fn try_from(value: f64) -> Result<Self, <RocketMode as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = RocketMode::iter()
            .find(|enm| (f64::from(*enm as u8) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
#[derive(
    Default,
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(u8)]
pub enum Class {
    #[strum(serialize = "None")]
    #[strum(props(docs = "", value = "0"))]
    #[default]
    None = 0u8,
    #[strum(serialize = "Helmet")]
    #[strum(props(docs = "", value = "1"))]
    Helmet = 1u8,
    #[strum(serialize = "Suit")]
    #[strum(props(docs = "", value = "2"))]
    Suit = 2u8,
    #[strum(serialize = "Back")]
    #[strum(props(docs = "", value = "3"))]
    Back = 3u8,
    #[strum(serialize = "GasFilter")]
    #[strum(props(docs = "", value = "4"))]
    GasFilter = 4u8,
    #[strum(serialize = "GasCanister")]
    #[strum(props(docs = "", value = "5"))]
    GasCanister = 5u8,
    #[strum(serialize = "Motherboard")]
    #[strum(props(docs = "", value = "6"))]
    Motherboard = 6u8,
    #[strum(serialize = "Circuitboard")]
    #[strum(props(docs = "", value = "7"))]
    Circuitboard = 7u8,
    #[strum(serialize = "DataDisk")]
    #[strum(props(docs = "", value = "8"))]
    DataDisk = 8u8,
    #[strum(serialize = "Organ")]
    #[strum(props(docs = "", value = "9"))]
    Organ = 9u8,
    #[strum(serialize = "Ore")]
    #[strum(props(docs = "", value = "10"))]
    Ore = 10u8,
    #[strum(serialize = "Plant")]
    #[strum(props(docs = "", value = "11"))]
    Plant = 11u8,
    #[strum(serialize = "Uniform")]
    #[strum(props(docs = "", value = "12"))]
    Uniform = 12u8,
    #[strum(serialize = "Entity")]
    #[strum(props(docs = "", value = "13"))]
    Entity = 13u8,
    #[strum(serialize = "Battery")]
    #[strum(props(docs = "", value = "14"))]
    Battery = 14u8,
    #[strum(serialize = "Egg")]
    #[strum(props(docs = "", value = "15"))]
    Egg = 15u8,
    #[strum(serialize = "Belt")]
    #[strum(props(docs = "", value = "16"))]
    Belt = 16u8,
    #[strum(serialize = "Tool")]
    #[strum(props(docs = "", value = "17"))]
    Tool = 17u8,
    #[strum(serialize = "Appliance")]
    #[strum(props(docs = "", value = "18"))]
    Appliance = 18u8,
    #[strum(serialize = "Ingot")]
    #[strum(props(docs = "", value = "19"))]
    Ingot = 19u8,
    #[strum(serialize = "Torpedo")]
    #[strum(props(docs = "", value = "20"))]
    Torpedo = 20u8,
    #[strum(serialize = "Cartridge")]
    #[strum(props(docs = "", value = "21"))]
    Cartridge = 21u8,
    #[strum(serialize = "AccessCard")]
    #[strum(props(docs = "", value = "22"))]
    AccessCard = 22u8,
    #[strum(serialize = "Magazine")]
    #[strum(props(docs = "", value = "23"))]
    Magazine = 23u8,
    #[strum(serialize = "Circuit")]
    #[strum(props(docs = "", value = "24"))]
    Circuit = 24u8,
    #[strum(serialize = "Bottle")]
    #[strum(props(docs = "", value = "25"))]
    Bottle = 25u8,
    #[strum(serialize = "ProgrammableChip")]
    #[strum(props(docs = "", value = "26"))]
    ProgrammableChip = 26u8,
    #[strum(serialize = "Glasses")]
    #[strum(props(docs = "", value = "27"))]
    Glasses = 27u8,
    #[strum(serialize = "CreditCard")]
    #[strum(props(docs = "", value = "28"))]
    CreditCard = 28u8,
    #[strum(serialize = "DirtCanister")]
    #[strum(props(docs = "", value = "29"))]
    DirtCanister = 29u8,
    #[strum(serialize = "SensorProcessingUnit")]
    #[strum(props(docs = "", value = "30"))]
    SensorProcessingUnit = 30u8,
    #[strum(serialize = "LiquidCanister")]
    #[strum(props(docs = "", value = "31"))]
    LiquidCanister = 31u8,
    #[strum(serialize = "LiquidBottle")]
    #[strum(props(docs = "", value = "32"))]
    LiquidBottle = 32u8,
    #[strum(serialize = "Wreckage")]
    #[strum(props(docs = "", value = "33"))]
    Wreckage = 33u8,
    #[strum(serialize = "SoundCartridge")]
    #[strum(props(docs = "", value = "34"))]
    SoundCartridge = 34u8,
    #[strum(serialize = "DrillHead")]
    #[strum(props(docs = "", value = "35"))]
    DrillHead = 35u8,
    #[strum(serialize = "ScanningHead")]
    #[strum(props(docs = "", value = "36"))]
    ScanningHead = 36u8,
    #[strum(serialize = "Flare")]
    #[strum(props(docs = "", value = "37"))]
    Flare = 37u8,
    #[strum(serialize = "Blocked")]
    #[strum(props(docs = "", value = "38"))]
    Blocked = 38u8,
    #[strum(serialize = "SuitMod")]
    #[strum(props(docs = "", value = "39"))]
    SuitMod = 39u8,
}
impl TryFrom<f64> for Class {
    type Error = super::ParseError;
    fn try_from(value: f64) -> Result<Self, <Class as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = Class::iter()
            .find(|enm| (f64::from(*enm as u8) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
#[derive(
    Default,
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(u8)]
pub enum SorterInstruction {
    #[strum(serialize = "None")]
    #[strum(props(docs = "", value = "0"))]
    #[default]
    None = 0u8,
    #[strum(serialize = "FilterPrefabHashEquals")]
    #[strum(props(docs = "", value = "1"))]
    FilterPrefabHashEquals = 1u8,
    #[strum(serialize = "FilterPrefabHashNotEquals")]
    #[strum(props(docs = "", value = "2"))]
    FilterPrefabHashNotEquals = 2u8,
    #[strum(serialize = "FilterSortingClassCompare")]
    #[strum(props(docs = "", value = "3"))]
    FilterSortingClassCompare = 3u8,
    #[strum(serialize = "FilterSlotTypeCompare")]
    #[strum(props(docs = "", value = "4"))]
    FilterSlotTypeCompare = 4u8,
    #[strum(serialize = "FilterQuantityCompare")]
    #[strum(props(docs = "", value = "5"))]
    FilterQuantityCompare = 5u8,
    #[strum(serialize = "LimitNextExecutionByCount")]
    #[strum(props(docs = "", value = "6"))]
    LimitNextExecutionByCount = 6u8,
}
impl TryFrom<f64> for SorterInstruction {
    type Error = super::ParseError;
    fn try_from(value: f64) -> Result<Self, <SorterInstruction as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = SorterInstruction::iter()
            .find(|enm| (f64::from(*enm as u8) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
#[derive(
    Default,
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(u8)]
pub enum SortingClass {
    #[strum(serialize = "Default")]
    #[strum(props(docs = "", value = "0"))]
    #[default]
    Default = 0u8,
    #[strum(serialize = "Kits")]
    #[strum(props(docs = "", value = "1"))]
    Kits = 1u8,
    #[strum(serialize = "Tools")]
    #[strum(props(docs = "", value = "2"))]
    Tools = 2u8,
    #[strum(serialize = "Resources")]
    #[strum(props(docs = "", value = "3"))]
    Resources = 3u8,
    #[strum(serialize = "Food")]
    #[strum(props(docs = "", value = "4"))]
    Food = 4u8,
    #[strum(serialize = "Clothing")]
    #[strum(props(docs = "", value = "5"))]
    Clothing = 5u8,
    #[strum(serialize = "Appliances")]
    #[strum(props(docs = "", value = "6"))]
    Appliances = 6u8,
    #[strum(serialize = "Atmospherics")]
    #[strum(props(docs = "", value = "7"))]
    Atmospherics = 7u8,
    #[strum(serialize = "Storage")]
    #[strum(props(docs = "", value = "8"))]
    Storage = 8u8,
    #[strum(serialize = "Ores")]
    #[strum(props(docs = "", value = "9"))]
    Ores = 9u8,
    #[strum(serialize = "Ices")]
    #[strum(props(docs = "", value = "10"))]
    Ices = 10u8,
}
impl TryFrom<f64> for SortingClass {
    type Error = super::ParseError;
    fn try_from(value: f64) -> Result<Self, <SortingClass as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = SortingClass::iter()
            .find(|enm| (f64::from(*enm as u8) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
#[derive(
    Default,
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(u8)]
pub enum SoundAlert {
    #[strum(serialize = "None")]
    #[strum(props(docs = "", value = "0"))]
    #[default]
    None = 0u8,
    #[strum(serialize = "Alarm2")]
    #[strum(props(docs = "", value = "1"))]
    Alarm2 = 1u8,
    #[strum(serialize = "Alarm3")]
    #[strum(props(docs = "", value = "2"))]
    Alarm3 = 2u8,
    #[strum(serialize = "Alarm4")]
    #[strum(props(docs = "", value = "3"))]
    Alarm4 = 3u8,
    #[strum(serialize = "Alarm5")]
    #[strum(props(docs = "", value = "4"))]
    Alarm5 = 4u8,
    #[strum(serialize = "Alarm6")]
    #[strum(props(docs = "", value = "5"))]
    Alarm6 = 5u8,
    #[strum(serialize = "Alarm7")]
    #[strum(props(docs = "", value = "6"))]
    Alarm7 = 6u8,
    #[strum(serialize = "Music1")]
    #[strum(props(docs = "", value = "7"))]
    Music1 = 7u8,
    #[strum(serialize = "Music2")]
    #[strum(props(docs = "", value = "8"))]
    Music2 = 8u8,
    #[strum(serialize = "Music3")]
    #[strum(props(docs = "", value = "9"))]
    Music3 = 9u8,
    #[strum(serialize = "Alarm8")]
    #[strum(props(docs = "", value = "10"))]
    Alarm8 = 10u8,
    #[strum(serialize = "Alarm9")]
    #[strum(props(docs = "", value = "11"))]
    Alarm9 = 11u8,
    #[strum(serialize = "Alarm10")]
    #[strum(props(docs = "", value = "12"))]
    Alarm10 = 12u8,
    #[strum(serialize = "Alarm11")]
    #[strum(props(docs = "", value = "13"))]
    Alarm11 = 13u8,
    #[strum(serialize = "Alarm12")]
    #[strum(props(docs = "", value = "14"))]
    Alarm12 = 14u8,
    #[strum(serialize = "Danger")]
    #[strum(props(docs = "", value = "15"))]
    Danger = 15u8,
    #[strum(serialize = "Warning")]
    #[strum(props(docs = "", value = "16"))]
    Warning = 16u8,
    #[strum(serialize = "Alert")]
    #[strum(props(docs = "", value = "17"))]
    Alert = 17u8,
    #[strum(serialize = "StormIncoming")]
    #[strum(props(docs = "", value = "18"))]
    StormIncoming = 18u8,
    #[strum(serialize = "IntruderAlert")]
    #[strum(props(docs = "", value = "19"))]
    IntruderAlert = 19u8,
    #[strum(serialize = "Depressurising")]
    #[strum(props(docs = "", value = "20"))]
    Depressurising = 20u8,
    #[strum(serialize = "Pressurising")]
    #[strum(props(docs = "", value = "21"))]
    Pressurising = 21u8,
    #[strum(serialize = "AirlockCycling")]
    #[strum(props(docs = "", value = "22"))]
    AirlockCycling = 22u8,
    #[strum(serialize = "PowerLow")]
    #[strum(props(docs = "", value = "23"))]
    PowerLow = 23u8,
    #[strum(serialize = "SystemFailure")]
    #[strum(props(docs = "", value = "24"))]
    SystemFailure = 24u8,
    #[strum(serialize = "Welcome")]
    #[strum(props(docs = "", value = "25"))]
    Welcome = 25u8,
    #[strum(serialize = "MalfunctionDetected")]
    #[strum(props(docs = "", value = "26"))]
    MalfunctionDetected = 26u8,
    #[strum(serialize = "HaltWhoGoesThere")]
    #[strum(props(docs = "", value = "27"))]
    HaltWhoGoesThere = 27u8,
    #[strum(serialize = "FireFireFire")]
    #[strum(props(docs = "", value = "28"))]
    FireFireFire = 28u8,
    #[strum(serialize = "One")]
    #[strum(props(docs = "", value = "29"))]
    One = 29u8,
    #[strum(serialize = "Two")]
    #[strum(props(docs = "", value = "30"))]
    Two = 30u8,
    #[strum(serialize = "Three")]
    #[strum(props(docs = "", value = "31"))]
    Three = 31u8,
    #[strum(serialize = "Four")]
    #[strum(props(docs = "", value = "32"))]
    Four = 32u8,
    #[strum(serialize = "Five")]
    #[strum(props(docs = "", value = "33"))]
    Five = 33u8,
    #[strum(serialize = "Floor")]
    #[strum(props(docs = "", value = "34"))]
    Floor = 34u8,
    #[strum(serialize = "RocketLaunching")]
    #[strum(props(docs = "", value = "35"))]
    RocketLaunching = 35u8,
    #[strum(serialize = "LiftOff")]
    #[strum(props(docs = "", value = "36"))]
    LiftOff = 36u8,
    #[strum(serialize = "TraderIncoming")]
    #[strum(props(docs = "", value = "37"))]
    TraderIncoming = 37u8,
    #[strum(serialize = "TraderLanded")]
    #[strum(props(docs = "", value = "38"))]
    TraderLanded = 38u8,
    #[strum(serialize = "PressureHigh")]
    #[strum(props(docs = "", value = "39"))]
    PressureHigh = 39u8,
    #[strum(serialize = "PressureLow")]
    #[strum(props(docs = "", value = "40"))]
    PressureLow = 40u8,
    #[strum(serialize = "TemperatureHigh")]
    #[strum(props(docs = "", value = "41"))]
    TemperatureHigh = 41u8,
    #[strum(serialize = "TemperatureLow")]
    #[strum(props(docs = "", value = "42"))]
    TemperatureLow = 42u8,
    #[strum(serialize = "PollutantsDetected")]
    #[strum(props(docs = "", value = "43"))]
    PollutantsDetected = 43u8,
    #[strum(serialize = "HighCarbonDioxide")]
    #[strum(props(docs = "", value = "44"))]
    HighCarbonDioxide = 44u8,
    #[strum(serialize = "Alarm1")]
    #[strum(props(docs = "", value = "45"))]
    Alarm1 = 45u8,
}
impl TryFrom<f64> for SoundAlert {
    type Error = super::ParseError;
    fn try_from(value: f64) -> Result<Self, <SoundAlert as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = SoundAlert::iter()
            .find(|enm| (f64::from(*enm as u8) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(u8)]
pub enum LogicTransmitterMode {
    #[strum(serialize = "Passive")]
    #[strum(props(docs = "", value = "0"))]
    Passive = 0u8,
    #[strum(serialize = "Active")]
    #[strum(props(docs = "", value = "1"))]
    Active = 1u8,
}
impl TryFrom<f64> for LogicTransmitterMode {
    type Error = super::ParseError;
    fn try_from(
        value: f64,
    ) -> Result<Self, <LogicTransmitterMode as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = LogicTransmitterMode::iter()
            .find(|enm| (f64::from(*enm as u8) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(u8)]
pub enum VentDirection {
    #[strum(serialize = "Outward")]
    #[strum(props(docs = "", value = "0"))]
    Outward = 0u8,
    #[strum(serialize = "Inward")]
    #[strum(props(docs = "", value = "1"))]
    Inward = 1u8,
}
impl TryFrom<f64> for VentDirection {
    type Error = super::ParseError;
    fn try_from(value: f64) -> Result<Self, <VentDirection as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = VentDirection::iter()
            .find(|enm| (f64::from(*enm as u8) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(u8)]
pub enum ConditionOperation {
    #[strum(serialize = "Equals")]
    #[strum(props(docs = "", value = "0"))]
    Equals = 0u8,
    #[strum(serialize = "Greater")]
    #[strum(props(docs = "", value = "1"))]
    Greater = 1u8,
    #[strum(serialize = "Less")]
    #[strum(props(docs = "", value = "2"))]
    Less = 2u8,
    #[strum(serialize = "NotEquals")]
    #[strum(props(docs = "", value = "3"))]
    NotEquals = 3u8,
}
impl TryFrom<f64> for ConditionOperation {
    type Error = super::ParseError;
    fn try_from(
        value: f64,
    ) -> Result<Self, <ConditionOperation as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = ConditionOperation::iter()
            .find(|enm| (f64::from(*enm as u8) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
pub enum BasicEnum {
    AirCon(AirConditioningMode),
    AirControl(AirControlMode),
    Color(ColorType),
    DaylightSensorMode(DaylightSensorMode),
    ElevatorMode(ElevatorMode),
    EntityState(EntityState),
    GasType(GasType),
    LogicSlotType(LogicSlotType),
    LogicType(LogicType),
    PowerMode(PowerMode),
    PrinterInstruction(PrinterInstruction),
    ReEntryProfile(ReEntryProfile),
    RobotMode(RobotMode),
    RocketMode(RocketMode),
    SlotClass(Class),
    SorterInstruction(SorterInstruction),
    SortingClass(SortingClass),
    Sound(SoundAlert),
    TransmitterMode(LogicTransmitterMode),
    Vent(VentDirection),
    Unnamed(ConditionOperation),
}
impl BasicEnum {
    pub fn get_value(&self) -> u32 {
        match self {
            Self::AirCon(enm) => *enm as u32,
            Self::AirControl(enm) => *enm as u32,
            Self::Color(enm) => *enm as u32,
            Self::DaylightSensorMode(enm) => *enm as u32,
            Self::ElevatorMode(enm) => *enm as u32,
            Self::EntityState(enm) => *enm as u32,
            Self::GasType(enm) => *enm as u32,
            Self::LogicSlotType(enm) => *enm as u32,
            Self::LogicType(enm) => *enm as u32,
            Self::PowerMode(enm) => *enm as u32,
            Self::PrinterInstruction(enm) => *enm as u32,
            Self::ReEntryProfile(enm) => *enm as u32,
            Self::RobotMode(enm) => *enm as u32,
            Self::RocketMode(enm) => *enm as u32,
            Self::SlotClass(enm) => *enm as u32,
            Self::SorterInstruction(enm) => *enm as u32,
            Self::SortingClass(enm) => *enm as u32,
            Self::Sound(enm) => *enm as u32,
            Self::TransmitterMode(enm) => *enm as u32,
            Self::Vent(enm) => *enm as u32,
            Self::Unnamed(enm) => *enm as u32,
        }
    }
    pub fn get_str(&self, prop: &str) -> Option<&'static str> {
        match self {
            Self::AirCon(enm) => enm.get_str(prop),
            Self::AirControl(enm) => enm.get_str(prop),
            Self::Color(enm) => enm.get_str(prop),
            Self::DaylightSensorMode(enm) => enm.get_str(prop),
            Self::ElevatorMode(enm) => enm.get_str(prop),
            Self::EntityState(enm) => enm.get_str(prop),
            Self::GasType(enm) => enm.get_str(prop),
            Self::LogicSlotType(enm) => enm.get_str(prop),
            Self::LogicType(enm) => enm.get_str(prop),
            Self::PowerMode(enm) => enm.get_str(prop),
            Self::PrinterInstruction(enm) => enm.get_str(prop),
            Self::ReEntryProfile(enm) => enm.get_str(prop),
            Self::RobotMode(enm) => enm.get_str(prop),
            Self::RocketMode(enm) => enm.get_str(prop),
            Self::SlotClass(enm) => enm.get_str(prop),
            Self::SorterInstruction(enm) => enm.get_str(prop),
            Self::SortingClass(enm) => enm.get_str(prop),
            Self::Sound(enm) => enm.get_str(prop),
            Self::TransmitterMode(enm) => enm.get_str(prop),
            Self::Vent(enm) => enm.get_str(prop),
            Self::Unnamed(enm) => enm.get_str(prop),
        }
    }
    pub fn iter() -> impl std::iter::Iterator<Item = Self> {
        use strum::IntoEnumIterator;
        AirConditioningMode::iter()
            .map(Self::AirCon)
            .chain(AirControlMode::iter().map(Self::AirControl))
            .chain(ColorType::iter().map(Self::Color))
            .chain(DaylightSensorMode::iter().map(Self::DaylightSensorMode))
            .chain(ElevatorMode::iter().map(Self::ElevatorMode))
            .chain(EntityState::iter().map(Self::EntityState))
            .chain(GasType::iter().map(Self::GasType))
            .chain(LogicSlotType::iter().map(Self::LogicSlotType))
            .chain(LogicType::iter().map(Self::LogicType))
            .chain(PowerMode::iter().map(Self::PowerMode))
            .chain(PrinterInstruction::iter().map(Self::PrinterInstruction))
            .chain(ReEntryProfile::iter().map(Self::ReEntryProfile))
            .chain(RobotMode::iter().map(Self::RobotMode))
            .chain(RocketMode::iter().map(Self::RocketMode))
            .chain(Class::iter().map(Self::SlotClass))
            .chain(SorterInstruction::iter().map(Self::SorterInstruction))
            .chain(SortingClass::iter().map(Self::SortingClass))
            .chain(SoundAlert::iter().map(Self::Sound))
            .chain(LogicTransmitterMode::iter().map(Self::TransmitterMode))
            .chain(VentDirection::iter().map(Self::Vent))
            .chain(ConditionOperation::iter().map(Self::Unnamed))
    }
}
impl std::str::FromStr for BasicEnum {
    type Err = super::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "aircon.cold" => Ok(Self::AirCon(AirConditioningMode::Cold)),
            "aircon.hot" => Ok(Self::AirCon(AirConditioningMode::Hot)),
            "aircontrol.draught" => Ok(Self::AirControl(AirControlMode::Draught)),
            "aircontrol.none" => Ok(Self::AirControl(AirControlMode::None)),
            "aircontrol.offline" => Ok(Self::AirControl(AirControlMode::Offline)),
            "aircontrol.pressure" => Ok(Self::AirControl(AirControlMode::Pressure)),
            "color.black" => Ok(Self::Color(ColorType::Black)),
            "color.blue" => Ok(Self::Color(ColorType::Blue)),
            "color.brown" => Ok(Self::Color(ColorType::Brown)),
            "color.gray" => Ok(Self::Color(ColorType::Gray)),
            "color.green" => Ok(Self::Color(ColorType::Green)),
            "color.khaki" => Ok(Self::Color(ColorType::Khaki)),
            "color.orange" => Ok(Self::Color(ColorType::Orange)),
            "color.pink" => Ok(Self::Color(ColorType::Pink)),
            "color.purple" => Ok(Self::Color(ColorType::Purple)),
            "color.red" => Ok(Self::Color(ColorType::Red)),
            "color.white" => Ok(Self::Color(ColorType::White)),
            "color.yellow" => Ok(Self::Color(ColorType::Yellow)),
            "daylightsensormode.default" => {
                Ok(Self::DaylightSensorMode(DaylightSensorMode::Default))
            }
            "daylightsensormode.horizontal" => {
                Ok(Self::DaylightSensorMode(DaylightSensorMode::Horizontal))
            }
            "daylightsensormode.vertical" => {
                Ok(Self::DaylightSensorMode(DaylightSensorMode::Vertical))
            }
            "elevatormode.downward" => Ok(Self::ElevatorMode(ElevatorMode::Downward)),
            "elevatormode.stationary" => Ok(Self::ElevatorMode(ElevatorMode::Stationary)),
            "elevatormode.upward" => Ok(Self::ElevatorMode(ElevatorMode::Upward)),
            "entitystate.alive" => Ok(Self::EntityState(EntityState::Alive)),
            "entitystate.dead" => Ok(Self::EntityState(EntityState::Dead)),
            "entitystate.decay" => Ok(Self::EntityState(EntityState::Decay)),
            "entitystate.unconscious" => Ok(Self::EntityState(EntityState::Unconscious)),
            "gastype.carbondioxide" => Ok(Self::GasType(GasType::CarbonDioxide)),
            "gastype.hydrogen" => Ok(Self::GasType(GasType::Hydrogen)),
            "gastype.liquidcarbondioxide" => {
                Ok(Self::GasType(GasType::LiquidCarbonDioxide))
            }
            "gastype.liquidhydrogen" => Ok(Self::GasType(GasType::LiquidHydrogen)),
            "gastype.liquidnitrogen" => Ok(Self::GasType(GasType::LiquidNitrogen)),
            "gastype.liquidnitrousoxide" => {
                Ok(Self::GasType(GasType::LiquidNitrousOxide))
            }
            "gastype.liquidoxygen" => Ok(Self::GasType(GasType::LiquidOxygen)),
            "gastype.liquidpollutant" => Ok(Self::GasType(GasType::LiquidPollutant)),
            "gastype.liquidvolatiles" => Ok(Self::GasType(GasType::LiquidVolatiles)),
            "gastype.nitrogen" => Ok(Self::GasType(GasType::Nitrogen)),
            "gastype.nitrousoxide" => Ok(Self::GasType(GasType::NitrousOxide)),
            "gastype.oxygen" => Ok(Self::GasType(GasType::Oxygen)),
            "gastype.pollutant" => Ok(Self::GasType(GasType::Pollutant)),
            "gastype.pollutedwater" => Ok(Self::GasType(GasType::PollutedWater)),
            "gastype.steam" => Ok(Self::GasType(GasType::Steam)),
            "gastype.undefined" => Ok(Self::GasType(GasType::Undefined)),
            "gastype.volatiles" => Ok(Self::GasType(GasType::Volatiles)),
            "gastype.water" => Ok(Self::GasType(GasType::Water)),
            "logicslottype.charge" => Ok(Self::LogicSlotType(LogicSlotType::Charge)),
            "logicslottype.chargeratio" => {
                Ok(Self::LogicSlotType(LogicSlotType::ChargeRatio))
            }
            "logicslottype.class" => Ok(Self::LogicSlotType(LogicSlotType::Class)),
            "logicslottype.damage" => Ok(Self::LogicSlotType(LogicSlotType::Damage)),
            "logicslottype.efficiency" => {
                Ok(Self::LogicSlotType(LogicSlotType::Efficiency))
            }
            "logicslottype.filtertype" => {
                Ok(Self::LogicSlotType(LogicSlotType::FilterType))
            }
            "logicslottype.growth" => Ok(Self::LogicSlotType(LogicSlotType::Growth)),
            "logicslottype.health" => Ok(Self::LogicSlotType(LogicSlotType::Health)),
            "logicslottype.linenumber" => {
                Ok(Self::LogicSlotType(LogicSlotType::LineNumber))
            }
            "logicslottype.lock" => Ok(Self::LogicSlotType(LogicSlotType::Lock)),
            "logicslottype.mature" => Ok(Self::LogicSlotType(LogicSlotType::Mature)),
            "logicslottype.maxquantity" => {
                Ok(Self::LogicSlotType(LogicSlotType::MaxQuantity))
            }
            "logicslottype.none" => Ok(Self::LogicSlotType(LogicSlotType::None)),
            "logicslottype.occupanthash" => {
                Ok(Self::LogicSlotType(LogicSlotType::OccupantHash))
            }
            "logicslottype.occupied" => Ok(Self::LogicSlotType(LogicSlotType::Occupied)),
            "logicslottype.on" => Ok(Self::LogicSlotType(LogicSlotType::On)),
            "logicslottype.open" => Ok(Self::LogicSlotType(LogicSlotType::Open)),
            "logicslottype.prefabhash" => {
                Ok(Self::LogicSlotType(LogicSlotType::PrefabHash))
            }
            "logicslottype.pressure" => Ok(Self::LogicSlotType(LogicSlotType::Pressure)),
            "logicslottype.pressureair" => {
                Ok(Self::LogicSlotType(LogicSlotType::PressureAir))
            }
            "logicslottype.pressurewaste" => {
                Ok(Self::LogicSlotType(LogicSlotType::PressureWaste))
            }
            "logicslottype.quantity" => Ok(Self::LogicSlotType(LogicSlotType::Quantity)),
            "logicslottype.referenceid" => {
                Ok(Self::LogicSlotType(LogicSlotType::ReferenceId))
            }
            "logicslottype.seeding" => Ok(Self::LogicSlotType(LogicSlotType::Seeding)),
            "logicslottype.sortingclass" => {
                Ok(Self::LogicSlotType(LogicSlotType::SortingClass))
            }
            "logicslottype.temperature" => {
                Ok(Self::LogicSlotType(LogicSlotType::Temperature))
            }
            "logicslottype.volume" => Ok(Self::LogicSlotType(LogicSlotType::Volume)),
            "logictype.acceleration" => Ok(Self::LogicType(LogicType::Acceleration)),
            "logictype.activate" => Ok(Self::LogicType(LogicType::Activate)),
            "logictype.airrelease" => Ok(Self::LogicType(LogicType::AirRelease)),
            "logictype.alignmenterror" => Ok(Self::LogicType(LogicType::AlignmentError)),
            "logictype.apex" => Ok(Self::LogicType(LogicType::Apex)),
            "logictype.autoland" => Ok(Self::LogicType(LogicType::AutoLand)),
            "logictype.autoshutoff" => Ok(Self::LogicType(LogicType::AutoShutOff)),
            "logictype.bestcontactfilter" => {
                Ok(Self::LogicType(LogicType::BestContactFilter))
            }
            "logictype.bpm" => Ok(Self::LogicType(LogicType::Bpm)),
            "logictype.burntimeremaining" => {
                Ok(Self::LogicType(LogicType::BurnTimeRemaining))
            }
            "logictype.celestialhash" => Ok(Self::LogicType(LogicType::CelestialHash)),
            "logictype.celestialparenthash" => {
                Ok(Self::LogicType(LogicType::CelestialParentHash))
            }
            "logictype.channel0" => Ok(Self::LogicType(LogicType::Channel0)),
            "logictype.channel1" => Ok(Self::LogicType(LogicType::Channel1)),
            "logictype.channel2" => Ok(Self::LogicType(LogicType::Channel2)),
            "logictype.channel3" => Ok(Self::LogicType(LogicType::Channel3)),
            "logictype.channel4" => Ok(Self::LogicType(LogicType::Channel4)),
            "logictype.channel5" => Ok(Self::LogicType(LogicType::Channel5)),
            "logictype.channel6" => Ok(Self::LogicType(LogicType::Channel6)),
            "logictype.channel7" => Ok(Self::LogicType(LogicType::Channel7)),
            "logictype.charge" => Ok(Self::LogicType(LogicType::Charge)),
            "logictype.chart" => Ok(Self::LogicType(LogicType::Chart)),
            "logictype.chartednavpoints" => {
                Ok(Self::LogicType(LogicType::ChartedNavPoints))
            }
            "logictype.clearmemory" => Ok(Self::LogicType(LogicType::ClearMemory)),
            "logictype.collectablegoods" => {
                Ok(Self::LogicType(LogicType::CollectableGoods))
            }
            "logictype.color" => Ok(Self::LogicType(LogicType::Color)),
            "logictype.combustion" => Ok(Self::LogicType(LogicType::Combustion)),
            "logictype.combustioninput" => {
                Ok(Self::LogicType(LogicType::CombustionInput))
            }
            "logictype.combustioninput2" => {
                Ok(Self::LogicType(LogicType::CombustionInput2))
            }
            "logictype.combustionlimiter" => {
                Ok(Self::LogicType(LogicType::CombustionLimiter))
            }
            "logictype.combustionoutput" => {
                Ok(Self::LogicType(LogicType::CombustionOutput))
            }
            "logictype.combustionoutput2" => {
                Ok(Self::LogicType(LogicType::CombustionOutput2))
            }
            "logictype.completionratio" => {
                Ok(Self::LogicType(LogicType::CompletionRatio))
            }
            "logictype.contacttypeid" => Ok(Self::LogicType(LogicType::ContactTypeId)),
            "logictype.currentcode" => Ok(Self::LogicType(LogicType::CurrentCode)),
            "logictype.currentresearchpodtype" => {
                Ok(Self::LogicType(LogicType::CurrentResearchPodType))
            }
            "logictype.density" => Ok(Self::LogicType(LogicType::Density)),
            "logictype.destinationcode" => {
                Ok(Self::LogicType(LogicType::DestinationCode))
            }
            "logictype.discover" => Ok(Self::LogicType(LogicType::Discover)),
            "logictype.distanceau" => Ok(Self::LogicType(LogicType::DistanceAu)),
            "logictype.distancekm" => Ok(Self::LogicType(LogicType::DistanceKm)),
            "logictype.drillcondition" => Ok(Self::LogicType(LogicType::DrillCondition)),
            "logictype.drymass" => Ok(Self::LogicType(LogicType::DryMass)),
            "logictype.eccentricity" => Ok(Self::LogicType(LogicType::Eccentricity)),
            "logictype.elevatorlevel" => Ok(Self::LogicType(LogicType::ElevatorLevel)),
            "logictype.elevatorspeed" => Ok(Self::LogicType(LogicType::ElevatorSpeed)),
            "logictype.entitystate" => Ok(Self::LogicType(LogicType::EntityState)),
            "logictype.environmentefficiency" => {
                Ok(Self::LogicType(LogicType::EnvironmentEfficiency))
            }
            "logictype.error" => Ok(Self::LogicType(LogicType::Error)),
            "logictype.exhaustvelocity" => {
                Ok(Self::LogicType(LogicType::ExhaustVelocity))
            }
            "logictype.exportcount" => Ok(Self::LogicType(LogicType::ExportCount)),
            "logictype.exportquantity" => Ok(Self::LogicType(LogicType::ExportQuantity)),
            "logictype.exportslothash" => Ok(Self::LogicType(LogicType::ExportSlotHash)),
            "logictype.exportslotoccupant" => {
                Ok(Self::LogicType(LogicType::ExportSlotOccupant))
            }
            "logictype.filtration" => Ok(Self::LogicType(LogicType::Filtration)),
            "logictype.flightcontrolrule" => {
                Ok(Self::LogicType(LogicType::FlightControlRule))
            }
            "logictype.flush" => Ok(Self::LogicType(LogicType::Flush)),
            "logictype.forcewrite" => Ok(Self::LogicType(LogicType::ForceWrite)),
            "logictype.forwardx" => Ok(Self::LogicType(LogicType::ForwardX)),
            "logictype.forwardy" => Ok(Self::LogicType(LogicType::ForwardY)),
            "logictype.forwardz" => Ok(Self::LogicType(LogicType::ForwardZ)),
            "logictype.fuel" => Ok(Self::LogicType(LogicType::Fuel)),
            "logictype.harvest" => Ok(Self::LogicType(LogicType::Harvest)),
            "logictype.horizontal" => Ok(Self::LogicType(LogicType::Horizontal)),
            "logictype.horizontalratio" => {
                Ok(Self::LogicType(LogicType::HorizontalRatio))
            }
            "logictype.idle" => Ok(Self::LogicType(LogicType::Idle)),
            "logictype.importcount" => Ok(Self::LogicType(LogicType::ImportCount)),
            "logictype.importquantity" => Ok(Self::LogicType(LogicType::ImportQuantity)),
            "logictype.importslothash" => Ok(Self::LogicType(LogicType::ImportSlotHash)),
            "logictype.importslotoccupant" => {
                Ok(Self::LogicType(LogicType::ImportSlotOccupant))
            }
            "logictype.inclination" => Ok(Self::LogicType(LogicType::Inclination)),
            "logictype.index" => Ok(Self::LogicType(LogicType::Index)),
            "logictype.interrogationprogress" => {
                Ok(Self::LogicType(LogicType::InterrogationProgress))
            }
            "logictype.linenumber" => Ok(Self::LogicType(LogicType::LineNumber)),
            "logictype.lock" => Ok(Self::LogicType(LogicType::Lock)),
            "logictype.manualresearchrequiredpod" => {
                Ok(Self::LogicType(LogicType::ManualResearchRequiredPod))
            }
            "logictype.mass" => Ok(Self::LogicType(LogicType::Mass)),
            "logictype.maximum" => Ok(Self::LogicType(LogicType::Maximum)),
            "logictype.mineablesinqueue" => {
                Ok(Self::LogicType(LogicType::MineablesInQueue))
            }
            "logictype.mineablesinvicinity" => {
                Ok(Self::LogicType(LogicType::MineablesInVicinity))
            }
            "logictype.minedquantity" => Ok(Self::LogicType(LogicType::MinedQuantity)),
            "logictype.minimumwattstocontact" => {
                Ok(Self::LogicType(LogicType::MinimumWattsToContact))
            }
            "logictype.mode" => Ok(Self::LogicType(LogicType::Mode)),
            "logictype.namehash" => Ok(Self::LogicType(LogicType::NameHash)),
            "logictype.navpoints" => Ok(Self::LogicType(LogicType::NavPoints)),
            "logictype.nextweathereventtime" => {
                Ok(Self::LogicType(LogicType::NextWeatherEventTime))
            }
            "logictype.none" => Ok(Self::LogicType(LogicType::None)),
            "logictype.on" => Ok(Self::LogicType(LogicType::On)),
            "logictype.open" => Ok(Self::LogicType(LogicType::Open)),
            "logictype.operationaltemperatureefficiency" => {
                Ok(Self::LogicType(LogicType::OperationalTemperatureEfficiency))
            }
            "logictype.orbitperiod" => Ok(Self::LogicType(LogicType::OrbitPeriod)),
            "logictype.orientation" => Ok(Self::LogicType(LogicType::Orientation)),
            "logictype.output" => Ok(Self::LogicType(LogicType::Output)),
            "logictype.passedmoles" => Ok(Self::LogicType(LogicType::PassedMoles)),
            "logictype.plant" => Ok(Self::LogicType(LogicType::Plant)),
            "logictype.plantefficiency1" => {
                Ok(Self::LogicType(LogicType::PlantEfficiency1))
            }
            "logictype.plantefficiency2" => {
                Ok(Self::LogicType(LogicType::PlantEfficiency2))
            }
            "logictype.plantefficiency3" => {
                Ok(Self::LogicType(LogicType::PlantEfficiency3))
            }
            "logictype.plantefficiency4" => {
                Ok(Self::LogicType(LogicType::PlantEfficiency4))
            }
            "logictype.plantgrowth1" => Ok(Self::LogicType(LogicType::PlantGrowth1)),
            "logictype.plantgrowth2" => Ok(Self::LogicType(LogicType::PlantGrowth2)),
            "logictype.plantgrowth3" => Ok(Self::LogicType(LogicType::PlantGrowth3)),
            "logictype.plantgrowth4" => Ok(Self::LogicType(LogicType::PlantGrowth4)),
            "logictype.planthash1" => Ok(Self::LogicType(LogicType::PlantHash1)),
            "logictype.planthash2" => Ok(Self::LogicType(LogicType::PlantHash2)),
            "logictype.planthash3" => Ok(Self::LogicType(LogicType::PlantHash3)),
            "logictype.planthash4" => Ok(Self::LogicType(LogicType::PlantHash4)),
            "logictype.planthealth1" => Ok(Self::LogicType(LogicType::PlantHealth1)),
            "logictype.planthealth2" => Ok(Self::LogicType(LogicType::PlantHealth2)),
            "logictype.planthealth3" => Ok(Self::LogicType(LogicType::PlantHealth3)),
            "logictype.planthealth4" => Ok(Self::LogicType(LogicType::PlantHealth4)),
            "logictype.positionx" => Ok(Self::LogicType(LogicType::PositionX)),
            "logictype.positiony" => Ok(Self::LogicType(LogicType::PositionY)),
            "logictype.positionz" => Ok(Self::LogicType(LogicType::PositionZ)),
            "logictype.power" => Ok(Self::LogicType(LogicType::Power)),
            "logictype.poweractual" => Ok(Self::LogicType(LogicType::PowerActual)),
            "logictype.powergeneration" => {
                Ok(Self::LogicType(LogicType::PowerGeneration))
            }
            "logictype.powerpotential" => Ok(Self::LogicType(LogicType::PowerPotential)),
            "logictype.powerrequired" => Ok(Self::LogicType(LogicType::PowerRequired)),
            "logictype.prefabhash" => Ok(Self::LogicType(LogicType::PrefabHash)),
            "logictype.pressure" => Ok(Self::LogicType(LogicType::Pressure)),
            "logictype.pressureefficiency" => {
                Ok(Self::LogicType(LogicType::PressureEfficiency))
            }
            "logictype.pressureexternal" => {
                Ok(Self::LogicType(LogicType::PressureExternal))
            }
            "logictype.pressureinput" => Ok(Self::LogicType(LogicType::PressureInput)),
            "logictype.pressureinput2" => Ok(Self::LogicType(LogicType::PressureInput2)),
            "logictype.pressureinternal" => {
                Ok(Self::LogicType(LogicType::PressureInternal))
            }
            "logictype.pressureoutput" => Ok(Self::LogicType(LogicType::PressureOutput)),
            "logictype.pressureoutput2" => {
                Ok(Self::LogicType(LogicType::PressureOutput2))
            }
            "logictype.pressuresetting" => {
                Ok(Self::LogicType(LogicType::PressureSetting))
            }
            "logictype.progress" => Ok(Self::LogicType(LogicType::Progress)),
            "logictype.quantity" => Ok(Self::LogicType(LogicType::Quantity)),
            "logictype.ratio" => Ok(Self::LogicType(LogicType::Ratio)),
            "logictype.ratiocarbondioxide" => {
                Ok(Self::LogicType(LogicType::RatioCarbonDioxide))
            }
            "logictype.ratiocarbondioxideinput" => {
                Ok(Self::LogicType(LogicType::RatioCarbonDioxideInput))
            }
            "logictype.ratiocarbondioxideinput2" => {
                Ok(Self::LogicType(LogicType::RatioCarbonDioxideInput2))
            }
            "logictype.ratiocarbondioxideoutput" => {
                Ok(Self::LogicType(LogicType::RatioCarbonDioxideOutput))
            }
            "logictype.ratiocarbondioxideoutput2" => {
                Ok(Self::LogicType(LogicType::RatioCarbonDioxideOutput2))
            }
            "logictype.ratiohydrogen" => Ok(Self::LogicType(LogicType::RatioHydrogen)),
            "logictype.ratioliquidcarbondioxide" => {
                Ok(Self::LogicType(LogicType::RatioLiquidCarbonDioxide))
            }
            "logictype.ratioliquidcarbondioxideinput" => {
                Ok(Self::LogicType(LogicType::RatioLiquidCarbonDioxideInput))
            }
            "logictype.ratioliquidcarbondioxideinput2" => {
                Ok(Self::LogicType(LogicType::RatioLiquidCarbonDioxideInput2))
            }
            "logictype.ratioliquidcarbondioxideoutput" => {
                Ok(Self::LogicType(LogicType::RatioLiquidCarbonDioxideOutput))
            }
            "logictype.ratioliquidcarbondioxideoutput2" => {
                Ok(Self::LogicType(LogicType::RatioLiquidCarbonDioxideOutput2))
            }
            "logictype.ratioliquidhydrogen" => {
                Ok(Self::LogicType(LogicType::RatioLiquidHydrogen))
            }
            "logictype.ratioliquidnitrogen" => {
                Ok(Self::LogicType(LogicType::RatioLiquidNitrogen))
            }
            "logictype.ratioliquidnitrogeninput" => {
                Ok(Self::LogicType(LogicType::RatioLiquidNitrogenInput))
            }
            "logictype.ratioliquidnitrogeninput2" => {
                Ok(Self::LogicType(LogicType::RatioLiquidNitrogenInput2))
            }
            "logictype.ratioliquidnitrogenoutput" => {
                Ok(Self::LogicType(LogicType::RatioLiquidNitrogenOutput))
            }
            "logictype.ratioliquidnitrogenoutput2" => {
                Ok(Self::LogicType(LogicType::RatioLiquidNitrogenOutput2))
            }
            "logictype.ratioliquidnitrousoxide" => {
                Ok(Self::LogicType(LogicType::RatioLiquidNitrousOxide))
            }
            "logictype.ratioliquidnitrousoxideinput" => {
                Ok(Self::LogicType(LogicType::RatioLiquidNitrousOxideInput))
            }
            "logictype.ratioliquidnitrousoxideinput2" => {
                Ok(Self::LogicType(LogicType::RatioLiquidNitrousOxideInput2))
            }
            "logictype.ratioliquidnitrousoxideoutput" => {
                Ok(Self::LogicType(LogicType::RatioLiquidNitrousOxideOutput))
            }
            "logictype.ratioliquidnitrousoxideoutput2" => {
                Ok(Self::LogicType(LogicType::RatioLiquidNitrousOxideOutput2))
            }
            "logictype.ratioliquidoxygen" => {
                Ok(Self::LogicType(LogicType::RatioLiquidOxygen))
            }
            "logictype.ratioliquidoxygeninput" => {
                Ok(Self::LogicType(LogicType::RatioLiquidOxygenInput))
            }
            "logictype.ratioliquidoxygeninput2" => {
                Ok(Self::LogicType(LogicType::RatioLiquidOxygenInput2))
            }
            "logictype.ratioliquidoxygenoutput" => {
                Ok(Self::LogicType(LogicType::RatioLiquidOxygenOutput))
            }
            "logictype.ratioliquidoxygenoutput2" => {
                Ok(Self::LogicType(LogicType::RatioLiquidOxygenOutput2))
            }
            "logictype.ratioliquidpollutant" => {
                Ok(Self::LogicType(LogicType::RatioLiquidPollutant))
            }
            "logictype.ratioliquidpollutantinput" => {
                Ok(Self::LogicType(LogicType::RatioLiquidPollutantInput))
            }
            "logictype.ratioliquidpollutantinput2" => {
                Ok(Self::LogicType(LogicType::RatioLiquidPollutantInput2))
            }
            "logictype.ratioliquidpollutantoutput" => {
                Ok(Self::LogicType(LogicType::RatioLiquidPollutantOutput))
            }
            "logictype.ratioliquidpollutantoutput2" => {
                Ok(Self::LogicType(LogicType::RatioLiquidPollutantOutput2))
            }
            "logictype.ratioliquidvolatiles" => {
                Ok(Self::LogicType(LogicType::RatioLiquidVolatiles))
            }
            "logictype.ratioliquidvolatilesinput" => {
                Ok(Self::LogicType(LogicType::RatioLiquidVolatilesInput))
            }
            "logictype.ratioliquidvolatilesinput2" => {
                Ok(Self::LogicType(LogicType::RatioLiquidVolatilesInput2))
            }
            "logictype.ratioliquidvolatilesoutput" => {
                Ok(Self::LogicType(LogicType::RatioLiquidVolatilesOutput))
            }
            "logictype.ratioliquidvolatilesoutput2" => {
                Ok(Self::LogicType(LogicType::RatioLiquidVolatilesOutput2))
            }
            "logictype.rationitrogen" => Ok(Self::LogicType(LogicType::RatioNitrogen)),
            "logictype.rationitrogeninput" => {
                Ok(Self::LogicType(LogicType::RatioNitrogenInput))
            }
            "logictype.rationitrogeninput2" => {
                Ok(Self::LogicType(LogicType::RatioNitrogenInput2))
            }
            "logictype.rationitrogenoutput" => {
                Ok(Self::LogicType(LogicType::RatioNitrogenOutput))
            }
            "logictype.rationitrogenoutput2" => {
                Ok(Self::LogicType(LogicType::RatioNitrogenOutput2))
            }
            "logictype.rationitrousoxide" => {
                Ok(Self::LogicType(LogicType::RatioNitrousOxide))
            }
            "logictype.rationitrousoxideinput" => {
                Ok(Self::LogicType(LogicType::RatioNitrousOxideInput))
            }
            "logictype.rationitrousoxideinput2" => {
                Ok(Self::LogicType(LogicType::RatioNitrousOxideInput2))
            }
            "logictype.rationitrousoxideoutput" => {
                Ok(Self::LogicType(LogicType::RatioNitrousOxideOutput))
            }
            "logictype.rationitrousoxideoutput2" => {
                Ok(Self::LogicType(LogicType::RatioNitrousOxideOutput2))
            }
            "logictype.ratiooxygen" => Ok(Self::LogicType(LogicType::RatioOxygen)),
            "logictype.ratiooxygeninput" => {
                Ok(Self::LogicType(LogicType::RatioOxygenInput))
            }
            "logictype.ratiooxygeninput2" => {
                Ok(Self::LogicType(LogicType::RatioOxygenInput2))
            }
            "logictype.ratiooxygenoutput" => {
                Ok(Self::LogicType(LogicType::RatioOxygenOutput))
            }
            "logictype.ratiooxygenoutput2" => {
                Ok(Self::LogicType(LogicType::RatioOxygenOutput2))
            }
            "logictype.ratiopollutant" => Ok(Self::LogicType(LogicType::RatioPollutant)),
            "logictype.ratiopollutantinput" => {
                Ok(Self::LogicType(LogicType::RatioPollutantInput))
            }
            "logictype.ratiopollutantinput2" => {
                Ok(Self::LogicType(LogicType::RatioPollutantInput2))
            }
            "logictype.ratiopollutantoutput" => {
                Ok(Self::LogicType(LogicType::RatioPollutantOutput))
            }
            "logictype.ratiopollutantoutput2" => {
                Ok(Self::LogicType(LogicType::RatioPollutantOutput2))
            }
            "logictype.ratiopollutedwater" => {
                Ok(Self::LogicType(LogicType::RatioPollutedWater))
            }
            "logictype.ratiosteam" => Ok(Self::LogicType(LogicType::RatioSteam)),
            "logictype.ratiosteaminput" => {
                Ok(Self::LogicType(LogicType::RatioSteamInput))
            }
            "logictype.ratiosteaminput2" => {
                Ok(Self::LogicType(LogicType::RatioSteamInput2))
            }
            "logictype.ratiosteamoutput" => {
                Ok(Self::LogicType(LogicType::RatioSteamOutput))
            }
            "logictype.ratiosteamoutput2" => {
                Ok(Self::LogicType(LogicType::RatioSteamOutput2))
            }
            "logictype.ratiovolatiles" => Ok(Self::LogicType(LogicType::RatioVolatiles)),
            "logictype.ratiovolatilesinput" => {
                Ok(Self::LogicType(LogicType::RatioVolatilesInput))
            }
            "logictype.ratiovolatilesinput2" => {
                Ok(Self::LogicType(LogicType::RatioVolatilesInput2))
            }
            "logictype.ratiovolatilesoutput" => {
                Ok(Self::LogicType(LogicType::RatioVolatilesOutput))
            }
            "logictype.ratiovolatilesoutput2" => {
                Ok(Self::LogicType(LogicType::RatioVolatilesOutput2))
            }
            "logictype.ratiowater" => Ok(Self::LogicType(LogicType::RatioWater)),
            "logictype.ratiowaterinput" => {
                Ok(Self::LogicType(LogicType::RatioWaterInput))
            }
            "logictype.ratiowaterinput2" => {
                Ok(Self::LogicType(LogicType::RatioWaterInput2))
            }
            "logictype.ratiowateroutput" => {
                Ok(Self::LogicType(LogicType::RatioWaterOutput))
            }
            "logictype.ratiowateroutput2" => {
                Ok(Self::LogicType(LogicType::RatioWaterOutput2))
            }
            "logictype.reentryaltitude" => {
                Ok(Self::LogicType(LogicType::ReEntryAltitude))
            }
            "logictype.reagents" => Ok(Self::LogicType(LogicType::Reagents)),
            "logictype.recipehash" => Ok(Self::LogicType(LogicType::RecipeHash)),
            "logictype.referenceid" => Ok(Self::LogicType(LogicType::ReferenceId)),
            "logictype.requesthash" => Ok(Self::LogicType(LogicType::RequestHash)),
            "logictype.requiredpower" => Ok(Self::LogicType(LogicType::RequiredPower)),
            "logictype.returnfuelcost" => Ok(Self::LogicType(LogicType::ReturnFuelCost)),
            "logictype.richness" => Ok(Self::LogicType(LogicType::Richness)),
            "logictype.rpm" => Ok(Self::LogicType(LogicType::Rpm)),
            "logictype.semimajoraxis" => Ok(Self::LogicType(LogicType::SemiMajorAxis)),
            "logictype.setting" => Ok(Self::LogicType(LogicType::Setting)),
            "logictype.settinginput" => Ok(Self::LogicType(LogicType::SettingInput)),
            "logictype.settingoutput" => Ok(Self::LogicType(LogicType::SettingOutput)),
            "logictype.signalid" => Ok(Self::LogicType(LogicType::SignalId)),
            "logictype.signalstrength" => Ok(Self::LogicType(LogicType::SignalStrength)),
            "logictype.sites" => Ok(Self::LogicType(LogicType::Sites)),
            "logictype.size" => Ok(Self::LogicType(LogicType::Size)),
            "logictype.sizex" => Ok(Self::LogicType(LogicType::SizeX)),
            "logictype.sizey" => Ok(Self::LogicType(LogicType::SizeY)),
            "logictype.sizez" => Ok(Self::LogicType(LogicType::SizeZ)),
            "logictype.solarangle" => Ok(Self::LogicType(LogicType::SolarAngle)),
            "logictype.solarirradiance" => {
                Ok(Self::LogicType(LogicType::SolarIrradiance))
            }
            "logictype.soundalert" => Ok(Self::LogicType(LogicType::SoundAlert)),
            "logictype.stress" => Ok(Self::LogicType(LogicType::Stress)),
            "logictype.survey" => Ok(Self::LogicType(LogicType::Survey)),
            "logictype.targetpadindex" => Ok(Self::LogicType(LogicType::TargetPadIndex)),
            "logictype.targetx" => Ok(Self::LogicType(LogicType::TargetX)),
            "logictype.targety" => Ok(Self::LogicType(LogicType::TargetY)),
            "logictype.targetz" => Ok(Self::LogicType(LogicType::TargetZ)),
            "logictype.temperature" => Ok(Self::LogicType(LogicType::Temperature)),
            "logictype.temperaturedifferentialefficiency" => {
                Ok(Self::LogicType(LogicType::TemperatureDifferentialEfficiency))
            }
            "logictype.temperatureexternal" => {
                Ok(Self::LogicType(LogicType::TemperatureExternal))
            }
            "logictype.temperatureinput" => {
                Ok(Self::LogicType(LogicType::TemperatureInput))
            }
            "logictype.temperatureinput2" => {
                Ok(Self::LogicType(LogicType::TemperatureInput2))
            }
            "logictype.temperatureoutput" => {
                Ok(Self::LogicType(LogicType::TemperatureOutput))
            }
            "logictype.temperatureoutput2" => {
                Ok(Self::LogicType(LogicType::TemperatureOutput2))
            }
            "logictype.temperaturesetting" => {
                Ok(Self::LogicType(LogicType::TemperatureSetting))
            }
            "logictype.throttle" => Ok(Self::LogicType(LogicType::Throttle)),
            "logictype.thrust" => Ok(Self::LogicType(LogicType::Thrust)),
            "logictype.thrusttoweight" => Ok(Self::LogicType(LogicType::ThrustToWeight)),
            "logictype.time" => Ok(Self::LogicType(LogicType::Time)),
            "logictype.timetodestination" => {
                Ok(Self::LogicType(LogicType::TimeToDestination))
            }
            "logictype.totalmoles" => Ok(Self::LogicType(LogicType::TotalMoles)),
            "logictype.totalmolesinput" => {
                Ok(Self::LogicType(LogicType::TotalMolesInput))
            }
            "logictype.totalmolesinput2" => {
                Ok(Self::LogicType(LogicType::TotalMolesInput2))
            }
            "logictype.totalmolesoutput" => {
                Ok(Self::LogicType(LogicType::TotalMolesOutput))
            }
            "logictype.totalmolesoutput2" => {
                Ok(Self::LogicType(LogicType::TotalMolesOutput2))
            }
            "logictype.totalquantity" => Ok(Self::LogicType(LogicType::TotalQuantity)),
            "logictype.trueanomaly" => Ok(Self::LogicType(LogicType::TrueAnomaly)),
            "logictype.velocitymagnitude" => {
                Ok(Self::LogicType(LogicType::VelocityMagnitude))
            }
            "logictype.velocityrelativex" => {
                Ok(Self::LogicType(LogicType::VelocityRelativeX))
            }
            "logictype.velocityrelativey" => {
                Ok(Self::LogicType(LogicType::VelocityRelativeY))
            }
            "logictype.velocityrelativez" => {
                Ok(Self::LogicType(LogicType::VelocityRelativeZ))
            }
            "logictype.velocityx" => Ok(Self::LogicType(LogicType::VelocityX)),
            "logictype.velocityy" => Ok(Self::LogicType(LogicType::VelocityY)),
            "logictype.velocityz" => Ok(Self::LogicType(LogicType::VelocityZ)),
            "logictype.vertical" => Ok(Self::LogicType(LogicType::Vertical)),
            "logictype.verticalratio" => Ok(Self::LogicType(LogicType::VerticalRatio)),
            "logictype.volume" => Ok(Self::LogicType(LogicType::Volume)),
            "logictype.volumeofliquid" => Ok(Self::LogicType(LogicType::VolumeOfLiquid)),
            "logictype.wattsreachingcontact" => {
                Ok(Self::LogicType(LogicType::WattsReachingContact))
            }
            "logictype.weight" => Ok(Self::LogicType(LogicType::Weight)),
            "logictype.workinggasefficiency" => {
                Ok(Self::LogicType(LogicType::WorkingGasEfficiency))
            }
            "powermode.charged" => Ok(Self::PowerMode(PowerMode::Charged)),
            "powermode.charging" => Ok(Self::PowerMode(PowerMode::Charging)),
            "powermode.discharged" => Ok(Self::PowerMode(PowerMode::Discharged)),
            "powermode.discharging" => Ok(Self::PowerMode(PowerMode::Discharging)),
            "powermode.idle" => Ok(Self::PowerMode(PowerMode::Idle)),
            "printerinstruction.devicesetlock" => {
                Ok(Self::PrinterInstruction(PrinterInstruction::DeviceSetLock))
            }
            "printerinstruction.ejectallreagents" => {
                Ok(Self::PrinterInstruction(PrinterInstruction::EjectAllReagents))
            }
            "printerinstruction.ejectreagent" => {
                Ok(Self::PrinterInstruction(PrinterInstruction::EjectReagent))
            }
            "printerinstruction.executerecipe" => {
                Ok(Self::PrinterInstruction(PrinterInstruction::ExecuteRecipe))
            }
            "printerinstruction.jumpifnextinvalid" => {
                Ok(Self::PrinterInstruction(PrinterInstruction::JumpIfNextInvalid))
            }
            "printerinstruction.jumptoaddress" => {
                Ok(Self::PrinterInstruction(PrinterInstruction::JumpToAddress))
            }
            "printerinstruction.missingrecipereagent" => {
                Ok(Self::PrinterInstruction(PrinterInstruction::MissingRecipeReagent))
            }
            "printerinstruction.none" => {
                Ok(Self::PrinterInstruction(PrinterInstruction::None))
            }
            "printerinstruction.stackpointer" => {
                Ok(Self::PrinterInstruction(PrinterInstruction::StackPointer))
            }
            "printerinstruction.waituntilnextvalid" => {
                Ok(Self::PrinterInstruction(PrinterInstruction::WaitUntilNextValid))
            }
            "reentryprofile.high" => Ok(Self::ReEntryProfile(ReEntryProfile::High)),
            "reentryprofile.max" => Ok(Self::ReEntryProfile(ReEntryProfile::Max)),
            "reentryprofile.medium" => Ok(Self::ReEntryProfile(ReEntryProfile::Medium)),
            "reentryprofile.none" => Ok(Self::ReEntryProfile(ReEntryProfile::None)),
            "reentryprofile.optimal" => Ok(Self::ReEntryProfile(ReEntryProfile::Optimal)),
            "robotmode.follow" => Ok(Self::RobotMode(RobotMode::Follow)),
            "robotmode.movetotarget" => Ok(Self::RobotMode(RobotMode::MoveToTarget)),
            "robotmode.none" => Ok(Self::RobotMode(RobotMode::None)),
            "robotmode.pathtotarget" => Ok(Self::RobotMode(RobotMode::PathToTarget)),
            "robotmode.roam" => Ok(Self::RobotMode(RobotMode::Roam)),
            "robotmode.storagefull" => Ok(Self::RobotMode(RobotMode::StorageFull)),
            "robotmode.unload" => Ok(Self::RobotMode(RobotMode::Unload)),
            "rocketmode.chart" => Ok(Self::RocketMode(RocketMode::Chart)),
            "rocketmode.discover" => Ok(Self::RocketMode(RocketMode::Discover)),
            "rocketmode.invalid" => Ok(Self::RocketMode(RocketMode::Invalid)),
            "rocketmode.mine" => Ok(Self::RocketMode(RocketMode::Mine)),
            "rocketmode.none" => Ok(Self::RocketMode(RocketMode::None)),
            "rocketmode.survey" => Ok(Self::RocketMode(RocketMode::Survey)),
            "slotclass.accesscard" => Ok(Self::SlotClass(Class::AccessCard)),
            "slotclass.appliance" => Ok(Self::SlotClass(Class::Appliance)),
            "slotclass.back" => Ok(Self::SlotClass(Class::Back)),
            "slotclass.battery" => Ok(Self::SlotClass(Class::Battery)),
            "slotclass.belt" => Ok(Self::SlotClass(Class::Belt)),
            "slotclass.blocked" => Ok(Self::SlotClass(Class::Blocked)),
            "slotclass.bottle" => Ok(Self::SlotClass(Class::Bottle)),
            "slotclass.cartridge" => Ok(Self::SlotClass(Class::Cartridge)),
            "slotclass.circuit" => Ok(Self::SlotClass(Class::Circuit)),
            "slotclass.circuitboard" => Ok(Self::SlotClass(Class::Circuitboard)),
            "slotclass.creditcard" => Ok(Self::SlotClass(Class::CreditCard)),
            "slotclass.datadisk" => Ok(Self::SlotClass(Class::DataDisk)),
            "slotclass.dirtcanister" => Ok(Self::SlotClass(Class::DirtCanister)),
            "slotclass.drillhead" => Ok(Self::SlotClass(Class::DrillHead)),
            "slotclass.egg" => Ok(Self::SlotClass(Class::Egg)),
            "slotclass.entity" => Ok(Self::SlotClass(Class::Entity)),
            "slotclass.flare" => Ok(Self::SlotClass(Class::Flare)),
            "slotclass.gascanister" => Ok(Self::SlotClass(Class::GasCanister)),
            "slotclass.gasfilter" => Ok(Self::SlotClass(Class::GasFilter)),
            "slotclass.glasses" => Ok(Self::SlotClass(Class::Glasses)),
            "slotclass.helmet" => Ok(Self::SlotClass(Class::Helmet)),
            "slotclass.ingot" => Ok(Self::SlotClass(Class::Ingot)),
            "slotclass.liquidbottle" => Ok(Self::SlotClass(Class::LiquidBottle)),
            "slotclass.liquidcanister" => Ok(Self::SlotClass(Class::LiquidCanister)),
            "slotclass.magazine" => Ok(Self::SlotClass(Class::Magazine)),
            "slotclass.motherboard" => Ok(Self::SlotClass(Class::Motherboard)),
            "slotclass.none" => Ok(Self::SlotClass(Class::None)),
            "slotclass.ore" => Ok(Self::SlotClass(Class::Ore)),
            "slotclass.organ" => Ok(Self::SlotClass(Class::Organ)),
            "slotclass.plant" => Ok(Self::SlotClass(Class::Plant)),
            "slotclass.programmablechip" => Ok(Self::SlotClass(Class::ProgrammableChip)),
            "slotclass.scanninghead" => Ok(Self::SlotClass(Class::ScanningHead)),
            "slotclass.sensorprocessingunit" => {
                Ok(Self::SlotClass(Class::SensorProcessingUnit))
            }
            "slotclass.soundcartridge" => Ok(Self::SlotClass(Class::SoundCartridge)),
            "slotclass.suit" => Ok(Self::SlotClass(Class::Suit)),
            "slotclass.suitmod" => Ok(Self::SlotClass(Class::SuitMod)),
            "slotclass.tool" => Ok(Self::SlotClass(Class::Tool)),
            "slotclass.torpedo" => Ok(Self::SlotClass(Class::Torpedo)),
            "slotclass.uniform" => Ok(Self::SlotClass(Class::Uniform)),
            "slotclass.wreckage" => Ok(Self::SlotClass(Class::Wreckage)),
            "sorterinstruction.filterprefabhashequals" => {
                Ok(Self::SorterInstruction(SorterInstruction::FilterPrefabHashEquals))
            }
            "sorterinstruction.filterprefabhashnotequals" => {
                Ok(Self::SorterInstruction(SorterInstruction::FilterPrefabHashNotEquals))
            }
            "sorterinstruction.filterquantitycompare" => {
                Ok(Self::SorterInstruction(SorterInstruction::FilterQuantityCompare))
            }
            "sorterinstruction.filterslottypecompare" => {
                Ok(Self::SorterInstruction(SorterInstruction::FilterSlotTypeCompare))
            }
            "sorterinstruction.filtersortingclasscompare" => {
                Ok(Self::SorterInstruction(SorterInstruction::FilterSortingClassCompare))
            }
            "sorterinstruction.limitnextexecutionbycount" => {
                Ok(Self::SorterInstruction(SorterInstruction::LimitNextExecutionByCount))
            }
            "sorterinstruction.none" => {
                Ok(Self::SorterInstruction(SorterInstruction::None))
            }
            "sortingclass.appliances" => Ok(Self::SortingClass(SortingClass::Appliances)),
            "sortingclass.atmospherics" => {
                Ok(Self::SortingClass(SortingClass::Atmospherics))
            }
            "sortingclass.clothing" => Ok(Self::SortingClass(SortingClass::Clothing)),
            "sortingclass.default" => Ok(Self::SortingClass(SortingClass::Default)),
            "sortingclass.food" => Ok(Self::SortingClass(SortingClass::Food)),
            "sortingclass.ices" => Ok(Self::SortingClass(SortingClass::Ices)),
            "sortingclass.kits" => Ok(Self::SortingClass(SortingClass::Kits)),
            "sortingclass.ores" => Ok(Self::SortingClass(SortingClass::Ores)),
            "sortingclass.resources" => Ok(Self::SortingClass(SortingClass::Resources)),
            "sortingclass.storage" => Ok(Self::SortingClass(SortingClass::Storage)),
            "sortingclass.tools" => Ok(Self::SortingClass(SortingClass::Tools)),
            "sound.airlockcycling" => Ok(Self::Sound(SoundAlert::AirlockCycling)),
            "sound.alarm1" => Ok(Self::Sound(SoundAlert::Alarm1)),
            "sound.alarm10" => Ok(Self::Sound(SoundAlert::Alarm10)),
            "sound.alarm11" => Ok(Self::Sound(SoundAlert::Alarm11)),
            "sound.alarm12" => Ok(Self::Sound(SoundAlert::Alarm12)),
            "sound.alarm2" => Ok(Self::Sound(SoundAlert::Alarm2)),
            "sound.alarm3" => Ok(Self::Sound(SoundAlert::Alarm3)),
            "sound.alarm4" => Ok(Self::Sound(SoundAlert::Alarm4)),
            "sound.alarm5" => Ok(Self::Sound(SoundAlert::Alarm5)),
            "sound.alarm6" => Ok(Self::Sound(SoundAlert::Alarm6)),
            "sound.alarm7" => Ok(Self::Sound(SoundAlert::Alarm7)),
            "sound.alarm8" => Ok(Self::Sound(SoundAlert::Alarm8)),
            "sound.alarm9" => Ok(Self::Sound(SoundAlert::Alarm9)),
            "sound.alert" => Ok(Self::Sound(SoundAlert::Alert)),
            "sound.danger" => Ok(Self::Sound(SoundAlert::Danger)),
            "sound.depressurising" => Ok(Self::Sound(SoundAlert::Depressurising)),
            "sound.firefirefire" => Ok(Self::Sound(SoundAlert::FireFireFire)),
            "sound.five" => Ok(Self::Sound(SoundAlert::Five)),
            "sound.floor" => Ok(Self::Sound(SoundAlert::Floor)),
            "sound.four" => Ok(Self::Sound(SoundAlert::Four)),
            "sound.haltwhogoesthere" => Ok(Self::Sound(SoundAlert::HaltWhoGoesThere)),
            "sound.highcarbondioxide" => Ok(Self::Sound(SoundAlert::HighCarbonDioxide)),
            "sound.intruderalert" => Ok(Self::Sound(SoundAlert::IntruderAlert)),
            "sound.liftoff" => Ok(Self::Sound(SoundAlert::LiftOff)),
            "sound.malfunctiondetected" => {
                Ok(Self::Sound(SoundAlert::MalfunctionDetected))
            }
            "sound.music1" => Ok(Self::Sound(SoundAlert::Music1)),
            "sound.music2" => Ok(Self::Sound(SoundAlert::Music2)),
            "sound.music3" => Ok(Self::Sound(SoundAlert::Music3)),
            "sound.none" => Ok(Self::Sound(SoundAlert::None)),
            "sound.one" => Ok(Self::Sound(SoundAlert::One)),
            "sound.pollutantsdetected" => Ok(Self::Sound(SoundAlert::PollutantsDetected)),
            "sound.powerlow" => Ok(Self::Sound(SoundAlert::PowerLow)),
            "sound.pressurehigh" => Ok(Self::Sound(SoundAlert::PressureHigh)),
            "sound.pressurelow" => Ok(Self::Sound(SoundAlert::PressureLow)),
            "sound.pressurising" => Ok(Self::Sound(SoundAlert::Pressurising)),
            "sound.rocketlaunching" => Ok(Self::Sound(SoundAlert::RocketLaunching)),
            "sound.stormincoming" => Ok(Self::Sound(SoundAlert::StormIncoming)),
            "sound.systemfailure" => Ok(Self::Sound(SoundAlert::SystemFailure)),
            "sound.temperaturehigh" => Ok(Self::Sound(SoundAlert::TemperatureHigh)),
            "sound.temperaturelow" => Ok(Self::Sound(SoundAlert::TemperatureLow)),
            "sound.three" => Ok(Self::Sound(SoundAlert::Three)),
            "sound.traderincoming" => Ok(Self::Sound(SoundAlert::TraderIncoming)),
            "sound.traderlanded" => Ok(Self::Sound(SoundAlert::TraderLanded)),
            "sound.two" => Ok(Self::Sound(SoundAlert::Two)),
            "sound.warning" => Ok(Self::Sound(SoundAlert::Warning)),
            "sound.welcome" => Ok(Self::Sound(SoundAlert::Welcome)),
            "transmittermode.active" => {
                Ok(Self::TransmitterMode(LogicTransmitterMode::Active))
            }
            "transmittermode.passive" => {
                Ok(Self::TransmitterMode(LogicTransmitterMode::Passive))
            }
            "vent.inward" => Ok(Self::Vent(VentDirection::Inward)),
            "vent.outward" => Ok(Self::Vent(VentDirection::Outward)),
            "equals" => Ok(Self::Unnamed(ConditionOperation::Equals)),
            "greater" => Ok(Self::Unnamed(ConditionOperation::Greater)),
            "less" => Ok(Self::Unnamed(ConditionOperation::Less)),
            "notequals" => Ok(Self::Unnamed(ConditionOperation::NotEquals)),
            _ => {
                Err(super::ParseError {
                    enm: s.to_string(),
                })
            }
        }
    }
}
impl std::fmt::Display for BasicEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AirCon(enm) => write!(f, "AirCon.{}", enm),
            Self::AirControl(enm) => write!(f, "AirControl.{}", enm),
            Self::Color(enm) => write!(f, "Color.{}", enm),
            Self::DaylightSensorMode(enm) => write!(f, "DaylightSensorMode.{}", enm),
            Self::ElevatorMode(enm) => write!(f, "ElevatorMode.{}", enm),
            Self::EntityState(enm) => write!(f, "EntityState.{}", enm),
            Self::GasType(enm) => write!(f, "GasType.{}", enm),
            Self::LogicSlotType(enm) => write!(f, "LogicSlotType.{}", enm),
            Self::LogicType(enm) => write!(f, "LogicType.{}", enm),
            Self::PowerMode(enm) => write!(f, "PowerMode.{}", enm),
            Self::PrinterInstruction(enm) => write!(f, "PrinterInstruction.{}", enm),
            Self::ReEntryProfile(enm) => write!(f, "ReEntryProfile.{}", enm),
            Self::RobotMode(enm) => write!(f, "RobotMode.{}", enm),
            Self::RocketMode(enm) => write!(f, "RocketMode.{}", enm),
            Self::SlotClass(enm) => write!(f, "SlotClass.{}", enm),
            Self::SorterInstruction(enm) => write!(f, "SorterInstruction.{}", enm),
            Self::SortingClass(enm) => write!(f, "SortingClass.{}", enm),
            Self::Sound(enm) => write!(f, "Sound.{}", enm),
            Self::TransmitterMode(enm) => write!(f, "TransmitterMode.{}", enm),
            Self::Vent(enm) => write!(f, "Vent.{}", enm),
            Self::Unnamed(enm) => write!(f, "_unnamed{}", enm),
        }
    }
}
