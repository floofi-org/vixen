use core::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct CPUStatusRegister {
    pub zero: bool,
    pub carry: bool,
    pub overflow: bool
}

impl Display for CPUStatusRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{zero}{carry}{overflow}",
               zero = if self.zero {
                   "z"
               } else {
                   "-"
               },
               carry = if self.carry {
                   "c"
               } else {
                   "-"
               },
               overflow = if self.overflow {
                   "o"
               } else {
                   "-"
               }
        )
    }
}

impl CPUStatusRegister {
    pub fn new() -> Self {
        Self {
            zero: false,
            carry: false,
            overflow: false,
        }
    }
}