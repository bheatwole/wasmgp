use crate::code_builder::CodeBuilder;
use crate::convert::{GetSlotConvert, SetSlotConvert};
use crate::*;
use anyhow::Result;
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
///     [ConstI32::new(0, 1), CountLeadingZeros::new(0, 0), Code::Return]
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
///     [ConstF32::new(1, 1.0), CountLeadingZeros::new(1, 0), Code::Return]
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
///     [ConstI32::new(0, 4), CountTrailingZeros::new(0, 0), Code::Return]
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
///     [ConstF32::new(1, 4.0), CountTrailingZeros::new(1, 0), Code::Return]
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
}

/// PopulationCount counts the number of one bits in the specified source slot and places it into the destination_slot.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code]
/// fn population_count_i32() -> u32 {
///     [ConstI32::new(0, 3), PopulationCount::new(0, 0), Code::Return]
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
///     [ConstF32::new(1, 7.1), PopulationCount::new(1, 0), Code::Return]
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
}

/// And performs the bitwise AND of two source integers and places the result in the destination slot
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(unsigned, 1)]
/// fn and_i32(v1: u32, v2: u32) -> u32 {
///     [And::new(0, 1, 2), Code::Return]
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
}

/// Performs the bitwise OR of two source integers and places the result in the destination slot
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(unsigned, 1)]
/// fn or_i32(v1: u32, v2: u32) -> u32 {
///     [Or::new(0, 1, 2), Code::Return]
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
}

/// Performs the bitwise XOR of two source integers and places the result in the destination slot
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(unsigned, 1)]
/// fn xor_i32(v1: u32, v2: u32) -> u32 {
///     [Xor::new(0, 1, 2), Code::Return]
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
}

pub struct ShiftLeft {
    left: Slot,
    right: Slot,
    destination: Slot,
}

impl ShiftLeft {
    pub fn new(left: Slot, right: Slot, destination: Slot) -> Code {
        Code::ShiftLeft(ShiftLeft {
            left,
            right,
            destination,
        })
    }
}

impl CodeBuilder for ShiftLeft {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        Ok(())
    }
}

pub struct ShiftRight {
    left: Slot,
    right: Slot,
    destination: Slot,
}

impl ShiftRight {
    pub fn new(left: Slot, right: Slot, destination: Slot) -> Code {
        Code::ShiftRight(ShiftRight {
            left,
            right,
            destination,
        })
    }
}

impl CodeBuilder for ShiftRight {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        Ok(())
    }
}

pub struct RotateLeft {
    left: Slot,
    right: Slot,
    destination: Slot,
}

impl RotateLeft {
    pub fn new(left: Slot, right: Slot, destination: Slot) -> Code {
        Code::RotateLeft(RotateLeft {
            left,
            right,
            destination,
        })
    }
}

impl CodeBuilder for RotateLeft {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        Ok(())
    }
}

pub struct RotateRight {
    left: Slot,
    right: Slot,
    destination: Slot,
}

impl RotateRight {
    pub fn new(left: Slot, right: Slot, destination: Slot) -> Code {
        Code::RotateRight(RotateRight {
            left,
            right,
            destination,
        })
    }
}

impl CodeBuilder for RotateRight {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        Ok(())
    }
}
