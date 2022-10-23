use crate::{Code, code_context::CodeContext};
use wasm_ast::Instruction;

pub trait CodeBuilder {
    /// Implementor will append one or more instructions onto the list
    /// TODO: code_context
    fn append_code(&self, context: &mut CodeContext, instruction_list: &mut Vec<Instruction>);
}

impl CodeBuilder for Vec<Code> {
    fn append_code(&self, context: &mut CodeContext, instruction_list: &mut Vec<Instruction>) {
        for code in self.iter() {
            code.append_code(context, instruction_list);
        }
    }
}
