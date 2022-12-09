use crate::game_state::GameState;

#[derive(Clone, Debug, PartialEq)]
pub struct GameResult {
    game_state: GameState,
}

impl wasmgp::RunResult for GameResult {}

impl GameResult {
    pub fn new(game_state: GameState) -> GameResult {
        GameResult { game_state }
    }

    pub fn cards_played(&self) -> usize {
        self.game_state.number_of_finished_cards()
    }
}
