use crate::code_builder::CodeBuilder;
use crate::indentation::Indentation;
use crate::{Code, CodeContext, GeneticEngine, Slot, ValueType};
use anyhow::Result;
use wasm_ast::{FloatType, Instruction, IntegerType, NumericInstruction, VariableInstruction};

/// Used to convert a slot value to the value expected for a stack operation
pub struct GetSlotConvert {
    slot: Slot,
    stack_type: ValueType,
}

impl GetSlotConvert {
    pub fn convert(
        slot: Slot,
        stack_type: ValueType,
        context: &CodeContext,
        instruction_list: &mut Vec<Instruction>,
    ) -> Result<()> {
        let convert = GetSlotConvert { slot, stack_type };
        convert.append_code(context, instruction_list)
    }
}

impl CodeBuilder for GetSlotConvert {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        let source_type = context.get_slot_value_type(self.slot)?;

        // Load the slot onto the stack
        instruction_list.push(VariableInstruction::LocalGet(self.slot as u32).into());

        // Perform a conversion of the type that our slot produced, to the type the next operation expects
        StackConvert::convert(source_type, self.stack_type, context, instruction_list)?;

        Ok(())
    }

    fn make_random_code(_engine: &mut GeneticEngine, _max_points: usize) -> Code {
        panic!("this CodeBuilder should not be created as random code")
    }

    fn print_for_rust(&self, _f: &mut std::string::String, _indentation: &mut Indentation) -> std::fmt::Result {
        Ok(())
    }
}

/// Used to convert a stack value to the value expected for a slot
pub struct SetSlotConvert {
    stack_type: ValueType,
    slot: Slot,
}

impl SetSlotConvert {
    pub fn convert(
        slot: Slot,
        stack_type: ValueType,
        context: &CodeContext,
        instruction_list: &mut Vec<Instruction>,
    ) -> Result<()> {
        let convert = SetSlotConvert { slot, stack_type };
        convert.append_code(context, instruction_list)
    }
}

impl CodeBuilder for SetSlotConvert {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        // Perform a conversion of the type that's on the stack to the type that the slot expects
        let destination_type = context.get_slot_value_type(self.slot)?;
        StackConvert::convert(self.stack_type, destination_type, context, instruction_list)?;

        // The top of the stack can now be set because the types are the same.
        instruction_list.push(VariableInstruction::LocalSet(self.slot as u32).into());

        Ok(())
    }

    fn make_random_code(_engine: &mut GeneticEngine, _max_points: usize) -> Code {
        panic!("this CodeBuilder should not be created as random code")
    }

    fn print_for_rust(&self, _f: &mut std::string::String, _indentation: &mut Indentation) -> std::fmt::Result {
        Ok(())
    }
}

/// Used to convert a stack value to another stack value
pub struct StackConvert {
    source_type: ValueType,
    destination_type: ValueType,
}

impl StackConvert {
    pub fn convert(
        source_type: ValueType,
        destination_type: ValueType,
        context: &CodeContext,
        instruction_list: &mut Vec<Instruction>,
    ) -> Result<()> {
        let convert = StackConvert {
            source_type,
            destination_type,
        };
        convert.append_code(context, instruction_list)
    }
}

impl CodeBuilder for StackConvert {
    fn append_code(&self, context: &CodeContext, instruction_list: &mut Vec<Instruction>) -> Result<()> {
        match (self.source_type, self.destination_type) {
            // None of these need converting
            (ValueType::I32, ValueType::I32) => {}
            (ValueType::I64, ValueType::I64) => {}
            (ValueType::F32, ValueType::F32) => {}
            (ValueType::F64, ValueType::F64) => {}

            // Convert I32 to...
            (ValueType::I32, ValueType::I64) => {
                instruction_list.push(NumericInstruction::ExtendWithSignExtension(context.sign_extension()).into());
            }
            (ValueType::I32, ValueType::F32) => {
                instruction_list.push(
                    NumericInstruction::Convert(FloatType::F32, IntegerType::I32, context.sign_extension()).into(),
                );
            }
            (ValueType::I32, ValueType::F64) => {
                instruction_list.push(
                    NumericInstruction::Convert(FloatType::F64, IntegerType::I32, context.sign_extension()).into(),
                );
            }

            // Convert I64 to...
            (ValueType::I64, ValueType::I32) => {
                instruction_list.push(NumericInstruction::Wrap.into());
            }
            (ValueType::I64, ValueType::F32) => {
                instruction_list.push(
                    NumericInstruction::Convert(FloatType::F32, IntegerType::I64, context.sign_extension()).into(),
                );
            }
            (ValueType::I64, ValueType::F64) => {
                instruction_list.push(
                    NumericInstruction::Convert(FloatType::F64, IntegerType::I64, context.sign_extension()).into(),
                );
            }

            // Convert F32 to...
            (ValueType::F32, ValueType::I32) => instruction_list.push(
                NumericInstruction::ConvertAndTruncateWithSaturation(
                    IntegerType::I32,
                    FloatType::F32,
                    context.sign_extension(),
                )
                .into(),
            ),
            (ValueType::F32, ValueType::I64) => instruction_list.push(
                NumericInstruction::ConvertAndTruncateWithSaturation(
                    IntegerType::I64,
                    FloatType::F32,
                    context.sign_extension(),
                )
                .into(),
            ),
            (ValueType::F32, ValueType::F64) => {
                instruction_list.push(NumericInstruction::Promote.into());
            }

            // Convert F64 to...
            (ValueType::F64, ValueType::I32) => instruction_list.push(
                NumericInstruction::ConvertAndTruncateWithSaturation(
                    IntegerType::I32,
                    FloatType::F64,
                    context.sign_extension(),
                )
                .into(),
            ),
            (ValueType::F64, ValueType::I64) => instruction_list.push(
                NumericInstruction::ConvertAndTruncateWithSaturation(
                    IntegerType::I64,
                    FloatType::F64,
                    context.sign_extension(),
                )
                .into(),
            ),
            (ValueType::F64, ValueType::F32) => {
                instruction_list.push(NumericInstruction::Demote.into());
            }
        }
        Ok(())
    }

    fn make_random_code(_engine: &mut GeneticEngine, _max_points: usize) -> Code {
        panic!("this CodeBuilder should not be created as random code")
    }

    fn print_for_rust(&self, _f: &mut std::string::String, _indentation: &mut Indentation) -> std::fmt::Result {
        Ok(())
    }
}
