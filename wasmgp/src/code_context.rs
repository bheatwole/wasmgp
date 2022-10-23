use crate::ValueType;
use std::{cell::RefCell, ops::Deref};
use wasm_ast::{LocalIndex, LabelIndex};

pub struct CodeContext {
    locals: RefCell<Vec<LocalSlot>>,

    // A stack of the looping constructs that have been entered. If at least one loop is involved, then the 'Break'
    // instructions are valid and will produce code. The LabelIndex on the stack is how far we need to branch to exit
    // the loop.
    break_stack: RefCell<Vec<LabelIndex>>,
}

impl CodeContext {
    pub fn new() -> CodeContext {
        CodeContext {
            locals: RefCell::new(vec![]),
            break_stack: RefCell::new(vec![]),
        }
    }

    /// Gets the next local variable index of the specified type that isn't already in use. If there is not currently
    /// a local of that type, a new one will be added. When the return value is dropped, that index is marked as unused
    /// and could be re-used by other code.
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

    /// Indicates that the code is entering a loop. The `branch_distance` is the LabelIndex needed to break out of the
    /// loop if desired. When the return value is dropped, it indicates that the loop is no longer active.
    pub fn entering_loop(&self, branch_distance: LabelIndex) -> DroppableBreakStackEntry {
        let mut break_stack = self.break_stack.borrow_mut();
        break_stack.push(branch_distance);

        DroppableBreakStackEntry { context: self }
    }

    /// Returns Some(branch_distance) if the context is currently inside a loop, where `branch_distance` is the
    /// LabelIndex needed in order to exit the loop. Returns None if the context is not inside a loop
    pub fn can_break(&self) -> Option<LabelIndex> {
        let break_stack = self.break_stack.borrow();
        break_stack.last().map(|i| *i)
    }

    fn pop_break_stack(&self) {
        let mut break_stack = self.break_stack.borrow_mut();
        assert!(break_stack.len() > 0);
        break_stack.pop();
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

pub struct DroppableBreakStackEntry<'a> {
    context: &'a CodeContext,
}

impl<'a> Drop for DroppableBreakStackEntry<'a> {
    fn drop(&mut self) {
        self.context.pop_break_stack();
    }
}

struct LocalSlot {
    value_type: ValueType,
    is_used: bool,
}
