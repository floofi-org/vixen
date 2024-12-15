use vixen::core::{instruction::Addressing, registers::RegisterId};

#[derive(Debug)]
pub enum Operand {
    Literal(u32),
    Register(RegisterId),
    Address(Address),
    Label(String),
    Indirect(OperandIndirect),
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum OperandIndirect {
    Register(RegisterId),
}

#[derive(Debug)]
pub enum Address {
    Absolute(u32),
    Relative(i32),
}

impl Operand {
    #[must_use]
    pub fn get_addressing(&self) -> Addressing {
        match self {
            Operand::Literal(_) => Addressing::Immediate,
            Operand::Register(_) => Addressing::Direct,
            Operand::Address(address) => address.get_addressing(),
            Operand::Label(_) => Addressing::Relative, // Relative to the start of the program
            Operand::Indirect(indirect) => indirect.get_addressing(),
        }
    }
}

impl OperandIndirect {
    #[must_use]
    pub fn get_addressing(&self) -> Addressing {
        match self {
            OperandIndirect::Register(_) => Addressing::RegisterIndirect,
        }
    }
}

impl Address {
    #[must_use]
    pub fn get_addressing(&self) -> Addressing {
        match self {
            Address::Absolute(_) => Addressing::Absolute,
            Address::Relative(_) => Addressing::Relative,
        }
    }
}

impl From<Operand> for u32 {
    fn from(value: Operand) -> Self {
        match value {
            Operand::Literal(literal) => literal,
            Operand::Register(reg) => reg.into(),
            Operand::Address(address) => address.into(),
            Operand::Label(_) => panic!("Tried to convert label to u32"),
            Operand::Indirect(indirect) => indirect.into(),
        }
    }
}

impl From<OperandIndirect> for u32 {
    fn from(value: OperandIndirect) -> Self {
        match value {
            OperandIndirect::Register(reg) => reg.into(),
        }
    }
}

impl From<Address> for u32 {
    fn from(value: Address) -> Self {
        #[allow(clippy::cast_sign_loss)]
        match value {
            Address::Absolute(address) => address,
            Address::Relative(address) => address as u32,
        }
    }
}
