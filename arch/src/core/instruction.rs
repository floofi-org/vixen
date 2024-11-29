use crate::core::instruction::instruction_mode::InstructionMode;
use crate::core::instruction::instruction_operation::InstructionOperation;
use crate::core::operand::Operand;
use crate::{instructions, InstructionResult};
use crate::core::interrupt::Interrupt;
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
            // 0x01?? - Arithmetic and Algebric Instructions - 4/11 implemented
            InstructionOperation::Add => instructions::arithmetic::add(self.mode, &self.operands, cpu),
            InstructionOperation::Sub => instructions::arithmetic::sub(self.mode, &self.operands, cpu),
            InstructionOperation::Min => instructions::arithmetic::min(self.mode, &self.operands, cpu),
            InstructionOperation::Max => instructions::arithmetic::max(self.mode, &self.operands, cpu),

            // 0x02?? - Logic Instructions - Complete
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

            // 0x03?? - Counting Instructions - Complete
            InstructionOperation::Inc => instructions::counting::inc(self.mode, &mut self.operands, cpu),
            InstructionOperation::Dec => instructions::counting::dec(self.mode, &mut self.operands, cpu),
            InstructionOperation::Ina => instructions::counting::ina(self.mode, &self.operands, cpu),
            InstructionOperation::Dea => instructions::counting::dea(self.mode, &self.operands, cpu),
            InstructionOperation::Inx => instructions::counting::inx(self.mode, &self.operands, cpu),
            InstructionOperation::Dex => instructions::counting::dex(self.mode, &self.operands, cpu),
            InstructionOperation::Iny => instructions::counting::iny(self.mode, &self.operands, cpu),
            InstructionOperation::Dey => instructions::counting::dey(self.mode, &self.operands, cpu),

            // 0x04?? - Comparison Instructions - 0/8 implemented
            // - nothing for now -

            // 0x05?? - Data Movement Instructions - 1/10 implemented
            InstructionOperation::Mov => instructions::data_movement::mov(self.mode, &mut self.operands, cpu),

            // 0x06?? - Control Flow Instructions - Complete
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

            // 0x07?? - Stack Instructions - 0/8 implemented
            // - nothing for now -

            // 0x08?? - MOS 6502 Compatibility Extensions - 0/16 implemented
            // - nothing for now -

            // If an instruction isn't implemented yet
            _ => Err(Interrupt::Failure),
        }
    }
}
