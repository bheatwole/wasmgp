use rand::seq::SliceRandom;
use rand::Rng;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, FromRepr};

use crate::suit::Suit;

#[derive(Copy, Clone, Debug, EnumIter, FromRepr, PartialEq)]
#[repr(i32)]
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

    pub fn is_solitaire_play_legal(self, can_play_on_top_of: Card) -> bool {
        self.is_red() != can_play_on_top_of.is_red()
            && self.index_in_suit() + 1 == can_play_on_top_of.index_in_suit()
    }

    pub fn is_next_card_in_suit(&self, previous_card: Option<Card>) -> bool {
        if let Some(previous_card) = previous_card {
            self.suit() == previous_card.suit()
                && self.index_in_suit() == previous_card.index_in_suit() + 1
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

impl From<i32> for Card {
    fn from(value: i32) -> Self {
        Card::from_repr(value).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::card::Card;

    #[test]
    fn index_in_suit() {
        assert_eq!(Card::AceOfSpades.index_in_suit(), 0);
        assert_eq!(Card::TwoOfSpades.index_in_suit(), 1);
        assert_eq!(Card::ThreeOfSpades.index_in_suit(), 2);
        assert_eq!(Card::FourOfSpades.index_in_suit(), 3);
        assert_eq!(Card::FiveOfSpades.index_in_suit(), 4);
        assert_eq!(Card::SixOfSpades.index_in_suit(), 5);
        assert_eq!(Card::SevenOfSpades.index_in_suit(), 6);
        assert_eq!(Card::EightOfSpades.index_in_suit(), 7);
        assert_eq!(Card::NineOfSpades.index_in_suit(), 8);
        assert_eq!(Card::TenOfSpades.index_in_suit(), 9);
        assert_eq!(Card::JackOfSpades.index_in_suit(), 10);
        assert_eq!(Card::QueenOfSpades.index_in_suit(), 11);
        assert_eq!(Card::KingOfSpades.index_in_suit(), 12);
        assert_eq!(Card::AceOfDiamonds.index_in_suit(), 0);
        assert_eq!(Card::TwoOfDiamonds.index_in_suit(), 1);
        assert_eq!(Card::ThreeOfDiamonds.index_in_suit(), 2);
        assert_eq!(Card::FourOfDiamonds.index_in_suit(), 3);
        assert_eq!(Card::FiveOfDiamonds.index_in_suit(), 4);
        assert_eq!(Card::SixOfDiamonds.index_in_suit(), 5);
        assert_eq!(Card::SevenOfDiamonds.index_in_suit(), 6);
        assert_eq!(Card::EightOfDiamonds.index_in_suit(), 7);
        assert_eq!(Card::NineOfDiamonds.index_in_suit(), 8);
        assert_eq!(Card::TenOfDiamonds.index_in_suit(), 9);
        assert_eq!(Card::JackOfDiamonds.index_in_suit(), 10);
        assert_eq!(Card::QueenOfDiamonds.index_in_suit(), 11);
        assert_eq!(Card::KingOfDiamonds.index_in_suit(), 12);
        assert_eq!(Card::AceOfClubs.index_in_suit(), 0);
        assert_eq!(Card::TwoOfClubs.index_in_suit(), 1);
        assert_eq!(Card::ThreeOfClubs.index_in_suit(), 2);
        assert_eq!(Card::FourOfClubs.index_in_suit(), 3);
        assert_eq!(Card::FiveOfClubs.index_in_suit(), 4);
        assert_eq!(Card::SixOfClubs.index_in_suit(), 5);
        assert_eq!(Card::SevenOfClubs.index_in_suit(), 6);
        assert_eq!(Card::EightOfClubs.index_in_suit(), 7);
        assert_eq!(Card::NineOfClubs.index_in_suit(), 8);
        assert_eq!(Card::TenOfClubs.index_in_suit(), 9);
        assert_eq!(Card::JackOfClubs.index_in_suit(), 10);
        assert_eq!(Card::QueenOfClubs.index_in_suit(), 11);
        assert_eq!(Card::KingOfClubs.index_in_suit(), 12);
        assert_eq!(Card::AceOfHearts.index_in_suit(), 0);
        assert_eq!(Card::TwoOfHearts.index_in_suit(), 1);
        assert_eq!(Card::ThreeOfHearts.index_in_suit(), 2);
        assert_eq!(Card::FourOfHearts.index_in_suit(), 3);
        assert_eq!(Card::FiveOfHearts.index_in_suit(), 4);
        assert_eq!(Card::SixOfHearts.index_in_suit(), 5);
        assert_eq!(Card::SevenOfHearts.index_in_suit(), 6);
        assert_eq!(Card::EightOfHearts.index_in_suit(), 7);
        assert_eq!(Card::NineOfHearts.index_in_suit(), 8);
        assert_eq!(Card::TenOfHearts.index_in_suit(), 9);
        assert_eq!(Card::JackOfHearts.index_in_suit(), 10);
        assert_eq!(Card::QueenOfHearts.index_in_suit(), 11);
        assert_eq!(Card::KingOfHearts.index_in_suit(), 12);
    }

    #[test]
    fn next_in_suit() {
        assert!(Card::AceOfSpades.is_next_card_in_suit(None));
        assert!(Card::TwoOfSpades.is_next_card_in_suit(Some(Card::AceOfSpades)));
        assert!(Card::ThreeOfSpades.is_next_card_in_suit(Some(Card::TwoOfSpades)));
        assert!(Card::FourOfSpades.is_next_card_in_suit(Some(Card::ThreeOfSpades)));
        assert!(Card::FiveOfSpades.is_next_card_in_suit(Some(Card::FourOfSpades)));
        assert!(Card::SixOfSpades.is_next_card_in_suit(Some(Card::FiveOfSpades)));
        assert!(Card::SevenOfSpades.is_next_card_in_suit(Some(Card::SixOfSpades)));
        assert!(Card::EightOfSpades.is_next_card_in_suit(Some(Card::SevenOfSpades)));
        assert!(Card::NineOfSpades.is_next_card_in_suit(Some(Card::EightOfSpades)));
        assert!(Card::TenOfSpades.is_next_card_in_suit(Some(Card::NineOfSpades)));
        assert!(Card::JackOfSpades.is_next_card_in_suit(Some(Card::TenOfSpades)));
        assert!(Card::QueenOfSpades.is_next_card_in_suit(Some(Card::JackOfSpades)));
        assert!(Card::KingOfSpades.is_next_card_in_suit(Some(Card::QueenOfSpades)));
        assert!(Card::AceOfDiamonds.is_next_card_in_suit(None));
        assert!(Card::TwoOfDiamonds.is_next_card_in_suit(Some(Card::AceOfDiamonds)));
        assert!(Card::ThreeOfDiamonds.is_next_card_in_suit(Some(Card::TwoOfDiamonds)));
        assert!(Card::FourOfDiamonds.is_next_card_in_suit(Some(Card::ThreeOfDiamonds)));
        assert!(Card::FiveOfDiamonds.is_next_card_in_suit(Some(Card::FourOfDiamonds)));
        assert!(Card::SixOfDiamonds.is_next_card_in_suit(Some(Card::FiveOfDiamonds)));
        assert!(Card::SevenOfDiamonds.is_next_card_in_suit(Some(Card::SixOfDiamonds)));
        assert!(Card::EightOfDiamonds.is_next_card_in_suit(Some(Card::SevenOfDiamonds)));
        assert!(Card::NineOfDiamonds.is_next_card_in_suit(Some(Card::EightOfDiamonds)));
        assert!(Card::TenOfDiamonds.is_next_card_in_suit(Some(Card::NineOfDiamonds)));
        assert!(Card::JackOfDiamonds.is_next_card_in_suit(Some(Card::TenOfDiamonds)));
        assert!(Card::QueenOfDiamonds.is_next_card_in_suit(Some(Card::JackOfDiamonds)));
        assert!(Card::KingOfDiamonds.is_next_card_in_suit(Some(Card::QueenOfDiamonds)));
        assert!(Card::AceOfClubs.is_next_card_in_suit(None));
        assert!(Card::TwoOfClubs.is_next_card_in_suit(Some(Card::AceOfClubs)));
        assert!(Card::ThreeOfClubs.is_next_card_in_suit(Some(Card::TwoOfClubs)));
        assert!(Card::FourOfClubs.is_next_card_in_suit(Some(Card::ThreeOfClubs)));
        assert!(Card::FiveOfClubs.is_next_card_in_suit(Some(Card::FourOfClubs)));
        assert!(Card::SixOfClubs.is_next_card_in_suit(Some(Card::FiveOfClubs)));
        assert!(Card::SevenOfClubs.is_next_card_in_suit(Some(Card::SixOfClubs)));
        assert!(Card::EightOfClubs.is_next_card_in_suit(Some(Card::SevenOfClubs)));
        assert!(Card::NineOfClubs.is_next_card_in_suit(Some(Card::EightOfClubs)));
        assert!(Card::TenOfClubs.is_next_card_in_suit(Some(Card::NineOfClubs)));
        assert!(Card::JackOfClubs.is_next_card_in_suit(Some(Card::TenOfClubs)));
        assert!(Card::QueenOfClubs.is_next_card_in_suit(Some(Card::JackOfClubs)));
        assert!(Card::KingOfClubs.is_next_card_in_suit(Some(Card::QueenOfClubs)));
        assert!(Card::AceOfHearts.is_next_card_in_suit(None));
        assert!(Card::TwoOfHearts.is_next_card_in_suit(Some(Card::AceOfHearts)));
        assert!(Card::ThreeOfHearts.is_next_card_in_suit(Some(Card::TwoOfHearts)));
        assert!(Card::FourOfHearts.is_next_card_in_suit(Some(Card::ThreeOfHearts)));
        assert!(Card::FiveOfHearts.is_next_card_in_suit(Some(Card::FourOfHearts)));
        assert!(Card::SixOfHearts.is_next_card_in_suit(Some(Card::FiveOfHearts)));
        assert!(Card::SevenOfHearts.is_next_card_in_suit(Some(Card::SixOfHearts)));
        assert!(Card::EightOfHearts.is_next_card_in_suit(Some(Card::SevenOfHearts)));
        assert!(Card::NineOfHearts.is_next_card_in_suit(Some(Card::EightOfHearts)));
        assert!(Card::TenOfHearts.is_next_card_in_suit(Some(Card::NineOfHearts)));
        assert!(Card::JackOfHearts.is_next_card_in_suit(Some(Card::TenOfHearts)));
        assert!(Card::QueenOfHearts.is_next_card_in_suit(Some(Card::JackOfHearts)));
        assert!(Card::KingOfHearts.is_next_card_in_suit(Some(Card::QueenOfHearts)));
    }
}
