use crate::code_builder::CodeBuilder;
use crate::convert::{GetSlotConvert, SetSlotConvert};
use crate::indentation::Indentation;
use crate::*;
use anyhow::Result;
use std::fmt::Write;
use wasm_ast::{Instruction, NumericInstruction};

/// Finds the absolute value of the source number and places it in the destination.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn absolute_value_f32(value: f32) -> f32 {
///     [AbsoluteValue::new(0, 1), Return::new()]
/// }
/// let func = AbsoluteValueF32::new().unwrap();
/// assert_eq!(3.0, func.call(3.0).unwrap());
/// assert_eq!(42.4242, func.call(-42.4242).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn absolute_value_i32(value: i32) -> i32 {
///     [AbsoluteValue::new(0, 1), Return::new()]
/// }
/// let func = AbsoluteValueI32::new().unwrap();
/// assert_eq!(3, func.call(3).unwrap());
/// assert_eq!(42, func.call(-42).unwrap());
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AbsoluteValue {
    source: Slot,
    destination: Slot,
}

impl AbsoluteValue {
    pub fn new(source: Slot, destination: Slot) -> Code {
        Code::AbsoluteValue(AbsoluteValue { source, destination })
    }
}

impl CodeBuilder for AbsoluteValue {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match context.get_slot_value_type(self.source)? {
            ValueType::F32 => ValueType::F32,
            _ => ValueType::F64,
        };
        GetSlotConvert::convert(self.source, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::AbsoluteValue(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}AbsoluteValue::new({}, {}),",
            indentation, self.source, self.destination
        )
    }
}

/// Flips the sign of the source number and places it in the destination.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn negate_f32(value: f32) -> f32 {
///     [Negate::new(0, 1), Return::new()]
/// }
/// let func = NegateF32::new().unwrap();
/// assert_eq!(-3.0, func.call(3.0).unwrap());
/// assert_eq!(42.4242, func.call(-42.4242).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn negate_i32(value: i32) -> i32 {
///     [Negate::new(0, 1), Return::new()]
/// }
/// let func = NegateI32::new().unwrap();
/// assert_eq!(-3, func.call(3).unwrap());
/// assert_eq!(42, func.call(-42).unwrap());
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Negate {
    source: Slot,
    destination: Slot,
}

impl Negate {
    pub fn new(source: Slot, destination: Slot) -> Code {
        Code::Negate(Negate { source, destination })
    }
}

impl CodeBuilder for Negate {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match context.get_slot_value_type(self.source)? {
            ValueType::F32 => ValueType::F32,
            _ => ValueType::F64,
        };
        GetSlotConvert::convert(self.source, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::Negate(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(f, "{}Negate::new({}, {}),", indentation, self.source, self.destination)
    }
}

/// Calculates the square root of the absolute value of the source number and places it in the destination.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn square_root_f32(value: f32) -> f32 {
///     [SquareRoot::new(0, 1), Return::new()]
/// }
/// let func = SquareRootF32::new().unwrap();
/// assert_eq!(3.0, func.call(9.0).unwrap());
/// assert_eq!(2.2, func.call(4.84).unwrap());
/// // Negative numbers are taken absolute value so that genetic code can operate without error
/// assert_eq!(5.0, func.call(-25.0).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn square_root_i32(value: i32) -> i32 {
///     [SquareRoot::new(0, 1), Return::new()]
/// }
/// let func = SquareRootI32::new().unwrap();
/// assert_eq!(3, func.call(9).unwrap());
/// // Negative numbers are taken absolute value so that genetic code can operate without error
/// assert_eq!(5, func.call(-25).unwrap());
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SquareRoot {
    source: Slot,
    destination: Slot,
}

impl SquareRoot {
    pub fn new(source: Slot, destination: Slot) -> Code {
        Code::SquareRoot(SquareRoot { source, destination })
    }
}

impl CodeBuilder for SquareRoot {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match context.get_slot_value_type(self.source)? {
            ValueType::F32 => ValueType::F32,
            _ => ValueType::F64,
        };
        GetSlotConvert::convert(self.source, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::AbsoluteValue(operate_as.into()).into());
        instruction_list.push(NumericInstruction::SquareRoot(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}SquareRoot::new({}, {}),",
            indentation, self.source, self.destination
        )
    }
}

/// Rounds the source number up to the next whole number and places it in the destination.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn ceiling_f32(value: f32) -> f32 {
///     [Ceiling::new(0, 1), Return::new()]
/// }
/// let func = CeilingF32::new().unwrap();
/// assert_eq!(3.0, func.call(2.1).unwrap());
/// assert_eq!(3.0, func.call(2.5).unwrap());
/// assert_eq!(3.0, func.call(2.9).unwrap());
/// assert_eq!(-2.0, func.call(-2.5).unwrap());
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Ceiling {
    source: Slot,
    destination: Slot,
}

impl Ceiling {
    pub fn new(source: Slot, destination: Slot) -> Code {
        Code::Ceiling(Ceiling { source, destination })
    }
}

impl CodeBuilder for Ceiling {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match context.get_slot_value_type(self.source)? {
            ValueType::F32 => ValueType::F32,
            _ => ValueType::F64,
        };
        GetSlotConvert::convert(self.source, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::Ceiling(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(f, "{}Ceiling::new({}, {}),", indentation, self.source, self.destination)
    }
}

/// Rounds the source number down to the next whole number and places it in the destination.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn floor_f32(value: f32) -> f32 {
///     [Floor::new(0, 1), Return::new()]
/// }
/// let func = FloorF32::new().unwrap();
/// assert_eq!(2.0, func.call(2.1).unwrap());
/// assert_eq!(2.0, func.call(2.5).unwrap());
/// assert_eq!(2.0, func.call(2.9).unwrap());
/// assert_eq!(-3.0, func.call(-2.5).unwrap());
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Floor {
    source: Slot,
    destination: Slot,
}

impl Floor {
    pub fn new(source: Slot, destination: Slot) -> Code {
        Code::Floor(Floor { source, destination })
    }
}

impl CodeBuilder for Floor {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match context.get_slot_value_type(self.source)? {
            ValueType::F32 => ValueType::F32,
            _ => ValueType::F64,
        };
        GetSlotConvert::convert(self.source, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::Floor(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(f, "{}Floor::new({}, {}),", indentation, self.source, self.destination)
    }
}

/// Rounds the source number to the nearest whole number and places it in the destination. Ties go to even numbers
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn nearest_f32(value: f32) -> f32 {
///     [Nearest::new(0, 1), Return::new()]
/// }
/// let func = NearestF32::new().unwrap();
/// assert_eq!(2.0, func.call(2.1).unwrap());
/// assert_eq!(3.0, func.call(2.9).unwrap());
/// // Ties go to even numbers
/// assert_eq!(2.0, func.call(2.5).unwrap());
/// assert_eq!(4.0, func.call(3.5).unwrap());
/// assert_eq!(-2.0, func.call(-2.5).unwrap());
/// assert_eq!(-4.0, func.call(-3.5).unwrap());
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Nearest {
    source: Slot,
    destination: Slot,
}

impl Nearest {
    pub fn new(source: Slot, destination: Slot) -> Code {
        Code::Nearest(Nearest { source, destination })
    }
}

impl CodeBuilder for Nearest {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match context.get_slot_value_type(self.source)? {
            ValueType::F32 => ValueType::F32,
            _ => ValueType::F64,
        };
        GetSlotConvert::convert(self.source, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::Nearest(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(f, "{}Nearest::new({}, {}),", indentation, self.source, self.destination)
    }
}

/// Returns the lesser of two source numbers and places it in the destination.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn min_f32(left: f32, right: f32) -> f32 {
///     [Min::new(0, 1, 2), Return::new()]
/// }
/// let func = MinF32::new().unwrap();
/// assert_eq!(3.0, func.call(3.0, 9.0).unwrap());
/// assert_eq!(2.2, func.call(4.84, 2.2).unwrap());
/// assert_eq!(-25.0, func.call(-25.0, 25.0).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn min_i32(left: i32, right: i32) -> i32 {
///     [Min::new(0, 1, 2), Return::new()]
/// }
/// let func = MinI32::new().unwrap();
/// assert_eq!(3, func.call(3, 9).unwrap());
/// assert_eq!(-25, func.call(25, -25).unwrap());
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Min {
    left: Slot,
    right: Slot,
    destination: Slot,
}

impl Min {
    pub fn new(left: Slot, right: Slot, destination: Slot) -> Code {
        Code::Min(Min {
            left,
            right,
            destination,
        })
    }
}

impl CodeBuilder for Min {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match (
            context.get_slot_value_type(self.left)?,
            context.get_slot_value_type(self.right)?,
        ) {
            (ValueType::F32, ValueType::F32) => ValueType::F32,
            _ => ValueType::F64,
        };
        GetSlotConvert::convert(self.left, operate_as, context, instruction_list)?;
        GetSlotConvert::convert(self.right, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::Minimum(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}Min::new({}, {}, {}),",
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
/// fn max_f32(left: f32, right: f32) -> f32 {
///     [Max::new(0, 1, 2), Return::new()]
/// }
/// let func = MaxF32::new().unwrap();
/// assert_eq!(9.0, func.call(3.0, 9.0).unwrap());
/// assert_eq!(4.84, func.call(4.84, 2.2).unwrap());
/// assert_eq!(25.0, func.call(-25.0, 25.0).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn max_i32(left: i32, right: i32) -> i32 {
///     [Max::new(0, 1, 2), Return::new()]
/// }
/// let func = MaxI32::new().unwrap();
/// assert_eq!(9, func.call(3, 9).unwrap());
/// assert_eq!(25, func.call(25, -25).unwrap());
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Max {
    left: Slot,
    right: Slot,
    destination: Slot,
}

impl Max {
    pub fn new(left: Slot, right: Slot, destination: Slot) -> Code {
        Code::Max(Max {
            left,
            right,
            destination,
        })
    }
}

impl CodeBuilder for Max {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match (
            context.get_slot_value_type(self.left)?,
            context.get_slot_value_type(self.right)?,
        ) {
            (ValueType::F32, ValueType::F32) => ValueType::F32,
            _ => ValueType::F64,
        };
        GetSlotConvert::convert(self.left, operate_as, context, instruction_list)?;
        GetSlotConvert::convert(self.right, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::Maximum(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}Max::new({}, {}, {}),",
            indentation, self.left, self.right, self.destination
        )
    }
}

/// Returns the value of the first source, but having the sign of the second and places it in the destination.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn copy_sign_f32(left: f32, right: f32) -> f32 {
///     [CopySign::new(0, 1, 2), Return::new()]
/// }
/// let func = CopySignF32::new().unwrap();
/// assert_eq!(3.0, func.call(3.0, 9.0).unwrap());
/// assert_eq!(-4.84, func.call(4.84, -2.2).unwrap());
/// assert_eq!(25.0, func.call(-25.0, 25.0).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn copy_sign_i32(left: i32, right: i32) -> i32 {
///     [CopySign::new(0, 1, 2), Return::new()]
/// }
/// let func = CopySignI32::new().unwrap();
/// assert_eq!(3, func.call(3, 9).unwrap());
/// assert_eq!(3, func.call(-3, 9).unwrap());
/// assert_eq!(-25, func.call(25, -25).unwrap());
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CopySign {
    left: Slot,
    right: Slot,
    destination: Slot,
}

impl CopySign {
    pub fn new(left: Slot, right: Slot, destination: Slot) -> Code {
        Code::CopySign(CopySign {
            left,
            right,
            destination,
        })
    }
}

impl CodeBuilder for CopySign {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match (
            context.get_slot_value_type(self.left)?,
            context.get_slot_value_type(self.right)?,
        ) {
            (ValueType::F32, ValueType::F32) => ValueType::F32,
            _ => ValueType::F64,
        };
        GetSlotConvert::convert(self.left, operate_as, context, instruction_list)?;
        GetSlotConvert::convert(self.right, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::CopySign(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}CopySign::new({}, {}, {}),",
            indentation, self.left, self.right, self.destination
        )
    }
}
