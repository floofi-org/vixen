use alloc::format;
use alloc::string::String;
use core::fmt::{Debug, Display, Formatter};
use crate::{CPUResult, Interrupt};

#[allow(edition_2024_expr_fragment_specifier)]
macro_rules! isa {
    ( $( $x: expr => $y: ident ),+ $(,)? ) => {
        impl TryFrom<u16> for Operation {
            type Error = Interrupt;

            fn try_from(value: u16) -> CPUResult<Self> {
                match value {
                    $(
                        $x => Ok(Self::$y),
                    )*
                        _ => Err(Interrupt::IllegalInstruction)
                }
            }
        }

        impl From<Operation> for u16 {
            fn from(value: Operation) -> u16 {
                match value {
                    $(
                        Operation::$y => $x,
                    )*
                }
            }
        }
    };
}


#[derive(Debug, Clone, Copy)]
pub enum Operation {
    /* 0x01?? */ Add, Sub, Mul, Div, Mod, Sqt, Cbt, Sqr, Cbe, Min, Max, Adc, Sbc, Asr,
    /* 0x02?? */ And, Or,  Xor, Nor, Nad, Imp, Not, Shl, Shr, Rol, Ror,
    /* 0x03?? */ Inc, Dec,
    /* 0x04?? */ Cmp, Lte, Gte, Srz, Src, Sro,
    /* 0x05?? */ Ldr, Str, Mov, Swp, Clr, Sec, Clc, Sei, Cli, Clv,
    /* 0x06?? */ Jmp, Jsr, Ret, Beq, Bne, Bec, Bnc, Beo, Bno, Int, Irt, Nop, Jam, Bpl, Bmi,
    /* 0x07?? */ Psh, Pll, Php, Plp
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&format!("{self:?}"), f)
    }
}

impl Operation {
    #[must_use]
    pub fn disassemble(value: u16) -> String {
        if let Ok(operation) = Operation::try_from(value) {
            format!("{operation} ").to_lowercase()
        } else {
            String::from("<unk> ")
        }
    }
}

isa! {
    // 0x01?? - Arithmetic and Algebric Instructions
    0x010 => Add,
    0x011 => Sub,
    0x012 => Mul,
    0x013 => Div,
    0x014 => Mod,
    0x015 => Sqt,
    0x016 => Cbt,
    0x017 => Sqr,
    0x018 => Cbe,
    0x019 => Min,
    0x01A => Max,
    0x01B => Adc,
    0x01C => Sbc,
    0x01D => Asr,

    // 0x02?? - Logic Instructions
    0x020 => And,
    0x021 => Or,
    0x022 => Xor,
    0x023 => Nor,
    0x024 => Nad,
    0x025 => Imp,
    0x026 => Not,
    0x027 => Shl,
    0x028 => Shr,
    0x029 => Rol,
    0x02A => Ror,

    // 0x03?? - Counting Instructions
    0x030 => Inc,
    0x031 => Dec,

    // 0x04?? - Comparison Instructions
    0x040 => Cmp,
    0x041 => Lte,
    0x042 => Gte,
    0x043 => Srz,
    0x044 => Src,
    0x045 => Sro,

    // 0x05?? - Data Movement Instructions
    0x050 => Ldr,
    0x051 => Str,
    0x052 => Mov,
    0x053 => Swp,
    0x054 => Clr,
    0x055 => Sec,
    0x056 => Clc,
    0x057 => Sei,
    0x058 => Cli,
    0x059 => Clv,

    // 0x06?? - Control Flow Instructions
    0x060 => Jmp,
    0x061 => Jsr,
    0x062 => Ret,
    0x063 => Beq,
    0x064 => Bne,
    0x065 => Bec,
    0x066 => Bnc,
    0x067 => Beo,
    0x068 => Bno,
    0x069 => Int,
    0x06A => Irt,
    0x06B => Nop,
    0x06C => Jam,
    0x06D => Bpl,
    0x06E => Bmi,

    // 0x07?? - Stack Instructions
    0x070 => Psh,
    0x071 => Pll,
    0x072 => Php,
    0x073 => Plp
}
