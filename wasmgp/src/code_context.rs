use crate::ValueType;
use std::{cell::RefCell, ops::Deref};
use wasm_ast::LocalIndex;

pub struct CodeContext {
    locals: RefCell<Vec<LocalSlot>>,
}

impl CodeContext {
    pub fn new() -> CodeContext {
        CodeContext {
            locals: RefCell::new(vec![]),
        }
    }

    pub fn get_unused_local(&self, value_type: ValueType) -> DroppableLocalIndex {
        let mut locals = self.locals.borrow_mut();
        let position = if let Some(position) = locals
            .iter()
            .position(|slot| !slot.is_used && slot.value_type == value_type)
        {
            position
        } else {
            let position = locals.len();
            locals.push(LocalSlot {
                value_type: value_type,
                is_used: true,
            });

            position
        };

        DroppableLocalIndex {
            context: self,
            index: position as LocalIndex,
        }
    }

    fn mark_unused(&self, position: LocalIndex) {
        let mut locals = self.locals.borrow_mut();
        assert!((position as usize) < locals.len());
        locals[position as usize].is_used = false;
    }
}

pub struct DroppableLocalIndex<'a> {
    context: &'a CodeContext,
    index: LocalIndex,
}

impl<'a> Deref for DroppableLocalIndex<'a> {
    type Target = LocalIndex;
    fn deref(&self) -> &Self::Target {
        &self.index
    }
}

impl<'a> Drop for DroppableLocalIndex<'a> {
    fn drop(&mut self) {
        self.context.mark_unused(self.index);
    }
}

struct LocalSlot {
    value_type: ValueType,
    is_used: bool,
}
