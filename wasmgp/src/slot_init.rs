#[derive(Clone, Debug, PartialEq)]
pub enum SlotInit {
    /// Initialize all slots to zero (default)
    Zero,

    /// Initialize all slots to one
    One,

    /// Initialize all slots to a random value
    Random,
}
