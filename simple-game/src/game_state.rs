use rand::{rngs::SmallRng, SeedableRng};

use crate::card::Card;
use crate::suit::Suit;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GameState {
    draw_pile: Vec<Card>,
    play_pile: Vec<Card>,
    top_card_in_finished_suits: Vec<Option<Card>>,
}

impl GameState {
    pub fn new(seed: u64) -> GameState {
        let mut rng = SmallRng::seed_from_u64(seed);
        let deck = Card::make_shuffled_deck(7, &mut rng);

        GameState {
            draw_pile: deck,
            play_pile: vec![],
            top_card_in_finished_suits: vec![None, None, None, None],
        }
    }

    pub fn draw_next_card(&mut self) {
        // If the draw pile is empty, recycle the remaining play pile.
        if 0 == self.draw_pile.len() {
            self.turn_over_play_pile_into_draw_pile();
        }

        if self.draw_pile.len() > 0 {
            self.play_pile.push(self.draw_pile.pop().unwrap());
        }
    }

    fn turn_over_play_pile_into_draw_pile(&mut self) {
        // The play pile is 'turned over' so that the bottom card is the next one drawn.
        while self.play_pile.len() > 0 {
            self.draw_pile.push(self.play_pile.pop().unwrap());
        }
    }

    pub fn move_top_play_pile_card_to_finish(&mut self) -> bool {
        if let Some(card) = self.top_card_of_play_pile() {
            if self.card_is_ready_to_finish(card) {
                let card = self.play_pile.pop().unwrap();
                self.push_card_on_finished_pile(card)
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn card_is_ready_to_finish(&self, card: Card) -> bool {
        let previous_card = self
            .top_card_in_finished_suits
            .get(card.suit() as usize)
            .unwrap();
        card.is_next_card_in_suit(*previous_card)
    }

    fn push_card_on_finished_pile(&mut self, card: Card) -> bool {
        self.top_card_in_finished_suits
            .get_mut(card.suit() as usize)
            .unwrap()
            .replace(card);
        true
    }

    pub fn top_card_of_play_pile(&self) -> Option<Card> {
        self.play_pile.last().copied()
    }

    fn number_of_finished_suit(&self, suit: Suit) -> usize {
        match self.top_card_in_finished_suits.get(suit as usize).unwrap() {
            None => 0,
            Some(card) => card.index_in_suit() + 1,
        }
    }

    fn number_of_finished_spades(&self) -> usize {
        self.number_of_finished_suit(Suit::Spades)
    }

    fn number_of_finished_diamonds(&self) -> usize {
        self.number_of_finished_suit(Suit::Diamonds)
    }

    fn number_of_finished_clubs(&self) -> usize {
        self.number_of_finished_suit(Suit::Clubs)
    }

    fn number_of_finished_hearts(&self) -> usize {
        self.number_of_finished_suit(Suit::Hearts)
    }

    pub fn number_of_finished_cards(&self) -> usize {
        self.number_of_finished_spades()
            + self.number_of_finished_diamonds()
            + self.number_of_finished_clubs()
            + self.number_of_finished_hearts()
    }
}

#[cfg(test)]
mod tests {
    use crate::card::Card;

    use super::GameState;

    #[test]
    fn can_cycle_through_deck() {
        let mut state = GameState::new(12);

        let mut first_order: Vec<Card> = vec![];
        for i in 0..52 {
            assert_eq!(52 - i, state.draw_pile.len());
            assert_eq!(i, state.play_pile.len());
            state.draw_next_card();
            assert_eq!(52 - i - 1, state.draw_pile.len());
            assert_eq!(i + 1, state.play_pile.len());

            first_order.push(*state.play_pile.last().unwrap());
        }

        for i in 0..52 {
            state.draw_next_card();
            assert_eq!(first_order.get(i), state.play_pile.get(i));
        }
    }

    #[test]
    fn card_is_ready_to_finish() {
        let mut state = GameState::new(12);

        let expected_order = vec![
            Card::AceOfDiamonds,
            Card::TwoOfDiamonds,
            Card::ThreeOfDiamonds,
            Card::FourOfDiamonds,
            Card::FiveOfDiamonds,
            Card::SixOfDiamonds,
            Card::SevenOfDiamonds,
            Card::EightOfDiamonds,
            Card::NineOfDiamonds,
            Card::TenOfDiamonds,
            Card::JackOfDiamonds,
            Card::QueenOfDiamonds,
            Card::KingOfDiamonds,
        ];

        for &card_to_play_next in expected_order.iter() {
            // Confirm that this card is the only one that 'is_ready_to_finish'
            for &card_to_test_is_ready in expected_order.iter() {
                let is_ready = state.card_is_ready_to_finish(card_to_test_is_ready);
                if card_to_play_next == card_to_test_is_ready {
                    assert_eq!(
                        is_ready, true,
                        "We wanted to play {:?} but it's marked as not ready",
                        card_to_play_next
                    );
                } else {
                    assert_eq!(
                        is_ready, false,
                        "It's time to play {:?} but card {:?} is ready",
                        card_to_play_next, card_to_test_is_ready
                    );
                }
            }

            // Finish this card
            assert!(state.push_card_on_finished_pile(card_to_play_next));
        }
    }

    #[test]
    fn can_complete_a_game() {
        let mut state = GameState::new(12);
        assert_eq!(52, state.draw_pile.len());

        while state.number_of_finished_cards() < 52 {
            state.draw_next_card();
            state.move_top_play_pile_card_to_finish();
        }
    }
}
