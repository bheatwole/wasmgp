use std::vec;

use anyhow::Result;
use wasmtime::{AsContextMut, Engine, Extern, Func, IntoFunc, Linker, Store};

use crate::{FunctionSignature, WorldConfiguration};

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
        }
    }

    /// Defines a named function that will be available to every individual
    pub fn add_function_import<Params, Args>(
        &mut self,
        name: &str,
        func: impl IntoFunc<T, Params, Args>,
    ) -> Result<()> {
        // Add the function to the linker
        self.linker.func_wrap(MODULE_NAME, name, func)?;

        // Get the type information about the function so that we know how to call it later
        let mut store = Store::new(&self.wasm_engine, T::default());
        if let Some(func) = self.get_extern_func_from_linker(&mut store, name) {
            let func_type = func.ty(&store);
            self.imported_functions
                .push(FunctionSignature::new_from_func_type(name, func_type));
        } else {
            panic!(
                "'host.{}' was just defined as an Extern::Func, but we got a different answer",
                name
            )
        }

        Ok(())
    }

    fn get_extern_func_from_linker(
        &self,
        store: impl AsContextMut<Data = T>,
        name: &str,
    ) -> Option<Func> {
        if let Some(ext) = self.linker.get(store, MODULE_NAME, name) {
            match ext {
                Extern::Func(f) => Some(f),
                _ => None,
            }
        } else {
            None
        }
    }
}
