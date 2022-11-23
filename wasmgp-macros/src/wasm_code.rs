use crate::block_stmts::BlockStmts;
use crate::slot_count::SlotCount;
use crate::state_type::StateType;
use crate::util::get_env_var;
use crate::var_list_type::VarListType;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::*;

/// This is the main
pub fn handle_macro(slot_count: &SlotCount, inner_fn: &mut ItemFn) -> Result<TokenStream> {
    // Determine the full path that we should reference the 'wasmgp' library in our code
    let path_to_wasmgp = if let Some(crate_name) = get_env_var("CARGO_CRATE_NAME") {
        if crate_name == "wasmgp" {
            // We should reference wasmgp by the name 'crate' unless we're compiling doc tests
            if let Some(test_path) = get_env_var("UNSTABLE_RUSTDOC_TEST_PATH") {
                if test_path.len() > 0 {
                    "wasmgp"
                } else {
                    "crate"
                }
            } else {
                "crate"
            }
        } else {
            "wasmgp"
        }
    } else {
        "wasmgp"
    };
    let wasmgp: Path = syn::parse_str::<Path>(&path_to_wasmgp)?;

    // Only keep the 'doc' attributes from what's supplied for the function
    inner_fn.attrs.retain(|attr| attr.path.is_ident("doc"));
    let docs = inner_fn.attrs.iter();

    // The visibility of the function becomes the visibility of the struct and methods
    let visibility = inner_fn.vis.clone();

    // Pull the name of the function. This name converted to PascalCase is also the name of the struct
    let function_name = inner_fn.sig.ident.to_string();
    let function_name_lit = Lit::Str(LitStr::new(function_name.as_str(), inner_fn.sig.ident.span()));
    let struct_name: Ident =
        syn::parse_str::<Ident>(&format!("{}", function_name.to_case(Case::Pascal)))?;

    // The state name is read from the generic parameters of the function
    let state_ident = StateType::from_generics(&inner_fn.sig.generics)?;
    let state_new_args = state_ident.for_fn_args();
    let state_store_arg = state_ident.for_store_arg();

    // Read the fn params and results into variables
    let param_var_list_type = VarListType::from_fn_args(inner_fn)?;
    let result_var_list_type = VarListType::from_fn_results(inner_fn)?;

    // We need to display the params and result in the middle of generic arguments
    let param_generic = param_var_list_type.for_generic_params();
    let result_generic = result_var_list_type.for_generic_params();

    // The FunctionSignature requires the params and results as a vec of ValueType
    let param_value_types = param_var_list_type.for_value_types(&wasmgp);
    let result_value_types = result_var_list_type.for_value_types(&wasmgp);

    // The `call` function requries the parameters to be formatted in two additional forms
    let param_call_fn_args = param_var_list_type.for_call_fn_args();
    let param_call_args = param_var_list_type.for_call_args();

    // Handle the slot_count construction
    let slot_count_constructor = slot_count.for_constructor(&wasmgp);

    // Pull out the body for use
    let body_block = BlockStmts::new(&inner_fn.block);

    Ok(quote! {
        #(#docs)*
        #visibility struct #struct_name {
            store: std::cell::RefCell<wasmtime::Store<#state_ident>>,
            func: wasmtime::TypedFunc<#param_generic, #result_generic>,
        }

        impl #struct_name {
            fn new(#state_new_args) -> anyhow::Result<#struct_name> {
                let name = #function_name_lit;
                let fs = #wasmgp::FunctionSignature::new(name, #param_value_types, #result_value_types);
                let slots = #slot_count_constructor;
                let context = #wasmgp::CodeContext::new(&fs, slots, #slot_count)?;
                let code = vec!#body_block;
                let mut builder = wasm_ast::ModuleBuilder::new();
                context.build(&mut builder, &code[..])?;
                let module = builder.build();
                let mut buffer = Vec::new();
                wasm_ast::emit_binary(&module, &mut buffer)?;
                let engine = wasmtime::Engine::default();
                let module = wasmtime::Module::new(&engine, &buffer[..])?;
                let mut store = wasmtime::Store::new(&engine, #state_store_arg);
                let instance = wasmtime::Instance::new(&mut store, &module, &vec![])?;
                let func = instance.get_typed_func::<#param_generic, #result_generic, _>(&mut store, name)?;
    
                Ok(#struct_name {
                    store: std::cell::RefCell::new(store),
                    func,
                })
            }
    
            fn call(&self #param_call_fn_args) -> anyhow::Result<#result_generic> {
                use std::ops::DerefMut;
                let mut store = self.store.borrow_mut();
                let results = self.func.call(store.deref_mut(), #param_call_args)?;
                Ok(results)
            }
        }
    }
    .into())
}
