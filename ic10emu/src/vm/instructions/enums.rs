use serde_derive::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumProperty, EnumString, FromRepr};
use crate::vm::object::traits::Programmable;
#[cfg(feature = "tsify")]
use tsify::Tsify;
#[cfg(feature = "tsify")]
use wasm_bindgen::prelude::*;
#[derive(
    Debug,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Serialize,
    Deserialize
)]
#[derive(EnumIter, EnumString, EnumProperty, FromRepr)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf, serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum InstructionOp {
    Nop,
    #[strum(
        props(
            example = "abs r? a(r?|num)",
            desc = "Register = the absolute value of a",
            operands = "2"
        )
    )]
    Abs,
    #[strum(
        props(
            example = "acos r? a(r?|num)",
            desc = "Returns the cosine of the specified angle (radians)",
            operands = "2"
        )
    )]
    Acos,
    #[strum(
        props(
            example = "add r? a(r?|num) b(r?|num)",
            desc = "Register = a + b.",
            operands = "3"
        )
    )]
    Add,
    #[strum(
        props(
            example = "alias str r?|d?",
            desc = "Labels register or device reference with name, device references also affect what shows on the screws on the IC base.",
            operands = "2"
        )
    )]
    Alias,
    #[strum(
        props(
            example = "and r? a(r?|num) b(r?|num)",
            desc = "Performs a bitwise logical AND operation on the binary representation of two values. Each bit of the result is determined by evaluating the corresponding bits of the input values. If both bits are 1, the resulting bit is set to 1. Otherwise the resulting bit is set to 0.",
            operands = "3"
        )
    )]
    And,
    #[strum(
        props(
            example = "asin r? a(r?|num)",
            desc = "Returns the angle (radians) whos sine is the specified value",
            operands = "2"
        )
    )]
    Asin,
    #[strum(
        props(
            example = "atan r? a(r?|num)",
            desc = "Returns the angle (radians) whos tan is the specified value",
            operands = "2"
        )
    )]
    Atan,
    #[strum(
        props(
            example = "atan2 r? a(r?|num) b(r?|num)",
            desc = "Returns the angle (radians) whose tangent is the quotient of two specified values: a (y) and b (x)",
            operands = "3"
        )
    )]
    Atan2,
    #[strum(
        props(
            example = "bap a(r?|num) b(r?|num) c(r?|num) d(r?|num)",
            desc = "Branch to line d if abs(a - b) <= max(c * max(abs(a), abs(b)), float.epsilon * 8)",
            operands = "4"
        )
    )]
    Bap,
    #[strum(
        props(
            example = "bapal a(r?|num) b(r?|num) c(r?|num) d(r?|num)",
            desc = "Branch to line c if a != b and store next line number in ra",
            operands = "4"
        )
    )]
    Bapal,
    #[strum(
        props(
            example = "bapz a(r?|num) b(r?|num) c(r?|num)",
            desc = "Branch to line c if abs(a) <= max(b * abs(a), float.epsilon * 8)",
            operands = "3"
        )
    )]
    Bapz,
    #[strum(
        props(
            example = "bapzal a(r?|num) b(r?|num) c(r?|num)",
            desc = "Branch to line c if abs(a) <= max(b * abs(a), float.epsilon * 8) and store next line number in ra",
            operands = "3"
        )
    )]
    Bapzal,
    #[strum(
        props(
            example = "bdns d? a(r?|num)",
            desc = "Branch to line a if device d isn't set",
            operands = "2"
        )
    )]
    Bdns,
    #[strum(
        props(
            example = "bdnsal d? a(r?|num)",
            desc = "Jump execution to line a and store next line number if device is not set",
            operands = "2"
        )
    )]
    Bdnsal,
    #[strum(
        props(
            example = "bdse d? a(r?|num)",
            desc = "Branch to line a if device d is set",
            operands = "2"
        )
    )]
    Bdse,
    #[strum(
        props(
            example = "bdseal d? a(r?|num)",
            desc = "Jump execution to line a and store next line number if device is set",
            operands = "2"
        )
    )]
    Bdseal,
    #[strum(
        props(
            example = "beq a(r?|num) b(r?|num) c(r?|num)",
            desc = "Branch to line c if a == b",
            operands = "3"
        )
    )]
    Beq,
    #[strum(
        props(
            example = "beqal a(r?|num) b(r?|num) c(r?|num)",
            desc = "Branch to line c if a == b and store next line number in ra",
            operands = "3"
        )
    )]
    Beqal,
    #[strum(
        props(
            example = "beqz a(r?|num) b(r?|num)",
            desc = "Branch to line b if a == 0",
            operands = "2"
        )
    )]
    Beqz,
    #[strum(
        props(
            example = "beqzal a(r?|num) b(r?|num)",
            desc = "Branch to line b if a == 0 and store next line number in ra",
            operands = "2"
        )
    )]
    Beqzal,
    #[strum(
        props(
            example = "bge a(r?|num) b(r?|num) c(r?|num)",
            desc = "Branch to line c if a >= b",
            operands = "3"
        )
    )]
    Bge,
    #[strum(
        props(
            example = "bgeal a(r?|num) b(r?|num) c(r?|num)",
            desc = "Branch to line c if a >= b and store next line number in ra",
            operands = "3"
        )
    )]
    Bgeal,
    #[strum(
        props(
            example = "bgez a(r?|num) b(r?|num)",
            desc = "Branch to line b if a >= 0",
            operands = "2"
        )
    )]
    Bgez,
    #[strum(
        props(
            example = "bgezal a(r?|num) b(r?|num)",
            desc = "Branch to line b if a >= 0 and store next line number in ra",
            operands = "2"
        )
    )]
    Bgezal,
    #[strum(
        props(
            example = "bgt a(r?|num) b(r?|num) c(r?|num)",
            desc = "Branch to line c if a > b",
            operands = "3"
        )
    )]
    Bgt,
    #[strum(
        props(
            example = "bgtal a(r?|num) b(r?|num) c(r?|num)",
            desc = "Branch to line c if a > b and store next line number in ra",
            operands = "3"
        )
    )]
    Bgtal,
    #[strum(
        props(
            example = "bgtz a(r?|num) b(r?|num)",
            desc = "Branch to line b if a > 0",
            operands = "2"
        )
    )]
    Bgtz,
    #[strum(
        props(
            example = "bgtzal a(r?|num) b(r?|num)",
            desc = "Branch to line b if a > 0 and store next line number in ra",
            operands = "2"
        )
    )]
    Bgtzal,
    #[strum(
        props(
            example = "ble a(r?|num) b(r?|num) c(r?|num)",
            desc = "Branch to line c if a <= b",
            operands = "3"
        )
    )]
    Ble,
    #[strum(
        props(
            example = "bleal a(r?|num) b(r?|num) c(r?|num)",
            desc = "Branch to line c if a <= b and store next line number in ra",
            operands = "3"
        )
    )]
    Bleal,
    #[strum(
        props(
            example = "blez a(r?|num) b(r?|num)",
            desc = "Branch to line b if a <= 0",
            operands = "2"
        )
    )]
    Blez,
    #[strum(
        props(
            example = "blezal a(r?|num) b(r?|num)",
            desc = "Branch to line b if a <= 0 and store next line number in ra",
            operands = "2"
        )
    )]
    Blezal,
    #[strum(
        props(
            example = "blt a(r?|num) b(r?|num) c(r?|num)",
            desc = "Branch to line c if a < b",
            operands = "3"
        )
    )]
    Blt,
    #[strum(
        props(
            example = "bltal a(r?|num) b(r?|num) c(r?|num)",
            desc = "Branch to line c if a < b and store next line number in ra",
            operands = "3"
        )
    )]
    Bltal,
    #[strum(
        props(
            example = "bltz a(r?|num) b(r?|num)",
            desc = "Branch to line b if a < 0",
            operands = "2"
        )
    )]
    Bltz,
    #[strum(
        props(
            example = "bltzal a(r?|num) b(r?|num)",
            desc = "Branch to line b if a < 0 and store next line number in ra",
            operands = "2"
        )
    )]
    Bltzal,
    #[strum(
        props(
            example = "bna a(r?|num) b(r?|num) c(r?|num) d(r?|num)",
            desc = "Branch to line d if abs(a - b) > max(c * max(abs(a), abs(b)), float.epsilon * 8)",
            operands = "4"
        )
    )]
    Bna,
    #[strum(
        props(
            example = "bnaal a(r?|num) b(r?|num) c(r?|num) d(r?|num)",
            desc = "Branch to line d if abs(a - b) <= max(c * max(abs(a), abs(b)), float.epsilon * 8) and store next line number in ra",
            operands = "4"
        )
    )]
    Bnaal,
    #[strum(
        props(
            example = "bnan a(r?|num) b(r?|num)",
            desc = "Branch to line b if a is not a number (NaN)",
            operands = "2"
        )
    )]
    Bnan,
    #[strum(
        props(
            example = "bnaz a(r?|num) b(r?|num) c(r?|num)",
            desc = "Branch to line c if abs(a) > max (b * abs(a), float.epsilon * 8)",
            operands = "3"
        )
    )]
    Bnaz,
    #[strum(
        props(
            example = "bnazal a(r?|num) b(r?|num) c(r?|num)",
            desc = "Branch to line c if abs(a) > max (b * abs(a), float.epsilon * 8) and store next line number in ra",
            operands = "3"
        )
    )]
    Bnazal,
    #[strum(
        props(
            example = "bne a(r?|num) b(r?|num) c(r?|num)",
            desc = "Branch to line c if a != b",
            operands = "3"
        )
    )]
    Bne,
    #[strum(
        props(
            example = "bneal a(r?|num) b(r?|num) c(r?|num)",
            desc = "Branch to line c if a != b and store next line number in ra",
            operands = "3"
        )
    )]
    Bneal,
    #[strum(
        props(
            example = "bnez a(r?|num) b(r?|num)",
            desc = "branch to line b if a != 0",
            operands = "2"
        )
    )]
    Bnez,
    #[strum(
        props(
            example = "bnezal a(r?|num) b(r?|num)",
            desc = "Branch to line b if a != 0 and store next line number in ra",
            operands = "2"
        )
    )]
    Bnezal,
    #[strum(
        props(
            example = "brap a(r?|num) b(r?|num) c(r?|num) d(r?|num)",
            desc = "Relative branch to line d if abs(a - b) <= max(c * max(abs(a), abs(b)), float.epsilon * 8)",
            operands = "4"
        )
    )]
    Brap,
    #[strum(
        props(
            example = "brapz a(r?|num) b(r?|num) c(r?|num)",
            desc = "Relative branch to line c if abs(a) <= max(b * abs(a), float.epsilon * 8)",
            operands = "3"
        )
    )]
    Brapz,
    #[strum(
        props(
            example = "brdns d? a(r?|num)",
            desc = "Relative jump to line a if device is not set",
            operands = "2"
        )
    )]
    Brdns,
    #[strum(
        props(
            example = "brdse d? a(r?|num)",
            desc = "Relative jump to line a if device is set",
            operands = "2"
        )
    )]
    Brdse,
    #[strum(
        props(
            example = "breq a(r?|num) b(r?|num) c(r?|num)",
            desc = "Relative branch to line c if a == b",
            operands = "3"
        )
    )]
    Breq,
    #[strum(
        props(
            example = "breqz a(r?|num) b(r?|num)",
            desc = "Relative branch to line b if a == 0",
            operands = "2"
        )
    )]
    Breqz,
    #[strum(
        props(
            example = "brge a(r?|num) b(r?|num) c(r?|num)",
            desc = "Relative jump to line c if a >= b",
            operands = "3"
        )
    )]
    Brge,
    #[strum(
        props(
            example = "brgez a(r?|num) b(r?|num)",
            desc = "Relative branch to line b if a >= 0",
            operands = "2"
        )
    )]
    Brgez,
    #[strum(
        props(
            example = "brgt a(r?|num) b(r?|num) c(r?|num)",
            desc = "relative jump to line c if a > b",
            operands = "3"
        )
    )]
    Brgt,
    #[strum(
        props(
            example = "brgtz a(r?|num) b(r?|num)",
            desc = "Relative branch to line b if a > 0",
            operands = "2"
        )
    )]
    Brgtz,
    #[strum(
        props(
            example = "brle a(r?|num) b(r?|num) c(r?|num)",
            desc = "Relative jump to line c if a <= b",
            operands = "3"
        )
    )]
    Brle,
    #[strum(
        props(
            example = "brlez a(r?|num) b(r?|num)",
            desc = "Relative branch to line b if a <= 0",
            operands = "2"
        )
    )]
    Brlez,
    #[strum(
        props(
            example = "brlt a(r?|num) b(r?|num) c(r?|num)",
            desc = "Relative jump to line c if a < b",
            operands = "3"
        )
    )]
    Brlt,
    #[strum(
        props(
            example = "brltz a(r?|num) b(r?|num)",
            desc = "Relative branch to line b if a < 0",
            operands = "2"
        )
    )]
    Brltz,
    #[strum(
        props(
            example = "brna a(r?|num) b(r?|num) c(r?|num) d(r?|num)",
            desc = "Relative branch to line d if abs(a - b) > max(c * max(abs(a), abs(b)), float.epsilon * 8)",
            operands = "4"
        )
    )]
    Brna,
    #[strum(
        props(
            example = "brnan a(r?|num) b(r?|num)",
            desc = "Relative branch to line b if a is not a number (NaN)",
            operands = "2"
        )
    )]
    Brnan,
    #[strum(
        props(
            example = "brnaz a(r?|num) b(r?|num) c(r?|num)",
            desc = "Relative branch to line c if abs(a) > max(b * abs(a), float.epsilon * 8)",
            operands = "3"
        )
    )]
    Brnaz,
    #[strum(
        props(
            example = "brne a(r?|num) b(r?|num) c(r?|num)",
            desc = "Relative branch to line c if a != b",
            operands = "3"
        )
    )]
    Brne,
    #[strum(
        props(
            example = "brnez a(r?|num) b(r?|num)",
            desc = "Relative branch to line b if a != 0",
            operands = "2"
        )
    )]
    Brnez,
    #[strum(
        props(
            example = "ceil r? a(r?|num)",
            desc = "Register = smallest integer greater than a",
            operands = "2"
        )
    )]
    Ceil,
    #[strum(
        props(
            example = "clr d?",
            desc = "Clears the stack memory for the provided device.",
            operands = "1"
        )
    )]
    Clr,
    #[strum(
        props(
            example = "clrd id(r?|num)",
            desc = "Seeks directly for the provided device id and clears the stack memory of that device",
            operands = "1"
        )
    )]
    Clrd,
    #[strum(
        props(
            example = "cos r? a(r?|num)",
            desc = "Returns the cosine of the specified angle (radians)",
            operands = "2"
        )
    )]
    Cos,
    #[strum(
        props(
            example = "define str num",
            desc = "Creates a label that will be replaced throughout the program with the provided value.",
            operands = "2"
        )
    )]
    Define,
    #[strum(
        props(
            example = "div r? a(r?|num) b(r?|num)",
            desc = "Register = a / b",
            operands = "3"
        )
    )]
    Div,
    #[strum(
        props(
            example = "exp r? a(r?|num)",
            desc = "Register = exp(a) or e^a",
            operands = "2"
        )
    )]
    Exp,
    #[strum(
        props(
            example = "floor r? a(r?|num)",
            desc = "Register = largest integer less than a",
            operands = "2"
        )
    )]
    Floor,
    #[strum(
        props(
            example = "get r? d? address(r?|num)",
            desc = "Using the provided device, attempts to read the stack value at the provided address, and places it in the register.",
            operands = "3"
        )
    )]
    Get,
    #[strum(
        props(
            example = "getd r? id(r?|num) address(r?|num)",
            desc = "Seeks directly for the provided device id, attempts to read the stack value at the provided address, and places it in the register.",
            operands = "3"
        )
    )]
    Getd,
    #[strum(props(example = "hcf", desc = "Halt and catch fire", operands = "0"))]
    Hcf,
    #[strum(props(example = "j int", desc = "Jump execution to line a", operands = "1"))]
    J,
    #[strum(
        props(
            example = "jal int",
            desc = "Jump execution to line a and store next line number in ra",
            operands = "1"
        )
    )]
    Jal,
    #[strum(props(example = "jr int", desc = "Relative jump to line a", operands = "1"))]
    Jr,
    #[strum(
        props(
            example = "l r? d? logicType",
            desc = "Loads device LogicType to register by housing index value.",
            operands = "3"
        )
    )]
    L,
    #[strum(props(example = "label d? str", desc = "DEPRECATED", operands = "2"))]
    Label,
    #[strum(
        props(
            example = "lb r? deviceHash logicType batchMode",
            desc = "Loads LogicType from all output network devices with provided type hash using the provide batch mode. Average (0), Sum (1), Minimum (2), Maximum (3). Can use either the word, or the number.",
            operands = "4"
        )
    )]
    Lb,
    #[strum(
        props(
            example = "lbn r? deviceHash nameHash logicType batchMode",
            desc = "Loads LogicType from all output network devices with provided type and name hashes using the provide batch mode. Average (0), Sum (1), Minimum (2), Maximum (3). Can use either the word, or the number.",
            operands = "5"
        )
    )]
    Lbn,
    #[strum(
        props(
            example = "lbns r? deviceHash nameHash slotIndex logicSlotType batchMode",
            desc = "Loads LogicSlotType from slotIndex from all output network devices with provided type and name hashes using the provide batch mode. Average (0), Sum (1), Minimum (2), Maximum (3). Can use either the word, or the number.",
            operands = "6"
        )
    )]
    Lbns,
    #[strum(
        props(
            example = "lbs r? deviceHash slotIndex logicSlotType batchMode",
            desc = "Loads LogicSlotType from slotIndex from all output network devices with provided type hash using the provide batch mode. Average (0), Sum (1), Minimum (2), Maximum (3). Can use either the word, or the number.",
            operands = "5"
        )
    )]
    Lbs,
    #[strum(
        props(
            example = "ld r? id(r?|num) logicType",
            desc = "Loads device LogicType to register by direct ID reference.",
            operands = "3"
        )
    )]
    Ld,
    #[strum(
        props(
            example = "log r? a(r?|num)",
            desc = "Register = base e log(a) or ln(a)",
            operands = "2"
        )
    )]
    Log,
    #[strum(
        props(
            example = "lr r? d? reagentMode int",
            desc = "Loads reagent of device's ReagentMode where a hash of the reagent type to check for. ReagentMode can be either Contents (0), Required (1), Recipe (2). Can use either the word, or the number.",
            operands = "4"
        )
    )]
    Lr,
    #[strum(
        props(
            example = "ls r? d? slotIndex logicSlotType",
            desc = "Loads slot LogicSlotType on device to register.",
            operands = "4"
        )
    )]
    Ls,
    #[strum(
        props(
            example = "max r? a(r?|num) b(r?|num)",
            desc = "Register = max of a or b",
            operands = "3"
        )
    )]
    Max,
    #[strum(
        props(
            example = "min r? a(r?|num) b(r?|num)",
            desc = "Register = min of a or b",
            operands = "3"
        )
    )]
    Min,
    #[strum(
        props(
            example = "mod r? a(r?|num) b(r?|num)",
            desc = "Register = a mod b (note: NOT a % b)",
            operands = "3"
        )
    )]
    Mod,
    #[strum(
        props(
            example = "move r? a(r?|num)",
            desc = "Register = provided num or register value.",
            operands = "2"
        )
    )]
    Move,
    #[strum(
        props(
            example = "mul r? a(r?|num) b(r?|num)",
            desc = "Register = a * b",
            operands = "3"
        )
    )]
    Mul,
    #[strum(
        props(
            example = "nor r? a(r?|num) b(r?|num)",
            desc = "Performs a bitwise logical NOR (NOT OR) operation on the binary representation of two values. Each bit of the result is determined by evaluating the corresponding bits of the input values. If both bits are 0, the resulting bit is set to 1. Otherwise, if at least one bit is 1, the resulting bit is set to 0.",
            operands = "3"
        )
    )]
    Nor,
    #[strum(
        props(
            example = "not r? a(r?|num)",
            desc = "Performs a bitwise logical NOT operation flipping each bit of the input value, resulting in a binary complement. If a bit is 1, it becomes 0, and if a bit is 0, it becomes 1.",
            operands = "2"
        )
    )]
    Not,
    #[strum(
        props(
            example = "or r? a(r?|num) b(r?|num)",
            desc = "Performs a bitwise logical OR operation on the binary representation of two values. Each bit of the result is determined by evaluating the corresponding bits of the input values. If either bit is 1, the resulting bit is set to 1. If both bits are 0, the resulting bit is set to 0.",
            operands = "3"
        )
    )]
    Or,
    #[strum(
        props(
            example = "peek r?",
            desc = "Register = the value at the top of the stack",
            operands = "1"
        )
    )]
    Peek,
    #[strum(
        props(
            example = "poke address(r?|num) value(r?|num)",
            desc = "Stores the provided value at the provided address in the stack.",
            operands = "2"
        )
    )]
    Poke,
    #[strum(
        props(
            example = "pop r?",
            desc = "Register = the value at the top of the stack and decrements sp",
            operands = "1"
        )
    )]
    Pop,
    #[strum(
        props(
            example = "push a(r?|num)",
            desc = "Pushes the value of a to the stack at sp and increments sp",
            operands = "1"
        )
    )]
    Push,
    #[strum(
        props(
            example = "put d? address(r?|num) value(r?|num)",
            desc = "Using the provided device, attempts to write the provided value to the stack at the provided address.",
            operands = "3"
        )
    )]
    Put,
    #[strum(
        props(
            example = "putd id(r?|num) address(r?|num) value(r?|num)",
            desc = "Seeks directly for the provided device id, attempts to write the provided value to the stack at the provided address.",
            operands = "3"
        )
    )]
    Putd,
    #[strum(
        props(
            example = "rand r?",
            desc = "Register = a random value x with 0 <= x < 1",
            operands = "1"
        )
    )]
    Rand,
    #[strum(
        props(
            example = "round r? a(r?|num)",
            desc = "Register = a rounded to nearest integer",
            operands = "2"
        )
    )]
    Round,
    #[strum(
        props(
            example = "s d? logicType r?",
            desc = "Stores register value to LogicType on device by housing index value.",
            operands = "3"
        )
    )]
    S,
    #[strum(
        props(
            example = "sap r? a(r?|num) b(r?|num) c(r?|num)",
            desc = "Register = 1 if abs(a - b) <= max(c * max(abs(a), abs(b)), float.epsilon * 8), otherwise 0",
            operands = "4"
        )
    )]
    Sap,
    #[strum(
        props(
            example = "sapz r? a(r?|num) b(r?|num)",
            desc = "Register = 1 if abs(a) <= max(b * abs(a), float.epsilon * 8), otherwise 0",
            operands = "3"
        )
    )]
    Sapz,
    #[strum(
        props(
            example = "sb deviceHash logicType r?",
            desc = "Stores register value to LogicType on all output network devices with provided type hash.",
            operands = "3"
        )
    )]
    Sb,
    #[strum(
        props(
            example = "sbn deviceHash nameHash logicType r?",
            desc = "Stores register value to LogicType on all output network devices with provided type hash and name.",
            operands = "4"
        )
    )]
    Sbn,
    #[strum(
        props(
            example = "sbs deviceHash slotIndex logicSlotType r?",
            desc = "Stores register value to LogicSlotType on all output network devices with provided type hash in the provided slot.",
            operands = "4"
        )
    )]
    Sbs,
    #[strum(
        props(
            example = "sd id(r?|num) logicType r?",
            desc = "Stores register value to LogicType on device by direct ID reference.",
            operands = "3"
        )
    )]
    Sd,
    #[strum(
        props(
            example = "sdns r? d?",
            desc = "Register = 1 if device is not set, otherwise 0",
            operands = "2"
        )
    )]
    Sdns,
    #[strum(
        props(
            example = "sdse r? d?",
            desc = "Register = 1 if device is set, otherwise 0.",
            operands = "2"
        )
    )]
    Sdse,
    #[strum(
        props(
            example = "select r? a(r?|num) b(r?|num) c(r?|num)",
            desc = "Register = b if a is non-zero, otherwise c",
            operands = "4"
        )
    )]
    Select,
    #[strum(
        props(
            example = "seq r? a(r?|num) b(r?|num)",
            desc = "Register = 1 if a == b, otherwise 0",
            operands = "3"
        )
    )]
    Seq,
    #[strum(
        props(
            example = "seqz r? a(r?|num)",
            desc = "Register = 1 if a == 0, otherwise 0",
            operands = "2"
        )
    )]
    Seqz,
    #[strum(
        props(
            example = "sge r? a(r?|num) b(r?|num)",
            desc = "Register = 1 if a >= b, otherwise 0",
            operands = "3"
        )
    )]
    Sge,
    #[strum(
        props(
            example = "sgez r? a(r?|num)",
            desc = "Register = 1 if a >= 0, otherwise 0",
            operands = "2"
        )
    )]
    Sgez,
    #[strum(
        props(
            example = "sgt r? a(r?|num) b(r?|num)",
            desc = "Register = 1 if a > b, otherwise 0",
            operands = "3"
        )
    )]
    Sgt,
    #[strum(
        props(
            example = "sgtz r? a(r?|num)",
            desc = "Register = 1 if a > 0, otherwise 0",
            operands = "2"
        )
    )]
    Sgtz,
    #[strum(
        props(
            example = "sin r? a(r?|num)",
            desc = "Returns the sine of the specified angle (radians)",
            operands = "2"
        )
    )]
    Sin,
    #[strum(
        props(
            example = "sla r? a(r?|num) b(r?|num)",
            desc = "Performs a bitwise arithmetic left shift operation on the binary representation of a value. It shifts the bits to the left and fills the vacated rightmost bits with a copy of the sign bit (the most significant bit).",
            operands = "3"
        )
    )]
    Sla,
    #[strum(
        props(
            example = "sle r? a(r?|num) b(r?|num)",
            desc = "Register = 1 if a <= b, otherwise 0",
            operands = "3"
        )
    )]
    Sle,
    #[strum(
        props(
            example = "sleep a(r?|num)",
            desc = "Pauses execution on the IC for a seconds",
            operands = "1"
        )
    )]
    Sleep,
    #[strum(
        props(
            example = "slez r? a(r?|num)",
            desc = "Register = 1 if a <= 0, otherwise 0",
            operands = "2"
        )
    )]
    Slez,
    #[strum(
        props(
            example = "sll r? a(r?|num) b(r?|num)",
            desc = "Performs a bitwise logical left shift operation on the binary representation of a value. It shifts the bits to the left and fills the vacated rightmost bits with zeros.",
            operands = "3"
        )
    )]
    Sll,
    #[strum(
        props(
            example = "slt r? a(r?|num) b(r?|num)",
            desc = "Register = 1 if a < b, otherwise 0",
            operands = "3"
        )
    )]
    Slt,
    #[strum(
        props(
            example = "sltz r? a(r?|num)",
            desc = "Register = 1 if a < 0, otherwise 0",
            operands = "2"
        )
    )]
    Sltz,
    #[strum(
        props(
            example = "sna r? a(r?|num) b(r?|num) c(r?|num)",
            desc = "Register = 1 if abs(a - b) > max(c * max(abs(a), abs(b)), float.epsilon * 8), otherwise 0",
            operands = "4"
        )
    )]
    Sna,
    #[strum(
        props(
            example = "snan r? a(r?|num)",
            desc = "Register = 1 if a is NaN, otherwise 0",
            operands = "2"
        )
    )]
    Snan,
    #[strum(
        props(
            example = "snanz r? a(r?|num)",
            desc = "Register = 0 if a is NaN, otherwise 1",
            operands = "2"
        )
    )]
    Snanz,
    #[strum(
        props(
            example = "snaz r? a(r?|num) b(r?|num)",
            desc = "Register = 1 if abs(a) > max(b * abs(a), float.epsilon), otherwise 0",
            operands = "3"
        )
    )]
    Snaz,
    #[strum(
        props(
            example = "sne r? a(r?|num) b(r?|num)",
            desc = "Register = 1 if a != b, otherwise 0",
            operands = "3"
        )
    )]
    Sne,
    #[strum(
        props(
            example = "snez r? a(r?|num)",
            desc = "Register = 1 if a != 0, otherwise 0",
            operands = "2"
        )
    )]
    Snez,
    #[strum(
        props(
            example = "sqrt r? a(r?|num)",
            desc = "Register = square root of a",
            operands = "2"
        )
    )]
    Sqrt,
    #[strum(
        props(
            example = "sra r? a(r?|num) b(r?|num)",
            desc = "Performs a bitwise arithmetic right shift operation on the binary representation of a value. It shifts the bits to the right and fills the vacated leftmost bits with a copy of the sign bit (the most significant bit).",
            operands = "3"
        )
    )]
    Sra,
    #[strum(
        props(
            example = "srl r? a(r?|num) b(r?|num)",
            desc = "Performs a bitwise logical right shift operation on the binary representation of a value. It shifts the bits to the right and fills the vacated leftmost bits with zeros",
            operands = "3"
        )
    )]
    Srl,
    #[strum(
        props(
            example = "ss d? slotIndex logicSlotType r?",
            desc = "Stores register value to device stored in a slot LogicSlotType on device.",
            operands = "4"
        )
    )]
    Ss,
    #[strum(
        props(
            example = "sub r? a(r?|num) b(r?|num)",
            desc = "Register = a - b.",
            operands = "3"
        )
    )]
    Sub,
    #[strum(
        props(
            example = "tan r? a(r?|num)",
            desc = "Returns the tan of the specified angle (radians) ",
            operands = "2"
        )
    )]
    Tan,
    #[strum(
        props(
            example = "trunc r? a(r?|num)",
            desc = "Register = a with fractional part removed",
            operands = "2"
        )
    )]
    Trunc,
    #[strum(
        props(
            example = "xor r? a(r?|num) b(r?|num)",
            desc = "Performs a bitwise logical XOR (exclusive OR) operation on the binary representation of two values. Each bit of the result is determined by evaluating the corresponding bits of the input values. If the bits are different (one bit is 0 and the other is 1), the resulting bit is set to 1. If the bits are the same (both 0 or both 1), the resulting bit is set to 0.",
            operands = "3"
        )
    )]
    Xor,
    #[strum(
        props(example = "yield", desc = "Pauses execution for 1 tick", operands = "0")
    )]
    Yield,
}
impl InstructionOp {
    pub fn num_operands(&self) -> usize {
        self.get_str("operands")
            .expect("instruction without operand property")
            .parse::<usize>()
            .expect("invalid instruction operand property")
    }
    pub fn execute<T>(
        &self,
        ic: &mut T,
        operands: &[crate::vm::instructions::operands::Operand],
    ) -> Result<(), crate::errors::ICError>
    where
        T: Programmable,
    {
        let num_operands = self.num_operands();
        if operands.len() != num_operands {
            return Err(
                crate::errors::ICError::mismatch_operands(
                    operands.len(),
                    num_operands as u32,
                ),
            );
        }
        match self {
            Self::Nop => Ok(()),
            Self::Abs => ic.execute_abs(&operands[0usize], &operands[1usize]),
            Self::Acos => ic.execute_acos(&operands[0usize], &operands[1usize]),
            Self::Add => {
                ic.execute_add(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Alias => ic.execute_alias(&operands[0usize], &operands[1usize]),
            Self::And => {
                ic.execute_and(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Asin => ic.execute_asin(&operands[0usize], &operands[1usize]),
            Self::Atan => ic.execute_atan(&operands[0usize], &operands[1usize]),
            Self::Atan2 => {
                ic.execute_atan2(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Bap => {
                ic.execute_bap(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                    &operands[3usize],
                )
            }
            Self::Bapal => {
                ic.execute_bapal(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                    &operands[3usize],
                )
            }
            Self::Bapz => {
                ic.execute_bapz(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Bapzal => {
                ic.execute_bapzal(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                )
            }
            Self::Bdns => ic.execute_bdns(&operands[0usize], &operands[1usize]),
            Self::Bdnsal => ic.execute_bdnsal(&operands[0usize], &operands[1usize]),
            Self::Bdse => ic.execute_bdse(&operands[0usize], &operands[1usize]),
            Self::Bdseal => ic.execute_bdseal(&operands[0usize], &operands[1usize]),
            Self::Beq => {
                ic.execute_beq(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Beqal => {
                ic.execute_beqal(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Beqz => ic.execute_beqz(&operands[0usize], &operands[1usize]),
            Self::Beqzal => ic.execute_beqzal(&operands[0usize], &operands[1usize]),
            Self::Bge => {
                ic.execute_bge(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Bgeal => {
                ic.execute_bgeal(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Bgez => ic.execute_bgez(&operands[0usize], &operands[1usize]),
            Self::Bgezal => ic.execute_bgezal(&operands[0usize], &operands[1usize]),
            Self::Bgt => {
                ic.execute_bgt(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Bgtal => {
                ic.execute_bgtal(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Bgtz => ic.execute_bgtz(&operands[0usize], &operands[1usize]),
            Self::Bgtzal => ic.execute_bgtzal(&operands[0usize], &operands[1usize]),
            Self::Ble => {
                ic.execute_ble(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Bleal => {
                ic.execute_bleal(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Blez => ic.execute_blez(&operands[0usize], &operands[1usize]),
            Self::Blezal => ic.execute_blezal(&operands[0usize], &operands[1usize]),
            Self::Blt => {
                ic.execute_blt(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Bltal => {
                ic.execute_bltal(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Bltz => ic.execute_bltz(&operands[0usize], &operands[1usize]),
            Self::Bltzal => ic.execute_bltzal(&operands[0usize], &operands[1usize]),
            Self::Bna => {
                ic.execute_bna(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                    &operands[3usize],
                )
            }
            Self::Bnaal => {
                ic.execute_bnaal(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                    &operands[3usize],
                )
            }
            Self::Bnan => ic.execute_bnan(&operands[0usize], &operands[1usize]),
            Self::Bnaz => {
                ic.execute_bnaz(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Bnazal => {
                ic.execute_bnazal(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                )
            }
            Self::Bne => {
                ic.execute_bne(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Bneal => {
                ic.execute_bneal(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Bnez => ic.execute_bnez(&operands[0usize], &operands[1usize]),
            Self::Bnezal => ic.execute_bnezal(&operands[0usize], &operands[1usize]),
            Self::Brap => {
                ic.execute_brap(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                    &operands[3usize],
                )
            }
            Self::Brapz => {
                ic.execute_brapz(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Brdns => ic.execute_brdns(&operands[0usize], &operands[1usize]),
            Self::Brdse => ic.execute_brdse(&operands[0usize], &operands[1usize]),
            Self::Breq => {
                ic.execute_breq(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Breqz => ic.execute_breqz(&operands[0usize], &operands[1usize]),
            Self::Brge => {
                ic.execute_brge(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Brgez => ic.execute_brgez(&operands[0usize], &operands[1usize]),
            Self::Brgt => {
                ic.execute_brgt(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Brgtz => ic.execute_brgtz(&operands[0usize], &operands[1usize]),
            Self::Brle => {
                ic.execute_brle(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Brlez => ic.execute_brlez(&operands[0usize], &operands[1usize]),
            Self::Brlt => {
                ic.execute_brlt(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Brltz => ic.execute_brltz(&operands[0usize], &operands[1usize]),
            Self::Brna => {
                ic.execute_brna(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                    &operands[3usize],
                )
            }
            Self::Brnan => ic.execute_brnan(&operands[0usize], &operands[1usize]),
            Self::Brnaz => {
                ic.execute_brnaz(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Brne => {
                ic.execute_brne(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Brnez => ic.execute_brnez(&operands[0usize], &operands[1usize]),
            Self::Ceil => ic.execute_ceil(&operands[0usize], &operands[1usize]),
            Self::Clr => ic.execute_clr(&operands[0usize]),
            Self::Clrd => ic.execute_clrd(&operands[0usize]),
            Self::Cos => ic.execute_cos(&operands[0usize], &operands[1usize]),
            Self::Define => ic.execute_define(&operands[0usize], &operands[1usize]),
            Self::Div => {
                ic.execute_div(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Exp => ic.execute_exp(&operands[0usize], &operands[1usize]),
            Self::Floor => ic.execute_floor(&operands[0usize], &operands[1usize]),
            Self::Get => {
                ic.execute_get(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Getd => {
                ic.execute_getd(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Hcf => ic.execute_hcf(),
            Self::J => ic.execute_j(&operands[0usize]),
            Self::Jal => ic.execute_jal(&operands[0usize]),
            Self::Jr => ic.execute_jr(&operands[0usize]),
            Self::L => {
                ic.execute_l(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Label => ic.execute_label(&operands[0usize], &operands[1usize]),
            Self::Lb => {
                ic.execute_lb(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                    &operands[3usize],
                )
            }
            Self::Lbn => {
                ic.execute_lbn(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                    &operands[3usize],
                    &operands[4usize],
                )
            }
            Self::Lbns => {
                ic.execute_lbns(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                    &operands[3usize],
                    &operands[4usize],
                    &operands[5usize],
                )
            }
            Self::Lbs => {
                ic.execute_lbs(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                    &operands[3usize],
                    &operands[4usize],
                )
            }
            Self::Ld => {
                ic.execute_ld(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Log => ic.execute_log(&operands[0usize], &operands[1usize]),
            Self::Lr => {
                ic.execute_lr(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                    &operands[3usize],
                )
            }
            Self::Ls => {
                ic.execute_ls(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                    &operands[3usize],
                )
            }
            Self::Max => {
                ic.execute_max(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Min => {
                ic.execute_min(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Mod => {
                ic.execute_mod(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Move => ic.execute_move(&operands[0usize], &operands[1usize]),
            Self::Mul => {
                ic.execute_mul(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Nor => {
                ic.execute_nor(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Not => ic.execute_not(&operands[0usize], &operands[1usize]),
            Self::Or => {
                ic.execute_or(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Peek => ic.execute_peek(&operands[0usize]),
            Self::Poke => ic.execute_poke(&operands[0usize], &operands[1usize]),
            Self::Pop => ic.execute_pop(&operands[0usize]),
            Self::Push => ic.execute_push(&operands[0usize]),
            Self::Put => {
                ic.execute_put(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Putd => {
                ic.execute_putd(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Rand => ic.execute_rand(&operands[0usize]),
            Self::Round => ic.execute_round(&operands[0usize], &operands[1usize]),
            Self::S => {
                ic.execute_s(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Sap => {
                ic.execute_sap(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                    &operands[3usize],
                )
            }
            Self::Sapz => {
                ic.execute_sapz(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Sb => {
                ic.execute_sb(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Sbn => {
                ic.execute_sbn(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                    &operands[3usize],
                )
            }
            Self::Sbs => {
                ic.execute_sbs(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                    &operands[3usize],
                )
            }
            Self::Sd => {
                ic.execute_sd(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Sdns => ic.execute_sdns(&operands[0usize], &operands[1usize]),
            Self::Sdse => ic.execute_sdse(&operands[0usize], &operands[1usize]),
            Self::Select => {
                ic.execute_select(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                    &operands[3usize],
                )
            }
            Self::Seq => {
                ic.execute_seq(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Seqz => ic.execute_seqz(&operands[0usize], &operands[1usize]),
            Self::Sge => {
                ic.execute_sge(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Sgez => ic.execute_sgez(&operands[0usize], &operands[1usize]),
            Self::Sgt => {
                ic.execute_sgt(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Sgtz => ic.execute_sgtz(&operands[0usize], &operands[1usize]),
            Self::Sin => ic.execute_sin(&operands[0usize], &operands[1usize]),
            Self::Sla => {
                ic.execute_sla(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Sle => {
                ic.execute_sle(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Sleep => ic.execute_sleep(&operands[0usize]),
            Self::Slez => ic.execute_slez(&operands[0usize], &operands[1usize]),
            Self::Sll => {
                ic.execute_sll(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Slt => {
                ic.execute_slt(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Sltz => ic.execute_sltz(&operands[0usize], &operands[1usize]),
            Self::Sna => {
                ic.execute_sna(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                    &operands[3usize],
                )
            }
            Self::Snan => ic.execute_snan(&operands[0usize], &operands[1usize]),
            Self::Snanz => ic.execute_snanz(&operands[0usize], &operands[1usize]),
            Self::Snaz => {
                ic.execute_snaz(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Sne => {
                ic.execute_sne(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Snez => ic.execute_snez(&operands[0usize], &operands[1usize]),
            Self::Sqrt => ic.execute_sqrt(&operands[0usize], &operands[1usize]),
            Self::Sra => {
                ic.execute_sra(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Srl => {
                ic.execute_srl(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Ss => {
                ic.execute_ss(
                    &operands[0usize],
                    &operands[1usize],
                    &operands[2usize],
                    &operands[3usize],
                )
            }
            Self::Sub => {
                ic.execute_sub(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Tan => ic.execute_tan(&operands[0usize], &operands[1usize]),
            Self::Trunc => ic.execute_trunc(&operands[0usize], &operands[1usize]),
            Self::Xor => {
                ic.execute_xor(&operands[0usize], &operands[1usize], &operands[2usize])
            }
            Self::Yield => ic.execute_yield(),
        }
    }
}
