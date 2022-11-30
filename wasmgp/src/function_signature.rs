use wasm_ast::{FunctionType, ResultType};
use wasmtime::FuncType;

use crate::ValueType;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionSignature {
    name: String,
    params: Vec<ValueType>,
    results: Vec<ValueType>,
}

impl FunctionSignature {
    pub fn new<S: Into<String>>(
        name: S,
        params: impl IntoIterator<Item = ValueType>,
        results: impl IntoIterator<Item = ValueType>,
    ) -> FunctionSignature {
        FunctionSignature {
            name: name.into(),
            params: params.into_iter().collect(),
            results: results.into_iter().collect(),
        }
    }

    pub fn new_from_func_type<S: Into<String>>(name: S, func_type: FuncType) -> FunctionSignature {
        let params = func_type.params().map(|v| v.into()).collect();
        let results = func_type.results().map(|v| v.into()).collect();

        FunctionSignature {
            name: name.into(),
            params,
            results,
        }
    }

    pub fn empty() -> FunctionSignature {
        FunctionSignature {
            name: "main".to_owned(),
            params: vec![],
            results: vec![],
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn params(&self) -> &[ValueType] {
        &self.params
    }

    pub fn params_ast(&self) -> Vec<wasm_ast::ValueType> {
        self.params.iter().map(|f| (*f).into()).collect()
    }

    pub fn results(&self) -> &[ValueType] {
        &self.results
    }

    pub fn results_ast(&self) -> Vec<wasm_ast::ValueType> {
        self.results.iter().map(|f| (*f).into()).collect()
    }
}

impl Into<FunctionType> for FunctionSignature {
    fn into(self) -> FunctionType {
        let params: Vec<wasm_ast::ValueType> = self.params.iter().map(|v| <ValueType>::into(*v)).collect();
        let results: Vec<wasm_ast::ValueType> = self.results.iter().map(|v| <ValueType>::into(*v)).collect();
        FunctionType::new(ResultType::new(params), ResultType::new(results))
    }
}
