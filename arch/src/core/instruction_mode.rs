use crate::{CPUResult, Interrupt};

#[derive(Debug, Clone, Copy)]
pub enum InstructionMode {
    Immediate,
    Implied,
    ZeroPage,
    Absolute
}

impl TryFrom<u8> for InstructionMode {
    type Error = Interrupt;

    fn try_from(value: u8) -> CPUResult<Self> {
        match value {
            0 => Ok(InstructionMode::Immediate),
            1 => Ok(InstructionMode::Implied),
            2 => Ok(InstructionMode::ZeroPage),
            3 => Ok(InstructionMode::Absolute),
            _ => Err(Interrupt::IllegalInstruction)
        }
    }
}
