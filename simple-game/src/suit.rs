#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Suit {
    Spades = 0,
    Diamonds,
    Clubs,
    Hearts,
}

impl Suit {}
