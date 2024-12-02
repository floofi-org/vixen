use crate::{CPUResult, Interrupt};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstructionMode {
    Immediate,
    Direct,
    ZeroPage,
    Absolute,
    Relative,
    Implied
}

impl TryFrom<u8> for InstructionMode {
    type Error = Interrupt;

    fn try_from(value: u8) -> CPUResult<Self> {
        match value {
            0x0 => Ok(InstructionMode::Immediate),
            0x1 => Ok(InstructionMode::Direct),
            0x2 => Ok(InstructionMode::ZeroPage),
            0x3 => Ok(InstructionMode::Absolute),
            0x4 => Ok(InstructionMode::Relative),
            0x5 => Ok(InstructionMode::Implied),
            _ => Err(Interrupt::IllegalInstruction)
        }
    }
}
