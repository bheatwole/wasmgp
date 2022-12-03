use crate::{Code, RunResult};

#[derive(Clone, Debug, PartialEq)]
pub struct Individual<R: RunResult> {
    code: Code,
    run_result: Option<R>,
}

impl<R: RunResult> Individual<R> {
    pub fn new(code: Code, initial_run_result: Option<R>) -> Individual<R> {
        Individual {
            code,
            run_result: initial_run_result,
        }
    }

    /// Borrows the Individual's code
    pub fn get_code(&self) -> &Code {
        &self.code
    }

    /// Sets the Individual's code to a new value
    pub fn set_code(&mut self, code: Code) {
        self.code = code
    }

    /// Borrows the current RunResult for the Individual
    pub fn get_run_result(&self) -> Option<&R> {
        self.run_result.as_ref()
    }

    /// Mutably borrows the current RunResult for the Individual, allowing for changes to results
    pub fn get_run_result_mut(&mut self) -> Option<&mut R> {
        self.run_result.as_mut()
    }

    /// Replaces the RunResult for this Individual
    pub fn set_run_result(&mut self, run_result: Option<R>) {
        self.run_result = run_result;
    }
}
