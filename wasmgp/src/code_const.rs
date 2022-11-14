use anyhow::Result;
use wasm_ast::{Instruction, NumericInstruction};

use crate::code_builder::CodeBuilder;
use crate::convert::SetSlotConvert;
use crate::{Code, CodeContext, Slot, ValueType};

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
}

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
}

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
}

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
}
