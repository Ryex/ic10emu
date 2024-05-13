pub mod enums;
pub mod operands;
pub mod traits;

use enums::InstructionOp;
use operands::Operand;
use serde_derive::{Deserialize, Serialize};

use phf::phf_map;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub instruction: InstructionOp,
    pub operands: Vec<Operand>,
}

#[allow(clippy::approx_constant)]
pub static CONSTANTS_LOOKUP: phf::Map<&'static str, f64> = phf_map! {
    "nan" => f64::NAN,
    "ninf" => f64::NEG_INFINITY,
    "deg2rad" => 0.0174532923847437f64,
    "rad2deg" => 57.2957801818848f64,
    "epsilon" => f64::EPSILON,
    "pinf" => f64::INFINITY,
    "pi" => 3.141592653589793f64,
};

