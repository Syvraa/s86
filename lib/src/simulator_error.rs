#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Debug)]
pub enum SimulatorError {
    InvalidMemAccess,
    EndOfInstructions,
}
