use wasmgp::{Individual, IslandCallbacks};

use crate::game_result::GameResult;
use crate::game_state::GameState;

pub struct IslandOne {}

impl IslandCallbacks<GameState, GameResult> for IslandOne {
    fn clone(&self) -> Box<dyn IslandCallbacks<GameState, GameResult>> {
        Box::new(IslandOne {})
    }

    fn run_individual(&mut self, individual: &mut Individual<GameState, GameResult>) {
        // Perform any simulation setup
        let state = GameState::new(12);

        // Execute the individual's code. Note that the individual temporarily owns the state.
        let (state, _) = individual.execute::<(), ()>(state, ());

        // Calculate how fit this individual is, and store that value. This is the where each island will emphasize
        // a different feature of an individual. One island may place a higher value on code size, another on
        // 'winning' at any cost, another on 'not losing', etc
        individual.set_run_result(Some(GameResult::new(state)));
    }
}
