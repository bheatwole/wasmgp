use std::cmp::Ordering;
use wasmgp::{Individual, IslandCallbacks};

use crate::game_state::GameState;
use crate::{game_result::GameResult, island_common::*};

pub struct IslandOne {
    common: IslandCommon,
}

impl IslandOne {
    pub fn new() -> IslandOne {
        IslandOne {
            common: IslandCommon::new(),
        }
    }
}

impl IslandCallbacks<GameState, GameResult> for IslandOne {
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
        // island_one_fitness_score_fn: run 100 games and score on most games won, then smallest code size
        let a_result = a.get_run_result().unwrap();
        let b_result = b.get_run_result().unwrap();
        let mut cmp = a_result.games_won().cmp(&b_result.games_won());

        if Ordering::Equal == cmp {
            let a_points: usize = a.get_code().iter().map(|c| c.points()).sum();
            let b_points: usize = b.get_code().iter().map(|c| c.points()).sum();
            cmp = a_points.cmp(&b_points);
        }

        cmp
    }

    fn score_individual(&self, i: &Individual<GameState, GameResult>) -> u64 {
        i.get_run_result().unwrap().games_won() as u64
    }

    fn clone(&self) -> Box<dyn IslandCallbacks<GameState, GameResult>> {
        Box::new(IslandOne {
            common: self.common.clone(),
        })
    }
}
