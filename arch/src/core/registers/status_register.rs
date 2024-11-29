use core::fmt::{Display, Formatter};

#[derive(Debug, Default, Copy, Clone)]
pub struct StatusRegister {
    pub zero: bool,
    pub carry: bool,
    pub overflow: bool,
    pub interrupt: bool,
    pub double_fault: bool,
    pub decimal: bool,
    pub interrupt_disable: bool
}

impl Display for StatusRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let zero = get_flag_char('z', self.zero);
        let carry = get_flag_char('c', self.carry);
        let overflow = get_flag_char('o', self.overflow);
        let interrupt = get_flag_char('b', self.interrupt);
        let double_fault = get_flag_char('f', self.double_fault);
        let decimal = get_flag_char('d', self.decimal);
        let interrupt_disable = get_flag_char('i', self.interrupt_disable);

        write!(f, "-{zero}{carry}{overflow}{interrupt}{double_fault}{decimal}{interrupt_disable}")
    }
}

impl From<StatusRegister> for u8 {
    fn from(value: StatusRegister) -> Self {
        let mut out = 0u8;
        out |= (value.zero as u8) << 6;
        out |= (value.carry as u8) << 5;
        out |= (value.overflow as u8) << 4;
        out |= (value.interrupt as u8) << 3;
        out |= (value.double_fault as u8) << 2;
        out |= (value.decimal as u8) << 1;
        out |= value.interrupt_disable as u8;
        out
    }
}

impl From<u8> for StatusRegister {
    fn from(value: u8) -> Self {
        StatusRegister {
            zero: (value & 0b01000000) == 0b01000000,
            carry: (value & 0b00100000) == 0b00100000,
            overflow: (value & 0b00010000) == 0b00010000,
            interrupt: (value & 0b00001000) == 0b00001000,
            double_fault: (value & 0b00000100) == 0b00000100,
            decimal: (value & 0b00000010) == 0b00000010,
            interrupt_disable: (value & 0b00000001) == 0b00000001,
        }
    }
}

fn get_flag_char(flag: char, state: bool) -> char {
    if state { flag } else { '-' }
}
