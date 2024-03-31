
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[serde_as]
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Stack(#[serde_as(as = "[_; 512]")] pub [f64; 512]);

#[serde_as]
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Registers(#[serde_as(as = "[_; 18]")] pub [f64; 18]);

include!(concat!(env!("OUT_DIR"), "/ts_types.rs"));