use thiserror::Error;

use crate::Slot;

#[derive(Error, Debug)]
pub enum WasmgpError {
    #[error("Invalid slot: {0}")]
    InvalidSlot(Slot),

    #[error(
        "The total number of slots used across all parameters, return and locals must be 256 or fewer, but got {0}"
    )]
    SlotCountTooLarge(usize),
}
