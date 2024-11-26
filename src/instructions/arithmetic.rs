use crate::core::interrupt::Interrupt;
use crate::core::operand::Operand;
use crate::cpu::CPU;
use crate::InstructionResult;

pub fn add(_operands: &[Operand; 2], _cpu: &CPU) -> InstructionResult {
    Err(Interrupt::Failure)
}
