use crate::core::MemoryCell;
use crate::core::Operand;
use crate::CPU;
use crate::InstructionResult;

pub fn cmp(operand: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let value1 = cpu.registers.r0;
    let value2 = operand[0].read_word()?;

    cpu.status_register.zero = value1 == value2;
    cpu.status_register.carry = value1 >= value2;
    cpu.status_register.negative = value1 < value2;

    Ok(())
}

pub fn lte(operand: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let value1 = cpu.registers.r0;
    let value2 = operand[0].read_word()?;

    cpu.status_register.zero = value1 <= value2;
    cpu.status_register.carry = value1 > value2;
    cpu.status_register.negative = value1 < value2;

    Ok(())
}

pub fn gte(operand: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let value1 = cpu.registers.r0;
    let value2 = operand[0].read_word()?;

    cpu.status_register.zero = value1 >= value2;
    cpu.status_register.carry = value1 < value2;
    cpu.status_register.negative = value1 < value2;

    Ok(())
}

pub fn srz(operand: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    operand[0].write_word(cpu, u32::from(cpu.status_register.zero))?;
    Ok(())
}

pub fn src(operand: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    operand[0].write_word(cpu, u32::from(cpu.status_register.carry))?;
    Ok(())
}

pub fn sro(operand: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    operand[0].write_word(cpu, u32::from(cpu.status_register.overflow))?;
    Ok(())
}
