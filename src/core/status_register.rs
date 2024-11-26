use core::fmt::{Display, Formatter};

#[derive(Debug, Default)]
pub struct StatusRegister {
    pub zero: bool,
    pub carry: bool,
    pub overflow: bool
}

impl Display for StatusRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let zero = get_flag_char('z', self.zero);
        let carry = get_flag_char('c', self.carry);
        let overflow = get_flag_char('o', self.overflow);

        write!(f, "{zero}{carry}{overflow}")
    }
}

fn get_flag_char(flag: char, state: bool) -> char {
    if state { flag } else { '-' }
}
