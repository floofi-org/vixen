use crate::core::instruction::mode::Addressing;
use crate::core::instruction::operation::Operation;
use crate::core::operand::Operand;
use crate::{instructions, InstructionResult};
use crate::cpu::CPU;

pub mod mode;
pub mod operation;

#[derive(Debug)]
pub struct Instruction {
    pub operation: Operation,
    pub mode: Addressing,
    pub operands: [Operand; 2]
}

impl Instruction {
    pub fn execute_unhandled(&mut self, cpu: &mut CPU) -> InstructionResult {
        match self.operation {
            // 0x01?? - Arithmetic and Algebric Instructions - 10/11 implemented
            Operation::Add => instructions::arithmetic::add(self.mode, &self.operands, cpu),
            Operation::Sub => instructions::arithmetic::sub(self.mode, &self.operands, cpu),
            Operation::Mul => instructions::arithmetic::mul(self.mode, &self.operands, cpu),
            Operation::Div => instructions::arithmetic::div(self.mode, &self.operands, cpu),
            Operation::Mod => instructions::arithmetic::mod_(self.mode, &self.operands, cpu),
            Operation::Sqt => instructions::arithmetic::sqt(self.mode, &self.operands, cpu),
            Operation::Cbt => instructions::arithmetic::cbt(self.mode, &self.operands, cpu),
            Operation::Sqr => instructions::arithmetic::sqr(self.mode, &self.operands, cpu),
            Operation::Cbe => instructions::arithmetic::cbe(self.mode, &self.operands, cpu),
            Operation::Min => instructions::arithmetic::min(self.mode, &self.operands, cpu),
            Operation::Max => instructions::arithmetic::max(self.mode, &self.operands, cpu),

            // 0x02?? - Logic Instructions
            Operation::And => instructions::logic::and(self.mode, &self.operands, cpu),
            Operation::Or => instructions::logic::or(self.mode, &self.operands, cpu),
            Operation::Xor => instructions::logic::xor(self.mode, &self.operands, cpu),
            Operation::Nor => instructions::logic::nor(self.mode, &self.operands, cpu),
            Operation::Nad => instructions::logic::nad(self.mode, &self.operands, cpu),
            Operation::Imp => instructions::logic::imp(self.mode, &self.operands, cpu),
            Operation::Not => instructions::logic::not(self.mode, &self.operands, cpu),
            Operation::Shl => instructions::logic::shl(self.mode, &mut self.operands, cpu),
            Operation::Shr => instructions::logic::shr(self.mode, &mut self.operands, cpu),
            Operation::Rol => instructions::logic::rol(self.mode, &mut self.operands, cpu),
            Operation::Ror => instructions::logic::ror(self.mode, &mut self.operands, cpu),

            // 0x03?? - Counting Instructions
            Operation::Inc => instructions::counting::inc(self.mode, &mut self.operands, cpu),
            Operation::Dec => instructions::counting::dec(self.mode, &mut self.operands, cpu),
            Operation::Ina => instructions::counting::ina(self.mode, &self.operands, cpu),
            Operation::Dea => instructions::counting::dea(self.mode, &self.operands, cpu),
            Operation::Inx => instructions::counting::inx(self.mode, &self.operands, cpu),
            Operation::Dex => instructions::counting::dex(self.mode, &self.operands, cpu),
            Operation::Iny => instructions::counting::iny(self.mode, &self.operands, cpu),
            Operation::Dey => instructions::counting::dey(self.mode, &self.operands, cpu),

            // 0x04?? - Comparison Instructions
            Operation::Cmp => instructions::comparison::cmp(self.mode, &self.operands, cpu),
            Operation::Cpx => instructions::comparison::cpx(self.mode, &self.operands, cpu),
            Operation::Cpy => instructions::comparison::cpy(self.mode, &self.operands, cpu),
            Operation::Lte => instructions::comparison::lte(self.mode, &self.operands, cpu),
            Operation::Gte => instructions::comparison::gte(self.mode, &self.operands, cpu),
            Operation::Srz => instructions::comparison::srz(self.mode, &mut self.operands, cpu),
            Operation::Src => instructions::comparison::src(self.mode, &mut self.operands, cpu),
            Operation::Sro => instructions::comparison::sro(self.mode, &mut self.operands, cpu),

            // 0x05?? - Data Movement Instructions
            Operation::Lda => instructions::data_movement::lda(self.mode, &mut self.operands, cpu),
            Operation::Ldx => instructions::data_movement::ldx(self.mode, &mut self.operands, cpu),
            Operation::Ldy => instructions::data_movement::ldy(self.mode, &mut self.operands, cpu),
            Operation::Ldz => instructions::data_movement::ldz(self.mode, &mut self.operands, cpu),
            Operation::Sta => instructions::data_movement::sta(self.mode, &mut self.operands, cpu),
            Operation::Stx => instructions::data_movement::stx(self.mode, &mut self.operands, cpu),
            Operation::Sty => instructions::data_movement::sty(self.mode, &mut self.operands, cpu),
            Operation::Mov => instructions::data_movement::mov(self.mode, &mut self.operands, cpu),
            Operation::Swp => instructions::data_movement::swp(self.mode, &mut self.operands, cpu),
            Operation::Clr => instructions::data_movement::clr(self.mode, &mut self.operands, cpu),

            // 0x06?? - Control Flow Instructions
            Operation::Jmp => instructions::control_flow::jmp(self.mode, &self.operands, cpu),
            Operation::Jsr => instructions::control_flow::jsr(self.mode, &self.operands, cpu),
            Operation::Ret => instructions::control_flow::ret(self.mode, &self.operands, cpu),
            Operation::Beq => instructions::control_flow::beq(self.mode, &self.operands, cpu),
            Operation::Bne => instructions::control_flow::bne(self.mode, &self.operands, cpu),
            Operation::Bec => instructions::control_flow::bec(self.mode, &self.operands, cpu),
            Operation::Bnc => instructions::control_flow::bnc(self.mode, &self.operands, cpu),
            Operation::Beo => instructions::control_flow::beo(self.mode, &self.operands, cpu),
            Operation::Bno => instructions::control_flow::bno(self.mode, &self.operands, cpu),
            Operation::Int => instructions::control_flow::int(self.mode, &self.operands, cpu),
            Operation::Irt => instructions::control_flow::irt(self.mode, &self.operands, cpu),
            Operation::Nop => instructions::control_flow::nop(self.mode, &self.operands, cpu),
            Operation::Jam => instructions::control_flow::jam(self.mode, &self.operands, cpu),

            // 0x07?? - Stack Instructions
            Operation::Pha => instructions::stack::pha(self.mode, &self.operands, cpu),
            Operation::Pla => instructions::stack::pla(self.mode, &self.operands, cpu),
            Operation::Phx => instructions::stack::phx(self.mode, &self.operands, cpu),
            Operation::Plx => instructions::stack::plx(self.mode, &self.operands, cpu),
            Operation::Phy => instructions::stack::phy(self.mode, &self.operands, cpu),
            Operation::Ply => instructions::stack::ply(self.mode, &self.operands, cpu),
            Operation::Psh => instructions::stack::psh(self.mode, &self.operands, cpu),
            Operation::Pll => instructions::stack::pll(self.mode, &mut self.operands, cpu),

            // 0x08?? - MOS 6502 Compatibility Extensions
            Operation::Bpl => instructions::mos6502::bpl(self.mode, &self.operands, cpu),
            Operation::Bmi => instructions::mos6502::bmi(self.mode, &self.operands, cpu),
            Operation::Adc => instructions::mos6502::adc(self.mode, &self.operands, cpu),
            Operation::Sbc => instructions::mos6502::sbc(self.mode, &self.operands, cpu),
            Operation::Bit => instructions::mos6502::bit(self.mode, &self.operands, cpu),
            Operation::Asr => instructions::mos6502::asr(self.mode, &mut self.operands, cpu),
            Operation::Sec => instructions::mos6502::sec(self.mode, &self.operands, cpu),
            Operation::Clc => instructions::mos6502::clc(self.mode, &self.operands, cpu),
            Operation::Sei => instructions::mos6502::sei(self.mode, &self.operands, cpu),
            Operation::Cli => instructions::mos6502::cli(self.mode, &self.operands, cpu),
            Operation::Clv => instructions::mos6502::clv(self.mode, &self.operands, cpu),
            Operation::Php => instructions::mos6502::php(self.mode, &self.operands, cpu),
            Operation::Plp => instructions::mos6502::plp(self.mode, &self.operands, cpu)
        }
    }
}
