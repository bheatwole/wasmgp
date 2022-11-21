use crate::slot_count::SlotCount;
use crate::state_type::StateType;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::*;

/// This is the main
pub fn handle_macro(slot_count: &SlotCount, inner_fn: &mut ItemFn) -> Result<TokenStream> {
    // Only keep the 'doc' attributes from what's supplied for the function
    inner_fn.attrs.retain(|attr| attr.path.is_ident("doc"));
    let docs = inner_fn.attrs.iter();

    // The visibility of the function becomes the visibility of the struct and methods
    let visibility = inner_fn.vis.clone();

    // Pull the name of the function. This name (converted to PascalCase is also the name of the struct)
    let function_name = inner_fn.sig.ident.to_string();
    let struct_name: Ident =
        syn::parse_str::<Ident>(&format!("{}", function_name.to_case(Case::Pascal)))?;

    // The state name is read from the generic parameters of the function
    let state_ident = StateType::from_generics(&inner_fn.sig.generics)?;

    Ok(quote! {
        #(#docs)*
        #visibility struct #struct_name {
            store: std::cell::RefCell<wasmtime::Store<#state_ident>>,
            func: wasmtime::TypedFunc<u32, u64>,
        }
    }
    .into())
}
