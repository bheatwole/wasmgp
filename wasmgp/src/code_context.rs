use crate::{ValueType};
use wasm_ast::LocalIndex;

pub struct CodeContext {
    locals: Vec<LocalSlot>,
}

impl CodeContext {
    pub fn new() -> CodeContext {
        CodeContext { locals: vec![] }
    }

    pub fn get_unused_local(&mut self, value_type: ValueType) -> LocalIndex {
        let position = if let Some(position) = self
            .locals
            .iter()
            .position(|slot| !slot.is_used && slot.value_type == value_type)
        {
            position
        } else {
            let position = self.locals.len();
            self.locals.push(LocalSlot {
                value_type: value_type,
                is_used: true,
            });

            position
        };

        position as LocalIndex
    }

    pub fn mark_unused(&mut self, position: LocalIndex) {
        assert!((position as usize) < self.locals.len());
        self.locals[position as usize].is_used = false;
    }
}

struct LocalSlot {
    value_type: ValueType,
    is_used: bool,
}
