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
            _ => todo!("Instruction is not implemented"),
        }
    }
}
