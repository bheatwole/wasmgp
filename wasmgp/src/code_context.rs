use crate::{slot, WasmgpError};
use crate::{code_builder::CodeBuilder, Code, FunctionSignature, Slot, SlotCount, ValueType};
use anyhow::{bail, Result};
use std::{cell::RefCell, ops::Deref};
use wasm_ast::{Export, Function, FunctionType, LabelIndex, LocalIndex, ModuleBuilder, ResultType, SignExtension};

pub struct CodeContext {
    signature: FunctionSignature,
    is_signed: bool,
    locals: RefCell<Vec<SlotInfo>>,

    // A stack of the looping constructs that have been entered. If at least one loop is involved, then the 'Break'
    // instructions are valid and will produce code. The LabelIndex on the stack is how far we need to branch to exit
    // the loop.
    break_stack: RefCell<Vec<LabelIndex>>,
}

impl CodeContext {
    /// Creates the context that will be used to generate the specific Wasm instructions for some genetic Code. The
    /// signature is the function that is called to run the code. The slots indicate the working variables used by the
    /// various instruction to simplify stack management. All integers will be intrepreted as signed or unsigned based
    /// on the value of `is_signed`. It is not current possible to mix signedness in an algorithm.
    ///
    /// The total number of slots used across all parameters, return and locals must be 256 or fewer.
    pub fn new(signature: &FunctionSignature, slots: SlotCount, is_signed: bool) -> Result<CodeContext> {
        let slot_count = signature.params().len() + signature.results().len() + slots.len();
        if slot_count > 256 {
            return Err(WasmgpError::SlotCountTooLarge(slot_count).into());
        }

        let mut locals = Vec::with_capacity(slot_count);
        for p in signature.params().iter() {
            locals.push(SlotInfo {
                index: locals.len() as u16,
                purpose: SlotPurpose::Parameter,
                value_type: *p,
                is_in_use: true,
            });
        }
        for r in signature.results().iter() {
            locals.push(SlotInfo {
                index: locals.len() as u16,
                purpose: SlotPurpose::Return,
                value_type: *r,
                is_in_use: true,
            });
        }
        for s in slots.iter() {
            locals.push(SlotInfo {
                index: locals.len() as u16,
                purpose: SlotPurpose::Local,
                value_type: s,
                is_in_use: true,
            });
        }

        Ok(CodeContext {
            signature: signature.clone(),
            is_signed,
            locals: RefCell::new(locals),
            break_stack: RefCell::new(vec![]),
        })
    }

    /// Adds a function to the specified builder. This adds three components to the WASM: a function type using the
    /// signature held by the context, the function body using the specified Code, and a function export using the name
    /// from the signature.
    pub fn build(&self, builder: &mut ModuleBuilder, code: &[Code]) -> Result<()> {
        // Add the function type
        let params = self.signature.params_ast();
        let results = self.signature.results_ast();
        let function_type = FunctionType::new(ResultType::from(params), ResultType::from(results));
        let function_type_index = builder.add_function_type(function_type)?;

        // Create the list of local variables
        let locals = ResultType::new(self.local_types());

        // Build the code
        let mut instruction_list = vec![];
        for c in code.iter() {
            c.append_code(&self, &mut instruction_list)?;
        }

        // Create the function
        let function = Function::new(function_type_index, locals, instruction_list.into());
        let function_index = builder.add_function(function)?;

        // Export it
        let export = Export::function(self.signature.name().clone().into(), function_index);
        builder.add_export(export);

        Ok(())
    }

    pub fn is_signed(&self) -> bool {
        self.is_signed
    }

    pub fn sign_extension(&self) -> SignExtension {
        if self.is_signed {
            SignExtension::Signed
        } else {
            SignExtension::Unsigned
        }
    }

    /// Returns a list of all the local variable types suitable for passing to wasm_ast::Function::new. Specifically,
    /// this list does NOT include the parameters as part of the list
    pub fn local_types(&self) -> Vec<wasm_ast::ValueType> {
        let locals = self.locals.borrow();
        locals
            .iter()
            .filter(|i| i.purpose != SlotPurpose::Parameter)
            .map(|i| i.value_type.into())
            .collect()
    }

    /// Returns a tuple of (SlotType, SlotBytes, is_initialized) for a slot that the code generator would like to use.
    /// If the slot was not initialized, false will be returned, but the slot will be marked as initialized for future
    /// calls to `get_slot_for_use`.
    ///
    /// Returns `None` if the slot is out of range of all slots, or has `SlotPurpose::Instruction`
    pub fn get_slot_for_use(&self, slot: Slot) -> Option<ValueType> {
        let locals = self.locals.borrow();
        if let Some(slot_info) = locals.get(slot as usize) {
            if SlotPurpose::Instruction == slot_info.purpose {
                None
            } else {
                Some(slot_info.value_type)
            }
        } else {
            None
        }
    }

    /// Returns a list of all the slot indices that are `SlotPurpose::Return`
    pub fn return_slots(&self) -> Vec<Slot> {
        let locals = self.locals.borrow();
        locals
            .iter()
            .filter_map(|slot_info| {
                if SlotPurpose::Return == slot_info.purpose {
                    Some(slot_info.index as Slot)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Gets the next local variable index of the specified type that isn't already in use. If there is not currently
    /// a local of that type, a new one will be added. When the return value is dropped, that index is marked as unused
    /// and could be re-used by other code.
    pub fn get_unused_local(&self, value_type: ValueType) -> DroppableLocalIndex {
        let mut locals = self.locals.borrow_mut();
        let position = if let Some(position) = locals.iter().position(|slot| {
            slot.purpose == SlotPurpose::Instruction && !slot.is_in_use && slot.value_type == value_type
        }) {
            locals[position].is_in_use = true;
            position
        } else {
            let position = locals.len();
            locals.push(SlotInfo {
                index: position as u16,
                purpose: SlotPurpose::Instruction,
                value_type: value_type,
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

    // If the `purpose` is `Instruction`, the slot is used for a while and then is available for another instruction to
    // use it. Always `true` for all other purpose types.
    is_in_use: bool,
}

#[cfg(test)]
mod tests {
    use crate::{FunctionSignature, ValueType};

    use super::CodeContext;

    #[test]
    fn get_local_types() {
        let fs = FunctionSignature::new("test", vec![ValueType::I32, ValueType::F64], vec![ValueType::F32]);
        let slots = crate::SlotCount {
            i32: 0,
            i64: 3,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();

        // The local types should NOT include parameters (these locals are created by the definition for the function),
        // but start with the return types, and then include each of the slots
        let locals = context.local_types();
        assert_eq!(4, locals.len());
        assert_eq!(wasm_ast::ValueType::F32, locals[0]);
        assert_eq!(wasm_ast::ValueType::I64, locals[1]);
        assert_eq!(wasm_ast::ValueType::I64, locals[2]);
        assert_eq!(wasm_ast::ValueType::I64, locals[3]);
    }

    #[test]
    fn get_slot_for_use() {
        let fs = FunctionSignature::new("test", vec![ValueType::I32, ValueType::F64], vec![ValueType::F32]);
        let slots = crate::SlotCount {
            i32: 0,
            i64: 3,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();

        // Getting a parameter slot returns an initialized type
        assert_eq!(Some(ValueType::I32), context.get_slot_for_use(0));
        assert_eq!(Some(ValueType::F64), context.get_slot_for_use(1));

        // Getting a return slot, returns the type.
        assert_eq!(Some(ValueType::F32), context.get_slot_for_use(2));

        // Getting a local slot, returns the type.
        assert_eq!(Some(ValueType::I64), context.get_slot_for_use(5));

        // If you make an Instruction slot and try to get it, it returns None
        assert_eq!(6, *(context.get_unused_local(ValueType::I32)));
        assert_eq!(None, context.get_slot_for_use(6));

        // Getting a slot that doesn't exist also returns None
        assert_eq!(None, context.get_slot_for_use(200));
    }

    #[test]
    fn get_unused_local() {
        let fs = FunctionSignature::new("test", vec![], vec![]);
        let slots = crate::SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();

        // Getting an unused local slot from an empty context returns the first slot
        let slot_zero = context.get_unused_local(ValueType::I32);
        assert_eq!(0, *slot_zero);

        // If we drop the slot, we can get it again
        drop(slot_zero);
        let slot_zero = context.get_unused_local(ValueType::I32);
        assert_eq!(0, *slot_zero);

        // If we don't drop the slot, we get the next slot when we ask for the same type
        let slot_one = context.get_unused_local(ValueType::I32);
        assert_eq!(0, *slot_zero);
        assert_eq!(1, *slot_one);

        // Even when we drop both, we'll get the next slot (2) when we ask for a different type
        drop(slot_zero);
        drop(slot_one);
        let slot_two = context.get_unused_local(ValueType::I64);
        assert_eq!(2, *slot_two);

        // All three of the locals we created should show up when we ask for the locals that the function needs to have
        let locals = context.local_types();
        assert_eq!(3, locals.len());
        assert_eq!(wasm_ast::ValueType::I32, locals[0]);
        assert_eq!(wasm_ast::ValueType::I32, locals[1]);
        assert_eq!(wasm_ast::ValueType::I64, locals[2]);
    }

    #[test]
    fn break_stack() {
        let fs = FunctionSignature::new("test", vec![], vec![]);
        let slots = crate::SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();

        // Calling 'can_break' with no loop returns None
        assert_eq!(None, context.can_break());

        // Entering a loop with a distance of 2, returns that distance from 'can_break'
        let outer = context.entering_loop(2);
        assert_eq!(Some(2), context.can_break());

        // 'can_break' may be called again
        assert_eq!(Some(2), context.can_break());

        // Entering a second loop with a distance of 1, returns that distance from 'can_break'
        let inner = context.entering_loop(1);
        assert_eq!(Some(1), context.can_break());

        // After the inner loop is dropped, the answer is '2' again
        drop(inner);
        assert_eq!(Some(2), context.can_break());

        // After the outer loop is dropped, the answer is None again
        drop(outer);
        assert_eq!(None, context.can_break());
    }
}
