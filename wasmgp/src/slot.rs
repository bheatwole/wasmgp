use crate::ValueType;

/// The genetic code references variables by 'slots'. This type references a slot that can be an I32, I64, F32 or F64.
pub type Slot = u8;

/// This type references a slot that can be an I32 or I64.
pub type IntegerSlot = u8;

/// This type references a slot that can be an F32 or F64.
pub type FloatSlot = u8;

/// When dealing with generic slots, we need to know whether it holds an integer or float
pub enum SlotType {
    Integer,
    Float,
}

/// When dealing with slots, we need to know its size
pub enum SlotBytes {
    /// I32 or F32
    Four,

    /// I64 or F64
    Eight,
}

/// When setting up the genetic algorithm, we give the code a certain number of local 'slots' to use in calculations.
/// Typically its best to choose one type (I32, F64, etc) and only use slots of that type, but in some algorithms it
/// may be necessary to use multiple types.
///
/// The total sum of all slots counts plus FunctionSignature.Params.Len plus FunctionSignature.Results.Len must fit into
/// a `u8` (256 max).
pub struct SlotCount {
    pub i32: u8,
    pub i64: u8,
    pub f32: u8,
    pub f64: u8,
}

impl SlotCount {
    pub fn len(&self) -> usize {
        self.i32 as usize + self.i64 as usize + self.f32 as usize + self.f64 as usize
    }

    pub fn iter(&self) -> SlotCountIterator {
        SlotCountIterator {
            slots: self,
            next: 0,
        }
    }
}

pub struct SlotCountIterator<'a> {
    slots: &'a SlotCount,
    next: usize,
}

impl<'a> Iterator for SlotCountIterator<'a> {
    type Item = ValueType;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current = self.next;
        self.next += 1;

        if current < self.slots.i32 as usize {
            return Some(ValueType::I32);
        }
        current -= self.slots.i32 as usize;

        if current < self.slots.i64 as usize {
            return Some(ValueType::I64);
        }
        current -= self.slots.i64 as usize;

        if current < self.slots.f32 as usize {
            return Some(ValueType::F32);
        }
        current -= self.slots.f32 as usize;

        if current < self.slots.f64 as usize {
            return Some(ValueType::F64);
        }

        None
    }
}
