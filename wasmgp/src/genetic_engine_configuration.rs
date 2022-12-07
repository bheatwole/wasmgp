use crate::Slot;

#[derive(Clone, Debug, PartialEq)]
pub struct GeneticEngineConfiguration {
    pub seed: Option<u64>,
    pub slot_count: Slot,
    pub mutation_rate: u8,
    pub crossover_rate: u8,
    pub max_mutation_points: u8,
    pub max_crossover_points: u8,
}

impl GeneticEngineConfiguration {
    pub fn new(seed: Option<u64>, slot_count: Slot) -> GeneticEngineConfiguration {
        GeneticEngineConfiguration {
            seed,
            slot_count,
            mutation_rate: 1,
            crossover_rate: 9,
            max_mutation_points: 1,
            max_crossover_points: 2,
        }
    }
}
