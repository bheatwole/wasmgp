use crate::{FunctionSignature, MigrationAlgorithm, SelectionCurve, SlotCount, SlotInit, ThreadingModel};

#[derive(Clone, Debug, PartialEq)]
pub struct WorldConfiguration {
    /// The signature of the main entry point for the individuals in the world
    ///
    /// The default is a 'main' function with no parameters and no results
    pub main_entry_point: FunctionSignature,

    /// The genetic code uses a number of local working variables. `work_slots` defines the number of each of the types
    /// supported by wasmgp that are available to the genetic code.
    ///
    /// The default is ten i32 working variables.
    pub work_slots: SlotCount,

    /// The work_slots will be initialized according to this parameter.
    pub work_slot_initialization: SlotInit,

    /// The genetic code works with either signed or unsigned arithmetic for all the genetic code.
    ///
    /// The default is unsigned
    pub is_signed: bool,

    /// The amount of Wasm memory that individuals may access. May be zero. Must be set to at least the size of any
    /// default data you will provide to the individual at runtime if you choose to pre-load a block of data. This will
    /// be rounded up to the nearest multiple of Wasm page size (usually 64K).
    ///
    /// The default is zero
    pub memory_size: usize,

    /// The number of milliseconds that an individual may run for. This time includes any calls out to host functions.
    /// Individuals that exceed the time limit will return a epoch_deadline_trap as the result.
    ///
    /// The default is 250ms
    pub individual_run_time_ms: u64,

    /// The maximum amount of code that any individual may have
    ///
    /// The default is 100
    pub individual_max_points: usize,

    /// The number of individuals on each island. Before running a generation, the island will be filled with the
    /// children of genetic selection if there was a previous generation, or new random individuals if there was no
    /// previous generation.
    pub individuals_per_island: usize,

    /// The number of individuals whose code will be copied as-is to the next generation. This can help preserve highly
    /// fit code. Set to zero to disable elitism. ref https://en.wikipedia.org/wiki/Genetic_algorithm#Elitism
    pub elite_individuals_per_generation: usize,

    /// After this many generations across all islands, some of the individual will migrate to new islands. Set to zero
    /// to disable automatic migrations.
    pub generations_between_migrations: usize,

    /// The number of individuals that will migrate from one island to another.
    pub number_of_individuals_migrating: usize,

    /// When it is time for a migration, a new island will be selected for the individual according to the specified
    /// algorithm.
    pub migration_algorithm: MigrationAlgorithm,

    /// If false, individuals selected for migration are removed from their home island. If true, the selected
    /// individuals are cloned and the clone is moved. The default is true
    pub clone_migrated_individuals: bool,

    /// The SelectionCurve that will be used when choosing which individual will participate in migration. The default
    /// is PreferenceForFit.
    pub select_for_migration: SelectionCurve,

    /// The SelectionCurve that will be used when choosing a fit parent for genetic operations. The default is
    /// PreferenceForFit.
    pub select_as_parent: SelectionCurve,

    /// The SelectionCurve used when choosing an elite individual to preserve for the next generation. The default is
    /// StrongPreferenceForFit.
    pub select_as_elite: SelectionCurve,

    /// Determine how the world runs with regards to multi-threading. Placeholder: currently multi-threading is not
    /// implemented
    pub threading_model: ThreadingModel,

    /// The average number of times the 'Mutation' genetic operation will be chosen. The `mutation_rate` and
    /// `crossover_rate` are summed and then a random value is picked in that range to the final rate is dependant upon
    /// both values.
    ///
    /// Set this to zero to disable mutation entirely.
    ///
    /// The default value is 1. If the default `crossover_rate` is also used, this equates to 10%
    pub mutation_rate: u8,

    /// The average number of times the 'Crossover' genetic operation will be chosen. The `mutation_rate` and
    /// `crossover_rate` are summed and then a random value is picked in that range to the final rate is dependant upon
    /// both values.
    ///
    /// Set this to zero to disable crossover entirely.
    ///
    /// The default value is 9. If the default `mutation_rate` is also used, this equates to 90%
    pub crossover_rate: u8,

    /// The maximum number of code items that will be mutated (replaced with new random code) when the 'Mutation'
    /// operation is chosen. The actual value is random between one and this number. Must be at least one if mutation
    /// is used at all.
    ///
    /// The default value is 1
    pub max_mutation_points: u8,

    /// The maximum number of times the code streams will be swapped during the Crossover operation. The actual value is
    /// random between one and this number. Must be at least one if crossover is used at all.
    ///
    /// The default value is 2
    pub max_crossover_points: u8,
}

impl WorldConfiguration {
    pub fn slot_count(&self) -> usize {
        self.main_entry_point.params().len() + self.main_entry_point.results().len() + self.work_slots.len()
    }
}

impl Default for WorldConfiguration {
    fn default() -> Self {
        WorldConfiguration {
            main_entry_point: FunctionSignature::empty(),
            work_slots: SlotCount {
                i32: 10,
                i64: 0,
                f32: 0,
                f64: 0,
            },
            work_slot_initialization: SlotInit::Zero,
            is_signed: false,
            memory_size: 0,
            individual_run_time_ms: 250,
            individual_max_points: 100,
            individuals_per_island: 100,
            elite_individuals_per_generation: 2,
            generations_between_migrations: 10,
            number_of_individuals_migrating: 10,
            migration_algorithm: MigrationAlgorithm::Circular,
            clone_migrated_individuals: true,
            select_for_migration: SelectionCurve::PreferenceForFit,
            select_as_parent: SelectionCurve::PreferenceForFit,
            select_as_elite: SelectionCurve::StrongPreferenceForFit,
            threading_model: ThreadingModel::None,
            mutation_rate: 1,
            crossover_rate: 9,
            max_mutation_points: 1,
            max_crossover_points: 2,
        }
    }
}
