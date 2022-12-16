use crate::code_builder::CodeBuilder;
use crate::indentation::Indentation;
use crate::CodeContext;
use crate::*;
use anyhow::Result;
use strum_macros::EnumIter;
use wasm_ast::Instruction;

#[derive(Clone, Debug, EnumIter, PartialEq)]
pub enum Code {
    // Const
    ConstI32(ConstI32),
    ConstI64(ConstI64),
    ConstF32(ConstF32),
    ConstF64(ConstF64),
    ConstOne(ConstOne),
    ConstZero(ConstZero),

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

    // Control
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
    pub fn points(&self) -> usize {
        match self {
            Code::If(instructions) => instructions.points(),
            Code::IfElse(instructions) => instructions.points(),
            Code::DoUntil(instructions) => instructions.points(),
            Code::DoWhile(instructions) => instructions.points(),
            Code::DoFor(instructions) => instructions.points(),
            _ => 1,
        }
    }

    /// Returns the minimum number of points consumed when generating this Code randomly
    pub fn minimum_points(&self) -> usize {
        match self {
            Code::If(_) => 2,
            Code::IfElse(_) => 3,
            Code::DoUntil(_) => 2,
            Code::DoWhile(_) => 2,
            Code::DoFor(_) => 2,
            _ => 1,
        }
    }

    /// Returns the default value for this type of code
    pub fn get_default(&self) -> Code {
        match self {
            Code::ConstI32(_) => Code::ConstI32(ConstI32::default()),
            Code::ConstI64(_) => Code::ConstI64(ConstI64::default()),
            Code::ConstF32(_) => Code::ConstF32(ConstF32::default()),
            Code::ConstF64(_) => Code::ConstF64(ConstF64::default()),
            Code::ConstOne(_) => Code::ConstOne(ConstOne::default()),
            Code::ConstZero(_) => Code::ConstZero(ConstZero::default()),
            Code::CountLeadingZeros(_) => Code::CountLeadingZeros(CountLeadingZeros::default()),
            Code::CountTrailingZeros(_) => Code::CountTrailingZeros(CountTrailingZeros::default()),
            Code::PopulationCount(_) => Code::PopulationCount(PopulationCount::default()),
            Code::And(_) => Code::And(And::default()),
            Code::Or(_) => Code::Or(Or::default()),
            Code::Xor(_) => Code::Xor(Xor::default()),
            Code::ShiftLeft(_) => Code::ShiftLeft(ShiftLeft::default()),
            Code::ShiftRight(_) => Code::ShiftRight(ShiftRight::default()),
            Code::RotateLeft(_) => Code::RotateLeft(RotateLeft::default()),
            Code::RotateRight(_) => Code::RotateRight(RotateRight::default()),
            Code::Add(_) => Code::Add(Add::default()),
            Code::Subtract(_) => Code::Subtract(Subtract::default()),
            Code::Multiply(_) => Code::Multiply(Multiply::default()),
            Code::Divide(_) => Code::Divide(Divide::default()),
            Code::Remainder(_) => Code::Remainder(Remainder::default()),
            Code::AbsoluteValue(_) => Code::AbsoluteValue(AbsoluteValue::default()),
            Code::Negate(_) => Code::Negate(Negate::default()),
            Code::SquareRoot(_) => Code::SquareRoot(SquareRoot::default()),
            Code::Ceiling(_) => Code::Ceiling(Ceiling::default()),
            Code::Floor(_) => Code::Floor(Floor::default()),
            Code::Nearest(_) => Code::Nearest(Nearest::default()),
            Code::Min(_) => Code::Min(Min::default()),
            Code::Max(_) => Code::Max(Max::default()),
            Code::CopySign(_) => Code::CopySign(CopySign::default()),
            Code::IsEqualZero(_) => Code::IsEqualZero(IsEqualZero::default()),
            Code::AreEqual(_) => Code::AreEqual(AreEqual::default()),
            Code::AreNotEqual(_) => Code::AreNotEqual(AreNotEqual::default()),
            Code::IsLessThan(_) => Code::IsLessThan(IsLessThan::default()),
            Code::IsGreaterThan(_) => Code::IsGreaterThan(IsGreaterThan::default()),
            Code::IsLessThanOrEqual(_) => Code::IsLessThanOrEqual(IsLessThanOrEqual::default()),
            Code::IsGreaterThanOrEqual(_) => Code::IsGreaterThanOrEqual(IsGreaterThanOrEqual::default()),
            Code::Return(_) => Code::Return(Return::default()),
            Code::CopySlot(_) => Code::CopySlot(CopySlot::default()),
            Code::Call(_) => Code::Call(Call::default()),
            Code::If(_) => Code::If(If::default()),
            Code::IfElse(_) => Code::IfElse(IfElse::default()),
            Code::DoUntil(_) => Code::DoUntil(DoUntil::default()),
            Code::DoWhile(_) => Code::DoWhile(DoWhile::default()),
            Code::DoFor(_) => Code::DoFor(DoFor::default()),
            Code::Break(_) => Code::Break(Break::default()),
            Code::BreakIf(_) => Code::BreakIf(BreakIf::default()),
        }
    }
}

impl CodeBuilder for Code {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        match self {
            Code::ConstI32(instruction) => instruction.append_code(context, instruction_list)?,
            Code::ConstI64(instruction) => instruction.append_code(context, instruction_list)?,
            Code::ConstF32(instruction) => instruction.append_code(context, instruction_list)?,
            Code::ConstF64(instruction) => instruction.append_code(context, instruction_list)?,
            Code::ConstOne(instruction) => instruction.append_code(context, instruction_list)?,
            Code::ConstZero(instruction) => instruction.append_code(context, instruction_list)?,
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

    fn make_random_code(&self, engine: &mut GeneticEngine, max_points: usize) -> Code {
        match self {
            Code::ConstI32(instruction) => instruction.make_random_code(engine, max_points),
            Code::ConstI64(instruction) => instruction.make_random_code(engine, max_points),
            Code::ConstF32(instruction) => instruction.make_random_code(engine, max_points),
            Code::ConstF64(instruction) => instruction.make_random_code(engine, max_points),
            Code::ConstOne(instruction) => instruction.make_random_code(engine, max_points),
            Code::ConstZero(instruction) => instruction.make_random_code(engine, max_points),
            Code::CountLeadingZeros(instruction) => instruction.make_random_code(engine, max_points),
            Code::CountTrailingZeros(instruction) => instruction.make_random_code(engine, max_points),
            Code::PopulationCount(instruction) => instruction.make_random_code(engine, max_points),
            Code::And(instruction) => instruction.make_random_code(engine, max_points),
            Code::Or(instruction) => instruction.make_random_code(engine, max_points),
            Code::Xor(instruction) => instruction.make_random_code(engine, max_points),
            Code::ShiftLeft(instruction) => instruction.make_random_code(engine, max_points),
            Code::ShiftRight(instruction) => instruction.make_random_code(engine, max_points),
            Code::RotateLeft(instruction) => instruction.make_random_code(engine, max_points),
            Code::RotateRight(instruction) => instruction.make_random_code(engine, max_points),
            Code::Add(instruction) => instruction.make_random_code(engine, max_points),
            Code::Subtract(instruction) => instruction.make_random_code(engine, max_points),
            Code::Multiply(instruction) => instruction.make_random_code(engine, max_points),
            Code::Divide(instruction) => instruction.make_random_code(engine, max_points),
            Code::Remainder(instruction) => instruction.make_random_code(engine, max_points),
            Code::AbsoluteValue(instruction) => instruction.make_random_code(engine, max_points),
            Code::Negate(instruction) => instruction.make_random_code(engine, max_points),
            Code::SquareRoot(instruction) => instruction.make_random_code(engine, max_points),
            Code::Ceiling(instruction) => instruction.make_random_code(engine, max_points),
            Code::Floor(instruction) => instruction.make_random_code(engine, max_points),
            Code::Nearest(instruction) => instruction.make_random_code(engine, max_points),
            Code::Min(instruction) => instruction.make_random_code(engine, max_points),
            Code::Max(instruction) => instruction.make_random_code(engine, max_points),
            Code::CopySign(instruction) => instruction.make_random_code(engine, max_points),
            Code::IsEqualZero(instruction) => instruction.make_random_code(engine, max_points),
            Code::AreEqual(instruction) => instruction.make_random_code(engine, max_points),
            Code::AreNotEqual(instruction) => instruction.make_random_code(engine, max_points),
            Code::IsLessThan(instruction) => instruction.make_random_code(engine, max_points),
            Code::IsGreaterThan(instruction) => instruction.make_random_code(engine, max_points),
            Code::IsLessThanOrEqual(instruction) => instruction.make_random_code(engine, max_points),
            Code::IsGreaterThanOrEqual(instruction) => instruction.make_random_code(engine, max_points),
            Code::Return(instruction) => instruction.make_random_code(engine, max_points),
            Code::CopySlot(instruction) => instruction.make_random_code(engine, max_points),
            Code::Call(instruction) => instruction.make_random_code(engine, max_points),
            Code::If(instruction) => instruction.make_random_code(engine, max_points),
            Code::IfElse(instruction) => instruction.make_random_code(engine, max_points),
            Code::DoUntil(instruction) => instruction.make_random_code(engine, max_points),
            Code::DoWhile(instruction) => instruction.make_random_code(engine, max_points),
            Code::DoFor(instruction) => instruction.make_random_code(engine, max_points),
            Code::Break(instruction) => instruction.make_random_code(engine, max_points),
            Code::BreakIf(instruction) => instruction.make_random_code(engine, max_points),
        }
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        match self {
            Code::ConstI32(instruction) => instruction.print_for_rust(f, indentation),
            Code::ConstI64(instruction) => instruction.print_for_rust(f, indentation),
            Code::ConstF32(instruction) => instruction.print_for_rust(f, indentation),
            Code::ConstF64(instruction) => instruction.print_for_rust(f, indentation),
            Code::ConstOne(instruction) => instruction.print_for_rust(f, indentation),
            Code::ConstZero(instruction) => instruction.print_for_rust(f, indentation),
            Code::CountLeadingZeros(instruction) => instruction.print_for_rust(f, indentation),
            Code::CountTrailingZeros(instruction) => instruction.print_for_rust(f, indentation),
            Code::PopulationCount(instruction) => instruction.print_for_rust(f, indentation),
            Code::And(instruction) => instruction.print_for_rust(f, indentation),
            Code::Or(instruction) => instruction.print_for_rust(f, indentation),
            Code::Xor(instruction) => instruction.print_for_rust(f, indentation),
            Code::ShiftLeft(instruction) => instruction.print_for_rust(f, indentation),
            Code::ShiftRight(instruction) => instruction.print_for_rust(f, indentation),
            Code::RotateLeft(instruction) => instruction.print_for_rust(f, indentation),
            Code::RotateRight(instruction) => instruction.print_for_rust(f, indentation),
            Code::Add(instruction) => instruction.print_for_rust(f, indentation),
            Code::Subtract(instruction) => instruction.print_for_rust(f, indentation),
            Code::Multiply(instruction) => instruction.print_for_rust(f, indentation),
            Code::Divide(instruction) => instruction.print_for_rust(f, indentation),
            Code::Remainder(instruction) => instruction.print_for_rust(f, indentation),
            Code::AbsoluteValue(instruction) => instruction.print_for_rust(f, indentation),
            Code::Negate(instruction) => instruction.print_for_rust(f, indentation),
            Code::SquareRoot(instruction) => instruction.print_for_rust(f, indentation),
            Code::Ceiling(instruction) => instruction.print_for_rust(f, indentation),
            Code::Floor(instruction) => instruction.print_for_rust(f, indentation),
            Code::Nearest(instruction) => instruction.print_for_rust(f, indentation),
            Code::Min(instruction) => instruction.print_for_rust(f, indentation),
            Code::Max(instruction) => instruction.print_for_rust(f, indentation),
            Code::CopySign(instruction) => instruction.print_for_rust(f, indentation),
            Code::IsEqualZero(instruction) => instruction.print_for_rust(f, indentation),
            Code::AreEqual(instruction) => instruction.print_for_rust(f, indentation),
            Code::AreNotEqual(instruction) => instruction.print_for_rust(f, indentation),
            Code::IsLessThan(instruction) => instruction.print_for_rust(f, indentation),
            Code::IsGreaterThan(instruction) => instruction.print_for_rust(f, indentation),
            Code::IsLessThanOrEqual(instruction) => instruction.print_for_rust(f, indentation),
            Code::IsGreaterThanOrEqual(instruction) => instruction.print_for_rust(f, indentation),
            Code::Return(instruction) => instruction.print_for_rust(f, indentation),
            Code::CopySlot(instruction) => instruction.print_for_rust(f, indentation),
            Code::Call(instruction) => instruction.print_for_rust(f, indentation),
            Code::If(instruction) => instruction.print_for_rust(f, indentation),
            Code::IfElse(instruction) => instruction.print_for_rust(f, indentation),
            Code::DoUntil(instruction) => instruction.print_for_rust(f, indentation),
            Code::DoWhile(instruction) => instruction.print_for_rust(f, indentation),
            Code::DoFor(instruction) => instruction.print_for_rust(f, indentation),
            Code::Break(instruction) => instruction.print_for_rust(f, indentation),
            Code::BreakIf(instruction) => instruction.print_for_rust(f, indentation),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use wasm_ast::{emit_binary, ModuleBuilder};
    use wasmtime::{Engine, Instance, Store};

    use crate::code_builder::CodeBuilder;
    use crate::indentation::Indentation;
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

    #[test]
    fn print() {
        let to_print = vec![
            ConstI32::new(0, 1),
            ConstI64::new(0, 1),
            ConstF32::new(0, 1f32),
            ConstF64::new(0, 1f64),
            ConstOne::new(0),
            ConstZero::new(0),
            CountLeadingZeros::new(0, 1),
            CountTrailingZeros::new(0, 1),
            PopulationCount::new(0, 1),
            And::new(0, 1, 2),
            Or::new(0, 1, 2),
            Xor::new(0, 1, 2),
            ShiftLeft::new(0, 1, 2),
            ShiftRight::new(0, 1, 2),
            RotateLeft::new(0, 1, 2),
            RotateRight::new(0, 1, 2),
            Add::new(0, 1, 2),
            Subtract::new(0, 1, 2),
            Multiply::new(0, 1, 2),
            Divide::new(0, 1, 2),
            Remainder::new(0, 1, 2),
            AbsoluteValue::new(0, 1),
            Negate::new(0, 1),
            SquareRoot::new(0, 1),
            Ceiling::new(0, 1),
            Floor::new(0, 1),
            Nearest::new(0, 1),
            Min::new(0, 1, 2),
            Max::new(0, 1, 2),
            CopySign::new(0, 1, 2),
            IsEqualZero::new(0, 1),
            AreEqual::new(0, 1, 2),
            AreNotEqual::new(0, 1, 2),
            IsLessThan::new(0, 1, 2),
            IsGreaterThan::new(0, 1, 2),
            IsLessThanOrEqual::new(0, 1, 2),
            IsGreaterThanOrEqual::new(0, 1, 2),
            Return::new(),
            CopySlot::new(0, 1),
            Call::new(0, vec![0, 1], vec![2, 3]),
            If::new(0, vec![Return::new()]),
            IfElse::new(0, vec![Return::new()], vec![]),
            DoUntil::new(0, vec![Return::new()]),
            DoWhile::new(0, vec![Return::new()]),
            DoFor::new(0, vec![Return::new()]),
            Break::new(),
            BreakIf::new(0),
        ];

        let mut indentation = Indentation::new(4, 0);
        let mut output = std::string::String::new();
        to_print.print_for_rust(&mut output, &mut indentation).unwrap();
        assert_eq!(
            output,
            "[
    ConstI32::new(0, 1),
    ConstI64::new(0, 1),
    ConstF32::new(0, 1f32),
    ConstF64::new(0, 1f64),
    ConstOne::new(0),
    ConstZero::new(0),
    CountLeadingZeros::new(0, 1),
    CountTrailingZeros::new(0, 1),
    PopulationCount::new(0, 1),
    And::new(0, 1, 2),
    Or::new(0, 1, 2),
    Xor::new(0, 1, 2),
    ShiftLeft::new(0, 1, 2),
    ShiftRight::new(0, 1, 2),
    RotateLeft::new(0, 1, 2),
    RotateRight::new(0, 1, 2),
    Add::new(0, 1, 2),
    Subtract::new(0, 1, 2),
    Multiply::new(0, 1, 2),
    Divide::new(0, 1, 2),
    Remainder::new(0, 1, 2),
    AbsoluteValue::new(0, 1),
    Negate::new(0, 1),
    SquareRoot::new(0, 1),
    Ceiling::new(0, 1),
    Floor::new(0, 1),
    Nearest::new(0, 1),
    Min::new(0, 1, 2),
    Max::new(0, 1, 2),
    CopySign::new(0, 1, 2),
    IsEqualZero::new(0, 1),
    AreEqual::new(0, 1, 2),
    AreNotEqual::new(0, 1, 2),
    IsLessThan::new(0, 1, 2),
    IsGreaterThan::new(0, 1, 2),
    IsLessThanOrEqual::new(0, 1, 2),
    IsGreaterThanOrEqual::new(0, 1, 2),
    Return::new(),
    CopySlot::new(0, 1),
    Call::new(0, vec![0, 1], vec![2, 3]),
    If::new(0, vec![
        Return::new(),
    ]),
    IfElse::new(0, vec![
        Return::new(),
    ], vec![
    ]),
    DoUntil::new(0, vec![
        Return::new(),
    ]),
    DoWhile::new(0, vec![
        Return::new(),
    ]),
    DoFor::new(0, vec![
        Return::new(),
    ]),
    Break::new(),
    BreakIf::new(0),
]"
        );
    }
}
