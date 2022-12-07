use crate::*;
use anyhow::Result;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use strum::IntoEnumIterator;
use wasm_ast::FunctionIndex;

pub struct GeneticEngine {
    rng: SmallRng,
    config: GeneticEngineConfiguration,
    weights: Vec<WeightEntry>,
    sum_of_weights: Option<usize>,
}

impl GeneticEngine {
    pub fn new(config: GeneticEngineConfiguration) -> GeneticEngine {
        let rng = small_rng_from_optional_seed(config.seed);
        let mut engine = GeneticEngine {
            rng: rng,
            config,
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
        self.rng.gen_range(0..self.config.slot_count)
    }

    /// Creates a random list of code up to the specified number of max_points
    pub fn random_code_list(&mut self, max_points: usize) -> Vec<Code> {
        let mut code = vec![];
        let mut points = self.rng.gen_range(1..=max_points);
        while points > 0 {
            let child = self.random_code(points);
            points -= child.mutation_points();
            code.push(child);
        }
        code
    }

    /// Creates a single random piece of code. `max_points` defines how many child Code elements items such as `IfElse`
    /// may also create.
    pub fn random_code(&mut self, max_points: usize) -> Code {
        assert!(
            max_points > 0,
            "you must have at least one point to generate any random code"
        );

        // Code that has children (If, IfElse, DoUntil, etc) need more than one point, so just re-pick if we need to
        let mut weighted_code = self.pick_random_weighted_code();
        while weighted_code.minimum_points() > max_points {
            weighted_code = self.pick_random_weighted_code();
        }
        weighted_code.make_random_code(self, max_points)
    }

    /// Randomly selects either a crossover or mutation as the genetic operation to perform.
    pub fn select_genetic_operation(&mut self) -> GeneticOperation {
        let mutation_rate = self.config.mutation_rate as usize;
        let total = self.config.crossover_rate as usize + mutation_rate;
        let pick = self.rng.gen_range(0..total);
        if pick < mutation_rate as usize {
            if self.config.max_mutation_points == 1 {
                GeneticOperation::Mutation(1)
            } else {
                let count = self.rng.gen_range(1..self.config.max_mutation_points);
                GeneticOperation::Mutation(count)
            }
        } else {
            if self.config.max_crossover_points == 1 {
                GeneticOperation::Crossover(1)
            } else {
                let count = self.rng.gen_range(1..self.config.max_crossover_points);
                GeneticOperation::Crossover(count)
            }
        }
    }

    /// Produces a random child of the two individuals that is either a mutation of the left individual, or the genetic
    /// crossover of both.
    ///
    /// The defined_names of the child will only include the code that is specifically named in the child's code. If
    /// both parents have the same defined_name, the value for that will come from the left individual.
    pub fn rand_child(&mut self, left: &[Code], right: &[Code]) -> Result<Vec<Code>> {
        match self.select_genetic_operation() {
            GeneticOperation::Mutation(count) => self.mutate(left, count),
            GeneticOperation::Crossover(count) => self.crossover(left, right, count),
        }
    }

    /// Mutates the parent by randomly selecting a point in the code, generating a new random code item and replacing
    /// the selected point with the new code.
    pub fn mutate(&mut self, parent: &[Code], mut count: u8) -> Result<Vec<Code>> {
        // Most code will replace a single item, but if we replace an item with an IfElse, we could be in for a lot of
        // code. Make sure it doesn't get larger than the allowed amount
        let parent_points: usize = parent.iter().map(|v| v.mutation_points()).sum();
        let max_additional_points = self.config.individual_max_points - parent_points;
        let mut additional_points = if max_additional_points > 1 {
            self.rng.gen_range(1..max_additional_points)
        } else {
            1
        };

        // Turn the parent into a stream
        let mut stream = CodeStream::to_stream(parent);

        // Mutate a single point in the stream the specified number of times
        while count > 0 {
            count -= 1;

            let mutation_point = self.rng.gen_range(0..stream.len());
            let replace_with_code = vec![self.random_code(additional_points)];

            // Turn the new code into a stream as well
            let replace_stream = CodeStream::to_stream(&replace_with_code);

            // Make a new stream with the new code in place of that one element
            let mut next_stream = vec![];
            if mutation_point > 0 {
                next_stream.extend(&stream[0..mutation_point]);
            }
            next_stream.extend(&replace_stream[..]);
            if mutation_point + 1 < stream.len() {
                next_stream.extend(&stream[mutation_point + 1..]);
            }

            // We have a list of borrowed items, clone them to turn them into our real stream. We can't use `.cloned()`
            // because the iterator items are `&&CodeStream`
            stream = next_stream.iter().map(|&x| x.clone()).collect();

            // If we got code larger than one point, we need to adjust the additional_points downward
            additional_points -= replace_with_code[0].mutation_points() - 1;
            if additional_points == 0 {
                break;
            }
        }

        // Turn the stream back into code
        Ok(CodeStream::from_stream(&mut stream.into_iter()))
    }

    /// Produces a random child that is a crossover of both parents. `count` random points along the shortest of the
    /// two code streams will be selected to swap the streams.
    pub fn crossover(&mut self, left_parent: &[Code], right_parent: &[Code], count: u8) -> Result<Vec<Code>> {
        todo!()
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

    /// Sets the weight of every Code variant to the specified value (reset with a default)
    pub fn reset_all_code_weights(&mut self, weight: u8) {
        for entry in self.weights.iter_mut() {
            entry.weight = weight;
        }
        self.sum_of_weights = None;
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
    use crate::*;

    #[test]
    fn test_random_slot() {
        // Use a specific seed so that we always get the same slots for the test
        let mut engine = GeneticEngine::new(GeneticEngineConfiguration::new(Some(1), 10));

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

    #[test]
    fn test_select_genetic_operation() {
        let mut config = GeneticEngineConfiguration::new(Some(1), 10);
        config.mutation_rate = 9; // equal chance of mutation and crossover
        config.max_crossover_points = 5;
        let mut engine = GeneticEngine::new(config);

        assert_eq!(engine.select_genetic_operation(), GeneticOperation::Mutation(1));
        assert_eq!(engine.select_genetic_operation(), GeneticOperation::Mutation(1));
        assert_eq!(engine.select_genetic_operation(), GeneticOperation::Crossover(1));
        assert_eq!(engine.select_genetic_operation(), GeneticOperation::Mutation(1));
        assert_eq!(engine.select_genetic_operation(), GeneticOperation::Mutation(1));
        assert_eq!(engine.select_genetic_operation(), GeneticOperation::Crossover(4));
        assert_eq!(engine.select_genetic_operation(), GeneticOperation::Crossover(2));
    }

    #[test]
    fn test_mutation() {
        let config = GeneticEngineConfiguration::new(Some(1), 10);
        let mut engine = GeneticEngine::new(config);

        // Start with some parent code
        let parent = vec![
            ConstI32::new(2, 1),
            ConstI32::new(3, 3),
            ConstI32::new(4, 0),
            CopySlot::new(0, 1),
            Remainder::new(1, 3, 5),
            AreEqual::new(5, 4, 5),
            DoUntil::new(
                5,
                vec![Add::new(1, 2, 1), Remainder::new(1, 3, 5), AreEqual::new(5, 4, 5)],
            ),
            Return::new(),
        ];

        // Mutate it once
        let child = engine.mutate(&parent[..], 1).unwrap();
        assert_eq!(
            child,
            vec![
                ConstI32::new(2, 1),
                ConstI32::new(3, 3),
                Subtract::new(4, 9, 1), // mutation
                CopySlot::new(0, 1),
                Remainder::new(1, 3, 5),
                AreEqual::new(5, 4, 5),
                DoUntil::new(
                    5,
                    vec![Add::new(1, 2, 1), Remainder::new(1, 3, 5), AreEqual::new(5, 4, 5)],
                ),
                Return::new(),
            ]
        );

        // Mutate it in three places
        let child = engine.mutate(&parent[..], 3).unwrap();
        assert_eq!(
            child,
            vec![
                ConstI32::new(2, 1),
                ConstI32::new(3, 3),
                ConstI32::new(4, 0),
                IsLessThan::new(5, 8, 5), // mutation
                Remainder::new(1, 3, 5),
                Or::new(1, 7, 6), // mutation
                DoUntil::new(
                    5,
                    vec![
                        Add::new(1, 2, 1),
                        And::new(3, 6, 9), // mutation
                        AreEqual::new(5, 4, 5)
                    ],
                ),
                Return::new(),
            ]
        );
    }
}
