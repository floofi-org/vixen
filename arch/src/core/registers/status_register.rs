use core::fmt::{Display, Formatter};

#[derive(Debug, Default, Copy, Clone)]
#[allow(clippy::struct_excessive_bools)]
// Enums cannot be used here
pub struct StatusRegister {
    pub negative: bool,
    pub overflow: bool,
    pub double_fault: bool,
    pub interrupt: bool,
    pub interrupt_disable: bool,
    pub zero: bool,
    pub carry: bool
}

impl Display for StatusRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let negative = get_flag_char('i', self.negative);
        let overflow = get_flag_char('o', self.overflow);
        let double_fault = get_flag_char('f', self.double_fault);
        let interrupt = get_flag_char('b', self.interrupt);
        let interrupt_disable = get_flag_char('i', self.interrupt_disable);
        let zero = get_flag_char('z', self.zero);
        let carry = get_flag_char('c', self.carry);

        write!(f, "{negative}{overflow}{double_fault}{interrupt}-{interrupt_disable}{zero}{carry}")
    }
}

impl From<StatusRegister> for u8 {
    fn from(value: StatusRegister) -> Self {
        let mut out = 0u8;
        out |= u8::from(value.negative) << 7;
        out |= u8::from(value.overflow) << 6;
        out |= u8::from(value.double_fault) << 5;
        out |= u8::from(value.interrupt) << 4;
        out |= u8::from(value.interrupt_disable) << 2;
        out |= u8::from(value.zero) << 1;
        out |= u8::from(value.carry);
        out
    }
}

impl From<u8> for StatusRegister {
    fn from(value: u8) -> Self {
        #[allow(clippy::unreadable_literal)]
        // They literally are just 8-bit binary numbers
        StatusRegister {
            negative: (value & 0b10000000) == 0b10000000,
            overflow: (value & 0b01000000) == 0b01000000,
            double_fault: (value & 0b00100000) == 0b00100000,
            interrupt: (value & 0b00010000) == 0b00010000,
            interrupt_disable: (value & 0b00000100) == 0b00000100,
            zero: (value & 0b00000010) == 0b00000010,
            carry: (value & 0b00000001) == 0b00000001,
        }
    }
}

fn get_flag_char(flag: char, state: bool) -> char {
    if state { flag } else { '-' }
}
