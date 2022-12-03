use crate::code_builder::CodeBuilder;
use crate::convert::{GetSlotConvert, SetSlotConvert};
use crate::indentation::Indentation;
use crate::*;
use anyhow::Result;
use std::fmt::Write;
use wasm_ast::{Instruction, NumericInstruction};

/// Returns the greater of two source numbers and places it in the destination.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn is_equal_zero_f32(value: f32) -> i32 {
///     [IsEqualZero::new(0, 1), Return::new()]
/// }
/// let func = IsEqualZeroF32::new().unwrap();
/// assert_eq!(0, func.call(3.3).unwrap());
/// assert_eq!(1, func.call(0.0).unwrap());
/// assert_eq!(0, func.call(0.1).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn is_equal_zero_i32(value: i32) -> i32 {
///     [IsEqualZero::new(0, 1), Return::new()]
/// }
/// let func = IsEqualZeroI32::new().unwrap();
/// assert_eq!(0, func.call(3).unwrap());
/// assert_eq!(1, func.call(0).unwrap());
/// assert_eq!(0, func.call(2).unwrap());
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IsEqualZero {
    source: Slot,
    destination: Slot,
}

impl IsEqualZero {
    pub fn new(source: Slot, destination: Slot) -> Code {
        Code::IsEqualZero(IsEqualZero { source, destination })
    }
}

impl CodeBuilder for IsEqualZero {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = context.get_slot_value_type(self.source)?;
        GetSlotConvert::convert(self.source, operate_as, context, instruction_list)?;
        match &operate_as {
            ValueType::F32 => {
                instruction_list.push(NumericInstruction::F32Constant(0.0).into());
                instruction_list.push(NumericInstruction::Equal(wasm_ast::NumberType::F32).into());
            }
            ValueType::F64 => {
                instruction_list.push(NumericInstruction::F64Constant(0.0).into());
                instruction_list.push(NumericInstruction::Equal(wasm_ast::NumberType::F64).into());
            }
            _ => instruction_list.push(NumericInstruction::EqualToZero(operate_as.into()).into()),
        }
        SetSlotConvert::convert(self.destination, ValueType::I32, context, instruction_list)?;
        Ok(())
    }

    fn make_random_code(&self, engine: &mut GeneticEngine, _max_points: usize) -> Code {
        IsEqualZero::new(engine.random_slot(), engine.random_slot())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}IsEqualZero::new({}, {}),",
            indentation, self.source, self.destination
        )
    }
}

/// Returns the greater of two source numbers and places it in the destination.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn are_equal_f32(left: f32, right: f32) -> i32 {
///     [AreEqual::new(0, 1, 2), Return::new()]
/// }
/// let func = AreEqualF32::new().unwrap();
/// assert_eq!(1, func.call(3.3, 3.3).unwrap());
/// assert_eq!(0, func.call(4.84, 4.82).unwrap());
/// assert_eq!(0, func.call(4.84, 4.85).unwrap());
/// assert_eq!(0, func.call(-5.0, -6.0).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn are_equal_i32(left: i32, right: i32) -> i32 {
///     [AreEqual::new(0, 1, 2), Return::new()]
/// }
/// let func = AreEqualI32::new().unwrap();
/// assert_eq!(1, func.call(3, 3).unwrap());
/// assert_eq!(0, func.call(3, 4).unwrap());
/// assert_eq!(0, func.call(3, 2).unwrap());
/// assert_eq!(0, func.call(-3, -2).unwrap());
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AreEqual {
    left: Slot,
    right: Slot,
    destination: Slot,
}

impl AreEqual {
    pub fn new(left: Slot, right: Slot, destination: Slot) -> Code {
        Code::AreEqual(AreEqual {
            left,
            right,
            destination,
        })
    }
}

impl CodeBuilder for AreEqual {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match (
            context.get_slot_value_type(self.left)?,
            context.get_slot_value_type(self.right)?,
        ) {
            (ValueType::I32, ValueType::I32) => ValueType::I32,
            (ValueType::I32, ValueType::I64) => ValueType::I64,
            (ValueType::I64, ValueType::I32) => ValueType::I64,
            (ValueType::I64, ValueType::I64) => ValueType::I64,
            (ValueType::F32, ValueType::F32) => ValueType::F32,
            _ => ValueType::F64,
        };
        GetSlotConvert::convert(self.left, operate_as, context, instruction_list)?;
        GetSlotConvert::convert(self.right, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::Equal(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, ValueType::I32, context, instruction_list)?;
        Ok(())
    }

    fn make_random_code(&self, engine: &mut GeneticEngine, _max_points: usize) -> Code {
        AreEqual::new(engine.random_slot(), engine.random_slot(), engine.random_slot())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}AreEqual::new({}, {}, {}),",
            indentation, self.left, self.right, self.destination
        )
    }
}

/// Returns the greater of two source numbers and places it in the destination.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn are_not_equal_f32(left: f32, right: f32) -> i32 {
///     [AreNotEqual::new(0, 1, 2), Return::new()]
/// }
/// let func = AreNotEqualF32::new().unwrap();
/// assert_eq!(0, func.call(3.3, 3.3).unwrap());
/// assert_eq!(1, func.call(4.84, 4.82).unwrap());
/// assert_eq!(1, func.call(4.84, 4.85).unwrap());
/// assert_eq!(1, func.call(-5.0, -6.0).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn are_not_equal_i32(left: i32, right: i32) -> i32 {
///     [AreNotEqual::new(0, 1, 2), Return::new()]
/// }
/// let func = AreNotEqualI32::new().unwrap();
/// assert_eq!(0, func.call(3, 3).unwrap());
/// assert_eq!(1, func.call(3, 4).unwrap());
/// assert_eq!(1, func.call(3, 2).unwrap());
/// assert_eq!(1, func.call(-3, -2).unwrap());
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AreNotEqual {
    left: Slot,
    right: Slot,
    destination: Slot,
}

impl AreNotEqual {
    pub fn new(left: Slot, right: Slot, destination: Slot) -> Code {
        Code::AreNotEqual(AreNotEqual {
            left,
            right,
            destination,
        })
    }
}

impl CodeBuilder for AreNotEqual {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match (
            context.get_slot_value_type(self.left)?,
            context.get_slot_value_type(self.right)?,
        ) {
            (ValueType::I32, ValueType::I32) => ValueType::I32,
            (ValueType::I32, ValueType::I64) => ValueType::I64,
            (ValueType::I64, ValueType::I32) => ValueType::I64,
            (ValueType::I64, ValueType::I64) => ValueType::I64,
            (ValueType::F32, ValueType::F32) => ValueType::F32,
            _ => ValueType::F64,
        };
        GetSlotConvert::convert(self.left, operate_as, context, instruction_list)?;
        GetSlotConvert::convert(self.right, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::NotEqual(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, ValueType::I32, context, instruction_list)?;
        Ok(())
    }

    fn make_random_code(&self, engine: &mut GeneticEngine, _max_points: usize) -> Code {
        AreNotEqual::new(engine.random_slot(), engine.random_slot(), engine.random_slot())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}AreNotEqual::new({}, {}, {}),",
            indentation, self.left, self.right, self.destination
        )
    }
}

/// Returns the greater of two source numbers and places it in the destination.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn is_less_than_f32(left: f32, right: f32) -> i32 {
///     [IsLessThan::new(0, 1, 2), Return::new()]
/// }
/// let func = IsLessThanF32::new().unwrap();
/// assert_eq!(0, func.call(3.3, 3.3).unwrap());
/// assert_eq!(0, func.call(4.84, 4.82).unwrap());
/// assert_eq!(1, func.call(4.84, 4.85).unwrap());
/// assert_eq!(0, func.call(-5.0, -6.0).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn is_less_than_i32(left: i32, right: i32) -> i32 {
///     [IsLessThan::new(0, 1, 2), Return::new()]
/// }
/// let func = IsLessThanI32::new().unwrap();
/// assert_eq!(0, func.call(3, 3).unwrap());
/// assert_eq!(1, func.call(3, 4).unwrap());
/// assert_eq!(0, func.call(3, 2).unwrap());
/// assert_eq!(1, func.call(-3, -2).unwrap());
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IsLessThan {
    left: Slot,
    right: Slot,
    destination: Slot,
}

impl IsLessThan {
    pub fn new(left: Slot, right: Slot, destination: Slot) -> Code {
        Code::IsLessThan(IsLessThan {
            left,
            right,
            destination,
        })
    }
}

impl CodeBuilder for IsLessThan {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match (
            context.get_slot_value_type(self.left)?,
            context.get_slot_value_type(self.right)?,
        ) {
            (ValueType::I32, ValueType::I32) => ValueType::I32,
            (ValueType::I32, ValueType::I64) => ValueType::I64,
            (ValueType::I64, ValueType::I32) => ValueType::I64,
            (ValueType::I64, ValueType::I64) => ValueType::I64,
            (ValueType::F32, ValueType::F32) => ValueType::F32,
            _ => ValueType::F64,
        };
        GetSlotConvert::convert(self.left, operate_as, context, instruction_list)?;
        GetSlotConvert::convert(self.right, operate_as, context, instruction_list)?;
        if operate_as.is_float() {
            instruction_list.push(NumericInstruction::LessThanFloat(operate_as.into()).into());
        } else {
            instruction_list
                .push(NumericInstruction::LessThanInteger(operate_as.into(), context.sign_extension()).into());
        }
        SetSlotConvert::convert(self.destination, ValueType::I32, context, instruction_list)?;
        Ok(())
    }

    fn make_random_code(&self, engine: &mut GeneticEngine, _max_points: usize) -> Code {
        IsLessThan::new(engine.random_slot(), engine.random_slot(), engine.random_slot())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}IsLessThan::new({}, {}, {}),",
            indentation, self.left, self.right, self.destination
        )
    }
}

/// Returns the greater of two source numbers and places it in the destination.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn is_greater_than_f32(left: f32, right: f32) -> i32 {
///     [IsGreaterThan::new(0, 1, 2), Return::new()]
/// }
/// let func = IsGreaterThanF32::new().unwrap();
/// assert_eq!(0, func.call(3.3, 3.3).unwrap());
/// assert_eq!(1, func.call(4.84, 4.82).unwrap());
/// assert_eq!(0, func.call(4.84, 4.85).unwrap());
/// assert_eq!(1, func.call(-5.0, -6.0).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn is_greater_than_i32(left: i32, right: i32) -> i32 {
///     [IsGreaterThan::new(0, 1, 2), Return::new()]
/// }
/// let func = IsGreaterThanI32::new().unwrap();
/// assert_eq!(0, func.call(3, 3).unwrap());
/// assert_eq!(0, func.call(3, 4).unwrap());
/// assert_eq!(1, func.call(3, 2).unwrap());
/// assert_eq!(0, func.call(-3, -2).unwrap());
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IsGreaterThan {
    left: Slot,
    right: Slot,
    destination: Slot,
}

impl IsGreaterThan {
    pub fn new(left: Slot, right: Slot, destination: Slot) -> Code {
        Code::IsGreaterThan(IsGreaterThan {
            left,
            right,
            destination,
        })
    }
}

impl CodeBuilder for IsGreaterThan {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match (
            context.get_slot_value_type(self.left)?,
            context.get_slot_value_type(self.right)?,
        ) {
            (ValueType::I32, ValueType::I32) => ValueType::I32,
            (ValueType::I32, ValueType::I64) => ValueType::I64,
            (ValueType::I64, ValueType::I32) => ValueType::I64,
            (ValueType::I64, ValueType::I64) => ValueType::I64,
            (ValueType::F32, ValueType::F32) => ValueType::F32,
            _ => ValueType::F64,
        };
        GetSlotConvert::convert(self.left, operate_as, context, instruction_list)?;
        GetSlotConvert::convert(self.right, operate_as, context, instruction_list)?;
        if operate_as.is_float() {
            instruction_list.push(NumericInstruction::GreaterThanFloat(operate_as.into()).into());
        } else {
            instruction_list
                .push(NumericInstruction::GreaterThanInteger(operate_as.into(), context.sign_extension()).into());
        }
        SetSlotConvert::convert(self.destination, ValueType::I32, context, instruction_list)?;
        Ok(())
    }

    fn make_random_code(&self, engine: &mut GeneticEngine, _max_points: usize) -> Code {
        IsGreaterThan::new(engine.random_slot(), engine.random_slot(), engine.random_slot())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}IsGreaterThan::new({}, {}, {}),",
            indentation, self.left, self.right, self.destination
        )
    }
}

/// Returns the greater of two source numbers and places it in the destination.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn is_less_than_or_equal_f32(left: f32, right: f32) -> i32 {
///     [IsLessThanOrEqual::new(0, 1, 2), Return::new()]
/// }
/// let func = IsLessThanOrEqualF32::new().unwrap();
/// assert_eq!(1, func.call(3.3, 3.3).unwrap());
/// assert_eq!(0, func.call(4.84, 4.82).unwrap());
/// assert_eq!(1, func.call(4.84, 4.85).unwrap());
/// assert_eq!(0, func.call(-5.0, -6.0).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn is_less_than_or_equal_i32(left: i32, right: i32) -> i32 {
///     [IsLessThanOrEqual::new(0, 1, 2), Return::new()]
/// }
/// let func = IsLessThanOrEqualI32::new().unwrap();
/// assert_eq!(1, func.call(3, 3).unwrap());
/// assert_eq!(1, func.call(3, 4).unwrap());
/// assert_eq!(0, func.call(3, 2).unwrap());
/// assert_eq!(1, func.call(-3, -2).unwrap());
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IsLessThanOrEqual {
    left: Slot,
    right: Slot,
    destination: Slot,
}

impl IsLessThanOrEqual {
    pub fn new(left: Slot, right: Slot, destination: Slot) -> Code {
        Code::IsLessThanOrEqual(IsLessThanOrEqual {
            left,
            right,
            destination,
        })
    }
}

impl CodeBuilder for IsLessThanOrEqual {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match (
            context.get_slot_value_type(self.left)?,
            context.get_slot_value_type(self.right)?,
        ) {
            (ValueType::I32, ValueType::I32) => ValueType::I32,
            (ValueType::I32, ValueType::I64) => ValueType::I64,
            (ValueType::I64, ValueType::I32) => ValueType::I64,
            (ValueType::I64, ValueType::I64) => ValueType::I64,
            (ValueType::F32, ValueType::F32) => ValueType::F32,
            _ => ValueType::F64,
        };
        GetSlotConvert::convert(self.left, operate_as, context, instruction_list)?;
        GetSlotConvert::convert(self.right, operate_as, context, instruction_list)?;
        if operate_as.is_float() {
            instruction_list.push(NumericInstruction::LessThanOrEqualToFloat(operate_as.into()).into());
        } else {
            instruction_list
                .push(NumericInstruction::LessThanOrEqualToInteger(operate_as.into(), context.sign_extension()).into());
        }
        SetSlotConvert::convert(self.destination, ValueType::I32, context, instruction_list)?;
        Ok(())
    }

    fn make_random_code(&self, engine: &mut GeneticEngine, _max_points: usize) -> Code {
        IsLessThanOrEqual::new(engine.random_slot(), engine.random_slot(), engine.random_slot())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}IsLessThanOrEqual::new({}, {}, {}),",
            indentation, self.left, self.right, self.destination
        )
    }
}

/// Returns the greater of two source numbers and places it in the destination.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn is_greater_than_or_equal_f32(left: f32, right: f32) -> i32 {
///     [IsGreaterThanOrEqual::new(0, 1, 2), Return::new()]
/// }
/// let func = IsGreaterThanOrEqualF32::new().unwrap();
/// assert_eq!(1, func.call(3.3, 3.3).unwrap());
/// assert_eq!(1, func.call(4.84, 4.82).unwrap());
/// assert_eq!(0, func.call(4.84, 4.85).unwrap());
/// assert_eq!(1, func.call(-5.0, -6.0).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn is_greater_than_or_equal_i32(left: i32, right: i32) -> i32 {
///     [IsGreaterThanOrEqual::new(0, 1, 2), Return::new()]
/// }
/// let func = IsGreaterThanOrEqualI32::new().unwrap();
/// assert_eq!(1, func.call(3, 3).unwrap());
/// assert_eq!(0, func.call(3, 4).unwrap());
/// assert_eq!(1, func.call(3, 2).unwrap());
/// assert_eq!(0, func.call(-3, -2).unwrap());
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IsGreaterThanOrEqual {
    left: Slot,
    right: Slot,
    destination: Slot,
}

impl IsGreaterThanOrEqual {
    pub fn new(left: Slot, right: Slot, destination: Slot) -> Code {
        Code::IsGreaterThanOrEqual(IsGreaterThanOrEqual {
            left,
            right,
            destination,
        })
    }
}

impl CodeBuilder for IsGreaterThanOrEqual {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match (
            context.get_slot_value_type(self.left)?,
            context.get_slot_value_type(self.right)?,
        ) {
            (ValueType::I32, ValueType::I32) => ValueType::I32,
            (ValueType::I32, ValueType::I64) => ValueType::I64,
            (ValueType::I64, ValueType::I32) => ValueType::I64,
            (ValueType::I64, ValueType::I64) => ValueType::I64,
            (ValueType::F32, ValueType::F32) => ValueType::F32,
            _ => ValueType::F64,
        };
        GetSlotConvert::convert(self.left, operate_as, context, instruction_list)?;
        GetSlotConvert::convert(self.right, operate_as, context, instruction_list)?;
        if operate_as.is_float() {
            instruction_list.push(NumericInstruction::GreaterThanOrEqualToFloat(operate_as.into()).into());
        } else {
            instruction_list.push(
                NumericInstruction::GreaterThanOrEqualToInteger(operate_as.into(), context.sign_extension()).into(),
            );
        }
        SetSlotConvert::convert(self.destination, ValueType::I32, context, instruction_list)?;
        Ok(())
    }

    fn make_random_code(&self, engine: &mut GeneticEngine, _max_points: usize) -> Code {
        IsGreaterThanOrEqual::new(engine.random_slot(), engine.random_slot(), engine.random_slot())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}IsGreaterThanOrEqual::new({}, {}, {}),",
            indentation, self.left, self.right, self.destination
        )
    }
}
