use crate::ValueType;
use wasm_ast::{FunctionType, ResultType};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MainEntryPoint {
    parameters: Vec<ValueType>,
    results: Vec<ValueType>,
}

impl MainEntryPoint {
    pub fn empty() -> MainEntryPoint {
        MainEntryPoint {
            parameters: vec![],
            results: vec![],
        }
    }

    pub fn new(parameters: Vec<ValueType>, results: Vec<ValueType>) -> MainEntryPoint {
        MainEntryPoint { parameters, results }
    }
}

impl Into<FunctionType> for MainEntryPoint {
    fn into(self) -> FunctionType {
        let ast_parameters = self.parameters.iter().map(|t| ValueType::into(*t)).collect();
        let ast_results = self.results.iter().map(|t| ValueType::into(*t)).collect();

        FunctionType::new(ResultType::new(ast_parameters), ResultType::new(ast_results))
    }
}
