use rand::{rngs::SmallRng, SeedableRng};

use crate::card::Card;
use crate::suit::Suit;

#[derive(Clone, Debug, PartialEq)]
pub struct GameState {
    draw_pile: Vec<Card>,
    play_pile: Vec<Card>,
    top_card_in_finished_suits: Vec<Option<Card>>,
    face_down_work_piles: Vec<Vec<Card>>,
    face_up_work_piles: Vec<Vec<Card>>,
}

impl GameState {
    pub fn new(seed: u64) -> GameState {
        let mut rng = SmallRng::seed_from_u64(seed);
        let deck = Card::make_shuffled_deck(7, &mut rng);

        let mut state = GameState {
            draw_pile: deck,
            play_pile: vec![],
            top_card_in_finished_suits: vec![None, None, None, None],
            face_down_work_piles: vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]],
            face_up_work_piles: vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]],
        };
        state.deal();

        state
    }

    fn deal(&mut self) {
        // When we're done dealing, the work piles should all have one face up card and 0, 1, 2, .. 6 face down cards
        for pile_index in 0..7 {
            let face_down = self.face_down_work_piles.get_mut(pile_index).unwrap();
            for _ in 0..pile_index {
                face_down.push(self.draw_pile.pop().unwrap());
            }

            let face_up = self.face_up_work_piles.get_mut(pile_index).unwrap();
            face_up.push(self.draw_pile.pop().unwrap());
        }
    }

    pub fn draw_next_three(&mut self) {
        // If the draw pile is empty, recycle the remaining play pile.
        if 0 == self.draw_pile.len() {
            self.turn_over_play_pile_into_draw_pile();
        }

        // Draw three cards
        self.draw_next_card();
        self.draw_next_card();
        self.draw_next_card();
    }

    fn turn_over_play_pile_into_draw_pile(&mut self) {
        // The play pile is 'turned over' so that the bottom card is the next one drawn.
        while self.play_pile.len() > 0 {
            self.draw_pile.push(self.play_pile.pop().unwrap());
        }
    }

    fn draw_next_card(&mut self) {
        if self.draw_pile.len() > 0 {
            self.play_pile.push(self.draw_pile.pop().unwrap());
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

    pub fn move_top_work_pile_card_to_finish(&mut self, work_pile_index: usize) -> bool {
        assert!(work_pile_index < 7);
        if let Some(card) = self.face_up_card_in_work_pile(work_pile_index, 0) {
            if self.card_is_ready_to_finish(card) {
                let work_pile = self.face_up_work_piles.get_mut(work_pile_index).unwrap();
                let card = work_pile.pop().unwrap();
                self.flip_over_top_face_down_work_pile_card_if_needed(work_pile_index);
                self.push_card_on_finished_pile(card)
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn move_top_play_pile_card_to_work_pile(&mut self, work_pile_index: usize) -> bool {
        assert!(work_pile_index < 7);
        if let Some(card_to_move) = self.top_card_of_play_pile() {
            if let Some(card_to_move_on_top_of) = self.face_up_card_in_work_pile(work_pile_index, 0)
            {
                if card_to_move.is_solitaire_play_legal(card_to_move_on_top_of) {
                    let card = self.play_pile.pop().unwrap();
                    let work_pile = self.face_up_work_piles.get_mut(work_pile_index).unwrap();
                    work_pile.push(card);
                    true
                } else {
                    false
                }
            } else {
                let card = self.play_pile.pop().unwrap();
                let work_pile = self.face_up_work_piles.get_mut(work_pile_index).unwrap();
                work_pile.push(card);
                true
            }
        } else {
            false
        }
    }

    pub fn move_work_pile_cards_to_another_work_pile(
        &mut self,
        source_work_pile_index: usize,
        number_of_cards_to_move: usize,
        destination_work_pile_index: usize,
    ) -> bool {
        assert!(source_work_pile_index < 7);
        assert!(destination_work_pile_index < 7);
        if let Some(card_to_move) =
            self.face_up_card_in_work_pile(source_work_pile_index, number_of_cards_to_move - 1)
        {
            if self.work_pile_is_empty(destination_work_pile_index) {
                self.move_cards_from_work_pile_to_work_pile_unchecked(
                    source_work_pile_index,
                    number_of_cards_to_move,
                    destination_work_pile_index,
                );
                true
            } else if let Some(card_to_move_on_top_of) =
                self.face_up_card_in_work_pile(destination_work_pile_index, 0)
            {
                if card_to_move.is_solitaire_play_legal(card_to_move_on_top_of) {
                    self.move_cards_from_work_pile_to_work_pile_unchecked(
                        source_work_pile_index,
                        number_of_cards_to_move,
                        destination_work_pile_index,
                    );
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    fn move_cards_from_work_pile_to_work_pile_unchecked(
        &mut self,
        source_work_pile_index: usize,
        number_of_cards_to_move: usize,
        destination_work_pile_index: usize,
    ) {
        let cards = self.pop_cards_off_work_pile(source_work_pile_index, number_of_cards_to_move);
        self.push_cards_onto_work_pile(destination_work_pile_index, cards);
    }

    fn pop_cards_off_work_pile(
        &mut self,
        work_pile_index: usize,
        number_of_cards_to_pop: usize,
    ) -> Vec<Card> {
        let mut cards = vec![];
        let work_pile = self.face_up_work_piles.get_mut(work_pile_index).unwrap();
        for _ in 0..number_of_cards_to_pop {
            cards.push(work_pile.pop().unwrap());
        }
        self.flip_over_top_face_down_work_pile_card_if_needed(work_pile_index);
        cards
    }

    fn push_cards_onto_work_pile(&mut self, work_pile_index: usize, mut cards: Vec<Card>) {
        let work_pile = self.face_up_work_piles.get_mut(work_pile_index).unwrap();
        while cards.len() > 0 {
            work_pile.push(cards.pop().unwrap());
        }
    }

    fn flip_over_top_face_down_work_pile_card_if_needed(&mut self, work_pile_index: usize) {
        let face_up = self.face_up_work_piles.get_mut(work_pile_index).unwrap();
        if 0 == face_up.len() {
            let face_down = self.face_down_work_piles.get_mut(work_pile_index).unwrap();
            if face_down.len() > 0 {
                face_up.push(face_down.pop().unwrap());
            }
        }
    }

    pub fn number_of_cards_in_draw_pile(&self) -> usize {
        self.draw_pile.len()
    }

    pub fn number_of_cards_in_play_pile(&self) -> usize {
        self.play_pile.len()
    }

    pub fn top_card_of_play_pile(&self) -> Option<Card> {
        self.play_pile.last().copied()
    }

    pub fn number_of_finished_suit(&self, suit: Suit) -> usize {
        match self.top_card_in_finished_suits.get(suit as usize).unwrap() {
            None => 0,
            Some(card) => card.index_in_suit() + 1,
        }
    }

    pub fn number_of_finished_spades(&self) -> usize {
        self.number_of_finished_suit(Suit::Spades)
    }

    pub fn number_of_finished_diamonds(&self) -> usize {
        self.number_of_finished_suit(Suit::Diamonds)
    }

    pub fn number_of_finished_clubs(&self) -> usize {
        self.number_of_finished_suit(Suit::Clubs)
    }

    pub fn number_of_finished_hearts(&self) -> usize {
        self.number_of_finished_suit(Suit::Hearts)
    }

    pub fn number_of_finished_cards(&self) -> usize {
        self.number_of_finished_spades()
            + self.number_of_finished_diamonds()
            + self.number_of_finished_clubs()
            + self.number_of_finished_hearts()
    }

    pub fn number_of_face_down_cards_in_work_pile(&self, work_pile_index: usize) -> usize {
        assert!(work_pile_index < 7);
        self.face_down_work_piles[work_pile_index].len()
    }

    pub fn number_of_face_up_cards_in_work_pile(&self, work_pile_index: usize) -> usize {
        assert!(work_pile_index < 7);
        self.face_up_work_piles[work_pile_index].len()
    }

    pub fn work_pile_is_empty(&self, work_pile_index: usize) -> bool {
        self.number_of_face_down_cards_in_work_pile(work_pile_index)
            + self.number_of_face_up_cards_in_work_pile(work_pile_index)
            == 0
    }

    pub fn face_up_card_in_work_pile(
        &self,
        work_pile_index: usize,
        number_of_cards_down: usize,
    ) -> Option<Card> {
        assert!(work_pile_index < 7);
        let work_pile = self.face_down_work_piles.get(work_pile_index).unwrap();
        work_pile.get(number_of_cards_down).copied()
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
