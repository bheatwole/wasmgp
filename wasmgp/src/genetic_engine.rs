use crate::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use strum::IntoEnumIterator;

pub struct GeneticEngine {
    rng: SmallRng,
    slot_count: Slot,
    weights: Vec<WeightEntry>,
    sum_of_weights: usize,
}

impl GeneticEngine {
    pub fn new(seed: Option<u64>, slot_count: Slot) -> GeneticEngine {
        let mut engine = GeneticEngine {
            rng: small_rng_from_optional_seed(seed),
            slot_count,
            weights: vec![],
            sum_of_weights: 0,
        };

        // Set the default weight of every instruction except for Call to be one. The Call instructions will be added
        // when there is a host function to call.
        let test_for_call = Code::Call(Call::default());
        for code in Code::iter() {
            if code != test_for_call {
                engine.sum_of_weights += 1;
                engine.weights.push(WeightEntry {
                    code,
                    weight: 1,
                    combined_weight: engine.sum_of_weights,
                });
            }
        }

        engine
    }

    pub fn random_slot(&mut self) -> Slot {
        self.rng.gen_range(0..self.slot_count)
    }

    pub fn random_code_list(&mut self, max_points: usize) -> Vec<Code> {
        let mut code = vec![];
        let points = self.rng.gen_range(1..max_points);
        for _i in 0..points {
            code.push(self.random_code(max_points));
        }
        code
    }

    pub fn random_code(&mut self, max_points: usize) -> Code {
        let weighted_code = self.pick_random_weighted_code();
        weighted_code.make_random_code(self, max_points)
    }

    fn pick_random_weighted_code(&mut self) -> Code {
        let pick = self.rng.gen_range(1..=self.sum_of_weights);
        let index = self.weights.partition_point(|entry| entry.combined_weight < pick);
        let entry = self.weights.get(index).unwrap();
        entry.code.clone()
    }
}

struct WeightEntry {
    code: Code,
    weight: u8,
    combined_weight: usize,
}

fn small_rng_from_optional_seed(rng_seed: Option<u64>) -> SmallRng {
    if let Some(seed) = rng_seed {
        SmallRng::seed_from_u64(seed)
    } else {
        SmallRng::from_entropy()
    }
}

#[cfg(test)]
mod tests {
    use crate::GeneticEngine;

    #[test]
    fn test_random_slot() {
        // Use a specific seed so that we always get the same slots for the test
        let mut engine = GeneticEngine::new(Some(1), 10);

        // Get some random slots
        assert_eq!(engine.random_slot(), 7);
        assert_eq!(engine.random_slot(), 0);
        assert_eq!(engine.random_slot(), 1);
        assert_eq!(engine.random_slot(), 3);
        assert_eq!(engine.random_slot(), 4);
    }
}
