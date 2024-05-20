use serde_derive::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumProperty, EnumString, FromRepr};
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
#[strum(use_phf)]
#[repr(u8)]
pub enum LogicBatchMethod {
    #[strum(serialize = "Average")]
    #[strum(props(docs = "", value = "0"))]
    Average = 0u8,
    #[strum(serialize = "Sum")]
    #[strum(props(docs = "", value = "1"))]
    Sum = 1u8,
    #[strum(serialize = "Minimum")]
    #[strum(props(docs = "", value = "2"))]
    Minimum = 2u8,
    #[strum(serialize = "Maximum")]
    #[strum(props(docs = "", value = "3"))]
    Maximum = 3u8,
}
impl TryFrom<f64> for LogicBatchMethod {
    type Error = super::ParseError;
    fn try_from(value: f64) -> Result<Self, <LogicBatchMethod as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = LogicBatchMethod::iter()
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
#[strum(use_phf)]
#[repr(u8)]
pub enum LogicReagentMode {
    #[strum(serialize = "Contents")]
    #[strum(props(docs = "", value = "0"))]
    Contents = 0u8,
    #[strum(serialize = "Required")]
    #[strum(props(docs = "", value = "1"))]
    Required = 1u8,
    #[strum(serialize = "Recipe")]
    #[strum(props(docs = "", value = "2"))]
    Recipe = 2u8,
    #[strum(serialize = "TotalContents")]
    #[strum(props(docs = "", value = "3"))]
    TotalContents = 3u8,
}
impl TryFrom<f64> for LogicReagentMode {
    type Error = super::ParseError;
    fn try_from(value: f64) -> Result<Self, <LogicReagentMode as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = LogicReagentMode::iter()
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
#[strum(use_phf)]
#[repr(u8)]
pub enum LogicSlotType {
    #[strum(serialize = "None")]
    #[strum(props(docs = "No description", value = "0"))]
    #[default]
    None = 0u8,
    #[strum(serialize = "Occupied")]
    #[strum(
        props(docs = "returns 0 when slot is not occupied, 1 when it is", value = "1")
    )]
    Occupied = 1u8,
    #[strum(serialize = "OccupantHash")]
    #[strum(
        props(
            docs = "returns the has of the current occupant, the unique identifier of the thing",
            value = "2"
        )
    )]
    OccupantHash = 2u8,
    #[strum(serialize = "Quantity")]
    #[strum(
        props(
            docs = "returns the current quantity, such as stack size, of the item in the slot",
            value = "3"
        )
    )]
    Quantity = 3u8,
    #[strum(serialize = "Damage")]
    #[strum(
        props(docs = "returns the damage state of the item in the slot", value = "4")
    )]
    Damage = 4u8,
    #[strum(serialize = "Efficiency")]
    #[strum(
        props(
            docs = "returns the growth efficiency of the plant in the slot",
            value = "5"
        )
    )]
    Efficiency = 5u8,
    #[strum(serialize = "Health")]
    #[strum(props(docs = "returns the health of the plant in the slot", value = "6"))]
    Health = 6u8,
    #[strum(serialize = "Growth")]
    #[strum(
        props(
            docs = "returns the current growth state of the plant in the slot",
            value = "7"
        )
    )]
    Growth = 7u8,
    #[strum(serialize = "Pressure")]
    #[strum(
        props(
            docs = "returns pressure of the slot occupants internal atmosphere",
            value = "8"
        )
    )]
    Pressure = 8u8,
    #[strum(serialize = "Temperature")]
    #[strum(
        props(
            docs = "returns temperature of the slot occupants internal atmosphere",
            value = "9"
        )
    )]
    Temperature = 9u8,
    #[strum(serialize = "Charge")]
    #[strum(
        props(
            docs = "returns current energy charge the slot occupant is holding",
            value = "10"
        )
    )]
    Charge = 10u8,
    #[strum(serialize = "ChargeRatio")]
    #[strum(
        props(
            docs = "returns current energy charge the slot occupant is holding as a ratio between 0 and 1 of its maximum",
            value = "11"
        )
    )]
    ChargeRatio = 11u8,
    #[strum(serialize = "Class")]
    #[strum(
        props(docs = "returns integer representing the class of object", value = "12")
    )]
    Class = 12u8,
    #[strum(serialize = "PressureWaste")]
    #[strum(
        props(
            docs = "returns pressure in the waste tank of the jetpack in this slot",
            value = "13"
        )
    )]
    PressureWaste = 13u8,
    #[strum(serialize = "PressureAir")]
    #[strum(
        props(
            docs = "returns pressure in the air tank of the jetpack in this slot",
            value = "14"
        )
    )]
    PressureAir = 14u8,
    #[strum(serialize = "MaxQuantity")]
    #[strum(
        props(docs = "returns the max stack size of the item in the slot", value = "15")
    )]
    MaxQuantity = 15u8,
    #[strum(serialize = "Mature")]
    #[strum(
        props(
            docs = "returns 1 if the plant in this slot is mature, 0 when it isn't",
            value = "16"
        )
    )]
    Mature = 16u8,
    #[strum(serialize = "PrefabHash")]
    #[strum(props(docs = "returns the hash of the structure in the slot", value = "17"))]
    PrefabHash = 17u8,
    #[strum(serialize = "Seeding")]
    #[strum(
        props(
            docs = "Whether a plant is seeding (ready to harvest seeds from). Returns 1 if seeding or 0 if not.",
            value = "18"
        )
    )]
    Seeding = 18u8,
    #[strum(serialize = "LineNumber")]
    #[strum(
        props(
            docs = "The line number of current execution for an integrated circuit running on this device. While this number can be written, use with caution",
            value = "19"
        )
    )]
    LineNumber = 19u8,
    #[strum(serialize = "Volume")]
    #[strum(props(docs = "No description available", value = "20"))]
    Volume = 20u8,
    #[strum(serialize = "Open")]
    #[strum(props(docs = "No description available", value = "21"))]
    Open = 21u8,
    #[strum(serialize = "On")]
    #[strum(props(docs = "No description available", value = "22"))]
    On = 22u8,
    #[strum(serialize = "Lock")]
    #[strum(props(docs = "No description available", value = "23"))]
    Lock = 23u8,
    #[strum(serialize = "SortingClass")]
    #[strum(props(docs = "No description available", value = "24"))]
    SortingClass = 24u8,
    #[strum(serialize = "FilterType")]
    #[strum(props(docs = "No description available", value = "25"))]
    FilterType = 25u8,
    #[strum(serialize = "ReferenceId")]
    #[strum(props(docs = "Unique Reference Identifier for this object", value = "26"))]
    ReferenceId = 26u8,
}
impl TryFrom<f64> for LogicSlotType {
    type Error = super::ParseError;
    fn try_from(value: f64) -> Result<Self, <LogicSlotType as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = LogicSlotType::iter()
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
#[strum(use_phf)]
#[repr(u16)]
pub enum LogicType {
    #[strum(serialize = "None")]
    #[strum(props(deprecated = "true", docs = "No description", value = "0"))]
    #[default]
    None = 0u16,
    #[strum(serialize = "Power")]
    #[strum(
        props(
            docs = "Can be read to return if the device is correctly powered or not, set via the power system, return 1 if powered and 0 if not",
            value = "1"
        )
    )]
    Power = 1u16,
    #[strum(serialize = "Open")]
    #[strum(props(docs = "1 if device is open, otherwise 0", value = "2"))]
    Open = 2u16,
    #[strum(serialize = "Mode")]
    #[strum(
        props(
            docs = "Integer for mode state, different devices will have different mode states available to them",
            value = "3"
        )
    )]
    Mode = 3u16,
    #[strum(serialize = "Error")]
    #[strum(props(docs = "1 if device is in error state, otherwise 0", value = "4"))]
    Error = 4u16,
    #[strum(serialize = "Pressure")]
    #[strum(props(docs = "The current pressure reading of the device", value = "5"))]
    Pressure = 5u16,
    #[strum(serialize = "Temperature")]
    #[strum(props(docs = "The current temperature reading of the device", value = "6"))]
    Temperature = 6u16,
    #[strum(serialize = "PressureExternal")]
    #[strum(props(docs = "Setting for external pressure safety, in KPa", value = "7"))]
    PressureExternal = 7u16,
    #[strum(serialize = "PressureInternal")]
    #[strum(props(docs = "Setting for internal pressure safety, in KPa", value = "8"))]
    PressureInternal = 8u16,
    #[strum(serialize = "Activate")]
    #[strum(
        props(
            docs = "1 if device is activated (usually means running), otherwise 0",
            value = "9"
        )
    )]
    Activate = 9u16,
    #[strum(serialize = "Lock")]
    #[strum(
        props(
            docs = "1 if device is locked, otherwise 0, can be set in most devices and prevents the user from access the values",
            value = "10"
        )
    )]
    Lock = 10u16,
    #[strum(serialize = "Charge")]
    #[strum(props(docs = "The current charge the device has", value = "11"))]
    Charge = 11u16,
    #[strum(serialize = "Setting")]
    #[strum(
        props(
            docs = "A variable setting that can be read or written, depending on the device",
            value = "12"
        )
    )]
    Setting = 12u16,
    #[strum(serialize = "Reagents")]
    #[strum(
        props(docs = "Total number of reagents recorded by the device", value = "13")
    )]
    Reagents = 13u16,
    #[strum(serialize = "RatioOxygen")]
    #[strum(props(docs = "The ratio of oxygen in device atmosphere", value = "14"))]
    RatioOxygen = 14u16,
    #[strum(serialize = "RatioCarbonDioxide")]
    #[strum(
        props(
            docs = "The ratio of <link=GasCarbonDioxide><color=#44AD83>Carbon Dioxide</color></link> in device atmosphere",
            value = "15"
        )
    )]
    RatioCarbonDioxide = 15u16,
    #[strum(serialize = "RatioNitrogen")]
    #[strum(props(docs = "The ratio of nitrogen in device atmosphere", value = "16"))]
    RatioNitrogen = 16u16,
    #[strum(serialize = "RatioPollutant")]
    #[strum(props(docs = "The ratio of pollutant in device atmosphere", value = "17"))]
    RatioPollutant = 17u16,
    #[strum(serialize = "RatioVolatiles")]
    #[strum(props(docs = "The ratio of volatiles in device atmosphere", value = "18"))]
    RatioVolatiles = 18u16,
    #[strum(serialize = "RatioWater")]
    #[strum(props(docs = "The ratio of water in device atmosphere", value = "19"))]
    RatioWater = 19u16,
    #[strum(serialize = "Horizontal")]
    #[strum(props(docs = "Horizontal setting of the device", value = "20"))]
    Horizontal = 20u16,
    #[strum(serialize = "Vertical")]
    #[strum(props(docs = "Vertical setting of the device", value = "21"))]
    Vertical = 21u16,
    #[strum(serialize = "SolarAngle")]
    #[strum(props(docs = "Solar angle of the device", value = "22"))]
    SolarAngle = 22u16,
    #[strum(serialize = "Maximum")]
    #[strum(props(docs = "Maximum setting of the device", value = "23"))]
    Maximum = 23u16,
    #[strum(serialize = "Ratio")]
    #[strum(
        props(
            docs = "Context specific value depending on device, 0 to 1 based ratio",
            value = "24"
        )
    )]
    Ratio = 24u16,
    #[strum(serialize = "PowerPotential")]
    #[strum(
        props(
            docs = "How much energy the device or network potentially provides",
            value = "25"
        )
    )]
    PowerPotential = 25u16,
    #[strum(serialize = "PowerActual")]
    #[strum(
        props(
            docs = "How much energy the device or network is actually using",
            value = "26"
        )
    )]
    PowerActual = 26u16,
    #[strum(serialize = "Quantity")]
    #[strum(props(docs = "Total quantity on the device", value = "27"))]
    Quantity = 27u16,
    #[strum(serialize = "On")]
    #[strum(
        props(
            docs = "The current state of the device, 0 for off, 1 for on",
            value = "28"
        )
    )]
    On = 28u16,
    #[strum(serialize = "ImportQuantity")]
    #[strum(
        props(
            deprecated = "true",
            docs = "Total quantity of items imported by the device",
            value = "29"
        )
    )]
    ImportQuantity = 29u16,
    #[strum(serialize = "ImportSlotOccupant")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "30"))]
    ImportSlotOccupant = 30u16,
    #[strum(serialize = "ExportQuantity")]
    #[strum(
        props(
            deprecated = "true",
            docs = "Total quantity of items exported by the device",
            value = "31"
        )
    )]
    ExportQuantity = 31u16,
    #[strum(serialize = "ExportSlotOccupant")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "32"))]
    ExportSlotOccupant = 32u16,
    #[strum(serialize = "RequiredPower")]
    #[strum(
        props(
            docs = "Idle operating power quantity, does not necessarily include extra demand power",
            value = "33"
        )
    )]
    RequiredPower = 33u16,
    #[strum(serialize = "HorizontalRatio")]
    #[strum(props(docs = "Radio of horizontal setting for device", value = "34"))]
    HorizontalRatio = 34u16,
    #[strum(serialize = "VerticalRatio")]
    #[strum(props(docs = "Radio of vertical setting for device", value = "35"))]
    VerticalRatio = 35u16,
    #[strum(serialize = "PowerRequired")]
    #[strum(
        props(docs = "Power requested from the device and/or network", value = "36")
    )]
    PowerRequired = 36u16,
    #[strum(serialize = "Idle")]
    #[strum(
        props(
            docs = "Returns 1 if the device is currently idle, otherwise 0",
            value = "37"
        )
    )]
    Idle = 37u16,
    #[strum(serialize = "Color")]
    #[strum(
        props(
            docs = "\n        Whether driven by concerns for clarity, safety or simple aesthetics, <link=Stationeers><color=#0080FFFF>Stationeers</color></link> have access to a small rainbow of colors for their constructions. These are the color setting for devices, represented as an integer.\n\n0: Blue\n1: Grey\n2: Green\n3: Orange\n4: Red\n5: Yellow\n6: White\n7: Black\n8: Brown\n9: Khaki\n10: Pink\n11: Purple\n\n        It is an unwavering universal law that anything higher than 11 will be purple. The <link=ODA><color=#0080FFFF>ODA</color></link> is powerless to change this. Similarly, anything lower than 0 will be Blue.\n      ",
            value = "38"
        )
    )]
    Color = 38u16,
    #[strum(serialize = "ElevatorSpeed")]
    #[strum(props(docs = "Current speed of the elevator", value = "39"))]
    ElevatorSpeed = 39u16,
    #[strum(serialize = "ElevatorLevel")]
    #[strum(props(docs = "Level the elevator is currently at", value = "40"))]
    ElevatorLevel = 40u16,
    #[strum(serialize = "RecipeHash")]
    #[strum(
        props(
            docs = "Current hash of the recipe the device is set to produce",
            value = "41"
        )
    )]
    RecipeHash = 41u16,
    #[strum(serialize = "ExportSlotHash")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "42"))]
    ExportSlotHash = 42u16,
    #[strum(serialize = "ImportSlotHash")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "43"))]
    ImportSlotHash = 43u16,
    #[strum(serialize = "PlantHealth1")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "44"))]
    PlantHealth1 = 44u16,
    #[strum(serialize = "PlantHealth2")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "45"))]
    PlantHealth2 = 45u16,
    #[strum(serialize = "PlantHealth3")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "46"))]
    PlantHealth3 = 46u16,
    #[strum(serialize = "PlantHealth4")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "47"))]
    PlantHealth4 = 47u16,
    #[strum(serialize = "PlantGrowth1")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "48"))]
    PlantGrowth1 = 48u16,
    #[strum(serialize = "PlantGrowth2")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "49"))]
    PlantGrowth2 = 49u16,
    #[strum(serialize = "PlantGrowth3")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "50"))]
    PlantGrowth3 = 50u16,
    #[strum(serialize = "PlantGrowth4")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "51"))]
    PlantGrowth4 = 51u16,
    #[strum(serialize = "PlantEfficiency1")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "52"))]
    PlantEfficiency1 = 52u16,
    #[strum(serialize = "PlantEfficiency2")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "53"))]
    PlantEfficiency2 = 53u16,
    #[strum(serialize = "PlantEfficiency3")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "54"))]
    PlantEfficiency3 = 54u16,
    #[strum(serialize = "PlantEfficiency4")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "55"))]
    PlantEfficiency4 = 55u16,
    #[strum(serialize = "PlantHash1")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "56"))]
    PlantHash1 = 56u16,
    #[strum(serialize = "PlantHash2")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "57"))]
    PlantHash2 = 57u16,
    #[strum(serialize = "PlantHash3")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "58"))]
    PlantHash3 = 58u16,
    #[strum(serialize = "PlantHash4")]
    #[strum(props(deprecated = "true", docs = "DEPRECATED", value = "59"))]
    PlantHash4 = 59u16,
    #[strum(serialize = "RequestHash")]
    #[strum(
        props(
            docs = "When set to the unique identifier, requests an item of the provided type from the device",
            value = "60"
        )
    )]
    RequestHash = 60u16,
    #[strum(serialize = "CompletionRatio")]
    #[strum(
        props(
            docs = "How complete the current production is for this device, between 0 and 1",
            value = "61"
        )
    )]
    CompletionRatio = 61u16,
    #[strum(serialize = "ClearMemory")]
    #[strum(
        props(
            docs = "When set to 1, clears the counter memory (e.g. ExportCount). Will set itself back to 0 when actioned",
            value = "62"
        )
    )]
    ClearMemory = 62u16,
    #[strum(serialize = "ExportCount")]
    #[strum(
        props(docs = "How many items exported since last ClearMemory", value = "63")
    )]
    ExportCount = 63u16,
    #[strum(serialize = "ImportCount")]
    #[strum(
        props(docs = "How many items imported since last ClearMemory", value = "64")
    )]
    ImportCount = 64u16,
    #[strum(serialize = "PowerGeneration")]
    #[strum(props(docs = "Returns how much power is being generated", value = "65"))]
    PowerGeneration = 65u16,
    #[strum(serialize = "TotalMoles")]
    #[strum(props(docs = "Returns the total moles of the device", value = "66"))]
    TotalMoles = 66u16,
    #[strum(serialize = "Volume")]
    #[strum(props(docs = "Returns the device atmosphere volume", value = "67"))]
    Volume = 67u16,
    #[strum(serialize = "Plant")]
    #[strum(
        props(
            docs = "Performs the planting action for any plant based machinery",
            value = "68"
        )
    )]
    Plant = 68u16,
    #[strum(serialize = "Harvest")]
    #[strum(
        props(
            docs = "Performs the harvesting action for any plant based machinery",
            value = "69"
        )
    )]
    Harvest = 69u16,
    #[strum(serialize = "Output")]
    #[strum(
        props(
            docs = "The output operation for a sort handling device, such as a stacker or sorter, when in logic mode the device will only action one repetition when set zero or above and then back to -1 and await further instructions",
            value = "70"
        )
    )]
    Output = 70u16,
    #[strum(serialize = "PressureSetting")]
    #[strum(
        props(
            docs = "The current setting for the internal pressure of the object (e.g. the Hardsuit Air release), in KPa",
            value = "71"
        )
    )]
    PressureSetting = 71u16,
    #[strum(serialize = "TemperatureSetting")]
    #[strum(
        props(
            docs = "The current setting for the internal temperature of the object (e.g. the Hardsuit A/C)",
            value = "72"
        )
    )]
    TemperatureSetting = 72u16,
    #[strum(serialize = "TemperatureExternal")]
    #[strum(
        props(
            docs = "The temperature of the outside of the device, usually the world atmosphere surrounding it",
            value = "73"
        )
    )]
    TemperatureExternal = 73u16,
    #[strum(serialize = "Filtration")]
    #[strum(
        props(
            docs = "The current state of the filtration system, for example Filtration = 1 for a Hardsuit sets filtration to On",
            value = "74"
        )
    )]
    Filtration = 74u16,
    #[strum(serialize = "AirRelease")]
    #[strum(
        props(
            docs = "The current state of the air release system, for example AirRelease = 1 for a Hardsuit sets Air Release to On",
            value = "75"
        )
    )]
    AirRelease = 75u16,
    #[strum(serialize = "PositionX")]
    #[strum(
        props(
            docs = "The current position in X dimension in world coordinates",
            value = "76"
        )
    )]
    PositionX = 76u16,
    #[strum(serialize = "PositionY")]
    #[strum(
        props(
            docs = "The current position in Y dimension in world coordinates",
            value = "77"
        )
    )]
    PositionY = 77u16,
    #[strum(serialize = "PositionZ")]
    #[strum(
        props(
            docs = "The current position in Z dimension in world coordinates",
            value = "78"
        )
    )]
    PositionZ = 78u16,
    #[strum(serialize = "VelocityMagnitude")]
    #[strum(props(docs = "The current magnitude of the velocity vector", value = "79"))]
    VelocityMagnitude = 79u16,
    #[strum(serialize = "VelocityRelativeX")]
    #[strum(
        props(
            docs = "The current velocity X relative to the forward vector of this",
            value = "80"
        )
    )]
    VelocityRelativeX = 80u16,
    #[strum(serialize = "VelocityRelativeY")]
    #[strum(
        props(
            docs = "The current velocity Y relative to the forward vector of this",
            value = "81"
        )
    )]
    VelocityRelativeY = 81u16,
    #[strum(serialize = "VelocityRelativeZ")]
    #[strum(
        props(
            docs = "The current velocity Z relative to the forward vector of this",
            value = "82"
        )
    )]
    VelocityRelativeZ = 82u16,
    #[strum(serialize = "RatioNitrousOxide")]
    #[strum(
        props(
            docs = "The ratio of <link=GasNitrousOxide><color=#44AD83>Nitrous Oxide</color></link> in device atmosphere",
            value = "83"
        )
    )]
    RatioNitrousOxide = 83u16,
    #[strum(serialize = "PrefabHash")]
    #[strum(props(docs = "The hash of the structure", value = "84"))]
    PrefabHash = 84u16,
    #[strum(serialize = "ForceWrite")]
    #[strum(props(docs = "Forces Logic Writer devices to rewrite value", value = "85"))]
    ForceWrite = 85u16,
    #[strum(serialize = "SignalStrength")]
    #[strum(
        props(docs = "Returns the degree offset of the strongest contact", value = "86")
    )]
    SignalStrength = 86u16,
    #[strum(serialize = "SignalID")]
    #[strum(
        props(
            docs = "Returns the contact ID of the strongest signal from this Satellite",
            value = "87"
        )
    )]
    SignalId = 87u16,
    #[strum(serialize = "TargetX")]
    #[strum(
        props(
            docs = "The target position in X dimension in world coordinates",
            value = "88"
        )
    )]
    TargetX = 88u16,
    #[strum(serialize = "TargetY")]
    #[strum(
        props(
            docs = "The target position in Y dimension in world coordinates",
            value = "89"
        )
    )]
    TargetY = 89u16,
    #[strum(serialize = "TargetZ")]
    #[strum(
        props(
            docs = "The target position in Z dimension in world coordinates",
            value = "90"
        )
    )]
    TargetZ = 90u16,
    #[strum(serialize = "SettingInput")]
    #[strum(props(docs = "<A:EN:-1139210406>", value = "91"))]
    SettingInput = 91u16,
    #[strum(serialize = "SettingOutput")]
    #[strum(props(docs = "<A:EN:1605622326>", value = "92"))]
    SettingOutput = 92u16,
    #[strum(serialize = "CurrentResearchPodType")]
    #[strum(props(docs = "<A:EN:1890273128>", value = "93"))]
    CurrentResearchPodType = 93u16,
    #[strum(serialize = "ManualResearchRequiredPod")]
    #[strum(
        props(
            docs = "Sets the pod type to search for a certain pod when breaking down a pods.",
            value = "94"
        )
    )]
    ManualResearchRequiredPod = 94u16,
    #[strum(serialize = "MineablesInVicinity")]
    #[strum(
        props(
            docs = "Returns the amount of potential mineables within an extended area around AIMEe.",
            value = "95"
        )
    )]
    MineablesInVicinity = 95u16,
    #[strum(serialize = "MineablesInQueue")]
    #[strum(
        props(
            docs = "Returns the amount of mineables AIMEe has queued up to mine.",
            value = "96"
        )
    )]
    MineablesInQueue = 96u16,
    #[strum(serialize = "NextWeatherEventTime")]
    #[strum(
        props(
            docs = "Returns in seconds when the next weather event is inbound.",
            value = "97"
        )
    )]
    NextWeatherEventTime = 97u16,
    #[strum(serialize = "Combustion")]
    #[strum(
        props(
            docs = "The assess atmosphere is on fire. Returns 1 if atmosphere is on fire, 0 if not.",
            value = "98"
        )
    )]
    Combustion = 98u16,
    #[strum(serialize = "Fuel")]
    #[strum(
        props(
            docs = "Gets the cost of fuel to return the rocket to your current world.",
            value = "99"
        )
    )]
    Fuel = 99u16,
    #[strum(serialize = "ReturnFuelCost")]
    #[strum(
        props(
            docs = "Gets the fuel remaining in your rocket's fuel tank.",
            value = "100"
        )
    )]
    ReturnFuelCost = 100u16,
    #[strum(serialize = "CollectableGoods")]
    #[strum(
        props(
            docs = "Gets the cost of fuel to return the rocket to your current world.",
            value = "101"
        )
    )]
    CollectableGoods = 101u16,
    #[strum(serialize = "Time")]
    #[strum(props(docs = "Time", value = "102"))]
    Time = 102u16,
    #[strum(serialize = "Bpm")]
    #[strum(props(docs = "Bpm", value = "103"))]
    Bpm = 103u16,
    #[strum(serialize = "EnvironmentEfficiency")]
    #[strum(
        props(
            docs = "The Environment Efficiency reported by the machine, as a float between 0 and 1",
            value = "104"
        )
    )]
    EnvironmentEfficiency = 104u16,
    #[strum(serialize = "WorkingGasEfficiency")]
    #[strum(
        props(
            docs = "The Working Gas Efficiency reported by the machine, as a float between 0 and 1",
            value = "105"
        )
    )]
    WorkingGasEfficiency = 105u16,
    #[strum(serialize = "PressureInput")]
    #[strum(
        props(
            docs = "The current pressure reading of the device's Input Network",
            value = "106"
        )
    )]
    PressureInput = 106u16,
    #[strum(serialize = "TemperatureInput")]
    #[strum(
        props(
            docs = "The current temperature reading of the device's Input Network",
            value = "107"
        )
    )]
    TemperatureInput = 107u16,
    #[strum(serialize = "RatioOxygenInput")]
    #[strum(
        props(docs = "The ratio of oxygen in device's input network", value = "108")
    )]
    RatioOxygenInput = 108u16,
    #[strum(serialize = "RatioCarbonDioxideInput")]
    #[strum(
        props(
            docs = "The ratio of <link=GasCarbonDioxide><color=#44AD83>Carbon Dioxide</color></link> in device's input network",
            value = "109"
        )
    )]
    RatioCarbonDioxideInput = 109u16,
    #[strum(serialize = "RatioNitrogenInput")]
    #[strum(
        props(docs = "The ratio of nitrogen in device's input network", value = "110")
    )]
    RatioNitrogenInput = 110u16,
    #[strum(serialize = "RatioPollutantInput")]
    #[strum(
        props(docs = "The ratio of pollutant in device's input network", value = "111")
    )]
    RatioPollutantInput = 111u16,
    #[strum(serialize = "RatioVolatilesInput")]
    #[strum(
        props(docs = "The ratio of volatiles in device's input network", value = "112")
    )]
    RatioVolatilesInput = 112u16,
    #[strum(serialize = "RatioWaterInput")]
    #[strum(props(docs = "The ratio of water in device's input network", value = "113"))]
    RatioWaterInput = 113u16,
    #[strum(serialize = "RatioNitrousOxideInput")]
    #[strum(
        props(
            docs = "The ratio of <link=GasNitrousOxide><color=#44AD83>Nitrous Oxide</color></link> in device's input network",
            value = "114"
        )
    )]
    RatioNitrousOxideInput = 114u16,
    #[strum(serialize = "TotalMolesInput")]
    #[strum(
        props(
            docs = "Returns the total moles of the device's Input Network",
            value = "115"
        )
    )]
    TotalMolesInput = 115u16,
    #[strum(serialize = "PressureInput2")]
    #[strum(
        props(
            docs = "The current pressure reading of the device's Input2 Network",
            value = "116"
        )
    )]
    PressureInput2 = 116u16,
    #[strum(serialize = "TemperatureInput2")]
    #[strum(
        props(
            docs = "The current temperature reading of the device's Input2 Network",
            value = "117"
        )
    )]
    TemperatureInput2 = 117u16,
    #[strum(serialize = "RatioOxygenInput2")]
    #[strum(
        props(docs = "The ratio of oxygen in device's Input2 network", value = "118")
    )]
    RatioOxygenInput2 = 118u16,
    #[strum(serialize = "RatioCarbonDioxideInput2")]
    #[strum(
        props(
            docs = "The ratio of <link=GasCarbonDioxide><color=#44AD83>Carbon Dioxide</color></link> in device's Input2 network",
            value = "119"
        )
    )]
    RatioCarbonDioxideInput2 = 119u16,
    #[strum(serialize = "RatioNitrogenInput2")]
    #[strum(
        props(docs = "The ratio of nitrogen in device's Input2 network", value = "120")
    )]
    RatioNitrogenInput2 = 120u16,
    #[strum(serialize = "RatioPollutantInput2")]
    #[strum(
        props(docs = "The ratio of pollutant in device's Input2 network", value = "121")
    )]
    RatioPollutantInput2 = 121u16,
    #[strum(serialize = "RatioVolatilesInput2")]
    #[strum(
        props(docs = "The ratio of volatiles in device's Input2 network", value = "122")
    )]
    RatioVolatilesInput2 = 122u16,
    #[strum(serialize = "RatioWaterInput2")]
    #[strum(
        props(docs = "The ratio of water in device's Input2 network", value = "123")
    )]
    RatioWaterInput2 = 123u16,
    #[strum(serialize = "RatioNitrousOxideInput2")]
    #[strum(
        props(
            docs = "The ratio of <link=GasNitrousOxide><color=#44AD83>Nitrous Oxide</color></link> in device's Input2 network",
            value = "124"
        )
    )]
    RatioNitrousOxideInput2 = 124u16,
    #[strum(serialize = "TotalMolesInput2")]
    #[strum(
        props(
            docs = "Returns the total moles of the device's Input2 Network",
            value = "125"
        )
    )]
    TotalMolesInput2 = 125u16,
    #[strum(serialize = "PressureOutput")]
    #[strum(
        props(
            docs = "The current pressure reading of the device's Output Network",
            value = "126"
        )
    )]
    PressureOutput = 126u16,
    #[strum(serialize = "TemperatureOutput")]
    #[strum(
        props(
            docs = "The current temperature reading of the device's Output Network",
            value = "127"
        )
    )]
    TemperatureOutput = 127u16,
    #[strum(serialize = "RatioOxygenOutput")]
    #[strum(
        props(docs = "The ratio of oxygen in device's Output network", value = "128")
    )]
    RatioOxygenOutput = 128u16,
    #[strum(serialize = "RatioCarbonDioxideOutput")]
    #[strum(
        props(
            docs = "The ratio of <link=GasCarbonDioxide><color=#44AD83>Carbon Dioxide</color></link> in device's Output network",
            value = "129"
        )
    )]
    RatioCarbonDioxideOutput = 129u16,
    #[strum(serialize = "RatioNitrogenOutput")]
    #[strum(
        props(docs = "The ratio of nitrogen in device's Output network", value = "130")
    )]
    RatioNitrogenOutput = 130u16,
    #[strum(serialize = "RatioPollutantOutput")]
    #[strum(
        props(docs = "The ratio of pollutant in device's Output network", value = "131")
    )]
    RatioPollutantOutput = 131u16,
    #[strum(serialize = "RatioVolatilesOutput")]
    #[strum(
        props(docs = "The ratio of volatiles in device's Output network", value = "132")
    )]
    RatioVolatilesOutput = 132u16,
    #[strum(serialize = "RatioWaterOutput")]
    #[strum(
        props(docs = "The ratio of water in device's Output network", value = "133")
    )]
    RatioWaterOutput = 133u16,
    #[strum(serialize = "RatioNitrousOxideOutput")]
    #[strum(
        props(
            docs = "The ratio of <link=GasNitrousOxide><color=#44AD83>Nitrous Oxide</color></link> in device's Output network",
            value = "134"
        )
    )]
    RatioNitrousOxideOutput = 134u16,
    #[strum(serialize = "TotalMolesOutput")]
    #[strum(
        props(
            docs = "Returns the total moles of the device's Output Network",
            value = "135"
        )
    )]
    TotalMolesOutput = 135u16,
    #[strum(serialize = "PressureOutput2")]
    #[strum(
        props(
            docs = "The current pressure reading of the device's Output2 Network",
            value = "136"
        )
    )]
    PressureOutput2 = 136u16,
    #[strum(serialize = "TemperatureOutput2")]
    #[strum(
        props(
            docs = "The current temperature reading of the device's Output2 Network",
            value = "137"
        )
    )]
    TemperatureOutput2 = 137u16,
    #[strum(serialize = "RatioOxygenOutput2")]
    #[strum(
        props(docs = "The ratio of oxygen in device's Output2 network", value = "138")
    )]
    RatioOxygenOutput2 = 138u16,
    #[strum(serialize = "RatioCarbonDioxideOutput2")]
    #[strum(
        props(
            docs = "The ratio of <link=GasCarbonDioxide><color=#44AD83>Carbon Dioxide</color></link> in device's Output2 network",
            value = "139"
        )
    )]
    RatioCarbonDioxideOutput2 = 139u16,
    #[strum(serialize = "RatioNitrogenOutput2")]
    #[strum(
        props(docs = "The ratio of nitrogen in device's Output2 network", value = "140")
    )]
    RatioNitrogenOutput2 = 140u16,
    #[strum(serialize = "RatioPollutantOutput2")]
    #[strum(
        props(docs = "The ratio of pollutant in device's Output2 network", value = "141")
    )]
    RatioPollutantOutput2 = 141u16,
    #[strum(serialize = "RatioVolatilesOutput2")]
    #[strum(
        props(docs = "The ratio of volatiles in device's Output2 network", value = "142")
    )]
    RatioVolatilesOutput2 = 142u16,
    #[strum(serialize = "RatioWaterOutput2")]
    #[strum(
        props(docs = "The ratio of water in device's Output2 network", value = "143")
    )]
    RatioWaterOutput2 = 143u16,
    #[strum(serialize = "RatioNitrousOxideOutput2")]
    #[strum(
        props(
            docs = "The ratio of <link=GasNitrousOxide><color=#44AD83>Nitrous Oxide</color></link> in device's Output2 network",
            value = "144"
        )
    )]
    RatioNitrousOxideOutput2 = 144u16,
    #[strum(serialize = "TotalMolesOutput2")]
    #[strum(
        props(
            docs = "Returns the total moles of the device's Output2 Network",
            value = "145"
        )
    )]
    TotalMolesOutput2 = 145u16,
    #[strum(serialize = "CombustionInput")]
    #[strum(
        props(
            docs = "The assess atmosphere is on fire. Returns 1 if device's input network is on fire, 0 if not.",
            value = "146"
        )
    )]
    CombustionInput = 146u16,
    #[strum(serialize = "CombustionInput2")]
    #[strum(
        props(
            docs = "The assess atmosphere is on fire. Returns 1 if device's Input2 network is on fire, 0 if not.",
            value = "147"
        )
    )]
    CombustionInput2 = 147u16,
    #[strum(serialize = "CombustionOutput")]
    #[strum(
        props(
            docs = "The assess atmosphere is on fire. Returns 1 if device's Output network is on fire, 0 if not.",
            value = "148"
        )
    )]
    CombustionOutput = 148u16,
    #[strum(serialize = "CombustionOutput2")]
    #[strum(
        props(
            docs = "The assess atmosphere is on fire. Returns 1 if device's Output2 network is on fire, 0 if not.",
            value = "149"
        )
    )]
    CombustionOutput2 = 149u16,
    #[strum(serialize = "OperationalTemperatureEfficiency")]
    #[strum(
        props(
            docs = "How the input pipe's temperature effects the machines efficiency",
            value = "150"
        )
    )]
    OperationalTemperatureEfficiency = 150u16,
    #[strum(serialize = "TemperatureDifferentialEfficiency")]
    #[strum(
        props(
            docs = "How the difference between the input pipe and waste pipe temperatures effect the machines efficiency",
            value = "151"
        )
    )]
    TemperatureDifferentialEfficiency = 151u16,
    #[strum(serialize = "PressureEfficiency")]
    #[strum(
        props(
            docs = "How the pressure of the input pipe and waste pipe effect the machines efficiency",
            value = "152"
        )
    )]
    PressureEfficiency = 152u16,
    #[strum(serialize = "CombustionLimiter")]
    #[strum(
        props(
            docs = "Retards the rate of combustion inside the machine (range: 0-100), with 0 being the slowest rate of combustion and 100 being the fastest",
            value = "153"
        )
    )]
    CombustionLimiter = 153u16,
    #[strum(serialize = "Throttle")]
    #[strum(
        props(
            docs = "Increases the rate at which the machine works (range: 0-100)",
            value = "154"
        )
    )]
    Throttle = 154u16,
    #[strum(serialize = "Rpm")]
    #[strum(
        props(
            docs = "The number of revolutions per minute that the device's spinning mechanism is doing",
            value = "155"
        )
    )]
    Rpm = 155u16,
    #[strum(serialize = "Stress")]
    #[strum(
        props(
            docs = "Machines get stressed when working hard. When Stress reaches 100 the machine will automatically shut down",
            value = "156"
        )
    )]
    Stress = 156u16,
    #[strum(serialize = "InterrogationProgress")]
    #[strum(
        props(
            docs = "Progress of this sattellite dish's interrogation of its current target, as a ratio from 0-1",
            value = "157"
        )
    )]
    InterrogationProgress = 157u16,
    #[strum(serialize = "TargetPadIndex")]
    #[strum(
        props(
            docs = "The index of the trader landing pad on this devices data network that it will try to call a trader in to land",
            value = "158"
        )
    )]
    TargetPadIndex = 158u16,
    #[strum(serialize = "SizeX")]
    #[strum(
        props(
            docs = "Size on the X (right) axis of the object in largeGrids (a largeGrid is 2meters)",
            value = "160"
        )
    )]
    SizeX = 160u16,
    #[strum(serialize = "SizeY")]
    #[strum(
        props(
            docs = "Size on the Y(Up) axis of the object in largeGrids (a largeGrid is 2meters)",
            value = "161"
        )
    )]
    SizeY = 161u16,
    #[strum(serialize = "SizeZ")]
    #[strum(
        props(
            docs = "Size on the Z(Forward) axis of the object in largeGrids (a largeGrid is 2meters)",
            value = "162"
        )
    )]
    SizeZ = 162u16,
    #[strum(serialize = "MinimumWattsToContact")]
    #[strum(
        props(
            docs = "Minimum required amount of watts from the dish hitting the target trader contact to start interrogating the contact",
            value = "163"
        )
    )]
    MinimumWattsToContact = 163u16,
    #[strum(serialize = "WattsReachingContact")]
    #[strum(
        props(
            docs = "The amount of watts actually hitting the contact. This is effected by the power of the dish and how far off-axis the dish is from the contact vector",
            value = "164"
        )
    )]
    WattsReachingContact = 164u16,
    #[strum(serialize = "Channel0")]
    #[strum(
        props(
            docs = "Channel on a cable network which should be considered volatile",
            value = "165"
        )
    )]
    Channel0 = 165u16,
    #[strum(serialize = "Channel1")]
    #[strum(
        props(
            docs = "Channel on a cable network which should be considered volatile",
            value = "166"
        )
    )]
    Channel1 = 166u16,
    #[strum(serialize = "Channel2")]
    #[strum(
        props(
            docs = "Channel on a cable network which should be considered volatile",
            value = "167"
        )
    )]
    Channel2 = 167u16,
    #[strum(serialize = "Channel3")]
    #[strum(
        props(
            docs = "Channel on a cable network which should be considered volatile",
            value = "168"
        )
    )]
    Channel3 = 168u16,
    #[strum(serialize = "Channel4")]
    #[strum(
        props(
            docs = "Channel on a cable network which should be considered volatile",
            value = "169"
        )
    )]
    Channel4 = 169u16,
    #[strum(serialize = "Channel5")]
    #[strum(
        props(
            docs = "Channel on a cable network which should be considered volatile",
            value = "170"
        )
    )]
    Channel5 = 170u16,
    #[strum(serialize = "Channel6")]
    #[strum(
        props(
            docs = "Channel on a cable network which should be considered volatile",
            value = "171"
        )
    )]
    Channel6 = 171u16,
    #[strum(serialize = "Channel7")]
    #[strum(
        props(
            docs = "Channel on a cable network which should be considered volatile",
            value = "172"
        )
    )]
    Channel7 = 172u16,
    #[strum(serialize = "LineNumber")]
    #[strum(
        props(
            docs = "The line number of current execution for an integrated circuit running on this device. While this number can be written, use with caution",
            value = "173"
        )
    )]
    LineNumber = 173u16,
    #[strum(serialize = "Flush")]
    #[strum(
        props(
            docs = "Set to 1 to activate the flush function on the device",
            value = "174"
        )
    )]
    Flush = 174u16,
    #[strum(serialize = "SoundAlert")]
    #[strum(props(docs = "Plays a sound alert on the devices speaker", value = "175"))]
    SoundAlert = 175u16,
    #[strum(serialize = "SolarIrradiance")]
    #[strum(props(docs = "<A:EN:-1627927313>", value = "176"))]
    SolarIrradiance = 176u16,
    #[strum(serialize = "RatioLiquidNitrogen")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidNitrogen><color=#44AD83>Liquid Nitrogen</color></link> in device atmosphere",
            value = "177"
        )
    )]
    RatioLiquidNitrogen = 177u16,
    #[strum(serialize = "RatioLiquidNitrogenInput")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidNitrogen><color=#44AD83>Liquid Nitrogen</color></link> in device's input network",
            value = "178"
        )
    )]
    RatioLiquidNitrogenInput = 178u16,
    #[strum(serialize = "RatioLiquidNitrogenInput2")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidNitrogen><color=#44AD83>Liquid Nitrogen</color></link> in device's Input2 network",
            value = "179"
        )
    )]
    RatioLiquidNitrogenInput2 = 179u16,
    #[strum(serialize = "RatioLiquidNitrogenOutput")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidNitrogen><color=#44AD83>Liquid Nitrogen</color></link> in device's Output network",
            value = "180"
        )
    )]
    RatioLiquidNitrogenOutput = 180u16,
    #[strum(serialize = "RatioLiquidNitrogenOutput2")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidNitrogen><color=#44AD83>Liquid Nitrogen</color></link> in device's Output2 network",
            value = "181"
        )
    )]
    RatioLiquidNitrogenOutput2 = 181u16,
    #[strum(serialize = "VolumeOfLiquid")]
    #[strum(
        props(
            docs = "The total volume of all liquids in Liters in the atmosphere",
            value = "182"
        )
    )]
    VolumeOfLiquid = 182u16,
    #[strum(serialize = "RatioLiquidOxygen")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidOxygen><color=#44AD83>Liquid Oxygen</color></link> in device's Atmosphere",
            value = "183"
        )
    )]
    RatioLiquidOxygen = 183u16,
    #[strum(serialize = "RatioLiquidOxygenInput")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidOxygen><color=#44AD83>Liquid Oxygen</color></link> in device's Input Atmosphere",
            value = "184"
        )
    )]
    RatioLiquidOxygenInput = 184u16,
    #[strum(serialize = "RatioLiquidOxygenInput2")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidOxygen><color=#44AD83>Liquid Oxygen</color></link> in device's Input2 Atmosphere",
            value = "185"
        )
    )]
    RatioLiquidOxygenInput2 = 185u16,
    #[strum(serialize = "RatioLiquidOxygenOutput")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidOxygen><color=#44AD83>Liquid Oxygen</color></link> in device's device's Output Atmosphere",
            value = "186"
        )
    )]
    RatioLiquidOxygenOutput = 186u16,
    #[strum(serialize = "RatioLiquidOxygenOutput2")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidOxygen><color=#44AD83>Liquid Oxygen</color></link> in device's Output2 Atmopshere",
            value = "187"
        )
    )]
    RatioLiquidOxygenOutput2 = 187u16,
    #[strum(serialize = "RatioLiquidVolatiles")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidVolatiles><color=#44AD83>Liquid Volatiles</color></link> in device's Atmosphere",
            value = "188"
        )
    )]
    RatioLiquidVolatiles = 188u16,
    #[strum(serialize = "RatioLiquidVolatilesInput")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidVolatiles><color=#44AD83>Liquid Volatiles</color></link> in device's Input Atmosphere",
            value = "189"
        )
    )]
    RatioLiquidVolatilesInput = 189u16,
    #[strum(serialize = "RatioLiquidVolatilesInput2")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidVolatiles><color=#44AD83>Liquid Volatiles</color></link> in device's Input2 Atmosphere",
            value = "190"
        )
    )]
    RatioLiquidVolatilesInput2 = 190u16,
    #[strum(serialize = "RatioLiquidVolatilesOutput")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidVolatiles><color=#44AD83>Liquid Volatiles</color></link> in device's device's Output Atmosphere",
            value = "191"
        )
    )]
    RatioLiquidVolatilesOutput = 191u16,
    #[strum(serialize = "RatioLiquidVolatilesOutput2")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidVolatiles><color=#44AD83>Liquid Volatiles</color></link> in device's Output2 Atmopshere",
            value = "192"
        )
    )]
    RatioLiquidVolatilesOutput2 = 192u16,
    #[strum(serialize = "RatioSteam")]
    #[strum(
        props(
            docs = "The ratio of <link=GasSteam><color=#44AD83>Steam</color></link> in device's Atmosphere",
            value = "193"
        )
    )]
    RatioSteam = 193u16,
    #[strum(serialize = "RatioSteamInput")]
    #[strum(
        props(
            docs = "The ratio of <link=GasSteam><color=#44AD83>Steam</color></link> in device's Input Atmosphere",
            value = "194"
        )
    )]
    RatioSteamInput = 194u16,
    #[strum(serialize = "RatioSteamInput2")]
    #[strum(
        props(
            docs = "The ratio of <link=GasSteam><color=#44AD83>Steam</color></link> in device's Input2 Atmosphere",
            value = "195"
        )
    )]
    RatioSteamInput2 = 195u16,
    #[strum(serialize = "RatioSteamOutput")]
    #[strum(
        props(
            docs = "The ratio of <link=GasSteam><color=#44AD83>Steam</color></link> in device's device's Output Atmosphere",
            value = "196"
        )
    )]
    RatioSteamOutput = 196u16,
    #[strum(serialize = "RatioSteamOutput2")]
    #[strum(
        props(
            docs = "The ratio of <link=GasSteam><color=#44AD83>Steam</color></link> in device's Output2 Atmopshere",
            value = "197"
        )
    )]
    RatioSteamOutput2 = 197u16,
    #[strum(serialize = "ContactTypeId")]
    #[strum(props(docs = "The type id of the contact.", value = "198"))]
    ContactTypeId = 198u16,
    #[strum(serialize = "RatioLiquidCarbonDioxide")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidCarbonDioxide><color=#44AD83>Liquid Carbon Dioxide</color></link> in device's Atmosphere",
            value = "199"
        )
    )]
    RatioLiquidCarbonDioxide = 199u16,
    #[strum(serialize = "RatioLiquidCarbonDioxideInput")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidCarbonDioxide><color=#44AD83>Liquid Carbon Dioxide</color></link> in device's Input Atmosphere",
            value = "200"
        )
    )]
    RatioLiquidCarbonDioxideInput = 200u16,
    #[strum(serialize = "RatioLiquidCarbonDioxideInput2")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidCarbonDioxide><color=#44AD83>Liquid Carbon Dioxide</color></link> in device's Input2 Atmosphere",
            value = "201"
        )
    )]
    RatioLiquidCarbonDioxideInput2 = 201u16,
    #[strum(serialize = "RatioLiquidCarbonDioxideOutput")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidCarbonDioxide><color=#44AD83>Liquid Carbon Dioxide</color></link> in device's device's Output Atmosphere",
            value = "202"
        )
    )]
    RatioLiquidCarbonDioxideOutput = 202u16,
    #[strum(serialize = "RatioLiquidCarbonDioxideOutput2")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidCarbonDioxide><color=#44AD83>Liquid Carbon Dioxide</color></link> in device's Output2 Atmopshere",
            value = "203"
        )
    )]
    RatioLiquidCarbonDioxideOutput2 = 203u16,
    #[strum(serialize = "RatioLiquidPollutant")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidPollutant><color=#44AD83>Liquid Pollutant</color></link> in device's Atmosphere",
            value = "204"
        )
    )]
    RatioLiquidPollutant = 204u16,
    #[strum(serialize = "RatioLiquidPollutantInput")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidPollutant><color=#44AD83>Liquid Pollutant</color></link> in device's Input Atmosphere",
            value = "205"
        )
    )]
    RatioLiquidPollutantInput = 205u16,
    #[strum(serialize = "RatioLiquidPollutantInput2")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidPollutant><color=#44AD83>Liquid Pollutant</color></link> in device's Input2 Atmosphere",
            value = "206"
        )
    )]
    RatioLiquidPollutantInput2 = 206u16,
    #[strum(serialize = "RatioLiquidPollutantOutput")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidPollutant><color=#44AD83>Liquid Pollutant</color></link> in device's device's Output Atmosphere",
            value = "207"
        )
    )]
    RatioLiquidPollutantOutput = 207u16,
    #[strum(serialize = "RatioLiquidPollutantOutput2")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidPollutant><color=#44AD83>Liquid Pollutant</color></link> in device's Output2 Atmopshere",
            value = "208"
        )
    )]
    RatioLiquidPollutantOutput2 = 208u16,
    #[strum(serialize = "RatioLiquidNitrousOxide")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidNitrousOxide><color=#44AD83>Liquid Nitrous Oxide</color></link> in device's Atmosphere",
            value = "209"
        )
    )]
    RatioLiquidNitrousOxide = 209u16,
    #[strum(serialize = "RatioLiquidNitrousOxideInput")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidNitrousOxide><color=#44AD83>Liquid Nitrous Oxide</color></link> in device's Input Atmosphere",
            value = "210"
        )
    )]
    RatioLiquidNitrousOxideInput = 210u16,
    #[strum(serialize = "RatioLiquidNitrousOxideInput2")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidNitrousOxide><color=#44AD83>Liquid Nitrous Oxide</color></link> in device's Input2 Atmosphere",
            value = "211"
        )
    )]
    RatioLiquidNitrousOxideInput2 = 211u16,
    #[strum(serialize = "RatioLiquidNitrousOxideOutput")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidNitrousOxide><color=#44AD83>Liquid Nitrous Oxide</color></link> in device's device's Output Atmosphere",
            value = "212"
        )
    )]
    RatioLiquidNitrousOxideOutput = 212u16,
    #[strum(serialize = "RatioLiquidNitrousOxideOutput2")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidNitrousOxide><color=#44AD83>Liquid Nitrous Oxide</color></link> in device's Output2 Atmopshere",
            value = "213"
        )
    )]
    RatioLiquidNitrousOxideOutput2 = 213u16,
    #[strum(serialize = "Progress")]
    #[strum(
        props(
            docs = "Progress of the rocket to the next node on the map expressed as a value between 0-1.",
            value = "214"
        )
    )]
    Progress = 214u16,
    #[strum(serialize = "DestinationCode")]
    #[strum(
        props(
            docs = "The Space Map Address of the rockets target Space Map Location",
            value = "215"
        )
    )]
    DestinationCode = 215u16,
    #[strum(serialize = "Acceleration")]
    #[strum(
        props(
            docs = "Change in velocity. Rockets that are deccelerating when landing will show this as negative value.",
            value = "216"
        )
    )]
    Acceleration = 216u16,
    #[strum(serialize = "ReferenceId")]
    #[strum(props(docs = "Unique Reference Identifier for this object", value = "217"))]
    ReferenceId = 217u16,
    #[strum(serialize = "AutoShutOff")]
    #[strum(
        props(
            docs = "Turns off all devices in the rocket upon reaching destination",
            value = "218"
        )
    )]
    AutoShutOff = 218u16,
    #[strum(serialize = "Mass")]
    #[strum(
        props(
            docs = "The total Mass of the rocket in kilograms including fuel and cargo. The more massive the rocket the more fuel will be required to move to a new location in space.",
            value = "219"
        )
    )]
    Mass = 219u16,
    #[strum(serialize = "DryMass")]
    #[strum(
        props(
            docs = "The Mass in kilograms of the rocket excluding fuel. The more massive the rocket the more fuel will be required to move to a new location in space.",
            value = "220"
        )
    )]
    DryMass = 220u16,
    #[strum(serialize = "Thrust")]
    #[strum(
        props(
            docs = "Total current thrust of all rocket engines on the rocket in Newtons.",
            value = "221"
        )
    )]
    Thrust = 221u16,
    #[strum(serialize = "Weight")]
    #[strum(
        props(
            docs = "Weight of Rocket in Newtons (Including fuel and cargo). Weight is effected by local body gravity.",
            value = "222"
        )
    )]
    Weight = 222u16,
    #[strum(serialize = "ThrustToWeight")]
    #[strum(
        props(
            docs = "Ratio of thrust to weight of rocket. Weight is effected by local body gravity. A rocket with a low thrust to weight will expend more fuel during launch and landing.",
            value = "223"
        )
    )]
    ThrustToWeight = 223u16,
    #[strum(serialize = "TimeToDestination")]
    #[strum(
        props(
            docs = "Estimated time in seconds until rocket arrives at target destination.",
            value = "224"
        )
    )]
    TimeToDestination = 224u16,
    #[strum(serialize = "BurnTimeRemaining")]
    #[strum(
        props(
            docs = "Estimated time in seconds until fuel is depleted. Calculated based on current fuel usage.",
            value = "225"
        )
    )]
    BurnTimeRemaining = 225u16,
    #[strum(serialize = "AutoLand")]
    #[strum(
        props(
            docs = "Engages the automatic landing algorithm. The rocket will automatically throttle and turn on and off its engines to achieve a smooth landing.",
            value = "226"
        )
    )]
    AutoLand = 226u16,
    #[strum(serialize = "ForwardX")]
    #[strum(
        props(
            docs = "The direction the entity is facing expressed as a normalized vector",
            value = "227"
        )
    )]
    ForwardX = 227u16,
    #[strum(serialize = "ForwardY")]
    #[strum(
        props(
            docs = "The direction the entity is facing expressed as a normalized vector",
            value = "228"
        )
    )]
    ForwardY = 228u16,
    #[strum(serialize = "ForwardZ")]
    #[strum(
        props(
            docs = "The direction the entity is facing expressed as a normalized vector",
            value = "229"
        )
    )]
    ForwardZ = 229u16,
    #[strum(serialize = "Orientation")]
    #[strum(
        props(
            docs = "The orientation of the entity in degrees in a plane relative towards the north origin",
            value = "230"
        )
    )]
    Orientation = 230u16,
    #[strum(serialize = "VelocityX")]
    #[strum(
        props(docs = "The world velocity of the entity in the X axis", value = "231")
    )]
    VelocityX = 231u16,
    #[strum(serialize = "VelocityY")]
    #[strum(
        props(docs = "The world velocity of the entity in the Y axis", value = "232")
    )]
    VelocityY = 232u16,
    #[strum(serialize = "VelocityZ")]
    #[strum(
        props(docs = "The world velocity of the entity in the Z axis", value = "233")
    )]
    VelocityZ = 233u16,
    #[strum(serialize = "PassedMoles")]
    #[strum(
        props(
            docs = "The number of moles that passed through this device on the previous simulation tick",
            value = "234"
        )
    )]
    PassedMoles = 234u16,
    #[strum(serialize = "ExhaustVelocity")]
    #[strum(props(docs = "The velocity of the exhaust gas in m/s", value = "235"))]
    ExhaustVelocity = 235u16,
    #[strum(serialize = "FlightControlRule")]
    #[strum(
        props(
            docs = "Flight control rule of rocket. None = 0, No AutoPilot. Normal = 1, Target Decent Apex of 60m. Alternate = 2, Velocity to High - Full throttle. Alternate2 = 3, Target an appropriate decent velocity as velocity is too low. FinalApproach = 4, Descend towards launch mount in a controlled manner.",
            value = "236"
        )
    )]
    FlightControlRule = 236u16,
    #[strum(serialize = "ReEntryAltitude")]
    #[strum(
        props(
            docs = "The altitude that the rocket will begin its decent to the pad. Must be between 25km and 120km",
            value = "237"
        )
    )]
    ReEntryAltitude = 237u16,
    #[strum(serialize = "Apex")]
    #[strum(
        props(
            docs = "The lowest altitude that the rocket will reach before it starts travelling upwards again.",
            value = "238"
        )
    )]
    Apex = 238u16,
    #[strum(serialize = "EntityState")]
    #[strum(
        props(
            docs = "The current entity state, such as whether it is dead, unconscious or alive, expressed as a state integer.",
            value = "239"
        )
    )]
    EntityState = 239u16,
    #[strum(serialize = "DrillCondition")]
    #[strum(
        props(
            docs = "The current condition of the drill head in this devices drill slot. Expressed as a ratio between 0 and 1.",
            value = "240"
        )
    )]
    DrillCondition = 240u16,
    #[strum(serialize = "Index")]
    #[strum(props(docs = "The current index for the device.", value = "241"))]
    Index = 241u16,
    #[strum(serialize = "CelestialHash")]
    #[strum(
        props(docs = "The current hash of the targeted celestial object.", value = "242")
    )]
    CelestialHash = 242u16,
    #[strum(serialize = "AlignmentError")]
    #[strum(
        props(
            docs = "The angular discrepancy between the telescope's current orientation and the target. Indicates how 'off target' the telescope is. Returns NaN when no target.",
            value = "243"
        )
    )]
    AlignmentError = 243u16,
    #[strum(serialize = "DistanceAu")]
    #[strum(
        props(
            docs = "The current distance to the celestial object, measured in astronomical units.",
            value = "244"
        )
    )]
    DistanceAu = 244u16,
    #[strum(serialize = "OrbitPeriod")]
    #[strum(
        props(
            docs = "The time it takes for an object to complete one full orbit around another object, measured in days. Indicates the duration of the orbital cycle.",
            value = "245"
        )
    )]
    OrbitPeriod = 245u16,
    #[strum(serialize = "Inclination")]
    #[strum(
        props(
            docs = "The tilt of an orbit's plane relative to the equatorial plane, measured in degrees. Defines the orbital plane's angle.",
            value = "246"
        )
    )]
    Inclination = 246u16,
    #[strum(serialize = "Eccentricity")]
    #[strum(
        props(
            docs = "A measure of how elliptical (oval) an orbit is. Ranges from 0 (a perfect circle) to 1 (a parabolic trajectory).",
            value = "247"
        )
    )]
    Eccentricity = 247u16,
    #[strum(serialize = "SemiMajorAxis")]
    #[strum(
        props(
            docs = "The longest radius of an elliptical orbit in astronomical units, measuring half the major axis. Determines the size of the orbit.",
            value = "248"
        )
    )]
    SemiMajorAxis = 248u16,
    #[strum(serialize = "DistanceKm")]
    #[strum(
        props(
            docs = "The current distance to the celestial object, measured in kilometers.",
            value = "249"
        )
    )]
    DistanceKm = 249u16,
    #[strum(serialize = "CelestialParentHash")]
    #[strum(
        props(
            docs = "The hash for the name of the parent the celestial is orbiting, 0 if there is no parent celestial.",
            value = "250"
        )
    )]
    CelestialParentHash = 250u16,
    #[strum(serialize = "TrueAnomaly")]
    #[strum(
        props(
            docs = "An angular parameter that defines the position of a body moving along a Keplerian orbit. It is the angle between the direction of periapsis and the current position of the body, as seen from the main focus of the ellipse (the point around which the object orbits).",
            value = "251"
        )
    )]
    TrueAnomaly = 251u16,
    #[strum(serialize = "RatioHydrogen")]
    #[strum(
        props(
            docs = "The ratio of <link=GasHydrogen><color=#44AD83>Hydrogen</color></link> in device's Atmopshere",
            value = "252"
        )
    )]
    RatioHydrogen = 252u16,
    #[strum(serialize = "RatioLiquidHydrogen")]
    #[strum(
        props(
            docs = "The ratio of <link=GasLiquidHydrogen><color=#44AD83>Liquid Hydrogen</color></link> in device's Atmopshere",
            value = "253"
        )
    )]
    RatioLiquidHydrogen = 253u16,
    #[strum(serialize = "RatioPollutedWater")]
    #[strum(
        props(docs = "The ratio of polluted water in device atmosphere", value = "254")
    )]
    RatioPollutedWater = 254u16,
    #[strum(serialize = "Discover")]
    #[strum(
        props(
            docs = "Progress status of Discovery scan at the rocket's target Space Map Location. Returns a clamped normalised value. If Discovery scan is not available returns -1.",
            value = "255"
        )
    )]
    Discover = 255u16,
    #[strum(serialize = "Chart")]
    #[strum(
        props(
            docs = "Progress status of Chart scan at the rocket's target Space Map Location. Returns a clamped normalised value. If Chart scan is not available returns -1.",
            value = "256"
        )
    )]
    Chart = 256u16,
    #[strum(serialize = "Survey")]
    #[strum(
        props(
            docs = "Progress status of Survey scan at the rocket's target Space Map Location. Returns a normalised value where 100% surveyed is equal to 1. If Survey scan is not available returns -1.",
            value = "257"
        )
    )]
    Survey = 257u16,
    #[strum(serialize = "NavPoints")]
    #[strum(
        props(
            docs = "The number of NavPoints at the rocket's target Space Map Location.",
            value = "258"
        )
    )]
    NavPoints = 258u16,
    #[strum(serialize = "ChartedNavPoints")]
    #[strum(
        props(
            docs = "The number of charted NavPoints at the rocket's target Space Map Location.",
            value = "259"
        )
    )]
    ChartedNavPoints = 259u16,
    #[strum(serialize = "Sites")]
    #[strum(
        props(
            docs = "The number of Sites that have been discovered at the rockets target Space Map location.",
            value = "260"
        )
    )]
    Sites = 260u16,
    #[strum(serialize = "CurrentCode")]
    #[strum(
        props(
            docs = "The Space Map Address of the rockets current Space Map Location",
            value = "261"
        )
    )]
    CurrentCode = 261u16,
    #[strum(serialize = "Density")]
    #[strum(
        props(
            docs = "The density of the rocket's target site's mine-able deposit.",
            value = "262"
        )
    )]
    Density = 262u16,
    #[strum(serialize = "Richness")]
    #[strum(
        props(
            docs = "The richness of the rocket's target site's mine-able deposit.",
            value = "263"
        )
    )]
    Richness = 263u16,
    #[strum(serialize = "Size")]
    #[strum(
        props(
            docs = "The size of the rocket's target site's mine-able deposit.",
            value = "264"
        )
    )]
    Size = 264u16,
    #[strum(serialize = "TotalQuantity")]
    #[strum(
        props(
            docs = "The estimated total quantity of resources available to mine at the rocket's target Space Map Site.",
            value = "265"
        )
    )]
    TotalQuantity = 265u16,
    #[strum(serialize = "MinedQuantity")]
    #[strum(
        props(
            docs = "The total number of resources that have been mined at the rocket's target Space Map Site.",
            value = "266"
        )
    )]
    MinedQuantity = 266u16,
    #[strum(serialize = "BestContactFilter")]
    #[strum(
        props(
            docs = "Filters the satellite's auto selection of targets to a single reference ID.",
            value = "267"
        )
    )]
    BestContactFilter = 267u16,
    #[strum(serialize = "NameHash")]
    #[strum(
        props(
            docs = "Provides the hash value for the name of the object as a 32 bit integer.",
            value = "268"
        )
    )]
    NameHash = 268u16,
}
impl TryFrom<f64> for LogicType {
    type Error = super::ParseError;
    fn try_from(value: f64) -> Result<Self, <LogicType as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = LogicType::iter()
            .find(|enm| (f64::from(*enm as u16) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}
