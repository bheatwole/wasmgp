use crate::{
    code_builder::CodeBuilder, code_context::CodeContext, FloatSlot, IntegerSlot, Slot, ValueType,
};
use wasm_ast::{
    BlockType, ControlInstruction, Expression, Instruction, IntegerType, NumberType,
    NumericInstruction, VariableInstruction,
};

pub enum Code {
    /// ConstI32(slot, value): Loads the specified value into a four-byte integer into the specified global variable
    /// slot. It may be interpreted as signed or unsigned later. If the slot is for floating-point values, it will be
    /// cast to a float.
    ConstI32(IntegerSlot, i32),

    /// ConstI64(slot, value): Loads the specified value into a eight-byte integer into the specified global variable
    /// slot. It may be interpreted as signed or unsigned later. If the slot is for floating-point values, it will be
    /// cast to a float.
    ConstI64(IntegerSlot, i64),

    /// ConstF32(slot, value): Loads the specified value into a four-byte global variable slot. If the slot is for
    /// integer values, it will be truncated.
    ConstF32(FloatSlot, f32),

    /// ConstF64(slot, value): Loads the specified value into a eight-byte global variable slot. If the slot is for
    /// integer values, it will be truncated.
    ConstF64(FloatSlot, f64),

    /// CountLeadingZeros(source_slot, destination_slot): Counts the number of leading zero bits in the specified source
    /// slot and places it into the destination_slot.
    CountLeadingZeros(IntegerSlot, IntegerSlot),

    /// CountTrailingZeros(source_slot, destination_slot): Counts the number of trailing zero bits in the specified
    /// source slot and places it into the destination_slot.
    CountTrailingZeros(IntegerSlot, IntegerSlot),

    /// PopulationCount(source_slot, destination_slot): Counts the number of one bits in the specified source slot and
    /// places it into the destination_slot.
    PopulationCount(IntegerSlot, IntegerSlot),

    /// AddInteger(left_slot, right_slot, result_slot): Places the result of left + right in the result slot.
    AddInteger(IntegerSlot, IntegerSlot, IntegerSlot),

    /// AddFloat(left_slot, right_slot, result_slot): Places the result of left + right in the result slot.
    AddFloat(FloatSlot, FloatSlot, FloatSlot),

    /// SubtractInteger(left_slot, right_slot, result_slot): Places the result of left - right in the result slot.
    SubtractInteger(IntegerSlot, IntegerSlot, IntegerSlot),

    /// SubtractFloat(left_slot, right_slot, result_slot): Places the result of left - right in the result slot.
    SubtractFloat(FloatSlot, FloatSlot, FloatSlot),

    /// MultiplyInteger(left_slot, right_slot, result_slot): Places the result of left * right in the result slot.
    MultiplyInteger(IntegerSlot, IntegerSlot, IntegerSlot),

    /// MultiplyFloat(left_slot, right_slot, result_slot): Places the result of left * right in the result slot.
    MultiplyFloat(FloatSlot, FloatSlot, FloatSlot),

    /// DivideSigned(dividend_slot, divisor_slot, result_slot): Places the result of dividend / divisor in the result
    /// slot. The code will leave the result untouched if the divisor is zero.
    DivideSigned(IntegerSlot, IntegerSlot, IntegerSlot),

    /// DivideUnsigned(dividend_slot, divisor_slot, result_slot): Places the result of dividend / divisor in the result
    /// slot. The code will leave the result untouched if the divisor is zero.
    DivideUnsigned(IntegerSlot, IntegerSlot, IntegerSlot),

    /// DivideFloat(dividend_slot, divisor_slot, result_slot): Places the result of dividend / divisor in the result
    /// slot. The code will leave the result untouched if the divisor is zero.
    DivideFloat(FloatSlot, FloatSlot, FloatSlot),

    /// RemainderSigned(dividend_slot, divisor_slot, result_slot): Places the result of dividend % divisor in the result
    /// slot. The code will leave the result untouched if the divisor is zero.
    RemainderSigned(IntegerSlot, IntegerSlot, IntegerSlot),

    /// RemainderUnsigned(dividend_slot, divisor_slot, result_slot): Places the result of dividend % divisor in the
    /// result slot. The code will leave the result untouched if the divisor is zero.
    RemainderUnsigned(IntegerSlot, IntegerSlot, IntegerSlot),

    And(IntegerSlot, IntegerSlot, IntegerSlot),
    Or(IntegerSlot, IntegerSlot, IntegerSlot),
    Xor(IntegerSlot, IntegerSlot, IntegerSlot),
    ShiftLeft(IntegerSlot, IntegerSlot, IntegerSlot),
    ShiftRightSigned(IntegerSlot, IntegerSlot, IntegerSlot),
    ShiftRightUnsigned(IntegerSlot, IntegerSlot, IntegerSlot),
    RotateLeft(IntegerSlot, IntegerSlot, IntegerSlot),
    RotateRight(IntegerSlot, IntegerSlot, IntegerSlot),
    AbsoluteValue(FloatSlot, FloatSlot),
    Negate(FloatSlot, FloatSlot),
    SquareRoot(FloatSlot, FloatSlot),
    Ceiling(FloatSlot, FloatSlot),
    Floor(FloatSlot, FloatSlot),
    Nearest(FloatSlot, FloatSlot),
    Min(FloatSlot, FloatSlot, FloatSlot),
    Max(FloatSlot, FloatSlot, FloatSlot),
    CopySign(FloatSlot, FloatSlot, FloatSlot),
    IsEqualZero(IntegerSlot, IntegerSlot),
    AreEqualInteger(IntegerSlot, IntegerSlot, IntegerSlot),
    AreNotEqualInteger(IntegerSlot, IntegerSlot, IntegerSlot),
    IsLessThanSigned(IntegerSlot, IntegerSlot, IntegerSlot),
    IsLessThanUnsigned(IntegerSlot, IntegerSlot, IntegerSlot),
    IsGreaterThanSigned(IntegerSlot, IntegerSlot, IntegerSlot),
    IsGreaterThanUnsigned(IntegerSlot, IntegerSlot, IntegerSlot),
    IsLessThanOrEqualSigned(IntegerSlot, IntegerSlot, IntegerSlot),
    IsLessThanOrEqualUnsigned(IntegerSlot, IntegerSlot, IntegerSlot),
    IsGreaterThanOrEqualSigned(IntegerSlot, IntegerSlot, IntegerSlot),
    IsGreaterThanOrEqualUnsigned(IntegerSlot, IntegerSlot, IntegerSlot),
    AreEqualFloat(FloatSlot, FloatSlot, FloatSlot),
    AreNotEqualFloat(FloatSlot, FloatSlot, FloatSlot),
    IsLessThanFloat(FloatSlot, FloatSlot, FloatSlot),
    IsGreaterThanFloat(FloatSlot, FloatSlot, FloatSlot),
    IsLessThanOrEqualFloat(FloatSlot, FloatSlot, FloatSlot),
    IsGreaterThanOrEqualFloat(FloatSlot, FloatSlot, FloatSlot),

    /// LoadI8(offset_slot, result_slot): Loads the i8 value at the memory index indicated by the offset into the result
    /// slot. The memory index will be cast into an integer and the calculation `offset % mem_size` applied before
    /// attempting to read the memory. The i8 value will be cast into the result slot type.
    LoadI8(IntegerSlot, IntegerSlot),
    LoadU8(IntegerSlot, IntegerSlot),
    LoadI16(IntegerSlot, IntegerSlot),
    LoadU16(IntegerSlot, IntegerSlot),
    LoadI32(IntegerSlot, IntegerSlot),
    LoadU32(IntegerSlot, IntegerSlot),
    LoadI64(IntegerSlot, IntegerSlot),
    LoadU64(IntegerSlot, IntegerSlot),
    LoadF32(IntegerSlot, FloatSlot),
    LoadF64(IntegerSlot, FloatSlot),
    StoreI8(IntegerSlot, IntegerSlot),
    StoreU8(IntegerSlot, IntegerSlot),
    StoreI16(IntegerSlot, IntegerSlot),
    StoreU16(IntegerSlot, IntegerSlot),
    StoreI32(IntegerSlot, IntegerSlot),
    StoreU32(IntegerSlot, IntegerSlot),
    StoreI64(IntegerSlot, IntegerSlot),
    StoreU64(IntegerSlot, IntegerSlot),
    StoreF32(IntegerSlot, FloatSlot),
    StoreF64(IntegerSlot, FloatSlot),

    /// Returns from a function, using the specified slots as return values. If more global variable are
    /// specified than are needed, they will be ignored. If more global variable are needed than supplied, the code will
    /// use globals 0..x until all return values are satisfied.
    Return(Vec<Slot>),

    /// Call(function_index, parameter_slots, return_slots): Calls the host or code function with the specified index
    /// (remainder the number of functions) and uses the specified list of global variables as parameters. If more global
    /// variables are specified than are needed, they will be ignored. If more global variables are needed than are
    /// supplied, the globals 0..x will be used until all parameters are satisfied. The returns values from the function
    /// will be placed into the global variables specified by 'return_slots'.
    Call(u32, Vec<Slot>, Vec<Slot>),

    /// If(compare_slot, do): If the value in the compare_slot is not zero, than the code listed in 'do' will execute.
    If(IntegerSlot, Vec<Code>),

    /// IfElse(compare_slot, do, else_do): If the value in the compare_slot is not zero, than the code listed in 'do'
    /// will execute. Otherwise, the code listed in 'else_do' will execute.
    IfElse(IntegerSlot, Vec<Code>, Vec<Code>),

    /// DoUntil(compare_slot, do): Will execute the code listed in 'do' until the value in the compare_slot is not zero.
    /// This will execute the 'do' block at least once.
    DoUntil(IntegerSlot, Vec<Code>),

    /// DoWhile(compare_slot, do): Will execute the code listed in 'do' while the value in the compare_slot is not zero.
    /// This will check the compare value before executing the 'do' code and so 'do' might never run.
    DoWhile(IntegerSlot, Vec<Code>),

    /// DoFor(times, do): Runs the code listed in 'do' a specific number of times chosen by the genetic algorithm (at
    /// code compile-time, not while the VM is running). Max of 65_535 loops
    DoFor(u16, Vec<Code>),

    /// Break: If the code is currently in the middle of a 'do' loop, exits the loop unconditionally. If the code is not
    /// in a loop, this is a null-op.
    Break,

    /// BreakIf(compare_slot) If the code is currently in the middle of a 'do' loop, exits the loop if the value in the
    /// compare_slot is not zero. If the code is not in a loop, this is a null-op.
    BreakIf(IntegerSlot),
}

impl Code {
    /// Returns the number of places where this code item could be mutated.
    pub fn mutation_points(&self) -> u32 {
        todo!()
    }
}

impl CodeBuilder for Code {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) {
        match self {
            Code::ConstI32(slot, value) => {
                instruction_list.push(NumericInstruction::I32Constant(*value as i32).into());
                instruction_list.push(VariableInstruction::LocalSet(*slot as u32).into());
            }
            Code::Return(slots) => {
                for slot in slots.iter() {
                    instruction_list.push(VariableInstruction::LocalGet(*slot as u32).into());
                }
            }
            Code::DoFor(times, do_block) => {
                // Set a new local with the number of loops remaining (might be zero already)
                let local_index = context.get_unused_local(ValueType::I32);
                instruction_list.push(NumericInstruction::I32Constant(*times as i32).into());
                instruction_list.push(VariableInstruction::LocalSet(*local_index).into());

                // Create the code for the innermost loop. A branch of '0' will bring us to the top of this loop and a
                // branch of '1' will bring us to the end of the block surrounding the loop
                let mut inner_instructions: Vec<Instruction> = vec![];

                // Branch to the end of the outer block if the remaining loop count is zero
                // br_if 1 (i32.eqz (get_local $x) )
                inner_instructions.push(VariableInstruction::LocalGet(*local_index).into());
                inner_instructions.push(NumericInstruction::EqualToZero(IntegerType::I32).into());
                inner_instructions.push(ControlInstruction::BranchIf(1).into());

                // 'Do' the code. When the `loop_label` is dropped, it indicates we can't break from that loop anymore
                {
                    let loop_label = context.entering_loop(1);
                    do_block.append_code(context, &mut inner_instructions);
                    drop(loop_label);
                }

                // Subtract one from the remaining loop count
                // (set_local $x (sub (get_local $x) (i32.const 1) ) )
                inner_instructions.push(VariableInstruction::LocalGet(*local_index).into());
                inner_instructions.push(NumericInstruction::I32Constant(1).into());
                inner_instructions.push(NumericInstruction::Subtract(NumberType::I32).into());
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
                instruction_list
                    .push(ControlInstruction::Block(BlockType::None, loop_expression).into());
            }

            _ => unimplemented!(),
        }
    }
}
#[cfg(test)]
mod tests {
    use std::vec;

    use wasm_ast::{emit_binary, ModuleBuilder};
    use wasmtime::{Engine, Instance, Store};

    use crate::{Code, CodeContext, FunctionSignature, SlotCount, ValueType};

    fn instanciate_binary(bytes: impl AsRef<[u8]>) -> (Store<()>, Instance) {
        let engine = Engine::default();
        let module = wasmtime::Module::new(&engine, bytes).unwrap();
        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &vec![]).unwrap();
        (store, instance)
    }

    #[test]
    fn const_i32_and_return() {
        let fs = FunctionSignature::new("const_i32_and_return", vec![], vec![ValueType::I32]);
        let slots = SlotCount {
            i32: 1,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots);

        let code = vec![Code::ConstI32(0, 42), Code::Return(vec![0])];

        let mut builder = ModuleBuilder::new();
        context.build(&mut builder, &code[..]);
        let module = builder.build();

        let mut buffer = Vec::new();
        emit_binary(&module, &mut buffer).unwrap();

        // Create an instance of the module and get a pointer to the exported function by its name
        let (mut store, instance) = instanciate_binary(&buffer[..]);
        let typed_func = instance
            .get_typed_func::<(), i32, _>(&mut store, "const_i32_and_return")
            .unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(42, result);
    }
}
