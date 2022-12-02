use crate::code_builder::CodeBuilder;
use crate::convert::{GetSlotConvert, SetSlotConvert};
use crate::indentation::Indentation;
use crate::*;
use anyhow::Result;
use std::fmt::Write;
use wasm_ast::{BlockType, ControlInstruction, Expression, Instruction, NumericInstruction};

/// Adds the values in the `left` and `right` slots, placing the results in the `destination` slot. All operands are
/// converted to the type of the result before the operation.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn add_i32(v1: i32, v2: i32) -> i32 {
///     [Add::new(0, 1, 2), Return::new()]
/// }
/// let func = AddI32::new().unwrap();
/// assert_eq!(39, func.call(0, 39).unwrap());
/// assert_eq!(8, func.call(1, 7).unwrap());
/// assert_eq!(1, func.call(3, -2).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn add_f32(v1: f32, v2: f32) -> u32 {
///     [Add::new(0, 1, 2), Return::new()]
/// }
/// let func = AddF32::new().unwrap();
/// assert_eq!(39, func.call(0.0, 39.0).unwrap());
/// assert_eq!(8, func.call(1.0, 7.0).unwrap());
/// assert_eq!(1, func.call(3.0, -2.0).unwrap());
/// // Fractions are truncated before operation
/// assert_eq!(30, func.call(15.5, 15.5).unwrap());
/// ```
#[derive(Default, PartialEq)]
pub struct Add {
    left: Slot,
    right: Slot,
    destination: Slot,
}

impl Add {
    pub fn new(left: Slot, right: Slot, destination: Slot) -> Code {
        Code::Add(Add {
            left,
            right,
            destination,
        })
    }
}

impl CodeBuilder for Add {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = context.get_slot_value_type(self.destination)?;
        GetSlotConvert::convert(self.left, operate_as, context, instruction_list)?;
        GetSlotConvert::convert(self.right, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::Add(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}Add::new({}, {}, {}),",
            indentation, self.left, self.right, self.destination
        )
    }
}

/// Subtracts the `right` value from the `left`, placing the results in the `destination` slot. All operands are
/// converted to the type of the result before the operation.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn subtract_i32(v1: i32, v2: i32) -> i32 {
///     [Subtract::new(0, 1, 2), Return::new()]
/// }
/// let func = SubtractI32::new().unwrap();
/// assert_eq!(-39, func.call(0, 39).unwrap());
/// assert_eq!(6, func.call(7, 1).unwrap());
/// assert_eq!(5, func.call(3, -2).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn subtract_f32(v1: f32, v2: f32) -> i32 {
///     [Subtract::new(0, 1, 2), Return::new()]
/// }
/// let func = SubtractF32::new().unwrap();
/// assert_eq!(-39, func.call(0.0, 39.0).unwrap());
/// assert_eq!(-6, func.call(1.0, 7.0).unwrap());
/// assert_eq!(5, func.call(3.0, -2.0).unwrap());
/// // Fractions are truncated before operation
/// assert_eq!(5, func.call(15.999, 10.999).unwrap());
/// ```
#[derive(Default, PartialEq)]
pub struct Subtract {
    left: Slot,
    right: Slot,
    destination: Slot,
}

impl Subtract {
    pub fn new(left: Slot, right: Slot, destination: Slot) -> Code {
        Code::Subtract(Subtract {
            left,
            right,
            destination,
        })
    }
}

impl CodeBuilder for Subtract {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = context.get_slot_value_type(self.destination)?;
        GetSlotConvert::convert(self.left, operate_as, context, instruction_list)?;
        GetSlotConvert::convert(self.right, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::Subtract(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}Subtract::new({}, {}, {}),",
            indentation, self.left, self.right, self.destination
        )
    }
}

/// Multiplies the values in the `left` and `right` slots, placing the results in the `destination` slot. All operands
/// are converted to the type of the result before the operation.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn multiply_i32(v1: i32, v2: i32) -> i32 {
///     [Multiply::new(0, 1, 2), Return::new()]
/// }
/// let func = MultiplyI32::new().unwrap();
/// assert_eq!(0, func.call(0, 39).unwrap());
/// assert_eq!(7, func.call(1, 7).unwrap());
/// assert_eq!(-6, func.call(3, -2).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn multiply_f32(v1: f32, v2: f32) -> i32 {
///     [Multiply::new(0, 1, 2), Return::new()]
/// }
/// let func = MultiplyF32::new().unwrap();
/// assert_eq!(0, func.call(0.0, 39.0).unwrap());
/// assert_eq!(7, func.call(1.0, 7.0).unwrap());
/// assert_eq!(-6, func.call(3.0, -2.0).unwrap());
/// // Fractions are truncated before operation
/// assert_eq!(225, func.call(15.5, 15.5).unwrap());
/// ```
#[derive(Default, PartialEq)]
pub struct Multiply {
    left: Slot,
    right: Slot,
    destination: Slot,
}

impl Multiply {
    pub fn new(left: Slot, right: Slot, destination: Slot) -> Code {
        Code::Multiply(Multiply {
            left,
            right,
            destination,
        })
    }
}

impl CodeBuilder for Multiply {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = context.get_slot_value_type(self.destination)?;
        GetSlotConvert::convert(self.left, operate_as, context, instruction_list)?;
        GetSlotConvert::convert(self.right, operate_as, context, instruction_list)?;
        instruction_list.push(NumericInstruction::Multiply(operate_as.into()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, instruction_list)?;
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}Multiply::new({}, {}, {}),",
            indentation, self.left, self.right, self.destination
        )
    }
}

/// Divides the `dividend` value by the `divisor`, and places the results in the `destination` slot. All operands are
/// converted to the type of the result before the operation.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn divide_i32(v1: i32, v2: i32) -> i32 {
///     [Divide::new(0, 1, 2), Return::new()]
/// }
/// let func = DivideI32::new().unwrap();
/// assert_eq!(0, func.call(0, 39).unwrap());
/// assert_eq!(7, func.call(7, 1).unwrap());
/// assert_eq!(-1, func.call(3, -2).unwrap());
/// // Division by zero is a noop (but does not fail)
/// assert_eq!(0, func.call(3, 0).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn divide_f32(v1: f32, v2: f32) -> i32 {
///     [Divide::new(0, 1, 2), Return::new()]
/// }
/// let func = DivideF32::new().unwrap();
/// assert_eq!(0, func.call(0.0, 39.0).unwrap());
/// assert_eq!(0, func.call(1.0, 7.0).unwrap());
/// assert_eq!(-1, func.call(3.0, -2.0).unwrap());
/// // Fractions are truncated before operation
/// assert_eq!(3, func.call(15.999, 5.999).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn divide_f64(v1: f64, v2: f64) -> f64 {
///     [Divide::new(0, 1, 2), Return::new()]
/// }
/// let func = DivideF64::new().unwrap();
/// assert_eq!(0.0, func.call(0.0, 39.0).unwrap());
/// assert_eq!(0.25, func.call(1.0, 4.0).unwrap());
/// assert_eq!(-1.5, func.call(3.0, -2.0).unwrap());
/// // Division by zero is a noop (but does not fail)
/// assert_eq!(0.0, func.call(3.0, 0.0).unwrap());
/// // Division by zero checks for floating point zero (true 0.0, not truncated to 0)
/// assert_eq!(4.0, func.call(2.0, 0.5).unwrap());
/// ```
#[derive(Default, PartialEq)]
pub struct Divide {
    dividend: Slot,
    divisor: Slot,
    destination: Slot,
}

impl Divide {
    pub fn new(dividend: Slot, divisor: Slot, destination: Slot) -> Code {
        Code::Divide(Divide {
            dividend,
            divisor,
            destination,
        })
    }
}

impl CodeBuilder for Divide {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = context.get_slot_value_type(self.destination)?;

        // If the divisor is zero, we need to skip the division. There is a specific command to check for zero for
        // integers, but we have to load a constant if we're working with floats
        let mut inner_instructions: Vec<Instruction> = vec![];
        GetSlotConvert::convert(self.divisor, operate_as, context, &mut inner_instructions)?;
        match &operate_as {
            ValueType::F32 => {
                inner_instructions.push(NumericInstruction::F32Constant(0.0).into());
                inner_instructions.push(NumericInstruction::Equal(operate_as.into()).into());
            }
            ValueType::F64 => {
                inner_instructions.push(NumericInstruction::F64Constant(0.0).into());
                inner_instructions.push(NumericInstruction::Equal(operate_as.into()).into());
            }
            _ => inner_instructions.push(NumericInstruction::EqualToZero(operate_as.into()).into()),
        }
        inner_instructions.push(ControlInstruction::BranchIf(0).into());

        // Now that the div/0 check is done, perform the real division
        GetSlotConvert::convert(self.dividend, operate_as, context, &mut inner_instructions)?;
        GetSlotConvert::convert(self.divisor, operate_as, context, &mut inner_instructions)?;
        inner_instructions.push(if operate_as.is_float() {
            NumericInstruction::DivideFloat(operate_as.into()).into()
        } else {
            NumericInstruction::DivideInteger(operate_as.into(), context.sign_extension()).into()
        });
        SetSlotConvert::convert(self.destination, operate_as, context, &mut inner_instructions)?;

        // All that goes into a block so that the branch has a target
        instruction_list.push(ControlInstruction::Block(BlockType::None, Expression::new(inner_instructions)).into());
        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}Divide::new({}, {}, {}),",
            indentation, self.dividend, self.divisor, self.destination
        )
    }
}

/// Divides the `dividend` value by the `divisor` using integer division, and places the remainder in the `destination`
/// slot. All operands are converted to integers before the operation.
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn remainder_i32(v1: i32, v2: i32) -> i32 {
///     [Remainder::new(0, 1, 2), Return::new()]
/// }
/// let func = RemainderI32::new().unwrap();
/// assert_eq!(0, func.call(0, 39).unwrap());
/// assert_eq!(6, func.call(6, 7).unwrap());
/// assert_eq!(1, func.call(3, -2).unwrap());
/// // Division by zero is a noop (but does not fail)
/// assert_eq!(0, func.call(3, 0).unwrap());
/// ```
///
/// ```
/// use wasmgp::*;
/// use wasmgp_macros::wasm_code;
///
/// #[wasm_code(signed)]
/// fn remainder_f32(v1: f32, v2: f32) -> i32 {
///     [Remainder::new(0, 1, 2), Return::new()]
/// }
/// let func = RemainderF32::new().unwrap();
/// assert_eq!(0, func.call(0.0, 39.0).unwrap());
/// assert_eq!(6, func.call(6.0, 7.0).unwrap());
/// assert_eq!(1, func.call(3.0, -2.0).unwrap());
/// // Fractions are truncated before operation
/// assert_eq!(5, func.call(15.999, 10.999).unwrap());
/// ```
#[derive(Default, PartialEq)]
pub struct Remainder {
    dividend: Slot,
    divisor: Slot,
    destination: Slot,
}

impl Remainder {
    pub fn new(dividend: Slot, divisor: Slot, destination: Slot) -> Code {
        Code::Remainder(Remainder {
            dividend,
            divisor,
            destination,
        })
    }
}

impl CodeBuilder for Remainder {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let operate_as = match (
            context.get_slot_value_type(self.dividend)?,
            context.get_slot_value_type(self.divisor)?,
        ) {
            (ValueType::I32, ValueType::I32) => ValueType::I32,
            _ => ValueType::I64,
        };

        // If the divisor is zero, we need to skip the division
        let mut inner_instructions: Vec<Instruction> = vec![];
        GetSlotConvert::convert(self.divisor, operate_as, context, &mut inner_instructions)?;
        inner_instructions.push(NumericInstruction::EqualToZero(operate_as.into()).into());
        inner_instructions.push(ControlInstruction::BranchIf(0).into());

        // Now that the div/0 check is done, perform the real division
        GetSlotConvert::convert(self.dividend, operate_as, context, &mut inner_instructions)?;
        GetSlotConvert::convert(self.divisor, operate_as, context, &mut inner_instructions)?;
        inner_instructions.push(NumericInstruction::Remainder(operate_as.into(), context.sign_extension()).into());
        SetSlotConvert::convert(self.destination, operate_as, context, &mut inner_instructions)?;

        // All that goes into a block so that the branch has a target
        instruction_list.push(ControlInstruction::Block(BlockType::None, Expression::new(inner_instructions)).into());

        Ok(())
    }

    fn print_for_rust(&self, f: &mut std::string::String, indentation: &mut Indentation) -> std::fmt::Result {
        writeln!(
            f,
            "{}Remainder::new({}, {}, {}),",
            indentation, self.dividend, self.divisor, self.destination
        )
    }
}
