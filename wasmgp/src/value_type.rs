/// Represents the different types that a parameter or result may have in wasmgp. We do not currently allow passing
/// references to functions and so we limit the types to the numerical types. Note that in Wasm, the 'I' type is used
/// for both signed and unsigned integers
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}

impl Into<wasm_ast::ValueType> for ValueType {
    fn into(self) -> wasm_ast::ValueType {
        match &self {
            ValueType::I32 => wasm_ast::ValueType::I32,
            ValueType::I64 => wasm_ast::ValueType::I64,
            ValueType::F32 => wasm_ast::ValueType::F32,
            ValueType::F64 => wasm_ast::ValueType::F64,
        }
    }
}

impl From<wasmtime::ValType> for ValueType {
    fn from(value: wasmtime::ValType) -> Self {
        match value {
            wasmtime::ValType::I32 => ValueType::I32,
            wasmtime::ValType::I64 => ValueType::I64,
            wasmtime::ValType::F32 => ValueType::F32,
            wasmtime::ValType::F64 => ValueType::F64,
            _ => panic!("unsupported wasmtime::ValType {:?}", value),
        }
    }
}