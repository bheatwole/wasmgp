use crate::{MigrationAlgorithm, SelectionCurve, ThreadingModel, MainEntryPoint};

#[derive(Clone, Debug, PartialEq)]
pub struct WorldConfiguration {
    /// The signature of the main entry point for the individuals in the world
    pub main_entry_point: MainEntryPoint,

    /// The amount of Wasm memory that individuals may access. May be zero. Must be set to at least the size of any
    /// default data you will provide to the individual at runtime if you choose to pre-load a block of data. This will
    /// be rounded up to the nearest multiple of Wasm page size (usually 64K)
    pub memory_size: usize,

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
}

impl Default for WorldConfiguration {
    fn default() -> Self {
        WorldConfiguration {
            main_entry_point: MainEntryPoint::empty(),
            memory_size: 0,
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
        }
    }
}
