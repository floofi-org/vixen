use crate::core::instruction::instruction_mode::InstructionMode;
use crate::core::interrupt::Interrupt;
use crate::core::memory_cell::MemoryCell;
use crate::core::operand::Operand;
use crate::cpu::CPU;
use crate::InstructionResult;

pub fn mov(mode: InstructionMode, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Implied | InstructionMode::ZeroPage | InstructionMode::Absolute = mode {
        let value = operands[0].read_word()?;
        operands[1].write_word(cpu, value)?;
        operands[0].write_word(cpu, 0)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}
