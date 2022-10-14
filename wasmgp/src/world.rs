use anyhow::Result;
use wasmtime::{Engine, IntoFunc, Linker};

use crate::WorldConfiguration;

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
pub struct World<T> {
    config: WorldConfiguration,
    wasm_engine: Engine,
    linker: Linker<T>,
}

impl<T> World<T> {
    pub fn new(config: WorldConfiguration) -> World<T> {
        let engine = Engine::default();
        let linker = Linker::new(&engine);

        World {
            config,
            wasm_engine: engine,
            linker: linker,
        }
    }

    /// Defines a named function that will be available to every individual
    pub fn add_function_import<Params, Args>(
        &mut self,
        name: &str,
        func: impl IntoFunc<T, Params, Args>,
    ) -> Result<()> {
        self.linker.func_wrap("host", name, func)?;
        Ok(())
    }
}
