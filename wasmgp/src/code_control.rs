use crate::code_builder::CodeBuilder;
use crate::convert::{GetSlotConvert, SetSlotConvert};
use crate::*;
use anyhow::Result;
use wasm_ast::{BlockType, ControlInstruction, Expression, Instruction, NumericInstruction, VariableInstruction};

/// Returns from a function. There are work variables of the appropriate types set aside to hold the return values.
/// The function should set the values of those slots prior to calling Return, however they are always initialized
/// to zero at the top of the function.
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

/// Call(function_index, parameter_slots, return_slots): Calls the host or code function with the specified index
/// (remainder the number of functions) and uses the specified list of work variables as parameters. If more work
/// variables are specified than are needed, they will be ignored. If more work variables are needed than are
/// supplied, the works 0..x will be used until all parameters are satisfied. The returns values from the function
/// will be placed into the work variables specified by 'return_slots'.
pub struct Call {
    function_index: u32,
    params: Vec<Slot>,
    results: Vec<Slot>,
}

impl Call {
    pub fn new(function_index: u32, params: Vec<Slot>, results: Vec<Slot>) -> Code {
        Code::Call(Call {
            function_index,
            params,
            results,
        })
    }
}

impl CodeBuilder for Call {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        unimplemented!();
        Ok(())
    }
}

/// If(compare_slot, do): If the value in the compare_slot is not zero, than the code listed in 'do' will execute.
pub struct If {
    if_not_zero: Slot,
    do_this: Vec<Code>,
}

impl If {
    pub fn new(if_not_zero: Slot, do_this: Vec<Code>) -> Code {
        Code::If(If { if_not_zero, do_this })
    }
}

impl CodeBuilder for If {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        unimplemented!();
        Ok(())
    }
}

/// IfElse(compare_slot, do, else_do): If the value in the compare_slot is not zero, than the code listed in 'do'
/// will execute. Otherwise, the code listed in 'else_do' will execute.
pub struct IfElse {
    if_not_zero: Slot,
    do_this: Vec<Code>,
    else_do_this: Vec<Code>,
}

impl IfElse {
    pub fn new(if_not_zero: Slot, do_this: Vec<Code>, else_do_this: Vec<Code>) -> Code {
        Code::IfElse(IfElse {
            if_not_zero,
            do_this,
            else_do_this,
        })
    }
}

impl CodeBuilder for IfElse {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        unimplemented!();
        Ok(())
    }
}

/// DoUntil(compare_slot, do): Will execute the code listed in 'do' until the value in the compare_slot is not zero.
/// This will execute the 'do' block at least once.
pub struct DoUntil {
    until_not_zero: Slot,
    do_this: Vec<Code>,
}

impl DoUntil {
    pub fn new(until_not_zero: Slot, do_this: Vec<Code>) -> Code {
        Code::DoUntil(DoUntil {
            until_not_zero,
            do_this,
        })
    }
}

impl CodeBuilder for DoUntil {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        unimplemented!();
        Ok(())
    }
}

/// DoWhile(compare_slot, do): Will execute the code listed in 'do' while the value in the compare_slot is not zero.
/// This will check the compare value before executing the 'do' code and so 'do' might never run.
pub struct DoWhile {
    while_not_zero: Slot,
    do_this: Vec<Code>,
}

impl DoWhile {
    pub fn new(while_not_zero: Slot, do_this: Vec<Code>) -> Code {
        Code::DoWhile(DoWhile {
            while_not_zero,
            do_this,
        })
    }
}

impl CodeBuilder for DoWhile {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        unimplemented!();
        Ok(())
    }
}

/// DoFor(times, do): Runs the code listed in 'do' a specific number of times chosen by the genetic algorithm (at
/// code compile-time, not while the VM is running). Max of 65_535 loops
pub struct DoFor {
    do_this: Vec<Code>,
    times: u16,
}

impl DoFor {
    pub fn new(do_this: Vec<Code>, times: u16) -> Code {
        Code::DoFor(DoFor { do_this, times })
    }
}

impl CodeBuilder for DoFor {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        // Set a new local with the number of loops remaining (might be zero already)
        let local_index = context.get_unused_local(ValueType::I32);
        instruction_list.push(NumericInstruction::I32Constant(self.times as i32).into());
        instruction_list.push(VariableInstruction::LocalSet(*local_index).into());

        // Create the code for the innermost loop. A branch of '0' will bring us to the top of this loop and a
        // branch of '1' will bring us to the end of the block surrounding the loop
        let mut inner_instructions: Vec<Instruction> = vec![];

        // Branch to the end of the outer block if the remaining loop count is zero
        // br_if 1 (i32.eqz (get_local $x) )
        inner_instructions.push(VariableInstruction::LocalGet(*local_index).into());
        inner_instructions.push(NumericInstruction::EqualToZero(ValueType::I32.into()).into());
        inner_instructions.push(ControlInstruction::BranchIf(1).into());

        // 'Do' the code. When the `loop_label` is dropped, it indicates we can't break from that loop anymore
        {
            let loop_label = context.entering_loop(1);
            self.do_this.append_code(context, &mut inner_instructions)?;
            drop(loop_label);
        }

        // Subtract one from the remaining loop count
        // (set_local $x (sub (get_local $x) (i32.const 1) ) )
        inner_instructions.push(VariableInstruction::LocalGet(*local_index).into());
        inner_instructions.push(NumericInstruction::I32Constant(1).into());
        inner_instructions.push(NumericInstruction::Subtract(ValueType::I32.into()).into());
        inner_instructions.push(VariableInstruction::LocalSet(*local_index).into());

        // Branch to the loop top (which will immediately check for zero loops remaining)
        inner_instructions.push(ControlInstruction::Branch(0).into());

        // Create a `loop` as the target or our 'keep going' jump. The loop does not enter or exit with any new
        // stack values
        let loop_expression = Expression::new(vec![ControlInstruction::Loop(
            BlockType::None,
            Expression::new(inner_instructions),
        )
        .into()]);

        // Create a `block` as the target of our 'exit' jump. The block does not enter or exit with any new
        // stack values
        instruction_list.push(ControlInstruction::Block(BlockType::None, loop_expression).into());
        Ok(())
    }
}

/// Break: If the code is currently in the middle of a 'do' loop, exits the loop unconditionally. If the code is not
/// in a loop, this is a null-op.
pub struct Break {}

impl Break {
    pub fn new() -> Code {
        Code::Break(Break {})
    }
}

impl CodeBuilder for Break {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        unimplemented!();
        Ok(())
    }
}

/// BreakIf(compare_slot) If the code is currently in the middle of a 'do' loop, exits the loop if the value in the
/// compare_slot is not zero. If the code is not in a loop, this is a null-op.
pub struct BreakIf {
    break_if_not_zero: Slot,
}

impl BreakIf {
    pub fn new(break_if_not_zero: Slot) -> Code {
        Code::BreakIf(BreakIf { break_if_not_zero })
    }
}

impl CodeBuilder for BreakIf {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        unimplemented!();
        Ok(())
    }
}
