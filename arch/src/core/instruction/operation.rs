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
    /* 0x01?? */ Add, Sub, Mul, Div, Mod,
    Sqrt,
    Cbrt,
    Sqre,
    Cube, Min, Max,
    Addc,
    Subc,
    Sar,
    Sal,
    /* 0x02?? */ And, Or,  Xor, Nor,
    Nand, Imp, Not, Shl, Shr, Rol, Ror,
    /* 0x03?? */ Inc, Dec,
    /* 0x04?? */ Cmp, Lte, Gte,
    Setz,
    Setc,
    Seto,
    /* 0x05?? */ Mov,
    Xchg, Clr,
    Stc, Clc,
    Sti, Cli, Clv,
    /* 0x06?? */ Jmp, Jmpl, Ret,
    Jz,
    Jnz,
    Jc,
    Jnc,
    Jo,
    Jno, Int,
    Iret,
    Irets, Nop,
    Hlt,
    Js,
    Jns,
    /* 0x07?? */
    Push,
    Pop,
    Pushf,
    Popf
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
    0x015 => Sqrt,
    0x016 => Cbrt,
    0x017 => Sqre,
    0x018 => Cube,
    0x019 => Min,
    0x01A => Max,
    0x01B => Addc,
    0x01C => Subc,
    0x01D => Sar,
    0x01E => Sal,

    // 0x02?? - Logic Instructions
    0x020 => And,
    0x021 => Or,
    0x022 => Xor,
    0x023 => Nor,
    0x024 => Nand,
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
    0x043 => Setz,
    0x044 => Setc,
    0x045 => Seto,

    // 0x05?? - Data Movement Instructions
    0x050 => Mov,
    0x051 => Xchg,
    0x052 => Clr,
    0x053 => Stc,
    0x054 => Clc,
    0x055 => Sti,
    0x056 => Cli,
    0x057 => Clv,

    // 0x06?? - Control Flow Instructions
    0x060 => Jmpl,
    0x061 => Jmp,
    0x062 => Ret,
    0x063 => Jz,
    0x064 => Jnz,
    0x065 => Jc,
    0x066 => Jnc,
    0x067 => Jo,
    0x068 => Jno,
    0x069 => Int,
    0x06A => Iret,
    0x06B => Irets,
    0x06C => Nop,
    0x06D => Hlt,
    0x06E => Js,
    0x06F => Jns,

    // 0x07?? - Stack Instructions
    0x070 => Push,
    0x071 => Pop,
    0x072 => Pushf,
    0x073 => Popf
}
