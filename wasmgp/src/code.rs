use crate::convert::{GetSlotConvert, SetSlotConvert};
use crate::{code_builder::CodeBuilder, code_context::CodeContext, Slot, ValueType};
use anyhow::Result;
use wasm_ast::{
    BlockType, ControlInstruction, Expression, Instruction, IntegerType, NumberType, NumericInstruction,
    VariableInstruction,
};

pub enum Code {
    /// ConstI32(slot, value): Loads the specified value into a four-byte integer into the specified work variable
    /// slot. If the slot is for floating-point values, it will be cast to a float.
    ConstI32(Slot, i32),

    /// ConstI64(slot, value): Loads the specified value into a eight-byte integer into the specified work variable
    /// slot. If the slot is for floating-point values, it will be cast to a float.
    ConstI64(Slot, i64),

    /// ConstF32(slot, value): Loads the specified value into a four-byte work variable slot. If the slot is for
    /// integer values, it will be truncated.
    ConstF32(Slot, f32),

    /// ConstF64(slot, value): Loads the specified value into a eight-byte work variable slot. If the slot is for
    /// integer values, it will be truncated.
    ConstF64(Slot, f64),

    /// CountLeadingZeros(source_slot, destination_slot): Counts the number of leading zero bits in the specified source
    /// slot and places it into the destination_slot.
    CountLeadingZeros(Slot, Slot),

    /// CountTrailingZeros(source_slot, destination_slot): Counts the number of trailing zero bits in the specified
    /// source slot and places it into the destination_slot.
    CountTrailingZeros(Slot, Slot),

    /// PopulationCount(source_slot, destination_slot): Counts the number of one bits in the specified source slot and
    /// places it into the destination_slot.
    PopulationCount(Slot, Slot),

    /// AddInteger(left_slot, right_slot, result_slot): Places the result of left + right in the result slot.
    AddInteger(Slot, Slot, Slot),

    /// AddFloat(left_slot, right_slot, result_slot): Places the result of left + right in the result slot.
    AddFloat(Slot, Slot, Slot),

    /// SubtractInteger(left_slot, right_slot, result_slot): Places the result of left - right in the result slot.
    SubtractInteger(Slot, Slot, Slot),

    /// SubtractFloat(left_slot, right_slot, result_slot): Places the result of left - right in the result slot.
    SubtractFloat(Slot, Slot, Slot),

    /// MultiplyInteger(left_slot, right_slot, result_slot): Places the result of left * right in the result slot.
    MultiplyInteger(Slot, Slot, Slot),

    /// MultiplyFloat(left_slot, right_slot, result_slot): Places the result of left * right in the result slot.
    MultiplyFloat(Slot, Slot, Slot),

    /// Divide(dividend_slot, divisor_slot, result_slot): Places the result of dividend / divisor in the result
    /// slot. The code will leave the result untouched if the divisor is zero.
    Divide(Slot, Slot, Slot),

    /// DivideFloat(dividend_slot, divisor_slot, result_slot): Places the result of dividend / divisor in the result
    /// slot. The code will leave the result untouched if the divisor is zero.
    DivideFloat(Slot, Slot, Slot),

    /// Remainder(dividend_slot, divisor_slot, result_slot): Places the result of dividend % divisor in the result
    /// slot. The code will leave the result untouched if the divisor is zero.
    Remainder(Slot, Slot, Slot),

    And(Slot, Slot, Slot),
    Or(Slot, Slot, Slot),
    Xor(Slot, Slot, Slot),
    ShiftLeft(Slot, Slot, Slot),
    ShiftRight(Slot, Slot, Slot),
    RotateLeft(Slot, Slot, Slot),
    RotateRight(Slot, Slot, Slot),
    AbsoluteValue(Slot, Slot),
    Negate(Slot, Slot),
    SquareRoot(Slot, Slot),
    Ceiling(Slot, Slot),
    Floor(Slot, Slot),
    Nearest(Slot, Slot),
    Min(Slot, Slot, Slot),
    Max(Slot, Slot, Slot),
    CopySign(Slot, Slot, Slot),
    IsEqualZero(Slot, Slot),
    AreEqualInteger(Slot, Slot, Slot),
    AreNotEqualInteger(Slot, Slot, Slot),
    IsLessThan(Slot, Slot, Slot),
    IsGreaterThan(Slot, Slot, Slot),
    IsLessThanOrEqual(Slot, Slot, Slot),
    IsGreaterThanOrEqual(Slot, Slot, Slot),
    AreEqualFloat(Slot, Slot, Slot),
    AreNotEqualFloat(Slot, Slot, Slot),
    IsLessThanFloat(Slot, Slot, Slot),
    IsGreaterThanFloat(Slot, Slot, Slot),
    IsLessThanOrEqualFloat(Slot, Slot, Slot),
    IsGreaterThanOrEqualFloat(Slot, Slot, Slot),

    /// LoadI8(offset_slot, result_slot): Loads the i8 value at the memory index indicated by the offset into the result
    /// slot. The memory index will be cast into an integer and the calculation `offset % mem_size` applied before
    /// attempting to read the memory. The i8 value will be cast into the result slot type.
    LoadI8(Slot, Slot),
    LoadU8(Slot, Slot),
    LoadI16(Slot, Slot),
    LoadU16(Slot, Slot),
    LoadI32(Slot, Slot),
    LoadU32(Slot, Slot),
    LoadI64(Slot, Slot),
    LoadU64(Slot, Slot),
    LoadF32(Slot, Slot),
    LoadF64(Slot, Slot),
    StoreI8(Slot, Slot),
    StoreU8(Slot, Slot),
    StoreI16(Slot, Slot),
    StoreU16(Slot, Slot),
    StoreI32(Slot, Slot),
    StoreU32(Slot, Slot),
    StoreI64(Slot, Slot),
    StoreU64(Slot, Slot),
    StoreF32(Slot, Slot),
    StoreF64(Slot, Slot),

    /// Returns from a function. There are work variables of the appropriate types set aside to hold the return values.
    /// The function should set the values of those slots prior to calling Return, however they are always initialized
    /// to zero at the top of the function.
    Return,

    /// Call(function_index, parameter_slots, return_slots): Calls the host or code function with the specified index
    /// (remainder the number of functions) and uses the specified list of work variables as parameters. If more work
    /// variables are specified than are needed, they will be ignored. If more work variables are needed than are
    /// supplied, the works 0..x will be used until all parameters are satisfied. The returns values from the function
    /// will be placed into the work variables specified by 'return_slots'.
    Call(u32, Vec<Slot>, Vec<Slot>),

    /// If(compare_slot, do): If the value in the compare_slot is not zero, than the code listed in 'do' will execute.
    If(Slot, Vec<Code>),

    /// IfElse(compare_slot, do, else_do): If the value in the compare_slot is not zero, than the code listed in 'do'
    /// will execute. Otherwise, the code listed in 'else_do' will execute.
    IfElse(Slot, Vec<Code>, Vec<Code>),

    /// DoUntil(compare_slot, do): Will execute the code listed in 'do' until the value in the compare_slot is not zero.
    /// This will execute the 'do' block at least once.
    DoUntil(Slot, Vec<Code>),

    /// DoWhile(compare_slot, do): Will execute the code listed in 'do' while the value in the compare_slot is not zero.
    /// This will check the compare value before executing the 'do' code and so 'do' might never run.
    DoWhile(Slot, Vec<Code>),

    /// DoFor(times, do): Runs the code listed in 'do' a specific number of times chosen by the genetic algorithm (at
    /// code compile-time, not while the VM is running). Max of 65_535 loops
    DoFor(u16, Vec<Code>),

    /// Break: If the code is currently in the middle of a 'do' loop, exits the loop unconditionally. If the code is not
    /// in a loop, this is a null-op.
    Break,

    /// BreakIf(compare_slot) If the code is currently in the middle of a 'do' loop, exits the loop if the value in the
    /// compare_slot is not zero. If the code is not in a loop, this is a null-op.
    BreakIf(Slot),
}

impl Code {
    /// Returns the number of places where this code item could be mutated.
    pub fn mutation_points(&self) -> u32 {
        todo!()
    }
}

impl CodeBuilder for Code {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        match self {
            Code::ConstI32(slot, value) => {
                instruction_list.push(NumericInstruction::I32Constant(*value).into());
                SetSlotConvert::convert(*slot, ValueType::I32, context, instruction_list)?;
            }
            Code::ConstI64(slot, value) => {
                instruction_list.push(NumericInstruction::I64Constant(*value).into());
                SetSlotConvert::convert(*slot, ValueType::I64, context, instruction_list)?;
            }
            Code::ConstF32(slot, value) => {
                instruction_list.push(NumericInstruction::F32Constant(*value).into());
                SetSlotConvert::convert(*slot, ValueType::F32, context, instruction_list)?;
            }
            Code::ConstF64(slot, value) => {
                instruction_list.push(NumericInstruction::F64Constant(*value).into());
                SetSlotConvert::convert(*slot, ValueType::F64, context, instruction_list)?;
            }
            Code::CountLeadingZeros(src, dest) => {
                let convert_to = match context.get_slot_value_type(*src)? {
                    ValueType::I32 => ValueType::I32,
                    _ => ValueType::I64,
                };
                GetSlotConvert::convert(*src, convert_to, context, instruction_list)?;
                instruction_list.push(NumericInstruction::CountLeadingZeros(convert_to.into()).into());
                SetSlotConvert::convert(*dest, convert_to, context, instruction_list)?;
            }
            Code::Return => {
                for slot in context.return_slots().iter() {
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
                    do_block.append_code(context, &mut inner_instructions)?;
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
                instruction_list.push(ControlInstruction::Block(BlockType::None, loop_expression).into());
            }

            _ => unimplemented!(),
        }

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use std::vec;

    use wasm_ast::{emit_binary, ModuleBuilder};
    use wasmtime::{Engine, Instance, Store};

    use crate::{Code, CodeContext, FunctionSignature, SlotCount, ValueType};

    fn build_code(context: CodeContext, code: Vec<Code>) -> (Store<()>, Instance) {
        let mut builder = ModuleBuilder::new();
        context.build(&mut builder, &code[..]).unwrap();
        let module = builder.build();

        let mut buffer = Vec::new();
        emit_binary(&module, &mut buffer).unwrap();

        // Create an instance of the module
        let engine = Engine::default();
        let module = wasmtime::Module::new(&engine, &buffer[..]).unwrap();
        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &vec![]).unwrap();
        (store, instance)
    }

    #[test]
    fn const_i32_and_return() {
        // Context
        let name = "const_i32_and_return";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::I32]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();

        // Code
        let code = vec![Code::ConstI32(0, 42), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), u32, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(42, result);
    }

    #[test]
    fn return_init_to_zero() {
        // Context
        let name = "return_init_to_zero";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::I32]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();

        // Code
        let code = vec![Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), u32, _>(&mut store, name).unwrap();

        // Call the function and confirm we get zero
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(0, result);
    }

    #[test]
    #[ignore = "negative integers spin forever, see: https://github.com/misalcedo/wasm-ast/issues/40"]
    fn const_i32_and_return_u64() {
        // Context
        let name = "const_i32_and_return_u64";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::I64]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();
        assert_eq!(ValueType::I64, context.get_slot_value_type(0).unwrap());

        // Code: because we're using unsigned math in Wasm, -1 should be 0xFFFFFFFF in u32/u64
        let code = vec![Code::ConstI32(0, -1), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), u64, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(0xFFFFFFFF, result);
    }

    #[test]
    fn const_i32_and_return_f32() {
        // Context
        let name = "const_i32_and_return_f32";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::F32]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();
        assert_eq!(ValueType::F32, context.get_slot_value_type(0).unwrap());

        // Code
        let code = vec![Code::ConstI32(0, 42), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), f32, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(42.0, result);
    }

    #[test]
    fn const_i32_and_return_f64() {
        // Context
        let name = "const_i32_and_return_f64";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::F64]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();
        assert_eq!(ValueType::F64, context.get_slot_value_type(0).unwrap());

        // Code
        let code = vec![Code::ConstI32(0, 42), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), f64, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(42.0, result);
    }

    #[test]
    #[ignore = "negative integers spin forever, see: https://github.com/misalcedo/wasm-ast/issues/40"]
    fn const_i64_and_return_i32() {
        // Context
        let name = "const_i64_and_return_i32";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::I32]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();
        assert_eq!(ValueType::I32, context.get_slot_value_type(0).unwrap());

        // Code:
        let code = vec![Code::ConstI64(0, -1), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), i32, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(-1, result);
    }

    #[test]
    #[ignore = "negative integers spin forever, see: https://github.com/misalcedo/wasm-ast/issues/40"]
    fn const_i64_and_return_u64() {
        // Context
        let name = "const_i64_and_return_u64";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::I64]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();
        assert_eq!(ValueType::I64, context.get_slot_value_type(0).unwrap());

        // Code: because we're using unsigned math in Wasm, -1 should be 0xFFFFFFFF in u32/u64
        let code = vec![Code::ConstI64(0, -1), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), u64, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(0xFFFFFFFF, result);
    }

    #[test]
    fn const_i64_and_return_f32() {
        // Context
        let name = "const_i64_and_return_f32";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::F32]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();
        assert_eq!(ValueType::F32, context.get_slot_value_type(0).unwrap());

        // Code
        let code = vec![Code::ConstI64(0, 42), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), f32, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(42.0, result);
    }

    #[test]
    fn const_i64_and_return_f64() {
        // Context
        let name = "const_i64_and_return_f64";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::F64]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();
        assert_eq!(ValueType::F64, context.get_slot_value_type(0).unwrap());

        // Code
        let code = vec![Code::ConstI64(0, 42), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), f64, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(42.0, result);
    }

    #[test]
    fn const_f32_and_return_i32() {
        // Context
        let name = "const_f32_and_return_i32";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::I32]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();
        assert_eq!(ValueType::I32, context.get_slot_value_type(0).unwrap());

        // Code: unsigned math, -1 saturates to 0
        let code = vec![Code::ConstF32(0, -1.0), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), i32, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(0, result);
    }

    #[test]
    fn const_f32_signed_and_return_i32() {
        // Context
        let name = "const_f32_signed_and_return_i32";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::I32]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, true).unwrap();
        assert_eq!(ValueType::I32, context.get_slot_value_type(0).unwrap());

        // Code: signed math, -1.0 saturates to -1
        let code = vec![Code::ConstF32(0, -1.0), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), i32, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(-1, result);
    }

    #[test]
    fn const_f32_and_return_u64() {
        // Context
        let name = "const_f32_and_return_u64";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::I64]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();
        assert_eq!(ValueType::I64, context.get_slot_value_type(0).unwrap());

        // Code: because we're using unsigned math in Wasm, -1.0 should be 0 in u32/u64
        let code = vec![Code::ConstF32(0, -1.0), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), u64, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(0, result);
    }

    #[test]
    fn const_f32_and_return_f32() {
        // Context
        let name = "const_f32_and_return_f32";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::F32]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();
        assert_eq!(ValueType::F32, context.get_slot_value_type(0).unwrap());

        // Code
        let code = vec![Code::ConstF32(0, 42.0), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), f32, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(42.0, result);
    }

    #[test]
    fn const_f32_and_return_f64() {
        // Context
        let name = "const_f32_and_return_f64";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::F64]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();
        assert_eq!(ValueType::F64, context.get_slot_value_type(0).unwrap());

        // Code
        let code = vec![Code::ConstF32(0, 42.0), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), f64, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(42.0, result);
    }

    #[test]
    fn const_f64_and_return_i32() {
        // Context
        let name = "const_f64_and_return_i32";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::I32]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();
        assert_eq!(ValueType::I32, context.get_slot_value_type(0).unwrap());

        // Code: unsigned math, -1 saturates to 0
        let code = vec![Code::ConstF64(0, -1.0), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), i32, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(0, result);
    }

    #[test]
    fn const_f64_signed_and_return_i32() {
        // Context
        let name = "const_f64_signed_and_return_i32";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::I32]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, true).unwrap();
        assert_eq!(ValueType::I32, context.get_slot_value_type(0).unwrap());

        // Code: signed math, -1.0 saturates to -1
        let code = vec![Code::ConstF64(0, -1.0), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), i32, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(-1, result);
    }

    #[test]
    fn const_f64_and_return_u64() {
        // Context
        let name = "const_f64_and_return_u64";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::I64]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();
        assert_eq!(ValueType::I64, context.get_slot_value_type(0).unwrap());

        // Code: because we're using unsigned math in Wasm, -1.0 should be 0 in u32/u64
        let code = vec![Code::ConstF64(0, -1.0), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), u64, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(0, result);
    }

    #[test]
    fn const_f64_and_return_f32() {
        // Context
        let name = "const_f64_and_return_f32";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::F32]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();
        assert_eq!(ValueType::F32, context.get_slot_value_type(0).unwrap());

        // Code
        let code = vec![Code::ConstF64(0, 42.0), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), f32, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(42.0, result);
    }

    #[test]
    fn const_f64_and_return_f64() {
        // Context
        let name = "const_f64_and_return_f64";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::F64]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();
        assert_eq!(ValueType::F64, context.get_slot_value_type(0).unwrap());

        // Code
        let code = vec![Code::ConstF64(0, 42.0), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), f64, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(42.0, result);
    }

    #[test]
    fn count_leading_zeros_i32() {
        // Context
        let name = "count_leading_zeros_i32";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::I32]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 0,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();

        // Code
        let code = vec![Code::ConstI32(0, 1), Code::CountLeadingZeros(0, 0), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), u32, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(31, result);
    }

    #[test]
    fn count_leading_zeros_f32() {
        // Context
        let name = "count_leading_zeros_f32";
        let fs = FunctionSignature::new(name, vec![], vec![ValueType::I32]);
        let slots = SlotCount {
            i32: 0,
            i64: 0,
            f32: 1,
            f64: 0,
        };
        let context = CodeContext::new(&fs, slots, false).unwrap();

        // Code: all float -> integer conversions use 64 bit integers
        let code = vec![Code::ConstF32(1, 1.0), Code::CountLeadingZeros(1, 0), Code::Return];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), u32, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(63, result);
    }
}
