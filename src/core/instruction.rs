use crate::core::instruction_mode::InstructionMode;
use crate::core::instruction_operation::InstructionOperation;
use crate::core::operand::Operand;
use crate::{instructions, InstructionResult};
use crate::cpu::CPU;

#[derive(Debug)]
pub struct Instruction {
    pub operation: InstructionOperation,
    pub mode: InstructionMode,
    pub operands: [Operand; 2]
}

impl Instruction {
    pub fn execute(&self, cpu: &CPU) -> InstructionResult {
        match self.operation {
            InstructionOperation::Add => instructions::arithmetic::add(&self.operands, cpu),
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
