use crate::core::instruction_mode::InstructionMode;
use crate::core::interrupt::Interrupt;
use crate::core::memory_cell::MemoryCell;
use crate::core::operand::Operand;
use crate::cpu::CPU;
use crate::InstructionResult;

pub fn add(mode: InstructionMode, operand: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Implied = mode {
        return Err(Interrupt::IllegalInstruction);
    }

    let number1 = operand[0].read()?;
    let number2 = operand[1].read()?;

    let sum = number1.overflowing_add(number2);
    cpu.status_register.overflow = sum.1;
    cpu.registers.a = sum.0;

    Ok(())
}
