use crate::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use strum::IntoEnumIterator;
use wasm_ast::FunctionIndex;

pub struct GeneticEngine {
    rng: SmallRng,
    slot_count: Slot,
    weights: Vec<WeightEntry>,
    sum_of_weights: Option<usize>,
}

impl GeneticEngine {
    pub fn new(seed: Option<u64>, slot_count: Slot) -> GeneticEngine {
        let mut engine = GeneticEngine {
            rng: small_rng_from_optional_seed(seed),
            slot_count,
            weights: vec![],
            sum_of_weights: None,
        };

        // Set the default weight of every instruction except for Call to be one. The Call instructions will be added
        // when there is a host function to call.
        let test_for_call = Code::Call(Call::default());
        for code in Code::iter() {
            if code != test_for_call {
                engine.weights.push(WeightEntry {
                    code,
                    weight: 1,
                    combined_weight: 0,
                });
            }
        }

        engine
    }

    /// Mutably borrows the random number generator
    pub fn rng(&mut self) -> &mut SmallRng {
        &mut self.rng
    }

    /// Returns a random working slot out of all the slots defined in the function (parameters, returns, SlotCount)
    pub fn random_slot(&mut self) -> Slot {
        self.rng.gen_range(0..self.slot_count)
    }

    /// Creates a random list of code up to the specified number of max_points
    pub fn random_code_list(&mut self, max_points: usize) -> Vec<Code> {
        let mut code = vec![];
        let points = self.rng.gen_range(1..max_points);
        for _i in 0..points {
            code.push(self.random_code(max_points));
        }
        code
    }

    /// Creates a single random piece of code. `max_points` defines how many child Code elements items such as `IfElse`
    /// may also create.
    pub fn random_code(&mut self, max_points: usize) -> Code {
        let weighted_code = self.pick_random_weighted_code();
        weighted_code.make_random_code(self, max_points)
    }

    fn pick_random_weighted_code(&mut self) -> Code {
        if self.sum_of_weights.is_none() {
            self.update_sum_of_weights();
        }

        let pick = self.rng.gen_range(1..=self.sum_of_weights.unwrap());
        let index = self.weights.partition_point(|entry| entry.combined_weight < pick);
        let entry = self.weights.get(index).unwrap();
        entry.code.clone()
    }

    /// Sets the weight (likelihood of this Code being selected by the genetic algorithm). The 'weight' concept operates
    /// as though each variant in the Code enum had 'weight' number of tickets in a drawing and one ticket was picked at
    /// random.
    ///
    /// Use a `weight` of zero if you wish to disallow a particular Code variant from being selected.
    ///
    /// Code::Call is handled slightly differently than all the other Code variants. Use `set_host_call_weight` to set a
    /// weight for a Code::Call.
    pub fn set_code_weight(&mut self, code: Code, weight: u8) {
        let default = code.get_default();
        let test_for_call = Code::Call(Call::default());
        if default == test_for_call {
            panic!("Code::Call weights should be set using set_host_call_weight instead");
        }
        self.internal_set_code_weight(default, weight);
    }

    /// Set the weight for a call to host function. Use this instead of `set_code_weight` for all `Code::Call` code.
    pub fn set_host_call_weight(&mut self, function_index: FunctionIndex, num_params: u8, num_results: u8, weight: u8) {
        let call = Call::new(function_index, vec![num_params], vec![num_results]);
        self.internal_set_code_weight(call, weight);
    }

    fn internal_set_code_weight(&mut self, code: Code, weight: u8) {
        // Update the existing entry for the weight or add a new entry. Setting weights should happen infrequently
        // enough that a list scan shouldn't impact performance.
        let existing_index = self.weights.iter().position(|entry| entry.code == code);
        if let Some(index) = existing_index {
            self.weights[index].weight = weight;
        } else {
            self.weights.push(WeightEntry {
                code,
                weight,
                combined_weight: 0,
            });
        }

        // The combined weight of all items is now probably wrong and needs to be recalculated
        self.sum_of_weights = None
    }

    fn update_sum_of_weights(&mut self) {
        // Set the combined_weight field to the sum of all entries up to and including this one. The `partition_point`
        // function will then be able to find the correct entry with a minimum number of lookups.
        let mut sum = 0;
        for entry in self.weights.iter_mut() {
            sum += entry.weight as usize;
            entry.combined_weight = sum;
        }
        self.sum_of_weights = Some(sum);
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

    #[test]
    fn verify_partition_point_function() {
        // The instruction entries table depend upon the following behavior from partition_point. If it ever stops
        // working like this, we need to know. Specifically only the first of a series of identical values is returned
        let entries = [1, 5, 5, 5, 10];
        assert_eq!(0, entries.partition_point(|&x| x < 1));
        assert_eq!(1, entries.partition_point(|&x| x < 2));
        assert_eq!(1, entries.partition_point(|&x| x < 3));
        assert_eq!(1, entries.partition_point(|&x| x < 4));
        assert_eq!(1, entries.partition_point(|&x| x < 5));
        assert_eq!(4, entries.partition_point(|&x| x < 6));
        assert_eq!(4, entries.partition_point(|&x| x < 10));
    }
}
