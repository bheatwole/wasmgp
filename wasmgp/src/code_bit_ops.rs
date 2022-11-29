use crate::code_builder::CodeBuilder;
use crate::convert::{GetSlotConvert, SetSlotConvert};
use crate::indentation::Indentation;
use crate::*;
use anyhow::Result;
use std::fmt::Write;
use wasm_ast::{Instruction, NumericInstruction};

/// CountLeadingZeros counts the number of leading zero bits in the specified source slot and places it into the
/// destination_slot.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code]
/// fn count_leading_zeros_i32() -> u32 {
///     [ConstI32::new(0, 1), CountLeadingZeros::new(0, 0), Return::new()]
/// }
/// let func = CountLeadingZerosI32::new().unwrap();
/// assert_eq!(31, func.call().unwrap());
/// ```
///
/// All float -> integer conversions use 64 bit integers
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(unsigned, 0, 0, 1, 0)]
/// fn count_leading_zeros_f32() -> u32 {
///     [ConstF32::new(1, 1.0), CountLeadingZeros::new(1, 0), Return::new()]
/// }
/// let func = CountLeadingZerosF32::new().unwrap();
/// assert_eq!(63, func.call().unwrap());
/// ```
pub struct CountLeadingZeros {
    source: Slot,
    destination: Slot,
}

impl CountLeadingZeros {
    pub fn new(source: Slot, destination: Slot) -> Code {
        Code::CountLeadingZeros(CountLeadingZeros { source, destination })
    }
}

impl CodeBuilder for CountLeadingZeros {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let convert_to = match context.get_slot_value_type(self.source)? {
            ValueType::I32 => ValueType::I32,
            _ => ValueType::I64,
        };
        GetSlotConvert::convert(self.source, convert_to, context, instruction_list)?;
        instruction_list.push(NumericInstruction::CountLeadingZeros(convert_to.into()).into());
        SetSlotConvert::convert(self.destination, convert_to, context, instruction_list)?;

        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}CountLeadingZeros::new({}, {}),",
            indentation, self.source, self.destination
        )
    }
}

/// CountTrailingZeros counts the number of trailing zero bits in the specified source slot and places it into the
/// destination_slot.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code]
/// fn count_trailing_zeros_i32() -> u32 {
///     [ConstI32::new(0, 4), CountTrailingZeros::new(0, 0), Return::new()]
/// }
/// let func = CountTrailingZerosI32::new().unwrap();
/// assert_eq!(2, func.call().unwrap());
/// ```
///
/// All float -> integer conversions use 64 bit integers
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(unsigned, 0, 0, 1, 0)]
/// fn count_trailing_zeros_f32() -> u32 {
///     [ConstF32::new(1, 4.0), CountTrailingZeros::new(1, 0), Return::new()]
/// }
/// let func = CountTrailingZerosF32::new().unwrap();
/// assert_eq!(2, func.call().unwrap());
/// ```
pub struct CountTrailingZeros {
    source: Slot,
    destination: Slot,
}

impl CountTrailingZeros {
    pub fn new(source: Slot, destination: Slot) -> Code {
        Code::CountTrailingZeros(CountTrailingZeros { source, destination })
    }
}

impl CodeBuilder for CountTrailingZeros {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let convert_to = match context.get_slot_value_type(self.source)? {
            ValueType::I32 => ValueType::I32,
            _ => ValueType::I64,
        };
        GetSlotConvert::convert(self.source, convert_to, context, instruction_list)?;
        instruction_list.push(NumericInstruction::CountTrailingZeros(convert_to.into()).into());
        SetSlotConvert::convert(self.destination, convert_to, context, instruction_list)?;

        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}CountTrailingZeros::new({}, {}),",
            indentation, self.source, self.destination
        )
    }
}

/// PopulationCount counts the number of one bits in the specified source slot and places it into the destination_slot.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code]
/// fn population_count_i32() -> u32 {
///     [ConstI32::new(0, 3), PopulationCount::new(0, 0), Return::new()]
/// }
/// let func = PopulationCountI32::new().unwrap();
/// assert_eq!(2, func.call().unwrap());
/// ```
///
/// All float -> integer conversions use 64 bit integers
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(unsigned, 0, 0, 1, 0)]
/// fn population_count_f32() -> u32 {
///     [ConstF32::new(1, 7.1), PopulationCount::new(1, 0), Return::new()]
/// }
/// let func = PopulationCountF32::new().unwrap();
/// assert_eq!(3, func.call().unwrap());
/// ```
pub struct PopulationCount {
    source: Slot,
    destination: Slot,
}

impl PopulationCount {
    pub fn new(source: Slot, destination: Slot) -> Code {
        Code::PopulationCount(PopulationCount { source, destination })
    }
}

impl CodeBuilder for PopulationCount {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let convert_to = match context.get_slot_value_type(self.source)? {
            ValueType::I32 => ValueType::I32,
            _ => ValueType::I64,
        };
        GetSlotConvert::convert(self.source, convert_to, context, instruction_list)?;
        instruction_list.push(NumericInstruction::CountOnes(convert_to.into()).into());
        SetSlotConvert::convert(self.destination, convert_to, context, instruction_list)?;

        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}PopulationCount::new({}, {}),",
            indentation, self.source, self.destination
        )
    }
}

/// And performs the bitwise AND of two source integers and places the result in the destination slot
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(unsigned)]
/// fn and_i32(v1: u32, v2: u32) -> u32 {
///     [And::new(0, 1, 2), Return::new()]
/// }
/// let func = AndI32::new().unwrap();
/// assert_eq!(0, func.call(0, 39).unwrap());
/// assert_eq!(1, func.call(1, 7).unwrap());
/// assert_eq!(2, func.call(3, 2).unwrap());
/// ```
pub struct And {
    left: Slot,
    right: Slot,
    destination: Slot,
}

impl And {
    pub fn new(left: Slot, right: Slot, destination: Slot) -> Code {
        Code::And(And {
            left,
            right,
            destination,
        })
    }
}

impl CodeBuilder for And {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match (
            context.get_slot_value_type(self.left)?,
            context.get_slot_value_type(self.right)?,
        ) {
            (ValueType::I32, ValueType::I32) => ValueType::I32,
            _ => ValueType::I64,
        };
        GetSlotConvert::convert(self.left, operate_as, context, instruction_list)?;
        GetSlotConvert::convert(self.right, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::And(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}And::new({}, {}, {}),",
            indentation, self.left, self.right, self.destination
        )
    }
}

/// Performs the bitwise OR of two source integers and places the result in the destination slot
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(unsigned)]
/// fn or_i32(v1: u32, v2: u32) -> u32 {
///     [Or::new(0, 1, 2), Return::new()]
/// }
/// let func = OrI32::new().unwrap();
/// assert_eq!(39, func.call(0, 39).unwrap());
/// assert_eq!(7, func.call(1, 7).unwrap());
/// assert_eq!(11, func.call(3, 8).unwrap());
/// ```
pub struct Or {
    left: Slot,
    right: Slot,
    destination: Slot,
}

impl Or {
    pub fn new(left: Slot, right: Slot, destination: Slot) -> Code {
        Code::Or(Or {
            left,
            right,
            destination,
        })
    }
}

impl CodeBuilder for Or {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match (
            context.get_slot_value_type(self.left)?,
            context.get_slot_value_type(self.right)?,
        ) {
            (ValueType::I32, ValueType::I32) => ValueType::I32,
            _ => ValueType::I64,
        };
        GetSlotConvert::convert(self.left, operate_as, context, instruction_list)?;
        GetSlotConvert::convert(self.right, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::Or(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}Or::new({}, {}, {}),",
            indentation, self.left, self.right, self.destination
        )
    }
}

/// Performs the bitwise XOR of two source integers and places the result in the destination slot
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(unsigned)]
/// fn xor_i32(v1: u32, v2: u32) -> u32 {
///     [Xor::new(0, 1, 2), Return::new()]
/// }
/// let func = XorI32::new().unwrap();
/// assert_eq!(39, func.call(0, 39).unwrap());
/// assert_eq!(6, func.call(1, 7).unwrap());
/// assert_eq!(1, func.call(3, 2).unwrap());
/// ```
pub struct Xor {
    left: Slot,
    right: Slot,
    destination: Slot,
}

impl Xor {
    pub fn new(left: Slot, right: Slot, destination: Slot) -> Code {
        Code::Xor(Xor {
            left,
            right,
            destination,
        })
    }
}

impl CodeBuilder for Xor {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match (
            context.get_slot_value_type(self.left)?,
            context.get_slot_value_type(self.right)?,
        ) {
            (ValueType::I32, ValueType::I32) => ValueType::I32,
            _ => ValueType::I64,
        };
        GetSlotConvert::convert(self.left, operate_as, context, instruction_list)?;
        GetSlotConvert::convert(self.right, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::Xor(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}Xor::new({}, {}, {}),",
            indentation, self.left, self.right, self.destination
        )
    }
}

/// Performs the bitwise shift left of a source integers by a specific number of bits and places the result in the
///  destination slot
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(unsigned)]
/// fn shift_left_i32(source: u32, bits: u32) -> u32 {
///     [ShiftLeft::new(0, 1, 2), Return::new()]
/// }
/// let func = ShiftLeftI32::new().unwrap();
/// assert_eq!(1, func.call(1, 0).unwrap());
/// assert_eq!(2, func.call(1, 1).unwrap());
/// assert_eq!(4, func.call(2, 1).unwrap());
/// // Large numbers of bits are taken modulo the size of the integer
/// assert_eq!(1, func.call(1, 32).unwrap());
/// assert_eq!(2, func.call(1, 33).unwrap());
/// assert_eq!(4, func.call(2, 33).unwrap());
/// ```
pub struct ShiftLeft {
    source: Slot,
    bits: Slot,
    destination: Slot,
}

impl ShiftLeft {
    pub fn new(source: Slot, bits: Slot, destination: Slot) -> Code {
        Code::ShiftLeft(ShiftLeft {
            source,
            bits,
            destination,
        })
    }
}

impl CodeBuilder for ShiftLeft {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match context.get_slot_value_type(self.source)? {
            ValueType::I32 => ValueType::I32,
            _ => ValueType::I64,
        };
        GetSlotConvert::convert(self.source, operate_as, context, instruction_list)?;
        GetSlotConvert::convert(self.bits, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::ShiftLeft(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}ShiftLeft::new({}, {}, {}),",
            indentation, self.source, self.bits, self.destination
        )
    }
}

/// Performs the bitwise shift right of a source integers by a specific number of bits and places the result in the
/// destination slot. If the context is signed, the signed bit will not be shifted
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(unsigned, 1)]
/// fn shift_right_u32(source: u32, bits: u32) -> u32 {
///     [ShiftRight::new(0, 1, 2), Return::new()]
/// }
/// let func = ShiftRightU32::new().unwrap();
/// assert_eq!(4, func.call(4, 0).unwrap());
/// assert_eq!(2, func.call(4, 1).unwrap());
/// assert_eq!(1, func.call(4, 2).unwrap());
/// assert_eq!(0, func.call(4, 3).unwrap());
/// // Large numbers of bits are taken modulo the size of the integer
/// assert_eq!(4, func.call(4, 32).unwrap());
/// assert_eq!(2, func.call(4, 33).unwrap());
/// assert_eq!(1, func.call(2, 33).unwrap());
/// ```
///
/// Signed code keeps the sign
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn shift_right_i32(source: i32, bits: i32) -> i32 {
///     [ShiftRight::new(0, 1, 2), Return::new()]
/// }
/// let func = ShiftRightI32::new().unwrap();
/// assert_eq!(-4, func.call(-4, 0).unwrap());
/// assert_eq!(-2, func.call(-4, 1).unwrap());
/// assert_eq!(-1, func.call(-4, 2).unwrap());
/// // Negative one shifted right any number of spots is still -1
/// assert_eq!(-1, func.call(-4, 16).unwrap());
/// // Large numbers of bits are taken modulo the size of the integer
/// assert_eq!(-4, func.call(-4, 32).unwrap());
/// assert_eq!(-2, func.call(-4, 33).unwrap());
/// assert_eq!(-1, func.call(-2, 33).unwrap());
/// ```
pub struct ShiftRight {
    source: Slot,
    bits: Slot,
    destination: Slot,
}

impl ShiftRight {
    pub fn new(source: Slot, bits: Slot, destination: Slot) -> Code {
        Code::ShiftRight(ShiftRight {
            source,
            bits,
            destination,
        })
    }
}

impl CodeBuilder for ShiftRight {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match context.get_slot_value_type(self.source)? {
            ValueType::I32 => ValueType::I32,
            _ => ValueType::I64,
        };
        GetSlotConvert::convert(self.source, operate_as, context, instruction_list)?;
        GetSlotConvert::convert(self.bits, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::ShiftRight(operate_as.into(), context.sign_extension()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}ShiftRight::new({}, {}, {}),",
            indentation, self.source, self.bits, self.destination
        )
    }
}

/// Performs the bitwise rotate left of a source integers by a specific number of bits and places the result in the
/// destination slot. Rotate does not take signedness into consideration
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn rotate_left_i32(source: i32, bits: i32) -> i32 {
///     [RotateLeft::new(0, 1, 2), Return::new()]
/// }
/// let func = RotateLeftI32::new().unwrap();
/// assert_eq!(1, func.call(1, 0).unwrap());
/// assert_eq!(2, func.call(1, 1).unwrap());
/// assert_eq!(4, func.call(1, 2).unwrap());
/// assert_eq!(1, func.call(1, 32).unwrap());
/// assert_eq!(i32::MIN, func.call(1, 31).unwrap());
/// ```
pub struct RotateLeft {
    source: Slot,
    bits: Slot,
    destination: Slot,
}

impl RotateLeft {
    pub fn new(source: Slot, bits: Slot, destination: Slot) -> Code {
        Code::RotateLeft(RotateLeft {
            source,
            bits,
            destination,
        })
    }
}

impl CodeBuilder for RotateLeft {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match context.get_slot_value_type(self.source)? {
            ValueType::I32 => ValueType::I32,
            _ => ValueType::I64,
        };
        GetSlotConvert::convert(self.source, operate_as, context, instruction_list)?;
        GetSlotConvert::convert(self.bits, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::RotateLeft(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}RotateLeft::new({}, {}, {}),",
            indentation, self.source, self.bits, self.destination
        )
    }
}

/// Performs the bitwise rotate right of a source integers by a specific number of bits and places the result in the
/// destination slot. Rotate does not take signedness into consideration
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn rotate_right_i32(source: i32, bits: i32) -> i32 {
///     [RotateRight::new(0, 1, 2), Return::new()]
/// }
/// let func = RotateRightI32::new().unwrap();
/// assert_eq!(1, func.call(1, 0).unwrap());
/// assert_eq!(i32::MIN, func.call(1, 1).unwrap());
/// assert_eq!(1, func.call(1, 32).unwrap());
/// assert_eq!(2, func.call(1, 31).unwrap());
/// ```

pub struct RotateRight {
    source: Slot,
    bits: Slot,
    destination: Slot,
}

impl RotateRight {
    pub fn new(source: Slot, bits: Slot, destination: Slot) -> Code {
        Code::RotateRight(RotateRight {
            source,
            bits,
            destination,
        })
    }
}

impl CodeBuilder for RotateRight {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match context.get_slot_value_type(self.source)? {
            ValueType::I32 => ValueType::I32,
            _ => ValueType::I64,
        };
        GetSlotConvert::convert(self.source, operate_as, context, instruction_list)?;
        GetSlotConvert::convert(self.bits, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::RotateRight(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}RotateRight::new({}, {}, {}),",
            indentation, self.source, self.bits, self.destination
        )
    }
}
