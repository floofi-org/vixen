use crate::core::Operand;
use crate::{instructions, InstructionResult};
use crate::CPU;

pub mod addressing;
pub mod operation;

pub use addressing::Addressing;
pub use operation::Operation;

#[derive(Debug)]
pub struct Instruction {
    pub operation: Operation,
    pub mode: Addressing,
    pub operands: [Operand; 2]
}

impl Instruction {
    pub fn execute_unhandled(&mut self, cpu: &mut CPU) -> InstructionResult {
        match self.operation {
            // 0x01?? - Arithmetic and Algebric Instructions
            Operation::Add => instructions::add(self.mode, &self.operands, cpu),
            Operation::Sub => instructions::sub(self.mode, &self.operands, cpu),
            Operation::Mul => instructions::mul(self.mode, &self.operands, cpu),
            Operation::Div => instructions::div(self.mode, &self.operands, cpu),
            Operation::Mod => instructions::mod_(self.mode, &self.operands, cpu),
            Operation::Sqt => instructions::sqt(self.mode, &self.operands, cpu),
            Operation::Cbt => instructions::cbt(self.mode, &self.operands, cpu),
            Operation::Sqr => instructions::sqr(self.mode, &self.operands, cpu),
            Operation::Cbe => instructions::cbe(self.mode, &self.operands, cpu),
            Operation::Min => instructions::min(self.mode, &self.operands, cpu),
            Operation::Max => instructions::max(self.mode, &self.operands, cpu),
            Operation::Adc => instructions::adc(self.mode, &self.operands, cpu),
            Operation::Sbc => instructions::sbc(self.mode, &self.operands, cpu),
            Operation::Asr => instructions::asr(self.mode, &mut self.operands, cpu),

            // 0x02?? - Logic Instructions
            Operation::And => instructions::and(self.mode, &self.operands, cpu),
            Operation::Or => instructions::or(self.mode, &self.operands, cpu),
            Operation::Xor => instructions::xor(self.mode, &self.operands, cpu),
            Operation::Nor => instructions::nor(self.mode, &self.operands, cpu),
            Operation::Nad => instructions::nad(self.mode, &self.operands, cpu),
            Operation::Imp => instructions::imp(self.mode, &self.operands, cpu),
            Operation::Not => instructions::not(self.mode, &self.operands, cpu),
            Operation::Shl => instructions::shl(self.mode, &mut self.operands, cpu),
            Operation::Shr => instructions::shr(self.mode, &mut self.operands, cpu),
            Operation::Rol => instructions::rol(self.mode, &mut self.operands, cpu),
            Operation::Ror => instructions::ror(self.mode, &mut self.operands, cpu),

            // 0x03?? - Counting Instructions
            Operation::Inc => instructions::inc(self.mode, &mut self.operands, cpu),
            Operation::Dec => instructions::dec(self.mode, &mut self.operands, cpu),

            // 0x04?? - Comparison Instructions
            Operation::Cmp => instructions::cmp(self.mode, &self.operands, cpu),
            Operation::Lte => instructions::lte(self.mode, &self.operands, cpu),
            Operation::Gte => instructions::gte(self.mode, &self.operands, cpu),
            Operation::Srz => instructions::srz(self.mode, &mut self.operands, cpu),
            Operation::Src => instructions::src(self.mode, &mut self.operands, cpu),
            Operation::Sro => instructions::sro(self.mode, &mut self.operands, cpu),

            // 0x05?? - Data Movement Instructions
            Operation::Ldr => instructions::ldr(self.mode, &mut self.operands, cpu),
            Operation::Str => instructions::str(self.mode, &mut self.operands, cpu),
            Operation::Mov => instructions::mov(self.mode, &mut self.operands, cpu),
            Operation::Swp => instructions::swp(self.mode, &mut self.operands, cpu),
            Operation::Clr => instructions::clr(self.mode, &mut self.operands, cpu),
            Operation::Sec => instructions::sec(self.mode, &self.operands, cpu),
            Operation::Clc => instructions::clc(self.mode, &self.operands, cpu),
            Operation::Sei => instructions::sei(self.mode, &self.operands, cpu),
            Operation::Cli => instructions::cli(self.mode, &self.operands, cpu),
            Operation::Clv => instructions::clv(self.mode, &self.operands, cpu),

            // 0x06?? - Control Flow Instructions
            Operation::Jmp => instructions::jmp(self.mode, &self.operands, cpu),
            Operation::Jsr => instructions::jsr(self.mode, &self.operands, cpu),
            Operation::Ret => instructions::ret(self.mode, &self.operands, cpu),
            Operation::Beq => instructions::beq(self.mode, &self.operands, cpu),
            Operation::Bne => instructions::bne(self.mode, &self.operands, cpu),
            Operation::Bec => instructions::bec(self.mode, &self.operands, cpu),
            Operation::Bnc => instructions::bnc(self.mode, &self.operands, cpu),
            Operation::Beo => instructions::beo(self.mode, &self.operands, cpu),
            Operation::Bno => instructions::bno(self.mode, &self.operands, cpu),
            Operation::Int => instructions::int(self.mode, &self.operands, cpu),
            Operation::Irt => instructions::irt(self.mode, &self.operands, cpu),
            Operation::Nop => instructions::nop(self.mode, &self.operands, cpu),
            Operation::Jam => instructions::jam(self.mode, &self.operands, cpu),
            Operation::Bpl => instructions::bpl(self.mode, &self.operands, cpu),
            Operation::Bmi => instructions::bmi(self.mode, &self.operands, cpu),

            // 0x07?? - Stack Instructions
            Operation::Psh => instructions::psh(self.mode, &self.operands, cpu),
            Operation::Pll => instructions::pll(self.mode, &mut self.operands, cpu),
            Operation::Php => instructions::php(self.mode, &self.operands, cpu),
            Operation::Plp => instructions::plp(self.mode, &self.operands, cpu)
        }
    }
}
