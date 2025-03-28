use crate::core::MemoryCell;
use crate::core::Operand;
use crate::CPU;
use crate::InstructionResult;

pub fn inc(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let initial = operands[0].read_word(cpu)?;
    let result = initial.overflowing_add(1);

    cpu.status_register.carry = result.1;
    cpu.status_register.overflow = initial == 2_147_483_646;
    operands[0].write_word(cpu, result.0)?;

    Ok(())
}

pub fn dec(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let initial = operands[0].read_word(cpu)?;
    let result = initial.overflowing_sub(1);

    cpu.status_register.carry = result.1;
    cpu.status_register.overflow = initial == 2_147_483_647;
    cpu.status_register.zero = result.0 == 0;
    operands[0].write_word(cpu, result.0)?;

    Ok(())
}
