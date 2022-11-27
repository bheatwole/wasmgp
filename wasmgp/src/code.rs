use crate::code_builder::CodeBuilder;
use crate::CodeContext;
use crate::*;
use anyhow::Result;
use wasm_ast::Instruction;

pub enum Code {
    // Const
    ConstI32(ConstI32),
    ConstI64(ConstI64),
    ConstF32(ConstF32),
    ConstF64(ConstF64),

    // Bitwise
    CountLeadingZeros(CountLeadingZeros),
    CountTrailingZeros(CountTrailingZeros),
    PopulationCount(PopulationCount),
    And(And),
    Or(Or),
    Xor(Xor),
    ShiftLeft(ShiftLeft),
    ShiftRight(ShiftRight),
    RotateLeft(RotateLeft),
    RotateRight(RotateRight),

    // Arithmetic
    Add(Add),
    Subtract(Subtract),
    Multiply(Multiply),
    Divide(Divide),
    Remainder(Remainder),

    // Float
    AbsoluteValue(AbsoluteValue),
    Negate(Negate),
    SquareRoot(SquareRoot),
    Ceiling(Ceiling),
    Floor(Floor),
    Nearest(Nearest),
    Min(Min),
    Max(Max),
    CopySign(CopySign),

    // Comparison
    IsEqualZero(IsEqualZero),
    AreEqual(AreEqual),
    AreNotEqual(AreNotEqual),
    IsLessThan(IsLessThan),
    IsGreaterThan(IsGreaterThan),
    IsLessThanOrEqual(IsLessThanOrEqual),
    IsGreaterThanOrEqual(IsGreaterThanOrEqual),

    /// LoadI8(offset_slot, result_slot): Loads the i8 value at the memory index indicated by the offset into the result
    /// slot. The memory index will be cast into an integer and the calculation `offset % mem_size` applied before
    /// attempting to read the memory. The i8 value will be cast into the result slot type.
    // LoadI8(Slot, Slot),
    // LoadU8(Slot, Slot),
    // LoadI16(Slot, Slot),
    // LoadU16(Slot, Slot),
    // LoadI32(Slot, Slot),
    // LoadU32(Slot, Slot),
    // LoadI64(Slot, Slot),
    // LoadU64(Slot, Slot),
    // LoadF32(Slot, Slot),
    // LoadF64(Slot, Slot),
    // StoreI8(Slot, Slot),
    // StoreU8(Slot, Slot),
    // StoreI16(Slot, Slot),
    // StoreU16(Slot, Slot),
    // StoreI32(Slot, Slot),
    // StoreU32(Slot, Slot),
    // StoreI64(Slot, Slot),
    // StoreU64(Slot, Slot),
    // StoreF32(Slot, Slot),
    // StoreF64(Slot, Slot),
    CopySlot(CopySlot),
    Return(Return),
    Call(Call),
    If(If),
    IfElse(IfElse),
    DoUntil(DoUntil),
    DoWhile(DoWhile),
    DoFor(DoFor),
    Break(Break),
    BreakIf(BreakIf),
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
            Code::ConstI32(instruction) => instruction.append_code(context, instruction_list)?,
            Code::ConstI64(instruction) => instruction.append_code(context, instruction_list)?,
            Code::ConstF32(instruction) => instruction.append_code(context, instruction_list)?,
            Code::ConstF64(instruction) => instruction.append_code(context, instruction_list)?,
            Code::CountLeadingZeros(instruction) => instruction.append_code(context, instruction_list)?,
            Code::CountTrailingZeros(instruction) => instruction.append_code(context, instruction_list)?,
            Code::PopulationCount(instruction) => instruction.append_code(context, instruction_list)?,
            Code::And(instruction) => instruction.append_code(context, instruction_list)?,
            Code::Or(instruction) => instruction.append_code(context, instruction_list)?,
            Code::Xor(instruction) => instruction.append_code(context, instruction_list)?,
            Code::ShiftLeft(instruction) => instruction.append_code(context, instruction_list)?,
            Code::ShiftRight(instruction) => instruction.append_code(context, instruction_list)?,
            Code::RotateLeft(instruction) => instruction.append_code(context, instruction_list)?,
            Code::RotateRight(instruction) => instruction.append_code(context, instruction_list)?,
            Code::Add(instruction) => instruction.append_code(context, instruction_list)?,
            Code::Subtract(instruction) => instruction.append_code(context, instruction_list)?,
            Code::Multiply(instruction) => instruction.append_code(context, instruction_list)?,
            Code::Divide(instruction) => instruction.append_code(context, instruction_list)?,
            Code::Remainder(instruction) => instruction.append_code(context, instruction_list)?,
            Code::AbsoluteValue(instruction) => instruction.append_code(context, instruction_list)?,
            Code::Negate(instruction) => instruction.append_code(context, instruction_list)?,
            Code::SquareRoot(instruction) => instruction.append_code(context, instruction_list)?,
            Code::Ceiling(instruction) => instruction.append_code(context, instruction_list)?,
            Code::Floor(instruction) => instruction.append_code(context, instruction_list)?,
            Code::Nearest(instruction) => instruction.append_code(context, instruction_list)?,
            Code::Min(instruction) => instruction.append_code(context, instruction_list)?,
            Code::Max(instruction) => instruction.append_code(context, instruction_list)?,
            Code::CopySign(instruction) => instruction.append_code(context, instruction_list)?,
            Code::IsEqualZero(instruction) => instruction.append_code(context, instruction_list)?,
            Code::AreEqual(instruction) => instruction.append_code(context, instruction_list)?,
            Code::AreNotEqual(instruction) => instruction.append_code(context, instruction_list)?,
            Code::IsLessThan(instruction) => instruction.append_code(context, instruction_list)?,
            Code::IsGreaterThan(instruction) => instruction.append_code(context, instruction_list)?,
            Code::IsLessThanOrEqual(instruction) => instruction.append_code(context, instruction_list)?,
            Code::IsGreaterThanOrEqual(instruction) => instruction.append_code(context, instruction_list)?,
            Code::Return(instruction) => instruction.append_code(context, instruction_list)?,
            Code::CopySlot(instruction) => instruction.append_code(context, instruction_list)?,
            Code::Call(instruction) => instruction.append_code(context, instruction_list)?,
            Code::If(instruction) => instruction.append_code(context, instruction_list)?,
            Code::IfElse(instruction) => instruction.append_code(context, instruction_list)?,
            Code::DoUntil(instruction) => instruction.append_code(context, instruction_list)?,
            Code::DoWhile(instruction) => instruction.append_code(context, instruction_list)?,
            Code::DoFor(instruction) => instruction.append_code(context, instruction_list)?,
            Code::Break(instruction) => instruction.append_code(context, instruction_list)?,
            Code::BreakIf(instruction) => instruction.append_code(context, instruction_list)?,
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use wasm_ast::{emit_binary, ModuleBuilder};
    use wasmtime::{Engine, Instance, Store};

    use crate::*;

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
        let code = vec![Return::new()];

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
        let code = vec![ConstI32::new(0, -1), Return::new()];

        // Compile and get function pointer to it
        let (mut store, instance) = build_code(context, code);
        let typed_func = instance.get_typed_func::<(), u64, _>(&mut store, name).unwrap();

        // Call the function and confirm we get the constant
        let result = typed_func.call(&mut store, ()).unwrap();
        assert_eq!(0xFFFFFFFF, result);
    }
}
