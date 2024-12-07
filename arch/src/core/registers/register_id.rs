use crate::core::Interrupt;
use crate::CPUResult;

#[derive(Debug, Clone, Copy)]
pub enum RegisterId {
    R0, R1, R2, R3, R4, R5, R6, R7,
    R8, R9, R10, R11, R12, R13, R14
}

impl TryFrom<u32> for RegisterId {
    type Error = Interrupt;

    fn try_from(value: u32) -> CPUResult<Self> {
        match value {
            0x0001 => Ok(Self::R0),
            0x0011 => Ok(Self::R1),
            0x0012 => Ok(Self::R2),
            0x1000 => Ok(Self::R3),
            0x1001 => Ok(Self::R4),
            0x1002 => Ok(Self::R5),
            0x1003 => Ok(Self::R6),
            0x1004 => Ok(Self::R7),
            0x1005 => Ok(Self::R8),
            0x1006 => Ok(Self::R9),
            0x1007 => Ok(Self::R10),
            0x1008 => Ok(Self::R11),
            0x1009 => Ok(Self::R12),
            0x100A => Ok(Self::R13),
            0x100B => Ok(Self::R14),
            _ => Err(Interrupt::IllegalMemory)
        }
    }
}
