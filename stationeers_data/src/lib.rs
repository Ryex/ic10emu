use std::collections::BTreeMap;

pub mod templates;
pub mod enums {
    use serde_derive::{Deserialize, Serialize};

    #[cfg(feature = "tsify")]
    use tsify::Tsify;
    #[cfg(feature = "tsify")]
    use wasm_bindgen::prelude::*;

    use std::fmt::Display;
    use strum::{AsRefStr, EnumIter, EnumString, FromRepr};

    pub mod basic;
    pub mod prefabs;
    pub mod script;

    #[derive(Debug)]
    pub struct ParseError {
        pub enm: String,
    }

    impl Display for ParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Unknown enum '{}'", self.enm)
        }
    }

    impl std::error::Error for ParseError {}

    #[derive(
        Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumString,
    )]
    #[cfg_attr(feature = "tsify", derive(Tsify))]
    #[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
    pub enum MemoryAccess {
        Read,
        Write,
        ReadWrite,
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
        FromRepr,
        EnumString,
    )]
    #[cfg_attr(feature = "tsify", derive(Tsify))]
    #[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
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
        FromRepr,
        EnumString,
    )]
    #[cfg_attr(feature = "tsify", derive(Tsify))]
    #[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
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
        FromRepr,
        EnumString,
    )]
    #[cfg_attr(feature = "tsify", derive(Tsify))]
    #[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
    #[repr(u32)]
    pub enum MachineTier {
        #[default]
        Undefined = 0,
        TierOne = 1,
        TierTwo = 2,
        TierThree = 3,
        #[serde(other)]
        Max,
    }

    #[derive(
        Default,
        Debug,
        Clone,
        Copy,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        EnumString,
        AsRefStr,
        EnumIter,
        FromRepr,
        Serialize,
        Deserialize,
    )]
    #[cfg_attr(feature = "tsify", derive(Tsify))]
    #[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
    pub enum Species {
        None,
        #[default]
        Human,
        Zrilian,
        Robot,
    }
}

#[must_use]
pub fn build_prefab_database() -> Option<BTreeMap<i32, templates::ObjectTemplate>> {
    #[cfg(feature = "prefab_database")]
    let map = Some(database::build_prefab_database());
    #[cfg(not(feature = "prefab_database"))]
    let map = None;

    map
}

#[cfg(feature = "prefab_database")]
pub mod database {
    mod prefab_map;
    pub use prefab_map::build_prefab_database;
}
