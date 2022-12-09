use rand::seq::SliceRandom;
use rand::Rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::suit::Suit;

#[derive(Copy, Clone, Debug, EnumIter, PartialEq)]
#[repr(u8)]
pub enum Card {
    AceOfSpades = 0,
    TwoOfSpades,
    ThreeOfSpades,
    FourOfSpades,
    FiveOfSpades,
    SixOfSpades,
    SevenOfSpades,
    EightOfSpades,
    NineOfSpades,
    TenOfSpades,
    JackOfSpades,
    QueenOfSpades,
    KingOfSpades,
    AceOfDiamonds,
    TwoOfDiamonds,
    ThreeOfDiamonds,
    FourOfDiamonds,
    FiveOfDiamonds,
    SixOfDiamonds,
    SevenOfDiamonds,
    EightOfDiamonds,
    NineOfDiamonds,
    TenOfDiamonds,
    JackOfDiamonds,
    QueenOfDiamonds,
    KingOfDiamonds,
    AceOfClubs,
    TwoOfClubs,
    ThreeOfClubs,
    FourOfClubs,
    FiveOfClubs,
    SixOfClubs,
    SevenOfClubs,
    EightOfClubs,
    NineOfClubs,
    TenOfClubs,
    JackOfClubs,
    QueenOfClubs,
    KingOfClubs,
    AceOfHearts,
    TwoOfHearts,
    ThreeOfHearts,
    FourOfHearts,
    FiveOfHearts,
    SixOfHearts,
    SevenOfHearts,
    EightOfHearts,
    NineOfHearts,
    TenOfHearts,
    JackOfHearts,
    QueenOfHearts,
    KingOfHearts,
}

impl Card {
    pub fn suit(self) -> Suit {
        let self_int_value = self as u8;

        if self_int_value <= Card::KingOfSpades as u8 {
            Suit::Spades
        } else if self_int_value <= Card::KingOfDiamonds as u8 {
            Suit::Diamonds
        } else if self_int_value <= Card::KingOfClubs as u8 {
            Suit::Clubs
        } else {
            Suit::Hearts
        }
    }

    pub fn index_in_suit(self) -> usize {
        (self as u8 % 13) as usize
    }

    pub fn is_red(self) -> bool {
        self.suit().is_red()
    }

    pub fn is_black(self) -> bool {
        self.suit().is_black()
    }

    pub fn is_solitaire_play_legal(self, can_play_on_top_of: Card) -> bool {
        self.is_red() != can_play_on_top_of.is_red()
            && self.index_in_suit() + 1 == can_play_on_top_of.index_in_suit()
    }

    pub fn is_next_card_in_suit(&self, previous_card: Option<Card>) -> bool {
        if let Some(previous_card) = previous_card {
            self.suit() == previous_card.suit()
                && self.index_in_suit() + 1 == previous_card.index_in_suit()
        } else {
            self.index_in_suit() == 0
        }
    }

    pub fn make_deck() -> Vec<Card> {
        let mut deck = vec![];
        for i in Card::iter() {
            deck.push(i);
        }
        deck
    }

    pub fn make_shuffled_deck<R>(shuffles: usize, rng: &mut R) -> Vec<Card>
    where
        R: Rng + ?Sized,
    {
        let mut deck = Card::make_deck();
        for _ in 0..shuffles {
            deck.shuffle(rng);
        }
        deck
    }
}
