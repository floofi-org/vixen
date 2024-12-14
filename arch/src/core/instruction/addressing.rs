use crate::{CPUResult, Interrupt};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Addressing {
    Immediate,
    Direct,
    Indirect,
    Absolute,
    Relative,
    Implied,
    RegisterIndirect
}

impl TryFrom<u8> for Addressing {
    type Error = Interrupt;

    fn try_from(value: u8) -> CPUResult<Self> {
        match value {
            0x0 => Ok(Addressing::Immediate),
            0x1 => Ok(Addressing::Direct),
            0x2 => Ok(Addressing::Indirect),
            0x3 => Ok(Addressing::Absolute),
            0x4 => Ok(Addressing::Relative),
            0x5 => Ok(Addressing::Implied),
            // TODO: Indexed
            0x7 => Ok(Addressing::RegisterIndirect),
            // TODO: Memory Indexed
            // TODO: Register Indexed
            _ => Err(Interrupt::IllegalInstruction)
        }
    }
}

impl From<Addressing> for u8 {
    fn from(val: Addressing) -> Self {
        match val {
            Addressing::Immediate => 0x0,
            Addressing::Direct => 0x1,
            Addressing::Indirect => 0x2,
            Addressing::Absolute => 0x3,
            Addressing::Relative => 0x4,
            Addressing::Implied => 0x5,
            Addressing::RegisterIndirect => 0x6,
        }
    }
}
