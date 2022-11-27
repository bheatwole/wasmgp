use crate::code_builder::CodeBuilder;
use crate::convert::{GetSlotConvert, SetSlotConvert};
use crate::*;
use anyhow::Result;
use wasm_ast::{BlockType, ControlInstruction, Expression, Instruction, NumericInstruction, VariableInstruction};

pub struct Return {}

impl Return {
    pub fn new() -> Code {
        Code::Return(Return {})
    }
}

impl CodeBuilder for Return {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        for slot in context.return_slots().iter() {
            instruction_list.push(VariableInstruction::LocalGet(*slot as u32).into());
        }
        Ok(())
    }
}
