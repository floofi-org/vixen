use crate::{isa, CPUResult, Interrupt};

#[derive(Debug, Clone, Copy)]
pub enum InstructionOperation {
    /* 0x01?? */ Add, Sub, Mul, Div, Mod, Sqt, Cbt, Sqr, Cbe, Min, Max,
    /* 0x02?? */ And, Or,  Xor, Nor, Nad, Imp, Not, Shl, Shr, Rol, Ror,
    /* 0x03?? */ Inc, Dec, Ina, Dea, Inx, Dex, Iny, Dey,
    /* 0x04?? */ Cmp, Cpx, Cpy, Lte, Gte, Sez, Sec, Seo,
    /* 0x05?? */ Lda, Ldx, Ldy, Ldz, Sta, Stx, Sty, Mov, Swp, Clr,
    /* 0x06?? */ Jmp, Jsr, Ret, Beq, Bne, Bec, Bnc, Beo, Bno, Int, Irt, Nop,
    /* 0x07?? */ Pha, Pla, Phx, Plx, Phy, Ply, Psh, Pll
}

impl InstructionOperation {
    pub fn disassemble(value: u16, mode: u8) -> String {
        if let Ok(operation) = InstructionOperation::try_from(value) {
            format!("{operation:?} ").to_lowercase()
        } else {
            format!("??({value:0>3X}{mode:0>1X}) ")
        }
    }
}

impl TryFrom<u16> for InstructionOperation {
    type Error = Interrupt;

    fn try_from(value: u16) -> CPUResult<Self> {
        isa! {
            value,

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
            0x032 => Ina,
            0x033 => Dea,
            0x034 => Inx,
            0x035 => Dex,
            0x036 => Iny,
            0x037 => Dey,

            // 0x04?? - Comparison Instructions
            0x040 => Cmp,
            0x041 => Cpx,
            0x042 => Cpy,
            0x043 => Lte,
            0x044 => Gte,
            0x045 => Sez,
            0x046 => Sec,
            0x047 => Seo,

            // 0x05?? - Data Movement Instructions
            0x050 => Lda,
            0x051 => Ldx,
            0x052 => Ldy,
            0x053 => Ldz,
            0x054 => Sta,
            0x055 => Stx,
            0x056 => Sty,
            0x057 => Mov,
            0x058 => Swp,
            0x059 => Clr,

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

            // 0x07?? - Stack Instructions
            0x070 => Pha,
            0x071 => Pla,
            0x072 => Phx,
            0x073 => Plx,
            0x074 => Phy,
            0x075 => Ply,
            0x076 => Psh,
            0x077 => Pll
        }
    }

}
