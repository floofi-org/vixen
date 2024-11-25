use crate::core::instruction_mode::InstructionMode;
use crate::core::instruction_operation::InstructionOperation;
use crate::core::operand::Operand;
use crate::{instructions, CPUInstructionResult};

#[derive(Debug)]
pub struct Instruction<'a> {
    pub operation: InstructionOperation,
    pub mode: InstructionMode,
    pub operands: [Operand<'a>; 2]
}

impl<'a> Instruction<'a> {
    pub fn execute(&self) -> CPUInstructionResult {
        match self.operation {
            InstructionOperation::Add => instructions::arithmetic::add(&self.operands),
            InstructionOperation::Sub => unimplemented!(),
            InstructionOperation::Mul => unimplemented!(),
            InstructionOperation::Div => unimplemented!(),
            InstructionOperation::Mod => unimplemented!(),
            InstructionOperation::Sqt => unimplemented!(),
            InstructionOperation::Cbt => unimplemented!(),
            InstructionOperation::Sqr => unimplemented!(),
            InstructionOperation::Cbe => unimplemented!(),
            InstructionOperation::Min => unimplemented!(),
            InstructionOperation::Max => unimplemented!(),
            InstructionOperation::And => unimplemented!(),
            InstructionOperation::Or => unimplemented!(),
            InstructionOperation::Xor => unimplemented!(),
            InstructionOperation::Nor => unimplemented!(),
            InstructionOperation::Nad => unimplemented!(),
            InstructionOperation::Imp => unimplemented!(),
            InstructionOperation::Not => unimplemented!(),
            InstructionOperation::Shl => unimplemented!(),
            InstructionOperation::Shr => unimplemented!(),
            InstructionOperation::Rol => unimplemented!(),
            InstructionOperation::Ror => unimplemented!(),
            InstructionOperation::Inc => unimplemented!(),
            InstructionOperation::Dec => unimplemented!(),
            InstructionOperation::Ina => unimplemented!(),
            InstructionOperation::Dea => unimplemented!(),
            InstructionOperation::Inx => unimplemented!(),
            InstructionOperation::Dex => unimplemented!(),
            InstructionOperation::Iny => unimplemented!(),
            InstructionOperation::Dey => unimplemented!(),
            InstructionOperation::Cmp => unimplemented!(),
            InstructionOperation::Cpx => unimplemented!(),
            InstructionOperation::Cpy => unimplemented!(),
            InstructionOperation::Lte => unimplemented!(),
            InstructionOperation::Gte => unimplemented!(),
            InstructionOperation::Sez => unimplemented!(),
            InstructionOperation::Sec => unimplemented!(),
            InstructionOperation::Seo => unimplemented!(),
            InstructionOperation::Lda => unimplemented!(),
            InstructionOperation::Ldx => unimplemented!(),
            InstructionOperation::Ldy => unimplemented!(),
            InstructionOperation::Ldz => unimplemented!(),
            InstructionOperation::Sta => unimplemented!(),
            InstructionOperation::Stx => unimplemented!(),
            InstructionOperation::Sty => unimplemented!(),
            InstructionOperation::Mov => unimplemented!(),
            InstructionOperation::Swp => unimplemented!(),
            InstructionOperation::Clr => unimplemented!(),
            InstructionOperation::Jmp => unimplemented!(),
            InstructionOperation::Jsr => unimplemented!(),
            InstructionOperation::Ret => unimplemented!(),
            InstructionOperation::Beq => unimplemented!(),
            InstructionOperation::Bne => unimplemented!(),
            InstructionOperation::Bec => unimplemented!(),
            InstructionOperation::Bnc => unimplemented!(),
            InstructionOperation::Beo => unimplemented!(),
            InstructionOperation::Bno => unimplemented!(),
            InstructionOperation::Int => unimplemented!(),
            InstructionOperation::Irt => unimplemented!(),
            InstructionOperation::Nop => unimplemented!(),
            InstructionOperation::Pha => unimplemented!(),
            InstructionOperation::Pla => unimplemented!(),
            InstructionOperation::Phx => unimplemented!(),
            InstructionOperation::Plx => unimplemented!(),
            InstructionOperation::Phy => unimplemented!(),
            InstructionOperation::Ply => unimplemented!(),
            InstructionOperation::Psh => unimplemented!(),
            InstructionOperation::Pll => unimplemented!(),
        }
    }
}

#[macro_export]
macro_rules! isa {
    ( $value: expr, $( $x: expr => $y: ident ),+ ) => {
        {
            match $value {
            $(
                $x => Ok(Self::$y),
            )*
                _ => Err(Interrupt::IllegalInstruction)
            }
        }
    };
}