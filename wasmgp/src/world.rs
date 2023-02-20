use crate::{
    Code, CodeContext, FunctionSignature, GeneticEngine, GeneticEngineConfiguration, Individual, Island,
    IslandCallbacks, MigrationAlgorithm, RunResult, WasmgpError, WorldConfiguration,
};
use anyhow::Result;
use rand::seq::SliceRandom;
use rand::Rng;
use std::thread;
use std::time::Duration;
use std::vec;
use wasm_ast::{FunctionIndex, Import, ModuleBuilder, Name};
use wasmtime::{AsContextMut, Config, Engine, Extern, Func, Instance, InstancePre, IntoFunc, Linker, Store};

pub type IslandId = usize;

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
pub struct World<T, R: RunResult> {
    config: WorldConfiguration,
    wasm_engine: Engine,
    genetic_engine: GeneticEngine,
    linker: Linker<T>,
    imported_functions: Vec<FunctionSignature>,
    module_builder: ModuleBuilder,
    islands: Vec<Island<T, R>>,
    generations_remaining_before_migration: usize,
}

impl<T: Default, R: RunResult> World<T, R> {
    pub fn new(config: WorldConfiguration) -> Result<World<T, R>> {
        if config.slot_count() > u8::MAX as usize {
            return Err(WasmgpError::SlotCountTooLarge(config.slot_count()).into());
        }
        let total_slots = config.slot_count() as u8;

        let mut engine_config = Config::default();
        engine_config.epoch_interruption(true);
        let engine = Engine::new(&engine_config)?;
        let linker = Linker::new(&engine);

        // Advance the engine's epoch once every millisecond
        let engine_for_timer = engine.clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(1));
            engine_for_timer.increment_epoch();
        });

        let generations_remaining_before_migration = config.generations_between_migrations;
        let mut genetic_config = GeneticEngineConfiguration::new(None, total_slots);
        genetic_config.individual_max_points = config.individual_max_points;
        genetic_config.mutation_rate = config.mutation_rate;
        genetic_config.max_mutation_points = config.max_mutation_points;
        if genetic_config.mutation_rate > 0 && genetic_config.max_mutation_points == 0 {
            return Err(WasmgpError::InvalidConfiguration(
                "must set max_mutation_points if mutation_rate is greater than zero".into(),
            )
            .into());
        }
        genetic_config.crossover_rate = config.crossover_rate;
        genetic_config.max_crossover_points = config.max_crossover_points;
        if genetic_config.crossover_rate > 0 && genetic_config.max_crossover_points == 0 {
            return Err(WasmgpError::InvalidConfiguration(
                "must set max_crossover_points if crossover_rate is greater than zero".into(),
            )
            .into());
        }

        Ok(World {
            config,
            wasm_engine: engine,
            genetic_engine: GeneticEngine::new(genetic_config),
            linker: linker,
            imported_functions: vec![],
            module_builder: ModuleBuilder::new(),
            islands: vec![],
            generations_remaining_before_migration,
        })
    }

    /// Defines a named function that will be available to every individual
    /// ```
    /// use wasmgp::*;
    /// use wasmtime::*;
    ///
    /// fn increment(mut caller: Caller<'_, u64>, amount: u64) -> u64 {
    ///     let value: &mut u64 = caller.data_mut();
    ///     *value += amount;
    ///     *value
    /// }
    ///
    /// let config = WorldConfiguration::default();
    /// let mut world = World::<u64, EmptyRunResult>::new(config).unwrap();
    /// world.add_function_import("increment", increment).unwrap();
    /// ```
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
            let type_index = self.module_builder.add_function_type(signature.clone().into())?;

            // Define an import in the module for this function type
            let import = Import::function(
                Name::new(String::from(MODULE_NAME)),
                Name::new(String::from(name)),
                type_index,
            );
            let function_index = self.module_builder.add_import(import)?;

            // Add this function to the weight table so that it may be randomly selected
            self.genetic_engine.set_host_call_weight(
                function_index,
                signature.params().len() as u8,
                signature.results().len() as u8,
                1,
            );

            Ok(function_index)
        } else {
            panic!(
                "'host.{}' was just defined as an Extern::Func, but we got a different answer",
                name
            )
        }
    }

    /// Sets the weight of every Code variant to the specified value (reset with a default)
    /// ```
    /// use wasmgp::*;
    ///
    /// let config = WorldConfiguration::default();
    /// let mut world = World::<(), EmptyRunResult>::new(config).unwrap();
    ///
    /// // Turn off everything
    /// world.reset_all_code_weights(0);
    ///
    /// // Except...
    /// world.set_code_weight(Code::Add(Add::default()), 1);
    /// world.set_code_weight(Code::If(If::default()), 1);
    /// world.set_code_weight(Code::IsEqualZero(IsEqualZero::default()), 1);
    /// ```
    pub fn reset_all_code_weights(&mut self, weight: u8) {
        self.genetic_engine.reset_all_code_weights(weight);
    }

    /// Sets the weight of the specified Code variant.
    /// ```
    /// use wasmgp::*;
    ///
    /// let config = WorldConfiguration::default();
    /// let mut world = World::<(), EmptyRunResult>::new(config).unwrap();
    ///
    /// // Add will now be selected with five time more liklihood than any other variant
    /// world.set_code_weight(Code::Add(Add::default()), 5);
    ///
    /// // If code will never be selected
    /// world.set_code_weight(Code::If(If::default()), 0);
    /// ```
    pub fn set_code_weight(&mut self, code: Code, weight: u8) {
        self.genetic_engine.set_code_weight(code, weight);
    }

    /// Sets the weight for a function previously imported with `add_function_import`
    /// ```
    /// use wasmgp::*;
    /// use wasmtime::*;
    ///
    /// fn increment(mut caller: Caller<'_, u64>, amount: u64) -> u64 {
    ///     let value: &mut u64 = caller.data_mut();
    ///     *value += amount;
    ///     *value
    /// }
    ///
    /// let config = WorldConfiguration::default();
    /// let mut world = World::<u64, EmptyRunResult>::new(config).unwrap();
    /// let function_index = world.add_function_import("increment", increment).unwrap();
    ///
    /// // Increment will be selected five time more often than the other Code variants
    /// world.set_function_import_weight(function_index, 5).unwrap();
    /// ```
    pub fn set_function_import_weight(&mut self, function_index: FunctionIndex, weight: u8) -> Result<()> {
        let signature = self
            .imported_functions
            .get(function_index as usize)
            .ok_or(WasmgpError::InvalidFunctionIndex(function_index))?;
        self.genetic_engine.set_host_call_weight(
            function_index,
            signature.params().len() as u8,
            signature.results().len() as u8,
            weight,
        );

        Ok(())
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
    pub fn instanciate(&mut self, store: impl AsContextMut<Data = T>, code: &[Code]) -> Result<Instance> {
        let mut builder = self.module_builder.clone();
        let context = CodeContext::new(
            &self.config.main_entry_point,
            self.config.work_slots.clone(),
            self.config.is_signed,
            self.config.work_slot_initialization,
        )?;
        context.build(&mut builder, &code[..], self.genetic_engine.rng())?;
        let module_ast = builder.build();
        let mut buffer = Vec::new();
        wasm_ast::emit_binary(&module_ast, &mut buffer)?;
        let module = wasmtime::Module::new(&self.wasm_engine, &buffer[..])?;
        self.linker.instantiate(store, &module)
    }

    /// Creates a wasmtime InstancePre for the specified Code
    pub fn instanciate_pre(&mut self, code: &[Code]) -> Result<InstancePre<T>> {
        let store = Store::new(&self.wasm_engine, T::default());
        let mut builder = self.module_builder.clone();
        let context = CodeContext::new(
            &self.config.main_entry_point,
            self.config.work_slots.clone(),
            self.config.is_signed,
            self.config.work_slot_initialization,
        )?;
        context.build(&mut builder, &code[..], self.genetic_engine.rng())?;
        let module_ast = builder.build();
        let mut buffer = Vec::new();
        wasm_ast::emit_binary(&module_ast, &mut buffer)?;
        let module = wasmtime::Module::new(&self.wasm_engine, &buffer[..])?;
        self.linker.instantiate_pre(store, &module)
    }

    /// Returns a copy of the ModuleBuilder. This builder includes any imports that were previously defined with
    /// `add_function_import`
    pub fn module_builder(&self) -> ModuleBuilder {
        self.module_builder.clone()
    }

    /// Adds a new island to the World that will use the specified callbacks to perform the various individual
    /// processing tasks required during its lifetime
    pub fn create_island(&mut self, callbacks: Box<dyn IslandCallbacks<T, R>>) -> IslandId {
        let id = self.islands.len();
        self.islands.push(Island::new(callbacks));

        id
    }

    /// Returns the total number of islands
    pub fn get_number_of_islands(&self) -> usize {
        self.islands.len()
    }

    /// Borrows an island by the specified ID
    pub fn get_island(&self, id: IslandId) -> Option<&Island<T, R>> {
        self.islands.get(id)
    }

    /// Mutably borrows an island by the specified ID
    pub fn get_island_mut(&mut self, id: IslandId) -> Option<&mut Island<T, R>> {
        self.islands.get_mut(id)
    }

    /// Removes all individuals from all islands
    pub fn reset_all_islands(&mut self) {
        for island in self.islands.iter_mut() {
            island.clear();
        }
    }

    /// Runs the next generation across all islands.
    #[cfg(not(feature = "async"))]
    pub fn run_one_generation(&mut self) {
        for island in self.islands.iter_mut() {
            island.run_one_generation();
        }

        // See if it is time for a migration
        if self.config.generations_between_migrations > 0 {
            self.generations_remaining_before_migration -= 1;
            if self.generations_remaining_before_migration == 0 {
                self.migrate_individuals_between_islands();
                self.generations_remaining_before_migration = self.config.generations_between_migrations;
            }
        }
    }

    /// Runs the next generation across all islands.
    #[cfg(feature = "async")]
    pub async fn run_one_generation(&mut self) {
        for island in self.islands.iter_mut() {
            island.run_one_generation().await;
        }

        // See if it is time for a migration
        if self.config.generations_between_migrations > 0 {
            self.generations_remaining_before_migration -= 1;
            if self.generations_remaining_before_migration == 0 {
                self.migrate_individuals_between_islands();
                self.generations_remaining_before_migration = self.config.generations_between_migrations;
            }
        }
    }

    /// Fills all islands with the children of the genetic algorithm, or with random individuals if there was no
    /// previous generation from which to draw upon.
    pub fn fill_all_islands(&mut self) -> Result<()> {
        for id in 0..self.islands.len() {
            let mut elite_remaining = self.config.elite_individuals_per_generation;
            while self.len_island_future_generation(id) < self.config.individuals_per_island {
                let island = self.islands.get(id).unwrap();
                let pick_elite = if elite_remaining > 0 {
                    elite_remaining -= 1;
                    true
                } else {
                    false
                };
                let next = if island.len() == 0 {
                    let code = self.genetic_engine.random_code_list(self.config.individual_max_points);
                    let instance_pre = self.instanciate_pre(&code[..])?;
                    Individual::new(
                        code,
                        self.config.main_entry_point.name().clone(),
                        instance_pre,
                        self.config.individual_run_time_ms,
                    )
                } else {
                    if pick_elite {
                        let elite = island
                            .select_one_individual(self.config.select_as_elite, self.genetic_engine.rng())
                            .unwrap();

                        elite.clone()
                    } else {
                        let left = island
                            .select_one_individual(self.config.select_as_parent, self.genetic_engine.rng())
                            .unwrap();
                        let right = island
                            .select_one_individual(self.config.select_as_parent, self.genetic_engine.rng())
                            .unwrap();
                        let code = self.genetic_engine.rand_child(left.get_code(), right.get_code())?;
                        let instance_pre = self.instanciate_pre(&code[..])?;
                        Individual::new(
                            code,
                            self.config.main_entry_point.name().clone(),
                            instance_pre,
                            self.config.individual_run_time_ms,
                        )
                    }
                };
                self.add_individual_to_island_future_generation(id, next);
            }

            // Now that the future generation is full, make it the current generation
            self.advance_island_generation(id);
        }

        Ok(())
    }

    fn len_island_future_generation(&self, id: IslandId) -> usize {
        self.islands.get(id).unwrap().len_future_generation()
    }

    fn add_individual_to_island_future_generation(&mut self, id: IslandId, individual: Individual<T, R>) {
        self.islands
            .get_mut(id)
            .unwrap()
            .add_individual_to_future_generation(individual)
    }

    fn advance_island_generation(&mut self, id: IslandId) {
        self.islands.get_mut(id).unwrap().advance_generation()
    }

    /// Runs generations until the specified function returns false
    #[cfg(not(feature = "async"))]
    pub fn run_generations_while<While>(&mut self, mut while_fn: While) -> Result<()>
    where
        While: FnMut(&World<T, R>) -> bool,
    {
        // Always run at least one generation
        let mut running = true;
        while running {
            self.fill_all_islands()?;
            self.run_one_generation();
            running = while_fn(self);
        }

        Ok(())
    }

    /// Runs generations until the specified function returns false
    #[cfg(feature = "async")]
    pub async fn run_generations_while<While>(&mut self, mut while_fn: While) -> Result<()>
    where
        While: FnMut(&World<T, R>) -> bool,
    {
        // Always run at least one generation
        let mut running = true;
        while running {
            self.fill_all_islands()?;
            self.run_one_generation().await;
            running = while_fn(self);
        }

        Ok(())
    }

    pub fn migrate_individuals_between_islands(&mut self) {
        let island_len = self.islands.len();

        // It only makes sense to migrate if there are at least two islands
        if island_len > 1 {
            match self.config.migration_algorithm {
                MigrationAlgorithm::Circular => self.migrate_all_islands_circular_n(1),
                MigrationAlgorithm::Cyclical(n) => self.migrate_all_islands_circular_n(n),
                MigrationAlgorithm::Incremental(n) => {
                    self.migrate_all_islands_circular_n(n);

                    // Increment 'n'. An 'n' of zero makes no sense, so when it gets there use '1' instead.
                    let mut next_n = self.island_at_distance(0, n + 1);
                    if next_n == 0 {
                        next_n = 1
                    }
                    self.config.migration_algorithm = MigrationAlgorithm::Incremental(next_n);
                }
                MigrationAlgorithm::RandomCircular => {
                    // Define a new order of islands and calculate the distance to the next island in this new order.
                    // For example, if there are 7 islands and the order starts with 2, 3: the first distance is 1.
                    // However if the order starts with 3, 2: the first distance is 6
                    //
                    // This algorithm achieves the desired goal of having individuals from each island migrate together
                    // to another random island, and each island is the source and destination exactly once.
                    let island_order = self.random_island_order();
                    let distances = World::<T, R>::distances_to_next_island(&island_order[..]);
                    for (source_id, n) in std::iter::zip(island_order, distances) {
                        self.migrate_one_island_circular_n(source_id, n);
                    }
                }
                MigrationAlgorithm::CompletelyRandom => {
                    let len = self.islands.len();

                    // For each migrating individual on each island, pick a random destination that is not the same
                    // island and migrate there.
                    for source_island_id in 0..len {
                        for _ in 0..self.config.number_of_individuals_migrating {
                            let mut destination_island_id = source_island_id;
                            while source_island_id != destination_island_id {
                                destination_island_id = self.genetic_engine.rng().gen_range(0..len);
                            }
                            self.migrate_one_individual_from_island_to_island(source_island_id, destination_island_id);
                        }
                    }
                }
            }
        }
    }

    fn migrate_one_individual_from_island_to_island(
        &mut self,
        source_island_id: IslandId,
        destination_island_id: IslandId,
    ) {
        let curve = self.config.select_for_migration;

        // Get the migrating individual from the source island
        let source_island = self.islands.get_mut(source_island_id).unwrap();
        let migrating: Individual<T, R> = if self.config.clone_migrated_individuals {
            source_island
                .select_one_individual(curve, self.genetic_engine.rng())
                .unwrap()
                .clone()
        } else {
            source_island
                .select_and_remove_one_individual(curve, self.genetic_engine.rng())
                .unwrap()
        };

        // Add it to the destination island
        let destination_island = self.islands.get_mut(destination_island_id).unwrap();
        destination_island.add_individual_to_future_generation(migrating);
    }

    // Calculates the ID of the island at a specific distance from the source. Wraps around when we get to the end of
    // the list.
    fn island_at_distance(&self, source_id: IslandId, distance: usize) -> IslandId {
        (source_id + distance) % self.islands.len()
    }

    fn migrate_all_islands_circular_n(&mut self, n: usize) {
        for source_island_id in 0..self.islands.len() {
            self.migrate_one_island_circular_n(source_island_id, n);
        }
    }

    fn migrate_one_island_circular_n(&mut self, source_island_id: IslandId, n: usize) {
        let destination_island_id = self.island_at_distance(source_island_id, n);
        for _ in 0..self.config.number_of_individuals_migrating {
            self.migrate_one_individual_from_island_to_island(source_island_id, destination_island_id);
        }
    }

    // Creates a Vec containing the source_id of each island exactly one time
    fn random_island_order(&mut self) -> Vec<IslandId> {
        let mut island_ids: Vec<IslandId> = (0..self.islands.len()).collect();
        island_ids.shuffle(self.genetic_engine.rng());

        island_ids
    }

    // Creates a Vec containing the distance to the previous island in the list for every entry in the parameter. The
    // distance for the first entry wraps around to the last item.
    fn distances_to_next_island(island_id: &[IslandId]) -> Vec<IslandId> {
        let len = island_id.len();
        let mut distances = Vec::with_capacity(len);
        let mut previous_source_id = island_id.last().unwrap();
        for source_id in island_id.iter() {
            let distance = ((previous_source_id + len) - source_id) % len;
            distances.push(distance);
            previous_source_id = source_id;
        }

        distances
    }
}
