use anyhow::Result;
use wasmtime::{InstancePre, Store, WasmParams, WasmResults};

use crate::{Code, RunResult};

pub struct Individual<T, R: RunResult> {
    code: Vec<Code>,
    function_name: String,
    instance_pre: InstancePre<T>,
    deadline: u64,
    run_result: Option<R>,
}

impl<T, R: RunResult> Individual<T, R> {
    pub(crate) fn new(
        code: Vec<Code>,
        function_name: String,
        instance_pre: InstancePre<T>,
        deadline: u64,
    ) -> Individual<T, R> {
        Individual {
            code,
            function_name,
            instance_pre,
            deadline,
            run_result: None,
        }
    }

    /// Borrows the Individual's code
    pub fn get_code(&self) -> &[Code] {
        &self.code[..]
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

    /// Executes the individual's code on the specified state and using the specified parameters. Both params and
    /// results are a tuple containing the variables.
    ///
    /// Note that `execute` temporarily owns the state, but will pass it back no matter whether the execution of the
    /// code succeeds or not.
    pub fn execute<Params, Results>(&mut self, state: T, params: Params) -> (T, Result<Results>)
    where
        Params: WasmParams,
        Results: WasmResults,
    {
        // Create a new instance that references the state. If this fails, we need to unpack the state to be able to
        // pass it back to the caller
        let engine = self.instance_pre.module().engine();
        let mut store = Store::new(engine, state);
        let result = self.instance_pre.instantiate(&mut store);
        if result.is_err() {
            let state = store.into_data();
            return (state, Err(result.unwrap_err()));
        }
        let instance = result.unwrap();

        // Get the typed function from the instance. If this fails, we need to unpack the state to be able to pass it
        // back to the caller.
        let result: Result<wasmtime::TypedFunc<Params, Results>, anyhow::Error> =
            instance.get_typed_func(&mut store, &self.function_name);
        if result.is_err() {
            let state = store.into_data();
            let err = result.err().unwrap();
            return (state, Err(err));
        }
        let func = result.unwrap();

        // Call the function. Unpack the state from the store and return the state and whatever the results of the
        // function were. This will run for the specified number of milliseconds at most.
        store.set_epoch_deadline(self.deadline);
        let result = func.call(&mut store, params);
        let state = store.into_data();
        (state, result)
    }
}

impl<T, R: RunResult> Clone for Individual<T, R> {
    fn clone(&self) -> Self {
        Self {
            code: self.code.clone(),
            function_name: self.function_name.clone(),
            instance_pre: self.instance_pre.clone(),
            deadline: self.deadline.clone(),
            run_result: self.run_result.clone(),
        }
    }
}
