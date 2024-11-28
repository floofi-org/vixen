use crate::{CPUResult, Interrupt};

#[derive(Debug, Clone, Copy)]
pub enum InstructionMode {
    Immediate,
    Implied,
    ZeroPage,
    Absolute,
    Relative
}

impl TryFrom<u8> for InstructionMode {
    type Error = Interrupt;

    fn try_from(value: u8) -> CPUResult<Self> {
        match value {
            0x0 => Ok(InstructionMode::Immediate),
            0x1 => Ok(InstructionMode::Implied),
            0x2 => Ok(InstructionMode::ZeroPage),
            0x3 => Ok(InstructionMode::Absolute),
            0x4 => Ok(InstructionMode::Relative),
            _ => Err(Interrupt::IllegalInstruction)
        }
    }
}
