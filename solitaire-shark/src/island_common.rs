use rand::{rngs::SmallRng, Rng, SeedableRng};
use wasmgp::Individual;

use crate::{game_result::GameResult, game_state::GameState};

const GAMES_PER_RUN: usize = 100;

#[derive(Clone)]
pub struct IslandCommon {
    rng: SmallRng,
    game_seeds: Vec<u64>,
}

impl IslandCommon {
    pub fn new() -> IslandCommon {
        IslandCommon {
            rng: SmallRng::from_entropy(),
            game_seeds: vec![],
        }
    }
}

impl IslandCommon {
    /// Before all individuals run, create 100 seeds for the games each will play. This gives every individual on an island
    /// the same 100 shuffled decks.
    pub fn generate_game_seeds(&mut self) {
        while self.game_seeds.len() < GAMES_PER_RUN {
            self.game_seeds.push(self.rng.gen());
        }
    }

    pub fn run_individual(&mut self, individual: &mut Individual<GameState, GameResult>) {
        let mut result = GameResult::new();

        // Play 100 games
        for game_index in 0..GAMES_PER_RUN {
            // Setup a new GameState using the seed that all individuals will use for this game in this run
            let game = GameState::new(*self.game_seeds.get(game_index).unwrap());

            // Execute the individual's code. Note that the individual temporarily owns the game.
            let (game, _) = individual.execute::<(), ()>(game, ());

            // Add the final game to the game results
            result.save_game(game);
        }

        // Save the output of all games in the GameResult for the Individual
        individual.set_run_result(Some(result));
    }
}
