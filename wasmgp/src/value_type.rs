/// Represents the different types that a parameter or result may have in wasmgp. We do not currently allow passing
/// references to functions and so we limit the types to the numerical types. Note that in Wasm, the 'I' type is used
/// for both signed and unsigned integers
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}