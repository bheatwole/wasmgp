mod card;
mod game_result;
mod game_state;
mod island_common;
mod island_five;
mod island_four;
mod island_one;
mod island_three;
mod island_two;
mod suit;

use game_result::GameResult;
use game_state::GameState;
use island_five::IslandFive;
use island_four::IslandFour;
use island_one::IslandOne;
use island_three::IslandThree;
use island_two::IslandTwo;
use wasmgp::*;

fn main() {
    // Configure the world
    let mut config = WorldConfiguration::default();
    config.main_entry_point = FunctionSignature::new("play_game", [], []);
    config.work_slots = SlotCount {
        i32: 10,
        i64: 0,
        f32: 0,
        f64: 0,
    };
    config.individual_run_time_ms = 10;

    // Create the world with the configuration we specified
    let mut world = World::<GameState, GameResult>::new(config).unwrap();

    // Add the functions that the genetic code can call
    world
        .add_function_import("draw_next_three", draw_next_three)
        .unwrap();
    world
        .add_function_import(
            "move_top_play_pile_card_to_finish",
            move_top_play_pile_card_to_finish,
        )
        .unwrap();
    world
        .add_function_import("card_is_ready_to_finish", card_is_ready_to_finish)
        .unwrap();
    world
        .add_function_import(
            "move_top_work_pile_card_to_finish",
            move_top_work_pile_card_to_finish,
        )
        .unwrap();
    world
        .add_function_import(
            "move_top_play_pile_card_to_work_pile",
            move_top_play_pile_card_to_work_pile,
        )
        .unwrap();
    world
        .add_function_import(
            "move_work_pile_cards_to_another_work_pile",
            move_work_pile_cards_to_another_work_pile,
        )
        .unwrap();
    world
        .add_function_import("number_of_cards_in_draw_pile", number_of_cards_in_draw_pile)
        .unwrap();
    world
        .add_function_import("number_of_cards_in_play_pile", number_of_cards_in_play_pile)
        .unwrap();
    world
        .add_function_import("number_of_finished_spades", number_of_finished_spades)
        .unwrap();
    world
        .add_function_import("number_of_finished_diamonds", number_of_finished_diamonds)
        .unwrap();
    world
        .add_function_import("number_of_finished_clubs", number_of_finished_clubs)
        .unwrap();
    world
        .add_function_import("number_of_finished_hearts", number_of_finished_hearts)
        .unwrap();
    world
        .add_function_import("number_of_finished_cards", number_of_finished_cards)
        .unwrap();
    world
        .add_function_import(
            "number_of_face_down_cards_in_work_pile",
            number_of_face_down_cards_in_work_pile,
        )
        .unwrap();
    world
        .add_function_import(
            "number_of_face_up_cards_in_work_pile",
            number_of_face_up_cards_in_work_pile,
        )
        .unwrap();
    world
        .add_function_import("work_pile_is_empty", work_pile_is_empty)
        .unwrap();
    world
        .add_function_import("face_up_card_in_work_pile", face_up_card_in_work_pile)
        .unwrap();

    // Setup the various islands that apply different kinds of pressure to the genetic algorithm.
    world.create_island(Box::new(IslandOne::new()));
    world.create_island(Box::new(IslandTwo::new()));
    world.create_island(Box::new(IslandThree::new()));
    world.create_island(Box::new(IslandFour::new()));
    world.create_island(Box::new(IslandFive::new()));

    // Run the world for 10_000 generations
    let mut generations_complete = 0;
    world
        .run_generations_while(|world| {
            generations_complete += 1;
            println!("Generation {} is complete", generations_complete);
            let most_fit_island_one = world.get_island(0).unwrap().most_fit_individual().unwrap();
            println!(
                "  island one:   {:.04}% games won",
                most_fit_island_one.get_run_result().unwrap().games_won() as f64 / 100.0f64
            );
            let most_fit_island_two = world.get_island(1).unwrap().most_fit_individual().unwrap();
            println!(
                "  island two:   {:.04} avg finished cards",
                most_fit_island_two
                    .get_run_result()
                    .unwrap()
                    .number_of_finished_cards() as f64
                    / 100.0f64
            );
            let most_fit_island_three = world.get_island(2).unwrap().most_fit_individual().unwrap();
            println!(
                "  island three:   {:.04} avg remaining draw+play cards",
                most_fit_island_three
                    .get_run_result()
                    .unwrap()
                    .number_of_draw_stack_cards() as f64
                    / 100.0f64
            );
            let most_fit_island_four = world.get_island(3).unwrap().most_fit_individual().unwrap();
            println!(
                "  island four:   {:.04} avg remaining face down cards",
                most_fit_island_four
                    .get_run_result()
                    .unwrap()
                    .number_of_face_down_cards() as f64
                    / 100.0f64
            );
            let most_fit_island_five = world.get_island(4).unwrap().most_fit_individual().unwrap();
            println!(
                "  island five:   {:.04} avg remaining face up cards",
                most_fit_island_five
                    .get_run_result()
                    .unwrap()
                    .number_of_face_up_cards() as f64
                    / 100.0f64
            );

            let mut indentation = Indentation::new(4, 0);
            let mut output = std::string::String::new();
            let code: Vec<Code> = most_fit_island_one
                .get_code()
                .iter()
                .map(|c| c.clone())
                .collect();

            code.print_for_rust(&mut output, &mut indentation).unwrap();
            println!("  code: {}", output);

            generations_complete < 10_000
        })
        .unwrap();
}

// Draws the next card and returns true if there are still cards to play
fn draw_next_three(mut caller: Caller<'_, GameState>) {
    let game: &mut GameState = caller.data_mut();
    game.draw_next_three();
}

fn move_top_play_pile_card_to_finish(mut caller: Caller<'_, GameState>) -> i32 {
    let game: &mut GameState = caller.data_mut();
    if game.move_top_play_pile_card_to_finish() {
        1
    } else {
        0
    }
}

fn card_is_ready_to_finish(mut caller: Caller<'_, GameState>, card: i32) -> i32 {
    let game: &mut GameState = caller.data_mut();
    let card = card.abs() % 52;
    if game.card_is_ready_to_finish(card.into()) {
        1
    } else {
        0
    }
}

fn move_top_work_pile_card_to_finish(
    mut caller: Caller<'_, GameState>,
    work_pile_index: i32,
) -> i32 {
    let game: &mut GameState = caller.data_mut();
    let work_pile_index = work_pile_index.abs() % 7;
    if game.move_top_work_pile_card_to_finish(work_pile_index as usize) {
        1
    } else {
        0
    }
}

fn move_top_play_pile_card_to_work_pile(
    mut caller: Caller<'_, GameState>,
    work_pile_index: i32,
) -> i32 {
    let game: &mut GameState = caller.data_mut();
    let work_pile_index = work_pile_index.abs() % 7;
    if game.move_top_play_pile_card_to_work_pile(work_pile_index as usize) {
        1
    } else {
        0
    }
}

fn move_work_pile_cards_to_another_work_pile(
    mut caller: Caller<'_, GameState>,
    source_work_pile_index: i32,
    number_of_cards_to_move: i32,
    destination_work_pile_index: i32,
) -> i32 {
    let game: &mut GameState = caller.data_mut();
    let source_work_pile_index = source_work_pile_index.abs() % 7;
    let destination_work_pile_index = destination_work_pile_index.abs() % 7;
    if game.move_work_pile_cards_to_another_work_pile(
        source_work_pile_index as usize,
        number_of_cards_to_move as usize,
        destination_work_pile_index as usize,
    ) {
        1
    } else {
        0
    }
}

fn number_of_cards_in_draw_pile(mut caller: Caller<'_, GameState>) -> i32 {
    let game: &mut GameState = caller.data_mut();
    game.number_of_cards_in_draw_pile() as i32
}

fn number_of_cards_in_play_pile(mut caller: Caller<'_, GameState>) -> i32 {
    let game: &mut GameState = caller.data_mut();
    game.number_of_cards_in_play_pile() as i32
}

fn number_of_finished_spades(mut caller: Caller<'_, GameState>) -> i32 {
    let game: &mut GameState = caller.data_mut();
    game.number_of_finished_spades() as i32
}

fn number_of_finished_diamonds(mut caller: Caller<'_, GameState>) -> i32 {
    let game: &mut GameState = caller.data_mut();
    game.number_of_finished_diamonds() as i32
}

fn number_of_finished_clubs(mut caller: Caller<'_, GameState>) -> i32 {
    let game: &mut GameState = caller.data_mut();
    game.number_of_finished_clubs() as i32
}

fn number_of_finished_hearts(mut caller: Caller<'_, GameState>) -> i32 {
    let game: &mut GameState = caller.data_mut();
    game.number_of_finished_hearts() as i32
}

fn number_of_finished_cards(mut caller: Caller<'_, GameState>) -> i32 {
    let game: &mut GameState = caller.data_mut();
    game.number_of_finished_cards() as i32
}

fn number_of_face_down_cards_in_work_pile(
    mut caller: Caller<'_, GameState>,
    work_pile_index: i32,
) -> i32 {
    let game: &mut GameState = caller.data_mut();
    let index = work_pile_index.abs() % 7;
    game.number_of_face_down_cards_in_work_pile(index as usize) as i32
}

fn number_of_face_up_cards_in_work_pile(
    mut caller: Caller<'_, GameState>,
    work_pile_index: i32,
) -> i32 {
    let game: &mut GameState = caller.data_mut();
    let index = work_pile_index.abs() % 7;
    game.number_of_face_up_cards_in_work_pile(index as usize) as i32
}

fn work_pile_is_empty(mut caller: Caller<'_, GameState>, work_pile_index: i32) -> i32 {
    let game: &mut GameState = caller.data_mut();
    let index = work_pile_index.abs() % 7;
    if game.work_pile_is_empty(index as usize) {
        1
    } else {
        0
    }
}

fn face_up_card_in_work_pile(
    mut caller: Caller<'_, GameState>,
    work_pile_index: i32,
    number_of_cards_down: i32,
) -> (i32, i32) {
    let game: &mut GameState = caller.data_mut();
    let index = work_pile_index.abs() % 7;
    if let Some(card) =
        game.face_up_card_in_work_pile(index as usize, number_of_cards_down as usize)
    {
        (1, card as i32)
    } else {
        (0, 0)
    }
}
