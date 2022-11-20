use proc_macro::TokenStream;
use syn::*;

mod slot_count;
mod wasm_code;

use slot_count::SlotCount;

/// The wasm_code macro takes a function whose body consists of a list of `Code` and turns it into a struct that can
/// be called as though it were a regular rust function
///
/// ```no_run
/// #[wasm_code(unsigned, 0, 0, 0, 0)]
/// fn double(value: u32) -> u64 {
///     [
///         Code::Add(0, 0, 1),
///         Code::Return,
///     ]
/// }
/// ```
///
/// will translate to:
///
/// ```no_run
/// struct Double {
///     store: std::cell::RefCell<wasmtime::Store<()>>,
///     func: wasmtime::TypedFunc<u32, u64>,
/// }
///
/// impl Double {
///     fn new() -> anyhow::Result<Double> {
///         let name = "double";
///         let fs = wasmgp::FunctionSignature::new(
///             name,
///             vec![wasmgp::ValueType::I32],
///             vec![wasmgp::ValueType::I64],
///         );
///         let slots = wasmgp::SlotCount {
///             i32: 0,
///             i64: 0,
///             f32: 0,
///             f64: 0,
///         };
///         let context = wasmgp::CodeContext::new(&fs, slots, false)?;
///         let code = vec![
///             Code::Add(0, 0, 1),
///             Code::Return
///         ];
///         let mut builder = wasm_ast::ModuleBuilder::new();
///         context.build(&mut builder, &code[..])?;
///         let module = builder.build();
///         let mut buffer = Vec::new();
///         wasm_ast::emit_binary(&module, &mut buffer)?;
///         let engine = wasmtime::Engine::default();
///         let module = wasmtime::Module::new(&engine, &buffer[..])?;
///         let mut store = Store::new(&engine, ());
///         let instance = Instance::new(&mut store, &module, &vec![])?;
///         let func = instance.get_typed_func::<u32, u64, _>(&mut store, name)?;
///
///         Ok(Double {
///             store: std::cell::RefCell::new(store),
///             func,
///         })
///     }
///
///     fn call(&mut self, value: u32) -> anyhow::Result<u64> {
///         let mut store = self.store.borrow_mut();
///         let results = self.func.call(store.deref_mut(), value)?;
///         Ok(results)
///     }
/// }
/// ```
///
/// and call be called with:
/// ```no_run
/// let func = Double::new().unwrap();
/// assert_eq!(4, func.call(2).unwrap());
/// assert_eq!(30, func.call(15).unwrap());
/// ```
///
/// If the Store needs a state value, it can be supplied as a generic parameter to macro fn:
/// ```no_run
/// #[wasm_code(unsigned, 0, 0, 0, 0)]
/// fn double_with_state<MyState>(value: u32) -> u64 {
///     [
///         Code::Add(0, 0, 1),
///         Code::Return,
///     ]
/// }
/// ```
///
/// will translate to:
///
/// ```no_run
/// struct DoubleWithState {
///     store: std::cell::RefCell<wasmtime::Store<MyState>>,
///     func: wasmtime::TypedFunc<u32, u64>,
/// }
///
/// impl DoubleWithState {
///     fn new(state: MyState) -> anyhow::Result<DoubleWithState> {
///         // ...
///         let mut store = Store::new(&engine, state);
///         // ...
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn wasm_code(attr: TokenStream, input: TokenStream) -> TokenStream {
    let slot_count = parse_macro_input!(attr as SlotCount);
    let mut item_fn = parse_macro_input!(input as ItemFn);
    wasm_code::handle_macro(&slot_count, &mut item_fn)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
