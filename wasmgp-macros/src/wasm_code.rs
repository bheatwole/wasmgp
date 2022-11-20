use crate::slot_count::SlotCount;
use proc_macro2::TokenStream;
use syn::*;

/// This is the main
pub fn handle_macro(slot_count: &SlotCount, inner_fn: &mut ItemFn) -> Result<TokenStream> {
    todo!();
}
