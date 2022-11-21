use crate::slot_count::SlotCount;
use crate::state_type::StateType;
use crate::var_list_type::VarListType;
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

    // Pull the name of the function. This name converted to PascalCase is also the name of the struct
    let function_name = inner_fn.sig.ident.to_string();
    let function_name_lit = Lit::Str(LitStr::new(function_name.as_str(), inner_fn.sig.ident.span()));
    let struct_name: Ident =
        syn::parse_str::<Ident>(&format!("{}", function_name.to_case(Case::Pascal)))?;

    // The state name is read from the generic parameters of the function
    let state_ident = StateType::from_generics(&inner_fn.sig.generics)?;

    // Read the fn params and results into variables
    let param_var_list_type = VarListType::from_fn_args(inner_fn)?;
    let result_var_list_type = VarListType::from_fn_results(inner_fn)?;

    // We need to display the params and result in the middle of generic arguments
    let param_generic = param_var_list_type.for_generic_params();
    let result_generic = result_var_list_type.for_generic_params();

    Ok(quote! {
        #(#docs)*
        #visibility struct #struct_name {
            store: std::cell::RefCell<wasmtime::Store<#state_ident>>,
            func: wasmtime::TypedFunc<#param_generic, #result_generic>,
        }

        impl #struct_name {
            fn new() -> anyhow::Result<#struct_name> {
                let name = #function_name_lit;
                let fs = crate::FunctionSignature::new(name, vec![crate::ValueType::I32], vec![crate::ValueType::I64]);
                let slots = crate::SlotCount {
                    i32: 0,
                    i64: 0,
                    f32: 0,
                    f64: 0,
                };
                let context = crate::CodeContext::new(&fs, slots, false)?;
                let code = vec![Code::Add(0, 0, 1), Code::Return];
                let mut builder = wasm_ast::ModuleBuilder::new();
                context.build(&mut builder, &code[..])?;
                let module = builder.build();
                let mut buffer = Vec::new();
                wasm_ast::emit_binary(&module, &mut buffer)?;
                let engine = wasmtime::Engine::default();
                let module = wasmtime::Module::new(&engine, &buffer[..])?;
                let mut store = wasmtime::Store::new(&engine, ());
                let instance = wasmtime::Instance::new(&mut store, &module, &vec![])?;
                let func = instance.get_typed_func::<#param_generic, #result_generic, _>(&mut store, name)?;
    
                Ok(#struct_name {
                    store: std::cell::RefCell::new(store),
                    func,
                })
            }
    
            fn call(&self, value: u32) -> anyhow::Result<#result_generic> {
                use std::ops::DerefMut;
                let mut store = self.store.borrow_mut();
                let results = self.func.call(store.deref_mut(), value)?;
                Ok(results)
            }
        }
    }
    .into())
}
