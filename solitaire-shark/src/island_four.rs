use std::cmp::Ordering;

use wasmgp::{Individual, IslandCallbacks};

use crate::{game_result::GameResult, game_state::GameState, island_common::*};

pub struct IslandFour {
    common: IslandCommon,
}

impl IslandFour {
    pub fn new() -> IslandFour {
        IslandFour {
            common: IslandCommon::new(),
        }
    }
}

impl IslandCallbacks<GameState, GameResult> for IslandFour {
    fn pre_generation_run(&mut self, _individuals: &[Individual<GameState, GameResult>]) {
        self.common.generate_game_seeds();
    }

    fn run_individual(&mut self, individual: &mut Individual<GameState, GameResult>) {
        self.common.run_individual(individual);
    }

    fn sort_individuals(
        &self,
        a: &Individual<GameState, GameResult>,
        b: &Individual<GameState, GameResult>,
    ) -> std::cmp::Ordering {
        // island_five_fitness_score_fn: run 100 games and score on fewest cards in face_up piles, then win rate
        let a_result = a.get_run_result().unwrap();
        let b_result = b.get_run_result().unwrap();
        let mut cmp = b_result
            .number_of_face_up_cards()
            .cmp(&a_result.number_of_face_up_cards());

        if Ordering::Equal == cmp {
            cmp = a_result.games_won().cmp(&b_result.games_won());
        }

        cmp
    }

    fn score_individual(&self, i: &Individual<GameState, GameResult>) -> u64 {
        5200 - i.get_run_result().unwrap().number_of_face_up_cards() as u64
    }

    fn clone(&self) -> Box<dyn IslandCallbacks<GameState, GameResult>> {
        Box::new(IslandFour {
            common: self.common.clone(),
        })
    }
}
