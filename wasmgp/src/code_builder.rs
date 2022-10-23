use crate::{code_context::CodeContext, Code};
use wasm_ast::Instruction;

pub trait CodeBuilder {
    /// Implementor will append one or more instructions onto the list
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>);
}

impl CodeBuilder for Vec<Code> {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) {
        for code in self.iter() {
            code.append_code(context, instruction_list);
        }
    }
}
