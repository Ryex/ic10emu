use std::collections::BTreeMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(rename = "Enums")]
pub struct Enums {
    #[serde(rename = "scriptEnums")]
    pub script_enums: BTreeMap<String, EnumListing>,
    #[serde(rename = "basicEnums")]
    pub basic_enums: BTreeMap<String, EnumListing>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(rename = "EnumListing")]
pub struct EnumListing {
    #[serde(rename = "enumName")]
    pub enum_name: String,
    pub values: BTreeMap<String, EnumEntry>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(rename = "EnumEntry")]
pub struct EnumEntry {
    pub value: i64,
    pub deprecated: bool,
    pub description: String,
}
