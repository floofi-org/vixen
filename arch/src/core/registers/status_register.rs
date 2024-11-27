use core::fmt::{Display, Formatter};

#[derive(Debug, Default)]
pub struct StatusRegister {
    pub zero: bool,
    pub carry: bool,
    pub overflow: bool,
    pub interrupt: bool,
    pub double_fault: bool
}

impl Display for StatusRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let zero = get_flag_char('z', self.zero);
        let carry = get_flag_char('c', self.carry);
        let overflow = get_flag_char('o', self.overflow);
        let interrupt = get_flag_char('i', self.interrupt);
        let double_fault = get_flag_char('f', self.double_fault);

        write!(f, "{zero}{carry}{overflow}{interrupt}{double_fault}")
    }
}

fn get_flag_char(flag: char, state: bool) -> char {
    if state { flag } else { '-' }
}
