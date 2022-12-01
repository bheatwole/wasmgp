use crate::Slot;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub struct GeneticEngine {
    rng: SmallRng,
    slot_count: Slot,
}

impl GeneticEngine {
    pub fn new(seed: Option<u64>, slot_count: Slot) -> GeneticEngine {
        GeneticEngine {
            rng: small_rng_from_optional_seed(seed),
            slot_count,
        }
    }

    pub fn random_slot(&mut self) -> Slot {
        self.rng.gen_range(0..self.slot_count)
    }
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
