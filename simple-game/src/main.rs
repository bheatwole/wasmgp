mod card;
mod game_result;
mod game_state;
mod island_one;
mod suit;

use game_result::GameResult;
use game_state::GameState;
use island_one::IslandOne;
use wasmgp::*;

fn main() {
    // Configure the world
    let mut config = WorldConfiguration::default();
    config.main_entry_point = FunctionSignature::new("play_game", [], []);
    config.work_slots = SlotCount {
        i32: 3,
        i64: 0,
        f32: 0,
        f64: 0,
    };
    config.individual_max_points = 6;

    // Create the world with the configuration we specified
    let mut world = World::<GameState, GameResult>::new(config).unwrap();

    // Turn off all instructions except...
    world.reset_all_code_weights(0);
    world.set_code_weight(Code::IsEqualZero(IsEqualZero::default()), 3);
    world.set_code_weight(Code::DoUntil(DoUntil::default()), 1);

    // Add the functions that the genetic code can call
    let function_index = world
        .add_function_import("draw_next_card", draw_next_card)
        .unwrap();
    world.set_function_import_weight(function_index, 3).unwrap();
    let function_index = world
        .add_function_import(
            "move_top_play_pile_card_to_finish",
            move_top_play_pile_card_to_finish,
        )
        .unwrap();
    world.set_function_import_weight(function_index, 3).unwrap();

    // Setup the various islands that apply different kinds of pressure to the genetic algorithm.
    world.create_island(Box::new(IslandOne {}));

    // Run the world for 10_000 generations
    let mut generations_complete = 0;
    world
        .run_generations_while(|world| {
            generations_complete += 1;
            println!("Generation {} is complete", generations_complete);
            let most_fit_island_one = world.get_island(0).unwrap().most_fit_individual().unwrap();
            let cards_played = most_fit_island_one.get_run_result().unwrap().cards_played();
            println!("  island one:   {} cards played", cards_played);

            let mut indentation = Indentation::new(4, 0);
            let mut output = std::string::String::new();
            let code: Vec<Code> = most_fit_island_one
                .get_code()
                .iter()
                .map(|c| c.clone())
                .collect();

            code.print_for_rust(&mut output, &mut indentation).unwrap();
            println!("  code: {}", output);

            generations_complete < 10_000 || cards_played == 52
        })
        .unwrap();
}

// Draws the next card and returns true if there are still cards to play
fn draw_next_card(mut caller: Caller<'_, GameState>) -> i32 {
    let game: &mut GameState = caller.data_mut();
    game.draw_next_card();
    (52 - game.number_of_finished_cards()) as i32
}

fn move_top_play_pile_card_to_finish(mut caller: Caller<'_, GameState>) -> i32 {
    let game: &mut GameState = caller.data_mut();
    if game.move_top_play_pile_card_to_finish() {
        1
    } else {
        0
    }
}
