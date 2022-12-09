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
