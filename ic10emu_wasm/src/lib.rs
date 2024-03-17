
#[macro_use]
mod utils;

use wasm_bindgen::prelude::*;
use ic10emu::VM;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
    let _vm = VM::new();
    log!("Hello from ic10emu!");
}
