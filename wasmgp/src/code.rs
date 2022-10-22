pub enum Code {
    /// ConstI32(slot, value): Loads the specified value into a four-byte integer into the specified local variable
    /// slot. It may be interpreted as signed or unsigned later. If the slot is for floating-point values, it will be
    /// cast to a float.
    ConstI32(u8, i32),

    /// ConstI64(slot, value): Loads the specified value into a eight-byte integer into the specified local variable
    /// slot. It may be interpreted as signed or unsigned later. If the slot is for floating-point values, it will be
    /// cast to a float.
    ConstI64(u8, i64),

    /// ConstF32(slot, value): Loads the specified value into a four-byte local variable slot. If the slot is for
    /// integer values, it will be truncated.
    ConstF32(u8, f32),

    /// ConstF64(slot, value): Loads the specified value into a eight-byte local variable slot. If the slot is for
    /// integer values, it will be truncated.
    ConstF64(u8, f64),

    /// CountLeadingZeros(source_slot, destination_slot): Counts the number of leading zero bits in the specified source
    /// slot. If the source slot is for floating-point values, it will be truncated to an integer of the same number of 
    /// bits and then counted. The count will be placed into the destination_slot.
    CountLeadingZeros(u8, u8),

    /// CountTrailingZeros(source_slot, destination_slot): Counts the number of trailing zero bits in the specified 
    /// source slot. If the source slot is for floating-point values, it will be truncated to an integer of the same 
    /// number of bits and then counted. The count will be placed into the destination_slot.
    CountTrailingZeros(u8, u8),

    /// PopulationCount(source_slot, destination_slot): Counts the number of one bits in the specified source slot. If 
    /// the source slot is for floating-point values, it will be truncated to an integer of the same number of bits and 
    /// then counted. The count will be placed into the destination_slot.
    PopulationCount(u8, u8),

    /// Add(left_slot, right_slot, result_slot): Places the result of left + right in the result slot. Both left and 
    /// right will be cast to the type of the result slot if needed before adding.
    Add(u8, u8, u8),

    /// Subtract(left_slot, right_slot, result_slot): Places the result of left - right in the result slot. Both left 
    /// and right will be cast to the type of the result slot if needed before subtracting.
    Subtract(u8, u8, u8),

    /// Multiply(left_slot, right_slot, result_slot): Places the result of left * right in the result slot. Both left 
    /// and right will be cast to the type of the result slot if needed before multiplying.
    Multiply(u8, u8, u8),

    /// DivideSigned(dividend_slot, divisor_slot, result_slot): Places the result of dividend / divisor in the result 
    /// slot. Both dividend and divisor will be cast to the type of the result slot if needed before dividing. The code
    /// will leave the result untouched if the divisor is zero.
    DivideSigned(u8, u8, u8),

    /// DivideUnsigned(dividend_slot, divisor_slot, result_slot): Places the result of dividend / divisor in the result 
    /// slot. Both dividend and divisor will be cast to the type of the result slot if needed before dividing. The code
    /// will leave the result untouched if the divisor is zero.
    DivideUnsigned(u8, u8, u8),

    /// RemainderSigned(dividend_slot, divisor_slot, result_slot): Places the result of dividend % divisor in the result 
    /// slot. Both dividend and divisor will be cast to an integer if needed before dividing. The code will leave the 
    /// result untouched if the divisor is zero.
    RemainderSigned(u8, u8, u8),

    /// RemainderUnsigned(dividend_slot, divisor_slot, result_slot): Places the result of dividend % divisor in the 
    /// result slot. Both dividend and divisor will be cast to an integer if needed before dividing. The code will leave 
    /// the result untouched if the divisor is zero.
    RemainderUnsigned(u8, u8, u8),

    And(u8, u8, u8),
    Or(u8, u8, u8),
    Xor(u8, u8, u8),
    ShiftLeft(u8, u8, u8),
    ShiftRightSigned(u8, u8, u8),
    ShiftRightUnsigned(u8, u8, u8),
    RotateLeft(u8, u8, u8),
    RotateRight(u8, u8, u8),
    AbsoluteValue(u8, u8),
    Negate(u8, u8),
    SquareRoot(u8, u8),
    Ceiling(u8, u8),
    Floor(u8, u8),
    Nearest(u8, u8),
    Min(u8, u8, u8),
    Max(u8, u8, u8),
    CopySign(u8, u8, u8),
    IsEqualZero(u8, u8),
    AreEqual(u8, u8, u8),
    AreNotEqual(u8, u8, u8),
    IsLessThanSigned(u8, u8, u8),
    IsLessThanUnsigned(u8, u8, u8),
    IsGreaterThanSigned(u8, u8, u8),
    IsGreaterThanUnsigned(u8, u8, u8),
    IsLessThanOrEqualSigned(u8, u8, u8),
    IsLessThanOrEqualUnsigned(u8, u8, u8),
    IsGreaterThanOrEqualSigned(u8, u8, u8),
    IsGreaterThanOrEqualUnsigned(u8, u8, u8),

    /// LoadI8(offset_slot, result_slot): Loads the i8 value at the memory index indicated by the offset into the result
    /// slot. The memory index will be cast into an integer and the calculation `offset % mem_size` applied before
    /// attempting to read the memory. The i8 value will be cast into the result slot type.
    LoadI8(u8, u8),
    LoadU8(u8, u8),
    LoadI16(u8, u8),
    LoadU16(u8, u8),
    LoadI32(u8, u8),
    LoadU32(u8, u8),
    LoadI64(u8, u8),
    LoadU64(u8, u8),
    LoadF32(u8, u8),
    LoadF64(u8, u8),
    StoreI8(u8, u8),
    StoreU8(u8, u8),
    StoreI16(u8, u8),
    StoreU16(u8, u8),
    StoreI32(u8, u8),
    StoreU32(u8, u8),
    StoreI64(u8, u8),
    StoreU64(u8, u8),
    StoreF32(u8, u8),
    StoreF64(u8, u8),

    /// Returns from a function, using the specified local variables as return values. If more local variable are
    /// specified than are needed, they will be ignored. If more local variable are needed than supplied, the code will
    /// use locals 0..x until all return values are satisfied.
    Return(Vec<u8>),

    /// Call(function_index, parameters): Calls the host or code function with the specified index (remainder the number
    /// of functions) and uses the specified list of local variables as parameters. If more local variables are 
    /// specified than are needed, they will be ignored. If more local variables are needed than are supplied, the 
    /// locals 0..x will be used until all parameters are satisfied.
    Call(u32, Vec<u8>),

    /// If(compare_slot, do): If the value in the compare_slot is not zero, than the code listed in 'do' will execute.
    If(u8, Vec<Code>),

    /// IfElse(compare_slot, do, else_do): If the value in the compare_slot is not zero, than the code listed in 'do' 
    /// will execute. Otherwise, the code listed in 'else_do' will execute.
    IfElse(u8, Vec<Code>, Vec<Code>),

    /// DoUntil(compare_slot, do): Will execute the code listed in 'do' until the value in the compare_slot is not zero.
    /// This will execute the 'do' block at least once.
    DoUntil(u8, Vec<Code>),

    /// DoWhile(compare_slot, do): Will execute the code listed in 'do' while the value in the compare_slot is not zero.
    /// This will check the compare value before executing the 'do' code and so 'do' might never run.
    DoWhile(u8, Vec<Code>),

    /// DoFor(times, do): Runs the code listed in 'do' a specific number of times chosen by the genetic algorithm (at
    /// code compile-time, not while the VM is running). Max of 65_535 loops
    DoFor(u16, Vec<Code>),

    /// Break: If the code is currently in the middle of a 'do' loop, exits the loop unconditionally. If the code is not
    /// in a loop, this is a null-op.
    Break,

    /// BreakIf(compare_slot) If the code is currently in the middle of a 'do' loop, exits the loop if the value in the
    /// compare_slot is not zero. If the code is not in a loop, this is a null-op.
    BreakIf(u8),
}