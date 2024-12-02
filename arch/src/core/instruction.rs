use crate::core::instruction::instruction_mode::InstructionMode;
use crate::core::instruction::instruction_operation::InstructionOperation;
use crate::core::operand::Operand;
use crate::{instructions, InstructionResult};
use crate::cpu::CPU;

pub mod instruction_mode;
pub mod instruction_operation;

#[derive(Debug)]
pub struct Instruction {
    pub operation: InstructionOperation,
    pub mode: InstructionMode,
    pub operands: [Operand; 2]
}

impl Instruction {
    pub fn execute_unhandled(&mut self, cpu: &mut CPU) -> InstructionResult {
        match self.operation {
            // 0x01?? - Arithmetic and Algebric Instructions - 10/11 implemented
            InstructionOperation::Add => instructions::arithmetic::add(self.mode, &self.operands, cpu),
            InstructionOperation::Sub => instructions::arithmetic::sub(self.mode, &self.operands, cpu),
            InstructionOperation::Mul => instructions::arithmetic::mul(self.mode, &self.operands, cpu),
            InstructionOperation::Div => instructions::arithmetic::div(self.mode, &self.operands, cpu),
            InstructionOperation::Mod => instructions::arithmetic::mod_(self.mode, &self.operands, cpu),
            InstructionOperation::Sqt => instructions::arithmetic::sqt(self.mode, &self.operands, cpu),
            InstructionOperation::Cbt => instructions::arithmetic::cbt(self.mode, &self.operands, cpu),
            InstructionOperation::Sqr => instructions::arithmetic::sqr(self.mode, &self.operands, cpu),
            InstructionOperation::Cbe => instructions::arithmetic::cbe(self.mode, &self.operands, cpu),
            InstructionOperation::Min => instructions::arithmetic::min(self.mode, &self.operands, cpu),
            InstructionOperation::Max => instructions::arithmetic::max(self.mode, &self.operands, cpu),

            // 0x02?? - Logic Instructions
            InstructionOperation::And => instructions::logic::and(self.mode, &self.operands, cpu),
            InstructionOperation::Or => instructions::logic::or(self.mode, &self.operands, cpu),
            InstructionOperation::Xor => instructions::logic::xor(self.mode, &self.operands, cpu),
            InstructionOperation::Nor => instructions::logic::nor(self.mode, &self.operands, cpu),
            InstructionOperation::Nad => instructions::logic::nad(self.mode, &self.operands, cpu),
            InstructionOperation::Imp => instructions::logic::imp(self.mode, &self.operands, cpu),
            InstructionOperation::Not => instructions::logic::not(self.mode, &self.operands, cpu),
            InstructionOperation::Shl => instructions::logic::shl(self.mode, &mut self.operands, cpu),
            InstructionOperation::Shr => instructions::logic::shr(self.mode, &mut self.operands, cpu),
            InstructionOperation::Rol => instructions::logic::rol(self.mode, &mut self.operands, cpu),
            InstructionOperation::Ror => instructions::logic::ror(self.mode, &mut self.operands, cpu),

            // 0x03?? - Counting Instructions
            InstructionOperation::Inc => instructions::counting::inc(self.mode, &mut self.operands, cpu),
            InstructionOperation::Dec => instructions::counting::dec(self.mode, &mut self.operands, cpu),
            InstructionOperation::Ina => instructions::counting::ina(self.mode, &self.operands, cpu),
            InstructionOperation::Dea => instructions::counting::dea(self.mode, &self.operands, cpu),
            InstructionOperation::Inx => instructions::counting::inx(self.mode, &self.operands, cpu),
            InstructionOperation::Dex => instructions::counting::dex(self.mode, &self.operands, cpu),
            InstructionOperation::Iny => instructions::counting::iny(self.mode, &self.operands, cpu),
            InstructionOperation::Dey => instructions::counting::dey(self.mode, &self.operands, cpu),

            // 0x04?? - Comparison Instructions
            InstructionOperation::Cmp => instructions::comparison::srz(self.mode, &mut self.operands, cpu),
            InstructionOperation::Cpx => instructions::comparison::srz(self.mode, &mut self.operands, cpu),
            InstructionOperation::Cpy => instructions::comparison::srz(self.mode, &mut self.operands, cpu),
            InstructionOperation::Lte => instructions::comparison::srz(self.mode, &mut self.operands, cpu),
            InstructionOperation::Gte => instructions::comparison::srz(self.mode, &mut self.operands, cpu),
            InstructionOperation::Srz => instructions::comparison::srz(self.mode, &mut self.operands, cpu),
            InstructionOperation::Src => instructions::comparison::src(self.mode, &mut self.operands, cpu),
            InstructionOperation::Sro => instructions::comparison::sro(self.mode, &mut self.operands, cpu),

            // 0x05?? - Data Movement Instructions
            InstructionOperation::Lda => instructions::data_movement::lda(self.mode, &mut self.operands, cpu),
            InstructionOperation::Ldx => instructions::data_movement::ldx(self.mode, &mut self.operands, cpu),
            InstructionOperation::Ldy => instructions::data_movement::ldy(self.mode, &mut self.operands, cpu),
            InstructionOperation::Ldz => instructions::data_movement::ldz(self.mode, &mut self.operands, cpu),
            InstructionOperation::Sta => instructions::data_movement::sta(self.mode, &mut self.operands, cpu),
            InstructionOperation::Stx => instructions::data_movement::stx(self.mode, &mut self.operands, cpu),
            InstructionOperation::Sty => instructions::data_movement::sty(self.mode, &mut self.operands, cpu),
            InstructionOperation::Mov => instructions::data_movement::mov(self.mode, &mut self.operands, cpu),
            InstructionOperation::Swp => instructions::data_movement::swp(self.mode, &mut self.operands, cpu),
            InstructionOperation::Clr => instructions::data_movement::clr(self.mode, &mut self.operands, cpu),

            // 0x06?? - Control Flow Instructions
            InstructionOperation::Jmp => instructions::control_flow::jmp(self.mode, &self.operands, cpu),
            InstructionOperation::Jsr => instructions::control_flow::jsr(self.mode, &self.operands, cpu),
            InstructionOperation::Ret => instructions::control_flow::ret(self.mode, &self.operands, cpu),
            InstructionOperation::Beq => instructions::control_flow::beq(self.mode, &self.operands, cpu),
            InstructionOperation::Bne => instructions::control_flow::bne(self.mode, &self.operands, cpu),
            InstructionOperation::Bec => instructions::control_flow::bec(self.mode, &self.operands, cpu),
            InstructionOperation::Bnc => instructions::control_flow::bnc(self.mode, &self.operands, cpu),
            InstructionOperation::Beo => instructions::control_flow::beo(self.mode, &self.operands, cpu),
            InstructionOperation::Bno => instructions::control_flow::bno(self.mode, &self.operands, cpu),
            InstructionOperation::Int => instructions::control_flow::int(self.mode, &self.operands, cpu),
            InstructionOperation::Irt => instructions::control_flow::irt(self.mode, &self.operands, cpu),
            InstructionOperation::Nop => instructions::control_flow::nop(self.mode, &self.operands, cpu),
            InstructionOperation::Jam => instructions::control_flow::jam(self.mode, &self.operands, cpu),

            // 0x07?? - Stack Instructions
            InstructionOperation::Pha => instructions::stack::pha(self.mode, &self.operands, cpu),
            InstructionOperation::Pla => instructions::stack::pla(self.mode, &self.operands, cpu),
            InstructionOperation::Phx => instructions::stack::phx(self.mode, &self.operands, cpu),
            InstructionOperation::Plx => instructions::stack::plx(self.mode, &self.operands, cpu),
            InstructionOperation::Phy => instructions::stack::phy(self.mode, &self.operands, cpu),
            InstructionOperation::Ply => instructions::stack::ply(self.mode, &self.operands, cpu),
            InstructionOperation::Psh => instructions::stack::psh(self.mode, &self.operands, cpu),
            InstructionOperation::Pll => instructions::stack::pll(self.mode, &mut self.operands, cpu),

            // 0x08?? - MOS 6502 Compatibility Extensions
            InstructionOperation::Bpl => instructions::mos6502::bpl(self.mode, &self.operands, cpu),
            InstructionOperation::Bmi => instructions::mos6502::bmi(self.mode, &self.operands, cpu),
            InstructionOperation::Adc => instructions::mos6502::adc(self.mode, &self.operands, cpu),
            InstructionOperation::Sbc => instructions::mos6502::sbc(self.mode, &self.operands, cpu),
            InstructionOperation::Bit => instructions::mos6502::bit(self.mode, &self.operands, cpu),
            InstructionOperation::Asr => instructions::mos6502::asr(self.mode, &mut self.operands, cpu),
            InstructionOperation::Sec => instructions::mos6502::sec(self.mode, &self.operands, cpu),
            InstructionOperation::Clc => instructions::mos6502::clc(self.mode, &self.operands, cpu),
            InstructionOperation::Sei => instructions::mos6502::sei(self.mode, &self.operands, cpu),
            InstructionOperation::Cli => instructions::mos6502::cli(self.mode, &self.operands, cpu),
            InstructionOperation::Clv => instructions::mos6502::clv(self.mode, &self.operands, cpu),
            InstructionOperation::Php => instructions::mos6502::php(self.mode, &self.operands, cpu),
            InstructionOperation::Plp => instructions::mos6502::plp(self.mode, &self.operands, cpu)
        }
    }
}
