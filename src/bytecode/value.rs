use super::instruction::Instruction;

#[derive(Default, Clone, Copy, Debug)]
pub struct Value(u64);

impl Value {
    const TAG_NUMBER: u64 = 0;
    const TAG_BOOLEAN: u64 = 1;
    const TAG_INSTRUCTION: u64 = 2;

    #[inline(always)]
    pub fn number(value: f64) -> Self {
        Self(value.to_bits())
    }

    #[inline(always)]
    pub fn boolean(value: bool) -> Self {
        Self(((value as u64) << 63) | Self::TAG_BOOLEAN)
    }

    #[inline(always)]
    pub fn instruction(ptr: *const Instruction) -> Self {
        Self(ptr as u64)
    }

    #[inline(always)]
    pub fn as_number(&self) -> f64 {
        f64::from_bits(self.0)
    }

    #[inline(always)]
    pub fn as_boolean(&self) -> bool {
        (self.0 >> 63) != 0
    }

    #[inline(always)]
    pub fn as_instruction(&self) -> *const Instruction {
        self.0 as *const Instruction
    }
}
