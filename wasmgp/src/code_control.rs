use crate::code_builder::CodeBuilder;
use crate::convert::{GetSlotConvert, SetSlotConvert};
use crate::indentation::Indentation;
use crate::*;
use anyhow::Result;
use std::fmt::Write;
use wasm_ast::{
    BlockType, ControlInstruction, Expression, FunctionIndex, Instruction, NumericInstruction, VariableInstruction,
};

/// Copies the value from one slot to another. The type will be converted if necessary
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code]
/// fn copy_slot_i32(value: i32) -> i32 {
///     [ CopySlot::new(0, 1), Return::new(), ]
/// }
/// let func = CopySlotI32::new().unwrap();
/// assert_eq!(1, func.call(1).unwrap());
/// assert_eq!(-2, func.call(-2).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn copy_slot_f32(value: i32) -> f32 {
///     [ CopySlot::new(0, 1), Return::new(), ]
/// }
/// let func = CopySlotF32::new().unwrap();
/// assert_eq!(1.0, func.call(1).unwrap());
/// assert_eq!(-2.0, func.call(-2).unwrap());
/// ```
pub struct CopySlot {
    source: Slot,
    destination: Slot,
}

impl CopySlot {
    pub fn new(source: Slot, destination: Slot) -> Code {
        Code::CopySlot(CopySlot { source, destination })
    }
}

impl CodeBuilder for CopySlot {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = context.get_slot_value_type(self.source)?;
        GetSlotConvert::convert(self.source, operate_as, context, instruction_list)?;
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}CopySlot::new({}, {}),",
            indentation, self.source, self.destination
        )
    }
}

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

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(f, "{}Return::new(),", indentation)
    }
}

/// Call(function_index, parameter_slots, return_slots): Calls the host or code function with the specified index
/// (remainder the number of functions) and uses the specified list of work variables as parameters. If more work
/// variables are specified than are needed, they will be ignored. If more work variables are needed than are
/// supplied, the works 0..x will be used until all parameters are satisfied. The returns values from the function
/// will be placed into the work variables specified by 'return_slots'.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// fn double(v: i32) -> i32 {
///     v * 2
/// }
///
/// #[wasm_code]
/// fn add_then_double(v1: i32, v2: i32) -> i32 {
///     [ Add::new(0, 1, 2), Call::new(0, vec![2], vec![2]), Return::new(), ]
/// }
///
/// let mut config = WorldConfiguration::default();
/// config.main_entry_point = FunctionSignature::new("add_then_double", vec![ValueType::I32, ValueType::I32], vec![ValueType::I32]);
/// let mut world = World::new(config);
/// let index = world.add_function_import("double", double).unwrap();
/// assert_eq!(0, index);
///
/// let func = AddThenDouble::new_with_world(&world).unwrap();
/// assert_eq!(6, func.call(1, 2).unwrap());
/// assert_eq!(-6, func.call(5, -8).unwrap());
/// ```
pub struct Call {
    function_index: FunctionIndex,
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
    fn append_code(&self, _context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        // Load each parameter slot onto the stack
        for &slot in self.params.iter() {
            instruction_list.push(VariableInstruction::LocalGet(slot as u32).into());
        }

        // Call the host function
        instruction_list.push(ControlInstruction::Call(self.function_index).into());

        // Put the results in the slot where they go (the top of the stack is the last result returned, so we need to
        // process our slots in reverse)
        for &slot in self.results.iter().rev() {
            instruction_list.push(VariableInstruction::LocalSet(slot as u32).into());
        }

        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}Call::new({}, vec![{}], vec![{}]),",
            indentation,
            self.function_index,
            self.params
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.results
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

/// If the value in the compare_slot is not zero, than the code listed in 'do' will execute.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(unsigned, 2)]
/// fn double_odds(value: u32) -> u32 {
///     [
///         ConstI32::new(2, 0),
///         ConstI32::new(3, 2),
///         Add::new(0, 2, 1),
///         Remainder::new(0, 3, 3),
///         If::new(3, vec![Add::new(0, 0, 1)]),
///         Return::new(),
///     ]
/// }
/// let func = DoubleOdds::new().unwrap();
/// assert_eq!(2, func.call(1).unwrap());
/// assert_eq!(2, func.call(2).unwrap());
/// assert_eq!(6, func.call(3).unwrap());
/// assert_eq!(4, func.call(4).unwrap());
/// ```
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
        let mut inner_instructions: Vec<Instruction> = vec![];
        self.do_this.append_code(context, &mut inner_instructions)?;

        GetSlotConvert::convert(self.if_not_zero, ValueType::I32, context, instruction_list)?;
        instruction_list
            .push(ControlInstruction::If(BlockType::None, Expression::new(inner_instructions), None).into());
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        write!(f, "{}If::new({}, vec!", indentation, self.if_not_zero)?;
        self.do_this.print_for_rust(f, indentation)?;
        writeln!(f, "),")
    }
}

/// If the value in the compare_slot is not zero, than the code listed in `do_this` will execute. Otherwise, the code
/// listed in 'else_do_this' will execute.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(unsigned, 1)]
/// fn double_odds_triple_evens(value: u32) -> u32 {
///     [
///         ConstI32::new(2, 2),
///         Remainder::new(0, 2, 2),
///         IfElse::new(2, vec![Add::new(0, 0, 1)], vec![Add::new(0, 0, 1), Add::new(0, 1, 1)]),
///         Return::new(),
///     ]
/// }
/// let func = DoubleOddsTripleEvens::new().unwrap();
/// assert_eq!(2, func.call(1).unwrap());
/// assert_eq!(6, func.call(2).unwrap());
/// assert_eq!(6, func.call(3).unwrap());
/// assert_eq!(12, func.call(4).unwrap());
/// ```
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
        let mut if_instructions: Vec<Instruction> = vec![];
        self.do_this.append_code(context, &mut if_instructions)?;
        let mut else_instructions: Vec<Instruction> = vec![];
        self.else_do_this.append_code(context, &mut else_instructions)?;

        GetSlotConvert::convert(self.if_not_zero, ValueType::I32, context, instruction_list)?;
        instruction_list.push(
            ControlInstruction::If(
                BlockType::None,
                Expression::new(if_instructions),
                Some(Expression::new(else_instructions)),
            )
            .into(),
        );
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        write!(f, "{}IfElse::new({}, vec!", indentation, self.if_not_zero)?;
        self.do_this.print_for_rust(f, indentation)?;
        write!(f, ", vec!")?;
        self.else_do_this.print_for_rust(f, indentation)?;
        writeln!(f, "),")
    }
}

/// DoUntil will execute the code listed in `do_this` until the value in the compare_slot is not zero. This will execute
/// the `do_this` block at least once.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(unsigned, 4)]
/// fn make_multiple_of_three(value: u32) -> u32 {
///     [
///         ConstI32::new(2, 1),
///         ConstI32::new(3, 3),
///         ConstI32::new(4, 0),
///         CopySlot::new(0, 1),
///         Remainder::new(1, 3, 5),
///         AreEqual::new(5, 4, 5),
///         DoUntil::new(5, vec![
///             Add::new(1, 2, 1),
///             Remainder::new(1, 3, 5),
///             AreEqual::new(5, 4, 5),
///         ]),
///         Return::new(),
///     ]
/// }
/// let func = MakeMultipleOfThree::new().unwrap();
/// assert_eq!(3, func.call(1).unwrap());
/// assert_eq!(3, func.call(2).unwrap());
/// // Because the 'do' loop runs at least one, we get the next multiple
/// assert_eq!(6, func.call(3).unwrap());
/// ```
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
        // Create the code for the innermost loop. A branch of '0' will bring us to the top of this loop and a
        // branch of '1' will bring us to the end of the block surrounding the loop
        let mut inner_instructions: Vec<Instruction> = vec![];

        // 'Do' the code. When the `loop_label` is dropped, it indicates we can't break from that loop anymore
        {
            let loop_label = context.entering_loop(1);
            self.do_this.append_code(context, &mut inner_instructions)?;
            drop(loop_label);
        }

        // Branch to the end of the outer block if the condition is not zero
        // br_if 1 (i32.ne 0 (get_local $x) )
        GetSlotConvert::convert(self.until_not_zero, ValueType::I32, context, &mut inner_instructions)?;
        inner_instructions.push(NumericInstruction::I32Constant(0).into());
        inner_instructions.push(NumericInstruction::NotEqual(ValueType::I32.into()).into());
        inner_instructions.push(ControlInstruction::BranchIf(1).into());

        // If our condition did not get hit, branch to the loop top
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

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        write!(f, "{}DoUntil::new({}, vec!", indentation, self.until_not_zero)?;
        self.do_this.print_for_rust(f, indentation)?;
        writeln!(f, "),")
    }
}

/// DoWhile(compare_slot, do): Will execute the code listed in 'do' while the value in the compare_slot is not zero.
/// This will check the compare value before executing the 'do' code and so 'do' might never run.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(unsigned, 4)]
/// fn make_multiple_of_three(value: u32) -> u32 {
///     [
///         ConstI32::new(2, 1),
///         ConstI32::new(3, 3),
///         ConstI32::new(4, 0),
///         CopySlot::new(0, 1),
///         Remainder::new(1, 3, 5),
///         AreEqual::new(5, 4, 5),
///         DoWhile::new(5, vec![
///             Add::new(1, 2, 1),
///             Remainder::new(1, 3, 5),
///             AreEqual::new(5, 4, 5),
///         ]),
///         Return::new(),
///     ]
/// }
/// let func = MakeMultipleOfThree::new().unwrap();
/// assert_eq!(3, func.call(1).unwrap());
/// assert_eq!(3, func.call(2).unwrap());
/// // Because the 'do' loop checks the condition first, we exit before adding any
/// assert_eq!(3, func.call(3).unwrap());
/// assert_eq!(6, func.call(4).unwrap());
/// ```
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
        // Create the code for the innermost loop. A branch of '0' will bring us to the top of this loop and a
        // branch of '1' will bring us to the end of the block surrounding the loop
        let mut inner_instructions: Vec<Instruction> = vec![];

        // Branch to the end of the outer block if the condition is not zero
        // br_if 1 (i32.ne 0 (get_local $x) )
        GetSlotConvert::convert(self.while_not_zero, ValueType::I32, context, &mut inner_instructions)?;
        inner_instructions.push(NumericInstruction::I32Constant(0).into());
        inner_instructions.push(NumericInstruction::NotEqual(ValueType::I32.into()).into());
        inner_instructions.push(ControlInstruction::BranchIf(1).into());

        // 'Do' the code. When the `loop_label` is dropped, it indicates we can't break from that loop anymore
        {
            let loop_label = context.entering_loop(1);
            self.do_this.append_code(context, &mut inner_instructions)?;
            drop(loop_label);
        }

        // If our condition did not get hit, branch to the loop top
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

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        write!(f, "{}DoWhile::new({}, vec!", indentation, self.while_not_zero)?;
        self.do_this.print_for_rust(f, indentation)?;
        writeln!(f, "),")
    }
}

/// DoFor(times, do): Runs the code listed in 'do' a specific number of times chosen by the genetic algorithm (at
/// code compile-time, not while the VM is running). Max of 65_535 loops
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code]
/// fn triple(value: u32) -> u32 {
///     [
///         DoFor::new(3, vec![
///             Add::new(0, 1, 1),
///         ]),
///         Return::new(),
///     ]
/// }
/// let func = Triple::new().unwrap();
/// assert_eq!(3, func.call(1).unwrap());
/// assert_eq!(6, func.call(2).unwrap());
/// assert_eq!(9, func.call(3).unwrap());
/// assert_eq!(0, func.call(0).unwrap());
/// ```
pub struct DoFor {
    do_this: Vec<Code>,
    times: u16,
}

impl DoFor {
    pub fn new(times: u16, do_this: Vec<Code>) -> Code {
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

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        write!(f, "{}DoFor::new({}, vec!", indentation, self.times)?;
        self.do_this.print_for_rust(f, indentation)?;
        writeln!(f, "),")
    }
}

/// Break: If the code is currently in the middle of a 'do' loop, exits the loop unconditionally. If the code is not
/// in a loop, this is a null-op.
///
/// A complicated way of returning the same value:
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code]
/// fn identity(value: u32) -> u32 {
///     [
///         DoFor::new(3, vec![
///             Add::new(0, 1, 1),
///             Break::new(),
///         ]),
///         Return::new(),
///     ]
/// }
/// let func = Identity::new().unwrap();
/// assert_eq!(1, func.call(1).unwrap());
/// assert_eq!(2, func.call(2).unwrap());
/// assert_eq!(3, func.call(3).unwrap());
/// assert_eq!(0, func.call(0).unwrap());
/// ```
///
/// Breaking while not in a loop does nothing:
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code]
/// fn double(value: u32) -> u32 {
///     [
///         Add::new(0, 1, 1),
///         Break::new(),
///         Add::new(0, 1, 1),
///         Return::new(),
///     ]
/// }
/// let func = Double::new().unwrap();
/// assert_eq!(2, func.call(1).unwrap());
/// assert_eq!(4, func.call(2).unwrap());
/// assert_eq!(6, func.call(3).unwrap());
/// assert_eq!(0, func.call(0).unwrap());
/// ```
pub struct Break {}

impl Break {
    pub fn new() -> Code {
        Code::Break(Break {})
    }
}

impl CodeBuilder for Break {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        if let Some(label_index) = context.can_break() {
            instruction_list.push(ControlInstruction::Branch(label_index).into());
        }
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(f, "{}Break::new(),", indentation)
    }
}

/// BreakIf(compare_slot) If the code is currently in the middle of a 'do' loop, exits the loop if the value in the
/// compare_slot is not zero. If the code is not in a loop, this is a null-op.
///
/// This code triples a number, but stops before the max value is reached
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(unsigned, 1)]
/// fn triples_up_to_max(value: u32, max: u32) -> u32 {
///     [
///         DoFor::new(3, vec![
///             Add::new(0, 2, 2),
///             IsGreaterThan::new(2, 1, 3),
///             BreakIf::new(3),
///         ]),
///         Return::new(),
///     ]
/// }
/// let func = TriplesUpToMax::new().unwrap();
/// assert_eq!(3, func.call(1, 5).unwrap());
/// assert_eq!(6, func.call(2, 5).unwrap());
/// assert_eq!(6, func.call(3, 5).unwrap());
/// ```
///
/// Breaking while not in a loop does nothing:
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(unsigned, 1)]
/// fn another_triples_up_to_max(value: u32, max: u32) -> u32 {
///     [
///         Add::new(0, 2, 2),
///         IsGreaterThan::new(2, 1, 3),
///         BreakIf::new(3),
///         Add::new(0, 2, 2),
///         IsGreaterThan::new(2, 1, 3),
///         BreakIf::new(3),
///         Add::new(0, 2, 2),
///         Return::new(),
///     ]
/// }
/// let func = AnotherTriplesUpToMax::new().unwrap();
/// assert_eq!(3, func.call(1, 5).unwrap());
/// assert_eq!(6, func.call(2, 5).unwrap());
/// assert_eq!(9, func.call(3, 5).unwrap());
/// ```
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
        if let Some(label_index) = context.can_break() {
            GetSlotConvert::convert(self.break_if_not_zero, ValueType::I32, context, instruction_list)?;
            instruction_list.push(ControlInstruction::BranchIf(label_index).into());
        }
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(f, "{}BreakIf::new({}),", indentation, self.break_if_not_zero)
    }
}

#[cfg(test)]
mod tests {
    use wasmgp_macros::wasm_code;

    use crate::*;

    #[wasm_code]
    fn test_call_order(v1: i32, v2: i32) -> (i32, i32, i32) {
        [Call::new(0, vec![0, 1], vec![2, 3, 4]), Return::new()]
    }

    // This test does not belong as a doc-test because it does not increase the readers understanding of the behavior.
    // It simply confirms that the code has the order of parameters and results as expected.
    #[test]
    fn test_call_order() {
        let mut config = WorldConfiguration::default();
        config.main_entry_point = FunctionSignature::new(
            "test_call_order",
            vec![ValueType::I32, ValueType::I32],
            vec![ValueType::I32, ValueType::I32, ValueType::I32],
        );
        let mut world = World::new(config);
        let index = world
            .add_function_import("do_it", |v1: i32, v2: i32| (v2, v1, v1 - v2))
            .unwrap();
        assert_eq!(0, index);
        let func = TestCallOrder::new_with_world(&world).unwrap();
        assert_eq!((3, 1, -2), func.call(1, 3).unwrap());
    }
}
