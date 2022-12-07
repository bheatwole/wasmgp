#[derive(Debug, PartialEq)]
pub enum GeneticOperation {
    /// A single point of code will be mutated the specified number of times
    Mutation(u8),

    /// The code from the two parents will be swapped at random positions the specified number of times
    Crossover(u8),
}
