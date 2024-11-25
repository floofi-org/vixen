use crate::core::interrupt::Interrupt;
use crate::core::operand::Operand;
use crate::CPUInstructionResult;

pub fn add(_operands: &[Operand; 2]) -> CPUInstructionResult {
    Err(Interrupt::Failure)
}