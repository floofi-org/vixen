use crate::core::MemoryCell;
use crate::core::Operand;
use crate::CPU;
use crate::InstructionResult;

pub fn mov(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let value = operands[1].read_word()?;
    operands[0].write_word(cpu, value)?;
    Ok(())
}

pub fn swp(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let value1 = operands[0].read_word()?;
    let value2 = operands[1].read_word()?;
    operands[1].write_word(cpu, value1)?;
    operands[0].write_word(cpu, value2)?;
    Ok(())
}

pub fn clr(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    operands[0].write_word(cpu, 0)?;
    cpu.status_register.zero = true;
    Ok(())
}

pub fn sec(_operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    cpu.status_register.carry = true;
    Ok(())
}

pub fn clc(_operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    cpu.status_register.carry = false;
    Ok(())
}

pub fn sei(_operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    cpu.status_register.interrupt_disable = true;
    Ok(())
}

pub fn cli(_operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    cpu.status_register.interrupt_disable = false;
    Ok(())
}

pub fn clv(_operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    cpu.status_register.overflow = false;
    Ok(())
}
