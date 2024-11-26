use crate::core::instruction_mode::InstructionMode;
use crate::core::interrupt::Interrupt;
use crate::core::memory_cell::MemoryCell;
use crate::core::operand::Operand;
use crate::cpu::CPU;
use crate::InstructionResult;

pub fn mov(mode: InstructionMode, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate = mode {
        return Err(Interrupt::IllegalInstruction);
    }
    
    let value = operands[0].read()?;
    operands[1].write(cpu, value)?;
    operands[0].write(cpu, 0)?;

    Ok(())
}
