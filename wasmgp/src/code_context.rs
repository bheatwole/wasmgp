use crate::{FunctionSignature, SlotCount, ValueType};
use std::{cell::RefCell, ops::Deref};
use wasm_ast::{LabelIndex, LocalIndex};

pub struct CodeContext {
    locals: RefCell<Vec<SlotInfo>>,

    // A stack of the looping constructs that have been entered. If at least one loop is involved, then the 'Break'
    // instructions are valid and will produce code. The LabelIndex on the stack is how far we need to branch to exit
    // the loop.
    break_stack: RefCell<Vec<LabelIndex>>,
}

impl CodeContext {
    pub fn new(signature: &FunctionSignature, slots: SlotCount) -> CodeContext {
        let slot_count = signature.params().len() + signature.results().len() + slots.len();
        assert!(slot_count <= 256);

        let mut locals = Vec::with_capacity(slot_count);
        for p in signature.params().iter() {
            locals.push(SlotInfo {
                index: locals.len() as u16,
                purpose: SlotPurpose::Parameter,
                value_type: *p,
                is_initialized: true,
                is_in_use: true,
            });
        }
        for r in signature.results().iter() {
            locals.push(SlotInfo {
                index: locals.len() as u16,
                purpose: SlotPurpose::Return,
                value_type: *r,
                is_initialized: false,
                is_in_use: true,
            });
        }
        for s in slots.iter() {
            locals.push(SlotInfo {
                index: locals.len() as u16,
                purpose: SlotPurpose::Local,
                value_type: s,
                is_initialized: false,
                is_in_use: true,
            });
        }

        CodeContext {
            locals: RefCell::new(locals),
            break_stack: RefCell::new(vec![]),
        }
    }

    /// Returns a list of all the local variable types suitable for passing to wasm_ast::Function::new. Specifically,
    /// this list does NOT include the parameters as part of the list
    pub fn local_types(&self) -> Vec<ValueType> {
        let locals = self.locals.borrow();
        locals.iter().filter(|i| i.purpose != SlotPurpose::Parameter).map(|i| i.value_type).collect()
    }

    /// Gets the next local variable index of the specified type that isn't already in use. If there is not currently
    /// a local of that type, a new one will be added. When the return value is dropped, that index is marked as unused
    /// and could be re-used by other code.
    pub fn get_unused_local(&self, value_type: ValueType) -> DroppableLocalIndex {
        let mut locals = self.locals.borrow_mut();
        let position = if let Some(position) = locals.iter().position(|slot| {
            slot.purpose == SlotPurpose::Instruction
                && !slot.is_in_use
                && slot.value_type == value_type
        }) {
            position
        } else {
            let position = locals.len();
            locals.push(SlotInfo {
                index: position as u16,
                purpose: SlotPurpose::Instruction,
                value_type: value_type,
                is_initialized: true,
                is_in_use: true,
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
        locals[position as usize].is_in_use = false;
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

#[derive(Eq, PartialEq)]
enum SlotPurpose {
    // This slot is set by the caller
    Parameter,

    // This slot is reserved to hold the values we will return
    Return,

    // The code may freely use this slot to perform calculations
    Local,

    // The implementation of an instruction required the use of this slot
    Instruction,
}

struct SlotInfo {
    // Sometimes the slot list is filtered, so it is useful to know the slot index. This is a u16 instead of a u8
    // because the sum of (Parameter, Return, Local) must be <= 256, but the use of Instruction slots may exceed a u8.
    index: u16,

    // Defines how this slot is used in the code.
    purpose: SlotPurpose,

    // Defines the value held in this slot
    value_type: ValueType,

    // The first time a slot it used, it is initialized to zero. This prevents random memory values from corrupting
    // runs of the algorithm.
    is_initialized: bool,

    // If the `purpose` is `Instruction`, the slot is used for a while and then is available for another instruction to
    // use it. Always `true` for all other purpose types.
    is_in_use: bool,
}
