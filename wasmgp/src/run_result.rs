use std::fmt::Debug;

/// This trait is a alias to avoid typing all the restrictions everytime we need to reference them
pub trait RunResult: Clone + Debug + PartialEq + 'static {}

/// This empty struct can be used when run results are not needed. Some tests and doctests make use of this
#[derive(Clone, Debug, PartialEq)]
pub struct EmptyRunResult {}

impl RunResult for EmptyRunResult {}
