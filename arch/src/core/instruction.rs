use crate::core::instruction::instruction_mode::InstructionMode;
use crate::core::instruction::instruction_operation::InstructionOperation;
use crate::core::operand::Operand;
use crate::{instructions, InstructionResult};
use crate::core::interrupt::Interrupt;
use crate::cpu::CPU;
use crate::cpu::stack::Stack;

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
            InstructionOperation::Add => instructions::arithmetic::add(self.mode, &self.operands, cpu),
            InstructionOperation::Mov => instructions::data_movement::mov(self.mode, &mut self.operands, cpu),
            InstructionOperation::Int => instructions::control_flow::int(self.mode, &self.operands, cpu),
            _ => Err(Interrupt::Failure),
        }
    }
}
