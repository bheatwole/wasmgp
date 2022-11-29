use crate::indentation::Indentation;
use crate::{code_context::CodeContext, Code};
use anyhow::Result;
use wasm_ast::Instruction;

pub trait CodeBuilder {
    /// Implementor will append one or more instructions onto the list
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()>;

    /// Implementor should print the code in such a way as to be able to copy-paste to rust code files.
    fn print_for_rust(&self, _f: &mut std::string::String, _indentation: &mut Indentation) -> std::fmt::Result {
        Ok(())
    }
}

impl CodeBuilder for Vec<Code> {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        for code in self.iter() {
            code.append_code(context, instruction_list)?;
        }

        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        use std::fmt::Write;
        writeln!(f, "[")?;
        indentation.indent();
        for code in self.iter() {
            code.print_for_rust(f, indentation)?;
        }
        indentation.outdent();
        write!(f, "{}]", indentation)
    }
}
