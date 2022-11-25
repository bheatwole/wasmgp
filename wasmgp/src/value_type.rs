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

impl ValueType {
    #[inline]
    pub fn is_float(&self) -> bool {
        match self {
            ValueType::F32 | ValueType::F64 => true,
            _ => false,
        }
    }
}

impl Into<wasm_ast::IntegerType> for ValueType {
    fn into(self) -> wasm_ast::IntegerType {
        match &self {
            ValueType::I32 => wasm_ast::IntegerType::I32,
            ValueType::I64 => wasm_ast::IntegerType::I64,
            _ => panic!("unsupported conversion of float to integer"),
        }
    }
}

impl Into<wasm_ast::FloatType> for ValueType {
    fn into(self) -> wasm_ast::FloatType {
        match &self {
            ValueType::F32 => wasm_ast::FloatType::F32,
            ValueType::F64 => wasm_ast::FloatType::F64,
            _ => panic!("unsupported conversion of integer to float"),
        }
    }
}

impl Into<wasm_ast::NumberType> for ValueType {
    fn into(self) -> wasm_ast::NumberType {
        match &self {
            ValueType::I32 => wasm_ast::NumberType::I32,
            ValueType::I64 => wasm_ast::NumberType::I64,
            ValueType::F32 => wasm_ast::NumberType::F32,
            ValueType::F64 => wasm_ast::NumberType::F64,
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
