use wasmtime::FuncType;

use crate::ValueType;

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
}
