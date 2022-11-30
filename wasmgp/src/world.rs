use crate::{Code, CodeContext, FunctionSignature, WorldConfiguration};
use anyhow::Result;
use std::vec;
use wasm_ast::{FunctionIndex, Import, ModuleBuilder, Name};
use wasmtime::{AsContextMut, Engine, Extern, Func, Instance, IntoFunc, Linker, Store};

pub const MODULE_NAME: &'static str = "host";

/// A WasmGP world holds the islands where individuals live. It contains the logic behind how individuals are tested,
/// how to progress from generation to generation, how to alter future generations, etc.
///
/// The generic parameter 'T' is the type of host state accessible to all individuals. That type is where implementors
/// will hold the logic of all the connections and data storage for resources that live outside the genetic code. Some
/// examples of 'T' are:
/// - VirtualStockMarket: An object that allows individuals to check prices and place orders in a virtual market.
/// - GameState: An object simulating a game, allowing individuals play it (and optimize strategies)
/// - CircuitTester: An object that allows individuals to place circuits and then tests the results.
///
/// The 'T' parameter must implement `Default` so that a Store<T> can be created when needed to test function types and
/// other setup calls. When the individual is run, it will use a value for `T` that has been created by the caller, not
/// a default instance.
pub struct World<T> {
    config: WorldConfiguration,
    wasm_engine: Engine,
    linker: Linker<T>,
    imported_functions: Vec<FunctionSignature>,
    module_builder: ModuleBuilder,
}

impl<T: Default> World<T> {
    pub fn new(config: WorldConfiguration) -> World<T> {
        let engine = Engine::default();
        let linker = Linker::new(&engine);

        World {
            config,
            wasm_engine: engine,
            linker: linker,
            imported_functions: vec![],
            module_builder: ModuleBuilder::new(),
        }
    }

    /// Defines a named function that will be available to every individual
    pub fn add_function_import<Params, Args>(
        &mut self,
        name: &str,
        func: impl IntoFunc<T, Params, Args>,
    ) -> Result<FunctionIndex> {
        // Add the function to the linker
        self.linker.func_wrap(MODULE_NAME, name, func)?;

        // Get the type information about the function so that we know how to call it later
        let mut store = Store::new(&self.wasm_engine, T::default());
        if let Some(func) = self.get_extern_func_from_linker(&mut store, name) {
            let func_type = func.ty(&store);

            // Add the signature of the function to our import list and also to the module builder
            let signature = FunctionSignature::new_from_func_type(name, func_type);
            self.imported_functions.push(signature.clone());
            let type_index = self.module_builder.add_function_type(signature.into())?;

            // Define an import in the module for this function type
            let import = Import::function(
                Name::new(String::from(MODULE_NAME)),
                Name::new(String::from(name)),
                type_index,
            );
            let function_index = self.module_builder.add_import(import)?;

            Ok(function_index)
        } else {
            panic!(
                "'host.{}' was just defined as an Extern::Func, but we got a different answer",
                name
            )
        }
    }

    fn get_extern_func_from_linker(&self, store: impl AsContextMut<Data = T>, name: &str) -> Option<Func> {
        if let Some(ext) = self.linker.get(store, MODULE_NAME, name) {
            match ext {
                Extern::Func(f) => Some(f),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn store(&self, data: T) -> Store<T> {
        Store::new(&self.wasm_engine, data)
    }

    /// Creates a wasmtime Instance for the specified Code
    pub fn instanciate(&self, store: impl AsContextMut<Data = T>, code: &[Code]) -> Result<Instance> {
        let mut builder = self.module_builder.clone();
        let context = CodeContext::new(
            &self.config.main_entry_point,
            self.config.work_slots.clone(),
            self.config.is_signed,
        )?;
        context.build(&mut builder, &code[..])?;
        let module_ast = builder.build();
        let mut buffer = Vec::new();
        wasm_ast::emit_binary(&module_ast, &mut buffer)?;
        let module = wasmtime::Module::new(&self.wasm_engine, &buffer[..])?;
        self.linker.instantiate(store, &module)
    }

    /// Returns a copy of the ModuleBuilder. This builder includes any imports that were previously defined with
    /// `add_function_import`
    pub fn module_builder(&self) -> ModuleBuilder {
        self.module_builder.clone()
    }
}
