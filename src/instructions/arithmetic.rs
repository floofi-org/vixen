use crate::core::interrupt::Interrupt;
use crate::core::operand::Operand;
use crate::cpu::CPU;
use crate::CPUInstructionResult;

pub fn add(_operands: &[Operand; 2], _cpu: &CPU) -> CPUInstructionResult {
    Err(Interrupt::Failure)
}