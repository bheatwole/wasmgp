use thiserror::Error;
use wasm_ast::FunctionIndex;

use crate::Slot;

#[derive(Error, Debug)]
pub enum WasmgpError {
    #[error("Invalid function index: {0}")]
    InvalidFunctionIndex(FunctionIndex),

    #[error("Invalid slot: {0}")]
    InvalidSlot(Slot),

    #[error(
        "The total number of slots used across all parameters, return and locals must be 256 or fewer, but got {0}"
    )]
    SlotCountTooLarge(usize),

    #[error("Configuration is not valid ({0})")]
    InvalidConfiguration(String),
}
