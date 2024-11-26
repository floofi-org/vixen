use crate::core::interrupt::Interrupt;
use crate::CPUResult;

#[derive(Debug, Clone, Copy)]
pub enum RegisterId {
    A, X, Y,
    R0, R1, R2, R3, R4, R5, R6, R7
}

impl TryFrom<u16> for RegisterId {
    type Error = Interrupt;

    fn try_from(value: u16) -> CPUResult<Self> {
        match value {
            0x0001 => Ok(Self::A),
            0x0011 => Ok(Self::X),
            0x0012 => Ok(Self::Y),
            0x1000 => Ok(Self::R0),
            0x1001 => Ok(Self::R1),
            0x1002 => Ok(Self::R2),
            0x1003 => Ok(Self::R3),
            0x1004 => Ok(Self::R4),
            0x1005 => Ok(Self::R5),
            0x1006 => Ok(Self::R6),
            0x1007 => Ok(Self::R7),
            _ => Err(Interrupt::IllegalMemory)
        }
    }
}
