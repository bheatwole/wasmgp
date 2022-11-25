use crate::code_builder::CodeBuilder;
use crate::convert::{GetSlotConvert, SetSlotConvert};
use crate::*;
use anyhow::Result;
use wasm_ast::{BlockType, ControlInstruction, Expression, Instruction, NumericInstruction};

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
        Ok(())
    }
}

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
        Ok(())
    }
}

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
        Ok(())
    }
}

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
        Ok(())
    }
}

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
        Ok(())
    }
}

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
        Ok(())
    }
}

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
        Ok(())
    }
}

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
        Ok(())
    }
}

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
        Ok(())
    }
}
