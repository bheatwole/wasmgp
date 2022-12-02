use anyhow::Result;
use std::fmt::Write;
use wasm_ast::{Instruction, NumericInstruction};

use crate::code_builder::CodeBuilder;
use crate::convert::SetSlotConvert;
use crate::indentation::Indentation;
use crate::{Code, CodeContext, Slot, ValueType};

#[derive(Default, PartialEq)]
pub struct ConstI32 {
    slot: Slot,
    value: i32,
}

impl ConstI32 {
    pub fn new(slot: Slot, value: i32) -> Code {
        Code::ConstI32(ConstI32 { slot, value })
    }
}

impl CodeBuilder for ConstI32 {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        instruction_list.push(NumericInstruction::I32Constant(self.value).into());
        SetSlotConvert::convert(self.slot, ValueType::I32, context, instruction_list)
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(f, "{}ConstI32::new({}, {}),", indentation, self.slot, self.value)
    }
}

#[derive(Default, PartialEq)]
pub struct ConstI64 {
    slot: Slot,
    value: i64,
}

impl ConstI64 {
    pub fn new(slot: Slot, value: i64) -> Code {
        Code::ConstI64(ConstI64 { slot, value })
    }
}

impl CodeBuilder for ConstI64 {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        instruction_list.push(NumericInstruction::I64Constant(self.value).into());
        SetSlotConvert::convert(self.slot, ValueType::I64, context, instruction_list)
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(f, "{}ConstI64::new({}, {}),", indentation, self.slot, self.value)
    }
}

#[derive(Default, PartialEq)]
pub struct ConstF32 {
    slot: Slot,
    value: f32,
}

impl ConstF32 {
    pub fn new(slot: Slot, value: f32) -> Code {
        Code::ConstF32(ConstF32 { slot, value })
    }
}

impl CodeBuilder for ConstF32 {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        instruction_list.push(NumericInstruction::F32Constant(self.value).into());
        SetSlotConvert::convert(self.slot, ValueType::F32, context, instruction_list)
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(f, "{}ConstF32::new({}, {}f32),", indentation, self.slot, self.value)
    }
}

#[derive(Default, PartialEq)]
pub struct ConstF64 {
    slot: Slot,
    value: f64,
}

impl ConstF64 {
    pub fn new(slot: Slot, value: f64) -> Code {
        Code::ConstF64(ConstF64 { slot, value })
    }
}

impl CodeBuilder for ConstF64 {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        instruction_list.push(NumericInstruction::F64Constant(self.value).into());
        SetSlotConvert::convert(self.slot, ValueType::F64, context, instruction_list)
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(f, "{}ConstF64::new({}, {}f64),", indentation, self.slot, self.value)
    }
}

#[cfg(test)]
mod tests {
    use wasmgp_macros::wasm_code;

    use crate::{ConstF32, ConstF64, ConstI32, ConstI64, Return};

    #[wasm_code]
    fn const_i32_and_return_i32() -> u32 {
        [ConstI32::new(0, 42), Return::new()]
    }

    #[test]
    fn const_i32_and_return_i32() {
        let func = ConstI32AndReturnI32::new().unwrap();
        assert_eq!(42u32, func.call().unwrap());
    }

    #[wasm_code]
    fn const_i32_and_return_i64() -> u64 {
        [ConstI32::new(0, 42), Return::new()]
    }

    #[test]
    fn const_i32_and_return_i64() {
        let func = ConstI32AndReturnI64::new().unwrap();
        assert_eq!(42u64, func.call().unwrap());
    }

    #[wasm_code]
    fn const_i32_and_return_f32() -> f32 {
        [ConstI32::new(0, 42), Return::new()]
    }

    #[test]
    fn const_i32_and_return_f32() {
        let func = ConstI32AndReturnF32::new().unwrap();
        assert_eq!(42f32, func.call().unwrap());
    }

    #[wasm_code]
    fn const_i32_and_return_f64() -> f64 {
        [ConstI32::new(0, 42), Return::new()]
    }

    #[test]
    fn const_i32_and_return_f64() {
        let func = ConstI32AndReturnF64::new().unwrap();
        assert_eq!(42f64, func.call().unwrap());
    }

    #[wasm_code]
    fn const_i64_and_return_i32() -> u32 {
        [ConstI64::new(0, 42), Return::new()]
    }

    #[test]
    fn const_i64_and_return_i32() {
        let func = ConstI64AndReturnI32::new().unwrap();
        assert_eq!(42u32, func.call().unwrap());
    }

    #[wasm_code]
    fn const_i64_and_return_i64() -> u64 {
        [ConstI64::new(0, 42), Return::new()]
    }

    #[test]
    fn const_i64_and_return_i64() {
        let func = ConstI64AndReturnI64::new().unwrap();
        assert_eq!(42u64, func.call().unwrap());
    }

    #[wasm_code]
    fn const_i64_and_return_f32() -> f32 {
        [ConstI64::new(0, 42), Return::new()]
    }

    #[test]
    fn const_i64_and_return_f32() {
        let func = ConstI64AndReturnF32::new().unwrap();
        assert_eq!(42f32, func.call().unwrap());
    }

    #[wasm_code]
    fn const_i64_and_return_f64() -> f64 {
        [ConstI64::new(0, 42), Return::new()]
    }

    #[test]
    fn const_i64_and_return_f64() {
        let func = ConstI64AndReturnF64::new().unwrap();
        assert_eq!(42f64, func.call().unwrap());
    }

    #[wasm_code]
    fn const_f32_and_return_u32() -> u32 {
        [ConstF32::new(0, -1.0), Return::new()]
    }

    #[test]
    fn const_f32_and_return_u32() {
        let func = ConstF32AndReturnU32::new().unwrap();
        // Code: unsigned math, -1 saturates to 0
        assert_eq!(0u32, func.call().unwrap());
    }

    #[wasm_code(signed)]
    fn const_f32_and_return_i32() -> i32 {
        [ConstF32::new(0, -1.0), Return::new()]
    }

    #[test]
    fn const_f32_and_return_i32() {
        let func = ConstF32AndReturnI32::new().unwrap();
        // Code: unsigned math, -1 saturates to -1
        assert_eq!(-1i32, func.call().unwrap());
    }

    #[wasm_code]
    fn const_f32_and_return_u64() -> u64 {
        [ConstF32::new(0, -1.0), Return::new()]
    }

    #[test]
    fn const_f32_and_return_u64() {
        let func = ConstF32AndReturnU64::new().unwrap();
        // Code: unsigned math, -1 saturates to 0
        assert_eq!(0u64, func.call().unwrap());
    }

    #[wasm_code(signed)]
    fn const_f32_and_return_i64() -> i64 {
        [ConstF32::new(0, -1.0), Return::new()]
    }

    #[test]
    fn const_f32_and_return_i64() {
        let func = ConstF32AndReturnI64::new().unwrap();
        // Code: unsigned math, -1 saturates to -1
        assert_eq!(-1i64, func.call().unwrap());
    }

    #[wasm_code]
    fn const_f32_and_return_f32() -> f32 {
        [ConstF32::new(0, -1.0), Return::new()]
    }

    #[test]
    fn const_f32_and_return_f32() {
        let func = ConstF32AndReturnF32::new().unwrap();
        assert_eq!(-1f32, func.call().unwrap());
    }

    #[wasm_code]
    fn const_f32_and_return_f64() -> f64 {
        [ConstF32::new(0, -1.0), Return::new()]
    }

    #[test]
    fn const_f32_and_return_f64() {
        let func = ConstF32AndReturnF64::new().unwrap();
        assert_eq!(-1f64, func.call().unwrap());
    }

    #[wasm_code]
    fn const_f64_and_return_u32() -> u32 {
        [ConstF64::new(0, -1.0), Return::new()]
    }

    #[test]
    fn const_f64_and_return_u32() {
        let func = ConstF64AndReturnU32::new().unwrap();
        // Code: unsigned math, -1 saturates to 0
        assert_eq!(0u32, func.call().unwrap());
    }

    #[wasm_code(signed)]
    fn const_f64_and_return_i32() -> i32 {
        [ConstF64::new(0, -1.0), Return::new()]
    }

    #[test]
    fn const_f64_and_return_i32() {
        let func = ConstF64AndReturnI32::new().unwrap();
        // Code: unsigned math, -1 saturates to -1
        assert_eq!(-1i32, func.call().unwrap());
    }

    #[wasm_code]
    fn const_f64_and_return_u64() -> u64 {
        [ConstF64::new(0, -1.0), Return::new()]
    }

    #[test]
    fn const_f64_and_return_u64() {
        let func = ConstF64AndReturnU64::new().unwrap();
        // Code: unsigned math, -1 saturates to 0
        assert_eq!(0u64, func.call().unwrap());
    }

    #[wasm_code(signed)]
    fn const_f64_and_return_i64() -> i64 {
        [ConstF64::new(0, -1.0), Return::new()]
    }

    #[test]
    fn const_f64_and_return_i64() {
        let func = ConstF64AndReturnI64::new().unwrap();
        // Code: unsigned math, -1 saturates to -1
        assert_eq!(-1i64, func.call().unwrap());
    }

    #[wasm_code]
    fn const_f64_and_return_f32() -> f32 {
        [ConstF64::new(0, -1.0), Return::new()]
    }

    #[test]
    fn const_f64_and_return_f32() {
        let func = ConstF64AndReturnF32::new().unwrap();
        assert_eq!(-1f32, func.call().unwrap());
    }

    #[wasm_code]
    fn const_f64_and_return_f64() -> f64 {
        [ConstF64::new(0, -1.0), Return::new()]
    }

    #[test]
    fn const_f64_and_return_f64() {
        let func = ConstF64AndReturnF64::new().unwrap();
        assert_eq!(-1f64, func.call().unwrap());
    }
}
